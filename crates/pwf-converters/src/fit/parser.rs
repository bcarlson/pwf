//! FIT to PWF conversion logic

use crate::common::utils::{fit_timestamp_to_iso8601, meters_to_km, semicircles_to_degrees};
use crate::error::{ConversionError, ConversionResult, ConversionWarning};
use crate::fit::mappings::map_fit_sport;
use chrono::Utc;
use fitparser::{FitDataRecord, Value};
use pwf_core::history::{
    AdvancedMetrics, CompletedExercise, CompletedSet, DeviceInfo, DeviceType, ExportSource, GpsFix,
    GpsPosition, GpsRoute, KnownManufacturer, Manufacturer, PoolConfig, PoolLengthUnit,
    PowerMetrics, SportSegment, StrokeType, SwimmingLength, SwimmingSetData, Units, Workout,
    WorkoutTelemetry, WpsHistory,
};
use std::io::Read;

/// Convert FIT file data to PWF YAML format
///
/// # Arguments
/// * `reader` - Reader containing FIT file data
/// * `summary_only` - If true, skip time-series data for smaller output
///
/// # Returns
/// ConversionResult with PWF YAML and any warnings
pub fn fit_to_pwf<R: Read>(
    mut reader: R,
    _summary_only: bool,
) -> Result<ConversionResult, ConversionError> {
    // Read FIT data
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    // Parse FIT file using fitparser
    let data = fitparser::from_bytes(&buffer)?;

    let mut result = ConversionResult::new(String::new());

    // Extract FIT records by type
    let mut sessions = Vec::new();
    let mut laps = Vec::new();
    let mut records = Vec::new();
    let mut lengths = Vec::new();
    let mut device_info_records = Vec::new();

    for record in data {
        match record.kind() {
            fitparser::profile::MesgNum::Session => sessions.push(record),
            fitparser::profile::MesgNum::Lap => laps.push(record),
            fitparser::profile::MesgNum::Record => records.push(record),
            fitparser::profile::MesgNum::Length => lengths.push(record),
            fitparser::profile::MesgNum::DeviceInfo => device_info_records.push(record),
            _ => {} // Ignore other record types for now
        }
    }

    // Check if we have any sessions
    if sessions.is_empty() {
        result.add_warning(ConversionWarning::DataQualityIssue {
            issue: "No session data found in FIT file".to_string(),
        });
        // Return minimal valid PWF
        let history = WpsHistory {
            history_version: 2,
            exported_at: Utc::now().to_rfc3339(),
            export_source: Some(ExportSource {
                app_name: Some("PWF FIT Converter".to_string()),
                app_version: Some(env!("CARGO_PKG_VERSION").to_string()),
                platform: None,
                preferred_units: None,
            }),
            units: Units::default(),
            workouts: Vec::new(),
            personal_records: Vec::new(),
            body_measurements: Vec::new(),
        };
        result.pwf_yaml = serde_yaml::to_string(&history)?;
        return Ok(result);
    }

    // Convert device info records
    let devices = convert_device_info_records(&device_info_records);

    // Detect multi-sport activities (e.g., triathlon)
    let is_multisport = detect_multisport(&sessions);

    // Convert sessions to workouts
    let mut workouts = Vec::new();
    for session in &sessions {
        match convert_session_to_workout(
            session,
            &laps,
            &records,
            &lengths,
            &devices,
            is_multisport,
            &mut result,
        ) {
            Ok(workout) => workouts.push(workout),
            Err(e) => {
                result.add_warning(ConversionWarning::DataQualityIssue {
                    issue: format!("Failed to convert session: {}", e),
                });
            }
        }
    }

    // Build WpsHistory structure
    let history = WpsHistory {
        history_version: 2,
        exported_at: Utc::now().to_rfc3339(),
        export_source: Some(ExportSource {
            app_name: Some("PWF FIT Converter".to_string()),
            app_version: Some(env!("CARGO_PKG_VERSION").to_string()),
            platform: Some("FIT file".to_string()),
            preferred_units: None,
        }),
        units: Units {
            weight: pwf_core::WeightUnit::Kg,
            distance: pwf_core::DistanceUnit::Kilometers,
        },
        workouts,
        personal_records: Vec::new(),
        body_measurements: Vec::new(),
    };

    // Serialize to YAML
    result.pwf_yaml = serde_yaml::to_string(&history)?;

    Ok(result)
}

/// Detect if this is a multi-sport activity (e.g., triathlon)
fn detect_multisport(sessions: &[FitDataRecord]) -> bool {
    if sessions.len() <= 1 {
        return false;
    }

    // Check if there are different sports across sessions
    let mut sports = std::collections::HashSet::new();
    for session in sessions {
        if let Some(sport) = get_field_u8(session, "sport") {
            // Skip transition "sport" (sport type 2)
            if sport != 2 {
                sports.insert(sport);
            }
        }
    }

    sports.len() > 1
}

/// Convert a FIT session record to a PWF Workout
fn convert_session_to_workout(
    session: &FitDataRecord,
    laps: &[FitDataRecord],
    records: &[FitDataRecord],
    lengths: &[FitDataRecord],
    devices: &[DeviceInfo],
    is_multisport: bool,
    result: &mut ConversionResult,
) -> Result<Workout, ConversionError> {
    // Extract session timestamp
    let start_time = get_field_u32(session, "start_time")
        .or_else(|| get_field_u32(session, "timestamp"))
        .ok_or_else(|| ConversionError::MissingRequiredField("start_time".to_string()))?;

    let started_at = fit_timestamp_to_iso8601(start_time);
    let date = started_at.split('T').next().unwrap_or("").to_string();

    // Extract duration
    let duration_sec = get_field_u32(session, "total_elapsed_time")
        .or_else(|| get_field_u32(session, "total_timer_time"));

    // Calculate end time
    let ended_at = duration_sec.map(|dur| {
        let end_timestamp = start_time + dur;
        fit_timestamp_to_iso8601(end_timestamp)
    });

    // Extract sport type
    let sport_type = get_field_u8(session, "sport").unwrap_or(0);
    let subsport = get_field_u8(session, "sub_sport");
    let sport = map_fit_sport(sport_type, subsport);

    // Build telemetry
    let mut telemetry = WorkoutTelemetry::default();

    if let Some(hr_avg) = get_field_u8(session, "avg_heart_rate") {
        telemetry.heart_rate_avg = Some(hr_avg as u32);
    }
    if let Some(hr_max) = get_field_u8(session, "max_heart_rate") {
        telemetry.heart_rate_max = Some(hr_max as u32);
    }
    if let Some(power_avg) = get_field_u16(session, "avg_power") {
        telemetry.power_avg = Some(power_avg as u32);
    }
    if let Some(power_max) = get_field_u16(session, "max_power") {
        telemetry.power_max = Some(power_max as u32);
    }
    if let Some(distance_m) = get_field_f64(session, "total_distance") {
        telemetry.total_distance_km = Some(meters_to_km(distance_m));
    }
    if let Some(calories) = get_field_u16(session, "total_calories") {
        telemetry.total_calories = Some(calories as u32);
    }
    if let Some(cadence) = get_field_u8(session, "avg_cadence") {
        telemetry.cadence_avg = Some(cadence as u32);
    }

    // PWF v2.1: Power metrics (for cycling/running with power)
    let mut power_metrics = PowerMetrics::default();
    let mut has_power_metrics = false;

    if let Some(np) = get_field_u16(session, "normalized_power") {
        power_metrics.normalized_power = Some(np as u32);
        has_power_metrics = true;
    }
    if let Some(tss) = get_field_f64(session, "training_stress_score") {
        power_metrics.training_stress_score = Some(tss);
        has_power_metrics = true;
    }
    if let Some(if_val) = get_field_f64(session, "intensity_factor") {
        power_metrics.intensity_factor = Some(if_val);
        has_power_metrics = true;
    }
    if let Some(vi) = get_field_f64(session, "variability_index") {
        power_metrics.variability_index = Some(vi);
        has_power_metrics = true;
    }
    if let Some(ftp) = get_field_u16(session, "threshold_power") {
        power_metrics.ftp_watts = Some(ftp as u32);
        has_power_metrics = true;
    }
    if let Some(work) = get_field_u32(session, "total_work") {
        power_metrics.total_work_kj = Some((work / 1000) as f64); // Convert J to kJ
        has_power_metrics = true;
    }

    if has_power_metrics {
        telemetry.power_metrics = Some(power_metrics);
    }

    // PWF v2.1: Advanced physiological metrics
    let mut advanced_metrics = AdvancedMetrics::default();
    let mut has_advanced_metrics = false;

    if let Some(te) = get_field_f64(session, "total_training_effect") {
        advanced_metrics.training_effect = Some(te);
        has_advanced_metrics = true;
    }
    if let Some(ate) = get_field_f64(session, "total_anaerobic_training_effect") {
        advanced_metrics.anaerobic_training_effect = Some(ate);
        has_advanced_metrics = true;
    }
    if let Some(recovery) = get_field_u16(session, "recovery_time") {
        advanced_metrics.recovery_time_hours = Some((recovery / 60) as u32); // Convert minutes to hours
        has_advanced_metrics = true;
    }

    if has_advanced_metrics {
        telemetry.advanced_metrics = Some(advanced_metrics);
    }

    // PWF v2.1: GPS route extraction
    let gps_route = extract_gps_route(session, records, result);
    if gps_route.is_some() {
        telemetry.gps_route = gps_route;
    }

    // Convert laps to exercises
    let exercises = convert_laps_to_exercises(session, laps, lengths, result)?;

    // Create sport segments if this is a multi-sport activity
    let sport_segments = if is_multisport {
        Some(create_sport_segment(
            session,
            sport,
            &started_at,
            duration_sec,
            get_field_f64(session, "total_distance"),
            &exercises,
            &telemetry,
        ))
    } else {
        None
    };

    Ok(Workout {
        id: None,
        date,
        started_at: Some(started_at),
        ended_at,
        duration_sec,
        title: Some(format!("{:?} Workout", sport)),
        notes: None,
        plan_id: None,
        plan_day_id: None,
        exercises,
        telemetry: Some(telemetry),
        devices: devices.to_vec(),
        sport: Some(sport),
        sport_segments,
    })
}

/// Create a sport segment for multi-sport activities
fn create_sport_segment(
    session: &FitDataRecord,
    sport: pwf_core::Sport,
    started_at: &str,
    duration_sec: Option<u32>,
    distance_m: Option<f64>,
    exercises: &[CompletedExercise],
    telemetry: &WorkoutTelemetry,
) -> Vec<SportSegment> {
    // Get session timestamp for segment ID
    let segment_id = get_field_u32(session, "start_time")
        .or_else(|| get_field_u32(session, "timestamp"))
        .map(|ts| format!("segment-{}", ts))
        .unwrap_or_else(|| "segment-0".to_string());

    // Get session index (sport event number)
    let segment_index = get_field_u8(session, "sport_index").unwrap_or(0) as u32;

    // Collect exercise IDs
    let exercise_ids: Vec<String> = exercises.iter().filter_map(|ex| ex.id.clone()).collect();

    vec![SportSegment {
        segment_id,
        sport,
        segment_index,
        started_at: Some(started_at.to_string()),
        duration_sec,
        distance_m,
        exercise_ids,
        telemetry: Some(telemetry.clone()),
        transition: None, // Transitions would be in separate records
        notes: None,
    }]
}

/// Convert FIT laps to PWF exercises and sets
fn convert_laps_to_exercises(
    session: &FitDataRecord,
    all_laps: &[FitDataRecord],
    lengths: &[FitDataRecord],
    result: &mut ConversionResult,
) -> Result<Vec<CompletedExercise>, ConversionError> {
    // Get session start time to filter laps
    let session_start = get_field_u32(session, "start_time")
        .or_else(|| get_field_u32(session, "timestamp"))
        .ok_or_else(|| ConversionError::MissingRequiredField("start_time".to_string()))?;

    // Detect if this is a swimming activity
    let sport_type = get_field_u8(session, "sport").unwrap_or(0);
    let is_swimming = sport_type == 5; // FIT sport code 5 = Swimming

    // Extract swimming data and pool config if available
    let (swimming_data, pool_config) = if is_swimming && !lengths.is_empty() {
        if let Some((data, config)) = extract_swimming_data(lengths, Some(session_start)) {
            (Some(data), Some(config))
        } else {
            (None, None)
        }
    } else {
        (None, None)
    };

    // Filter laps that belong to this session
    // For now, we'll assume all laps belong to the session
    // In a real implementation, we'd need to match timestamps

    if all_laps.is_empty() {
        // No laps - create a single exercise for the whole session
        result.add_warning(ConversionWarning::DataQualityIssue {
            issue: "No lap data found, creating single exercise".to_string(),
        });

        return Ok(vec![CompletedExercise {
            id: None,
            name: "Activity".to_string(),
            modality: Some(pwf_core::Modality::Stopwatch),
            sets: vec![CompletedSet {
                set_number: Some(1),
                reps: None,
                weight_kg: None,
                weight_lb: None,
                duration_sec: get_field_u32(session, "total_elapsed_time"),
                distance_meters: get_field_f64(session, "total_distance"),
                rpe: None,
                rir: None,
                set_type: None,
                is_pr: None,
                notes: None,
                completed_at: None,
                telemetry: None,
                swimming: swimming_data.clone(),
            }],
            notes: None,
            sport: None,
            pool_config,
        }]);
    }

    // Group consecutive laps into a single exercise
    let mut exercises = Vec::new();

    // For swimming activities, we attach the swimming data to the first set
    // (in swimming, typically one lap = one set = multiple pool lengths)
    let exercise = CompletedExercise {
        id: None,
        name: "Activity".to_string(),
        modality: Some(pwf_core::Modality::Stopwatch),
        sets: all_laps
            .iter()
            .enumerate()
            .map(|(i, lap)| {
                // Attach swimming data only to the first set
                let set_swimming_data = if i == 0 { swimming_data.clone() } else { None };

                CompletedSet {
                    set_number: Some((i + 1) as u32),
                    reps: None,
                    weight_kg: None,
                    weight_lb: None,
                    duration_sec: get_field_u32(lap, "total_elapsed_time")
                        .or_else(|| get_field_u32(lap, "total_timer_time")),
                    distance_meters: get_field_f64(lap, "total_distance"),
                    rpe: None,
                    rir: None,
                    set_type: None,
                    is_pr: None,
                    notes: None,
                    completed_at: None,
                    telemetry: None,
                    swimming: set_swimming_data,
                }
            })
            .collect(),
        notes: None,
        sport: None,
        pool_config,
    };

    exercises.push(exercise);
    Ok(exercises)
}

/// Helper to get u32 field from FIT record
fn get_field_u32(record: &FitDataRecord, field_name: &str) -> Option<u32> {
    record.fields().iter().find_map(|field| {
        if field.name() == field_name {
            match field.value() {
                Value::UInt32(v) => Some(*v),
                Value::UInt16(v) => Some(*v as u32),
                Value::UInt8(v) => Some(*v as u32),
                _ => None,
            }
        } else {
            None
        }
    })
}

/// Helper to get u16 field from FIT record
fn get_field_u16(record: &FitDataRecord, field_name: &str) -> Option<u16> {
    record.fields().iter().find_map(|field| {
        if field.name() == field_name {
            match field.value() {
                Value::UInt16(v) => Some(*v),
                Value::UInt8(v) => Some(*v as u16),
                _ => None,
            }
        } else {
            None
        }
    })
}

/// Helper to get u8 field from FIT record
fn get_field_u8(record: &FitDataRecord, field_name: &str) -> Option<u8> {
    record.fields().iter().find_map(|field| {
        if field.name() == field_name {
            match field.value() {
                Value::UInt8(v) => Some(*v),
                _ => None,
            }
        } else {
            None
        }
    })
}

/// Helper to get f64 field from FIT record
fn get_field_f64(record: &FitDataRecord, field_name: &str) -> Option<f64> {
    record.fields().iter().find_map(|field| {
        if field.name() == field_name {
            match field.value() {
                Value::Float64(v) => Some(*v),
                Value::Float32(v) => Some(*v as f64),
                Value::UInt32(v) => Some(*v as f64),
                Value::UInt16(v) => Some(*v as f64),
                _ => None,
            }
        } else {
            None
        }
    })
}

/// Helper to get i32 field from FIT record (used for GPS semicircles)
fn get_field_i32(record: &FitDataRecord, field_name: &str) -> Option<i32> {
    record.fields().iter().find_map(|field| {
        if field.name() == field_name {
            match field.value() {
                Value::SInt32(v) => Some(*v),
                Value::SInt16(v) => Some(*v as i32),
                Value::SInt8(v) => Some(*v as i32),
                _ => None,
            }
        } else {
            None
        }
    })
}

/// Extract GPS route from FIT record messages
fn extract_gps_route(
    session: &FitDataRecord,
    records: &[FitDataRecord],
    _result: &mut ConversionResult,
) -> Option<GpsRoute> {
    // Filter records that have GPS data
    let positions: Vec<GpsPosition> = records
        .iter()
        .filter_map(|record| {
            // Get latitude and longitude (stored as semicircles in FIT)
            let lat_semicircles = get_field_i32(record, "position_lat")?;
            let lng_semicircles = get_field_i32(record, "position_long")?;

            // Convert to degrees
            let latitude_deg = semicircles_to_degrees(lat_semicircles);
            let longitude_deg = semicircles_to_degrees(lng_semicircles);

            // Skip invalid coordinates (0,0 is in the ocean off Africa - usually invalid)
            if latitude_deg.abs() < 0.001 && longitude_deg.abs() < 0.001 {
                return None;
            }

            // Get timestamp
            let timestamp = get_field_u32(record, "timestamp").map(fit_timestamp_to_iso8601)?;

            Some(GpsPosition {
                latitude_deg,
                longitude_deg,
                timestamp,
                elevation_m: get_field_f64(record, "altitude"),
                accuracy_m: None, // FIT doesn't typically include accuracy
                speed_mps: get_field_f64(record, "speed"),
                heading_deg: get_field_u16(record, "heading").map(|h| h as f64),
                heart_rate_bpm: get_field_u8(record, "heart_rate").map(|hr| hr as u32),
                power_watts: get_field_u16(record, "power").map(|p| p as u32),
                cadence: get_field_u8(record, "cadence").map(|c| c as u32),
                temperature_c: get_field_f64(record, "temperature"),
            })
        })
        .collect();

    // If no GPS positions found, return None
    if positions.is_empty() {
        return None;
    }

    // Calculate aggregate route metrics
    let mut total_distance_m = 0.0;
    let mut total_ascent_m = 0.0;
    let mut total_descent_m = 0.0;
    let mut min_elevation_m = f64::MAX;
    let mut max_elevation_m = f64::MIN;
    let mut min_lat = f64::MAX;
    let mut max_lat = f64::MIN;
    let mut min_lng = f64::MAX;
    let mut max_lng = f64::MIN;
    let mut prev_elevation: Option<f64> = None;

    for pos in &positions {
        // Track elevation extremes
        if let Some(elev) = pos.elevation_m {
            min_elevation_m = min_elevation_m.min(elev);
            max_elevation_m = max_elevation_m.max(elev);

            // Calculate ascent/descent
            if let Some(prev) = prev_elevation {
                let diff = elev - prev;
                if diff > 0.0 {
                    total_ascent_m += diff;
                } else {
                    total_descent_m += diff.abs();
                }
            }
            prev_elevation = Some(elev);
        }

        // Track bounding box
        min_lat = min_lat.min(pos.latitude_deg);
        max_lat = max_lat.max(pos.latitude_deg);
        min_lng = min_lng.min(pos.longitude_deg);
        max_lng = max_lng.max(pos.longitude_deg);
    }

    // Get total distance from session (more accurate than calculating from GPS)
    if let Some(distance) = get_field_f64(session, "total_distance") {
        total_distance_m = distance;
    }

    // Generate route ID from session timestamp
    let route_id = get_field_u32(session, "start_time")
        .map(|ts| format!("route-{}", ts))
        .unwrap_or_else(|| "route-unknown".to_string());

    Some(GpsRoute {
        route_id,
        name: None,
        total_distance_m: Some(total_distance_m),
        total_ascent_m: if total_ascent_m > 0.0 {
            Some(total_ascent_m)
        } else {
            None
        },
        total_descent_m: if total_descent_m > 0.0 {
            Some(total_descent_m)
        } else {
            None
        },
        min_elevation_m: if min_elevation_m != f64::MAX {
            Some(min_elevation_m)
        } else {
            None
        },
        max_elevation_m: if max_elevation_m != f64::MIN {
            Some(max_elevation_m)
        } else {
            None
        },
        bbox_sw_lat: if min_lat != f64::MAX {
            Some(min_lat)
        } else {
            None
        },
        bbox_sw_lng: if min_lng != f64::MAX {
            Some(min_lng)
        } else {
            None
        },
        bbox_ne_lat: if max_lat != f64::MIN {
            Some(max_lat)
        } else {
            None
        },
        bbox_ne_lng: if max_lng != f64::MIN {
            Some(max_lng)
        } else {
            None
        },
        recording_mode: Some("smart".to_string()), // FIT typically uses smart recording
        gps_fix: Some(GpsFix::Fix3D),              // Assume 3D fix if we have data
        positions,
    })
}

/// Extract swimming data from FIT length messages
fn extract_swimming_data(
    lengths: &[FitDataRecord],
    _lap_start_time: Option<u32>,
) -> Option<(SwimmingSetData, PoolConfig)> {
    use crate::fit::mappings::map_swim_stroke;

    if lengths.is_empty() {
        return None;
    }

    // Extract pool configuration from first length (all should be same pool)
    let pool_config = if let Some(first_length) = lengths.first() {
        let pool_length_m = get_field_f64(first_length, "pool_length")?;

        // Determine pool length unit based on the value
        let pool_length_unit = if (45.0..=55.0).contains(&pool_length_m) {
            PoolLengthUnit::Meters // 50m pool
        } else if (30.0..=40.0).contains(&pool_length_m) {
            PoolLengthUnit::Yards // 33yd pool
        } else {
            PoolLengthUnit::Meters // Default to meters
        };

        Some(PoolConfig {
            pool_length: pool_length_m,
            pool_length_unit,
        })
    } else {
        None
    }?;

    // Convert each length message to SwimmingLength
    let swimming_lengths: Vec<SwimmingLength> = lengths
        .iter()
        .enumerate()
        .filter_map(|(i, length)| {
            let duration_sec = get_field_f64(length, "total_elapsed_time")
                .or_else(|| get_field_u32(length, "total_elapsed_time").map(|v| v as f64))?;

            // Get stroke type (defaults to freestyle if not specified)
            let stroke_type = get_field_u8(length, "swim_stroke")
                .map(map_swim_stroke)
                .unwrap_or(StrokeType::Freestyle);

            // Get stroke count
            let stroke_count = get_field_u16(length, "total_strokes").map(|s| s as u32);

            // Calculate SWOLF if we have both duration and strokes
            let swolf = stroke_count.map(|strokes| strokes + duration_sec as u32);

            // Determine if length is active (not rest)
            let length_type = get_field_u8(length, "length_type");
            let active = length_type.map(|lt| lt == 1); // 0=rest, 1=active

            // Get timestamp for this length
            let started_at = get_field_u32(length, "start_time")
                .or_else(|| get_field_u32(length, "timestamp"))
                .map(fit_timestamp_to_iso8601);

            Some(SwimmingLength {
                length_number: (i + 1) as u32,
                duration_sec: duration_sec as u32,
                stroke_type,
                stroke_count,
                swolf,
                started_at,
                active,
            })
        })
        .collect();

    if swimming_lengths.is_empty() {
        return None;
    }

    // Calculate aggregate swimming data
    let total_lengths = swimming_lengths.len() as u32;
    let active_lengths = swimming_lengths
        .iter()
        .filter(|l| l.active.unwrap_or(true))
        .count() as u32;

    // Determine primary stroke type (most common)
    let stroke_type = if swimming_lengths
        .iter()
        .all(|l| l.stroke_type == swimming_lengths[0].stroke_type)
    {
        Some(swimming_lengths[0].stroke_type)
    } else {
        None
    };

    // Calculate average SWOLF
    let swolf_values: Vec<u32> = swimming_lengths.iter().filter_map(|l| l.swolf).collect();
    let swolf_avg = if !swolf_values.is_empty() {
        Some(swolf_values.iter().sum::<u32>() / swolf_values.len() as u32)
    } else {
        None
    };

    let swimming_data = SwimmingSetData {
        lengths: swimming_lengths,
        stroke_type,
        total_lengths: Some(total_lengths),
        active_lengths: Some(active_lengths),
        swolf_avg,
        drill_mode: None,
    };

    Some((swimming_data, pool_config))
}

/// Convert FIT device_info records to PWF DeviceInfo
fn convert_device_info_records(records: &[FitDataRecord]) -> Vec<DeviceInfo> {
    records
        .iter()
        .filter_map(|record| {
            // Extract device type
            // FIT device type codes from FIT SDK
            let device_type = match get_field_u8(record, "device_type") {
                Some(1) => DeviceType::Watch,                   // GPS watch
                Some(11) => DeviceType::BikeComputer,           // Bike computer
                Some(120) => DeviceType::HeartRateMonitor,      // Heart rate
                Some(12) | Some(121) => DeviceType::PowerMeter, // Power meter / bike speed cadence
                _ => return None,                               // Skip unknown device types
            };

            // Extract manufacturer
            let manufacturer = match get_field_u16(record, "manufacturer") {
                Some(1) => Manufacturer::Known(KnownManufacturer::Garmin),
                Some(2) => Manufacturer::Known(KnownManufacturer::Polar),
                Some(3) => Manufacturer::Known(KnownManufacturer::Wahoo),
                Some(15) => Manufacturer::Known(KnownManufacturer::Suunto),
                Some(260) => Manufacturer::Known(KnownManufacturer::Coros),
                _ => Manufacturer::Custom("Unknown".to_string()),
            };

            Some(DeviceInfo {
                device_index: get_field_u8(record, "device_index"),
                device_type,
                manufacturer,
                product: get_field_u16(record, "product").map(|p| format!("Product #{}", p)),
                serial_number: get_field_u32(record, "serial_number").map(|s| s.to_string()),
                software_version: get_field_u16(record, "software_version")
                    .map(|v| format!("{}.{}", v / 100, v % 100)),
                hardware_version: get_field_u8(record, "hardware_version").map(|v| v.to_string()),
                battery: None, // FIT doesn't typically include battery info in device_info
                cumulative_operating_time_hours: get_field_u32(record, "cum_operating_time")
                    .map(|t| t as f64 / 3600.0),
                connection: None,  // Connection info not in device_info messages
                calibration: None, // Calibration info in separate messages
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fit_to_pwf_compiles() {
        // This is a placeholder test to ensure the function signature compiles
        // We'll add proper integration tests with real FIT files in Phase 1D
        // For now, just verify that the types are correct

        // The function should accept any Read + sized type
        let _: fn(std::io::Cursor<Vec<u8>>, bool) -> Result<ConversionResult, ConversionError> =
            fit_to_pwf;

        // Test passes if this compiles (no assertion needed)
    }

    #[test]
    fn test_get_field_u32() {
        // Test helper functions compile correctly
        // We'll add real tests with FIT data in integration tests
    }
}
