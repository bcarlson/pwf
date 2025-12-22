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
    use fitparser::profile::MesgNum;
    use fitparser::{FitDataField, FitDataRecord};

    /// Helper to create a mock FIT record for testing
    fn create_mock_record(mesg_num: MesgNum, fields: Vec<(&str, Value)>) -> FitDataRecord {
        let mut record = FitDataRecord::new(mesg_num);
        for (i, (name, value)) in fields.into_iter().enumerate() {
            let field = FitDataField::new(name.to_string(), i as u8, value, String::new());
            record.push(field);
        }
        record
    }

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

    // Tests for get_field_u32
    #[test]
    fn test_get_field_u32_with_uint32() {
        let record = create_mock_record(
            MesgNum::Session,
            vec![("start_time", Value::UInt32(1234567890))],
        );
        assert_eq!(get_field_u32(&record, "start_time"), Some(1234567890));
    }

    #[test]
    fn test_get_field_u32_with_uint16() {
        let record =
            create_mock_record(MesgNum::Session, vec![("test_field", Value::UInt16(5000))]);
        assert_eq!(get_field_u32(&record, "test_field"), Some(5000));
    }

    #[test]
    fn test_get_field_u32_with_uint8() {
        let record = create_mock_record(MesgNum::Session, vec![("test_field", Value::UInt8(255))]);
        assert_eq!(get_field_u32(&record, "test_field"), Some(255));
    }

    #[test]
    fn test_get_field_u32_wrong_type() {
        let record = create_mock_record(
            MesgNum::Session,
            vec![("test_field", Value::Float64(123.45))],
        );
        assert_eq!(get_field_u32(&record, "test_field"), None);
    }

    #[test]
    fn test_get_field_u32_missing_field() {
        let record = create_mock_record(MesgNum::Session, vec![]);
        assert_eq!(get_field_u32(&record, "nonexistent"), None);
    }

    // Tests for get_field_u16
    #[test]
    fn test_get_field_u16_with_uint16() {
        let record = create_mock_record(MesgNum::Session, vec![("avg_power", Value::UInt16(250))]);
        assert_eq!(get_field_u16(&record, "avg_power"), Some(250));
    }

    #[test]
    fn test_get_field_u16_with_uint8() {
        let record = create_mock_record(MesgNum::Session, vec![("test_field", Value::UInt8(100))]);
        assert_eq!(get_field_u16(&record, "test_field"), Some(100));
    }

    #[test]
    fn test_get_field_u16_wrong_type() {
        let record =
            create_mock_record(MesgNum::Session, vec![("test_field", Value::UInt32(70000))]);
        assert_eq!(get_field_u16(&record, "test_field"), None);
    }

    #[test]
    fn test_get_field_u16_missing_field() {
        let record = create_mock_record(MesgNum::Session, vec![]);
        assert_eq!(get_field_u16(&record, "nonexistent"), None);
    }

    // Tests for get_field_u8
    #[test]
    fn test_get_field_u8_with_uint8() {
        let record = create_mock_record(MesgNum::Session, vec![("sport", Value::UInt8(5))]);
        assert_eq!(get_field_u8(&record, "sport"), Some(5));
    }

    #[test]
    fn test_get_field_u8_wrong_type() {
        let record = create_mock_record(MesgNum::Session, vec![("test_field", Value::UInt16(300))]);
        assert_eq!(get_field_u8(&record, "test_field"), None);
    }

    #[test]
    fn test_get_field_u8_missing_field() {
        let record = create_mock_record(MesgNum::Session, vec![]);
        assert_eq!(get_field_u8(&record, "nonexistent"), None);
    }

    // Tests for get_field_f64
    #[test]
    fn test_get_field_f64_with_float64() {
        let record = create_mock_record(
            MesgNum::Session,
            vec![("total_distance", Value::Float64(5432.1))],
        );
        assert_eq!(get_field_f64(&record, "total_distance"), Some(5432.1));
    }

    #[test]
    fn test_get_field_f64_with_float32() {
        let record = create_mock_record(
            MesgNum::Session,
            vec![("test_field", Value::Float32(123.45))],
        );
        // Float32 has precision issues, so we check approximate equality
        let result = get_field_f64(&record, "test_field");
        assert!(result.is_some());
        assert!((result.unwrap() - 123.45).abs() < 0.01);
    }

    #[test]
    fn test_get_field_f64_with_uint32() {
        let record =
            create_mock_record(MesgNum::Session, vec![("test_field", Value::UInt32(1000))]);
        assert_eq!(get_field_f64(&record, "test_field"), Some(1000.0));
    }

    #[test]
    fn test_get_field_f64_with_uint16() {
        let record = create_mock_record(MesgNum::Session, vec![("test_field", Value::UInt16(500))]);
        assert_eq!(get_field_f64(&record, "test_field"), Some(500.0));
    }

    #[test]
    fn test_get_field_f64_wrong_type() {
        let record = create_mock_record(MesgNum::Session, vec![("test_field", Value::UInt8(5))]);
        assert_eq!(get_field_f64(&record, "test_field"), None);
    }

    #[test]
    fn test_get_field_f64_missing_field() {
        let record = create_mock_record(MesgNum::Session, vec![]);
        assert_eq!(get_field_f64(&record, "nonexistent"), None);
    }

    // Tests for get_field_i32
    #[test]
    fn test_get_field_i32_with_sint32() {
        let record = create_mock_record(
            MesgNum::Record,
            vec![("position_lat", Value::SInt32(123456789))],
        );
        assert_eq!(get_field_i32(&record, "position_lat"), Some(123456789));
    }

    #[test]
    fn test_get_field_i32_with_sint16() {
        let record = create_mock_record(MesgNum::Record, vec![("test_field", Value::SInt16(-500))]);
        assert_eq!(get_field_i32(&record, "test_field"), Some(-500));
    }

    #[test]
    fn test_get_field_i32_with_sint8() {
        let record = create_mock_record(MesgNum::Record, vec![("test_field", Value::SInt8(-50))]);
        assert_eq!(get_field_i32(&record, "test_field"), Some(-50));
    }

    #[test]
    fn test_get_field_i32_negative_value() {
        let record = create_mock_record(
            MesgNum::Record,
            vec![("position_lat", Value::SInt32(-123456789))],
        );
        assert_eq!(get_field_i32(&record, "position_lat"), Some(-123456789));
    }

    #[test]
    fn test_get_field_i32_wrong_type() {
        let record = create_mock_record(MesgNum::Record, vec![("test_field", Value::UInt32(100))]);
        assert_eq!(get_field_i32(&record, "test_field"), None);
    }

    #[test]
    fn test_get_field_i32_missing_field() {
        let record = create_mock_record(MesgNum::Record, vec![]);
        assert_eq!(get_field_i32(&record, "nonexistent"), None);
    }

    // Tests for detect_multisport
    #[test]
    fn test_detect_multisport_single_session() {
        let sessions = vec![create_mock_record(
            MesgNum::Session,
            vec![("sport", Value::UInt8(1))],
        )];
        assert!(!detect_multisport(&sessions));
    }

    #[test]
    fn test_detect_multisport_empty_sessions() {
        let sessions: Vec<FitDataRecord> = vec![];
        assert!(!detect_multisport(&sessions));
    }

    #[test]
    fn test_detect_multisport_same_sport() {
        let sessions = vec![
            create_mock_record(MesgNum::Session, vec![("sport", Value::UInt8(1))]),
            create_mock_record(MesgNum::Session, vec![("sport", Value::UInt8(1))]),
        ];
        assert!(!detect_multisport(&sessions));
    }

    #[test]
    fn test_detect_multisport_different_sports() {
        let sessions = vec![
            create_mock_record(MesgNum::Session, vec![("sport", Value::UInt8(0))]), // Running
            create_mock_record(MesgNum::Session, vec![("sport", Value::UInt8(1))]), // Cycling
        ];
        assert!(detect_multisport(&sessions));
    }

    #[test]
    fn test_detect_multisport_with_transition() {
        // Transition sport (type 2) should be ignored
        let sessions = vec![
            create_mock_record(MesgNum::Session, vec![("sport", Value::UInt8(0))]), // Running
            create_mock_record(MesgNum::Session, vec![("sport", Value::UInt8(2))]), // Transition
            create_mock_record(MesgNum::Session, vec![("sport", Value::UInt8(0))]), // Running
        ];
        assert!(!detect_multisport(&sessions)); // Same sport (ignoring transition)
    }

    #[test]
    fn test_detect_multisport_triathlon() {
        let sessions = vec![
            create_mock_record(MesgNum::Session, vec![("sport", Value::UInt8(5))]), // Swimming
            create_mock_record(MesgNum::Session, vec![("sport", Value::UInt8(2))]), // Transition
            create_mock_record(MesgNum::Session, vec![("sport", Value::UInt8(1))]), // Cycling
            create_mock_record(MesgNum::Session, vec![("sport", Value::UInt8(2))]), // Transition
            create_mock_record(MesgNum::Session, vec![("sport", Value::UInt8(0))]), // Running
        ];
        assert!(detect_multisport(&sessions));
    }

    // Tests for device info conversion
    #[test]
    fn test_convert_device_info_garmin_watch() {
        let records = vec![create_mock_record(
            MesgNum::DeviceInfo,
            vec![
                ("device_type", Value::UInt8(1)),   // GPS watch
                ("manufacturer", Value::UInt16(1)), // Garmin
                ("product", Value::UInt16(1234)),   // Product ID
                ("serial_number", Value::UInt32(987654321)),
                ("software_version", Value::UInt16(1050)), // Version 10.50
                ("hardware_version", Value::UInt8(3)),
            ],
        )];

        let devices = convert_device_info_records(&records);
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].device_type, DeviceType::Watch);
        assert_eq!(
            devices[0].manufacturer,
            Manufacturer::Known(KnownManufacturer::Garmin)
        );
        assert_eq!(devices[0].product, Some("Product #1234".to_string()));
        assert_eq!(devices[0].serial_number, Some("987654321".to_string()));
        assert_eq!(devices[0].software_version, Some("10.50".to_string()));
        assert_eq!(devices[0].hardware_version, Some("3".to_string()));
    }

    #[test]
    fn test_convert_device_info_bike_computer() {
        let records = vec![create_mock_record(
            MesgNum::DeviceInfo,
            vec![
                ("device_type", Value::UInt8(11)),  // Bike computer
                ("manufacturer", Value::UInt16(3)), // Wahoo
                ("device_index", Value::UInt8(0)),
            ],
        )];

        let devices = convert_device_info_records(&records);
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].device_type, DeviceType::BikeComputer);
        assert_eq!(
            devices[0].manufacturer,
            Manufacturer::Known(KnownManufacturer::Wahoo)
        );
        assert_eq!(devices[0].device_index, Some(0));
    }

    #[test]
    fn test_convert_device_info_heart_rate_monitor() {
        let records = vec![create_mock_record(
            MesgNum::DeviceInfo,
            vec![
                ("device_type", Value::UInt8(120)), // Heart rate monitor
                ("manufacturer", Value::UInt16(2)), // Polar
            ],
        )];

        let devices = convert_device_info_records(&records);
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].device_type, DeviceType::HeartRateMonitor);
        assert_eq!(
            devices[0].manufacturer,
            Manufacturer::Known(KnownManufacturer::Polar)
        );
    }

    #[test]
    fn test_convert_device_info_power_meter() {
        let records = vec![create_mock_record(
            MesgNum::DeviceInfo,
            vec![
                ("device_type", Value::UInt8(12)),   // Power meter
                ("manufacturer", Value::UInt16(15)), // Suunto
            ],
        )];

        let devices = convert_device_info_records(&records);
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].device_type, DeviceType::PowerMeter);
        assert_eq!(
            devices[0].manufacturer,
            Manufacturer::Known(KnownManufacturer::Suunto)
        );
    }

    #[test]
    fn test_convert_device_info_coros_device() {
        let records = vec![create_mock_record(
            MesgNum::DeviceInfo,
            vec![
                ("device_type", Value::UInt8(1)),     // Watch
                ("manufacturer", Value::UInt16(260)), // Coros
            ],
        )];

        let devices = convert_device_info_records(&records);
        assert_eq!(devices.len(), 1);
        assert_eq!(
            devices[0].manufacturer,
            Manufacturer::Known(KnownManufacturer::Coros)
        );
    }

    #[test]
    fn test_convert_device_info_unknown_manufacturer() {
        let records = vec![create_mock_record(
            MesgNum::DeviceInfo,
            vec![
                ("device_type", Value::UInt8(1)),     // Watch
                ("manufacturer", Value::UInt16(999)), // Unknown
            ],
        )];

        let devices = convert_device_info_records(&records);
        assert_eq!(devices.len(), 1);
        assert_eq!(
            devices[0].manufacturer,
            Manufacturer::Custom("Unknown".to_string())
        );
    }

    #[test]
    fn test_convert_device_info_unknown_device_type() {
        let records = vec![create_mock_record(
            MesgNum::DeviceInfo,
            vec![
                ("device_type", Value::UInt8(99)),  // Unknown type
                ("manufacturer", Value::UInt16(1)), // Garmin
            ],
        )];

        let devices = convert_device_info_records(&records);
        assert_eq!(devices.len(), 0); // Unknown device types are filtered out
    }

    #[test]
    fn test_convert_device_info_multiple_devices() {
        let records = vec![
            create_mock_record(
                MesgNum::DeviceInfo,
                vec![
                    ("device_type", Value::UInt8(1)),
                    ("manufacturer", Value::UInt16(1)),
                ],
            ),
            create_mock_record(
                MesgNum::DeviceInfo,
                vec![
                    ("device_type", Value::UInt8(120)),
                    ("manufacturer", Value::UInt16(2)),
                ],
            ),
        ];

        let devices = convert_device_info_records(&records);
        assert_eq!(devices.len(), 2);
    }

    #[test]
    fn test_convert_device_info_with_operating_time() {
        let records = vec![create_mock_record(
            MesgNum::DeviceInfo,
            vec![
                ("device_type", Value::UInt8(1)),
                ("manufacturer", Value::UInt16(1)),
                ("cum_operating_time", Value::UInt32(7200)), // 2 hours in seconds
            ],
        )];

        let devices = convert_device_info_records(&records);
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].cumulative_operating_time_hours, Some(2.0));
    }

    // Tests for GPS route extraction
    #[test]
    fn test_extract_gps_route_empty_records() {
        let session = create_mock_record(
            MesgNum::Session,
            vec![
                ("start_time", Value::UInt32(1000000)),
                ("total_distance", Value::Float64(5000.0)),
            ],
        );
        let records: Vec<FitDataRecord> = vec![];
        let mut result = ConversionResult::new(String::new());

        let route = extract_gps_route(&session, &records, &mut result);
        assert!(route.is_none());
    }

    #[test]
    fn test_extract_gps_route_invalid_coordinates() {
        let session = create_mock_record(
            MesgNum::Session,
            vec![
                ("start_time", Value::UInt32(1000000)),
                ("total_distance", Value::Float64(5000.0)),
            ],
        );
        // GPS coordinates at (0, 0) should be filtered out as invalid
        let records = vec![create_mock_record(
            MesgNum::Record,
            vec![
                ("position_lat", Value::SInt32(0)),
                ("position_long", Value::SInt32(0)),
                ("timestamp", Value::UInt32(1000000)),
            ],
        )];
        let mut result = ConversionResult::new(String::new());

        let route = extract_gps_route(&session, &records, &mut result);
        assert!(route.is_none()); // Should be filtered out
    }

    #[test]
    fn test_extract_gps_route_valid_position() {
        let session = create_mock_record(
            MesgNum::Session,
            vec![
                ("start_time", Value::UInt32(1000000)),
                ("total_distance", Value::Float64(5000.0)),
            ],
        );
        // Valid GPS coordinates (San Francisco area: ~37.77°N, 122.42°W)
        // Using semicircles: degrees = semicircles * (180 / 2^31)
        let lat_semicircles = 454_000_000; // Should be ~37.95 degrees
        let lng_semicircles = -1_472_000_000; // Should be ~-123.24 degrees
        let records = vec![create_mock_record(
            MesgNum::Record,
            vec![
                ("position_lat", Value::SInt32(lat_semicircles)),
                ("position_long", Value::SInt32(lng_semicircles)),
                ("timestamp", Value::UInt32(1000000)),
                ("altitude", Value::Float64(50.0)),
                ("speed", Value::Float64(3.5)),
                ("heart_rate", Value::UInt8(150)),
            ],
        )];
        let mut result = ConversionResult::new(String::new());

        let route = extract_gps_route(&session, &records, &mut result);
        assert!(route.is_some());
        let route = route.unwrap();
        assert_eq!(route.positions.len(), 1);
        assert_eq!(route.total_distance_m, Some(5000.0));
        // Check that coordinates are in reasonable range (not filtered as invalid)
        assert!(route.positions[0].latitude_deg.abs() > 1.0); // Not near 0,0
        assert!(route.positions[0].longitude_deg.abs() > 1.0);
        assert_eq!(route.positions[0].elevation_m, Some(50.0));
        assert_eq!(route.positions[0].speed_mps, Some(3.5));
        assert_eq!(route.positions[0].heart_rate_bpm, Some(150));
    }

    #[test]
    fn test_extract_gps_route_elevation_tracking() {
        let session = create_mock_record(
            MesgNum::Session,
            vec![
                ("start_time", Value::UInt32(1000000)),
                ("total_distance", Value::Float64(1000.0)),
            ],
        );
        // Create records with varying elevation
        let records = vec![
            create_mock_record(
                MesgNum::Record,
                vec![
                    ("position_lat", Value::SInt32(454_000_000)),
                    ("position_long", Value::SInt32(-1_472_000_000)),
                    ("timestamp", Value::UInt32(1000000)),
                    ("altitude", Value::Float64(100.0)),
                ],
            ),
            create_mock_record(
                MesgNum::Record,
                vec![
                    ("position_lat", Value::SInt32(454_000_100)),
                    ("position_long", Value::SInt32(-1_472_000_100)),
                    ("timestamp", Value::UInt32(1000010)),
                    ("altitude", Value::Float64(150.0)), // +50m ascent
                ],
            ),
            create_mock_record(
                MesgNum::Record,
                vec![
                    ("position_lat", Value::SInt32(454_000_200)),
                    ("position_long", Value::SInt32(-1_472_000_200)),
                    ("timestamp", Value::UInt32(1000020)),
                    ("altitude", Value::Float64(120.0)), // -30m descent
                ],
            ),
        ];
        let mut result = ConversionResult::new(String::new());

        let route = extract_gps_route(&session, &records, &mut result);
        assert!(route.is_some());
        let route = route.unwrap();
        assert_eq!(route.positions.len(), 3);
        assert_eq!(route.min_elevation_m, Some(100.0));
        assert_eq!(route.max_elevation_m, Some(150.0));
        assert_eq!(route.total_ascent_m, Some(50.0));
        assert_eq!(route.total_descent_m, Some(30.0));
    }

    #[test]
    fn test_extract_gps_route_bounding_box() {
        let session = create_mock_record(
            MesgNum::Session,
            vec![
                ("start_time", Value::UInt32(1000000)),
                ("total_distance", Value::Float64(1000.0)),
            ],
        );
        let records = vec![
            create_mock_record(
                MesgNum::Record,
                vec![
                    ("position_lat", Value::SInt32(454_000_000)), // ~37.77°
                    ("position_long", Value::SInt32(-1_472_000_000)), // ~-122.42°
                    ("timestamp", Value::UInt32(1000000)),
                ],
            ),
            create_mock_record(
                MesgNum::Record,
                vec![
                    ("position_lat", Value::SInt32(455_000_000)), // ~37.86°
                    ("position_long", Value::SInt32(-1_471_000_000)), // ~-122.33°
                    ("timestamp", Value::UInt32(1000010)),
                ],
            ),
        ];
        let mut result = ConversionResult::new(String::new());

        let route = extract_gps_route(&session, &records, &mut result);
        assert!(route.is_some());
        let route = route.unwrap();
        assert!(route.bbox_sw_lat.is_some());
        assert!(route.bbox_sw_lng.is_some());
        assert!(route.bbox_ne_lat.is_some());
        assert!(route.bbox_ne_lng.is_some());
        assert!(route.bbox_sw_lat.unwrap() < route.bbox_ne_lat.unwrap());
        assert!(route.bbox_sw_lng.unwrap() < route.bbox_ne_lng.unwrap());
    }

    #[test]
    fn test_extract_gps_route_with_power_and_cadence() {
        let session = create_mock_record(
            MesgNum::Session,
            vec![("start_time", Value::UInt32(1000000))],
        );
        let records = vec![create_mock_record(
            MesgNum::Record,
            vec![
                ("position_lat", Value::SInt32(454_000_000)),
                ("position_long", Value::SInt32(-1_472_000_000)),
                ("timestamp", Value::UInt32(1000000)),
                ("power", Value::UInt16(250)),
                ("cadence", Value::UInt8(90)),
                ("temperature", Value::Float64(20.5)),
            ],
        )];
        let mut result = ConversionResult::new(String::new());

        let route = extract_gps_route(&session, &records, &mut result);
        assert!(route.is_some());
        let route = route.unwrap();
        assert_eq!(route.positions[0].power_watts, Some(250));
        assert_eq!(route.positions[0].cadence, Some(90));
        assert_eq!(route.positions[0].temperature_c, Some(20.5));
    }

    // Tests for create_sport_segment
    #[test]
    fn test_create_sport_segment_basic() {
        let session = create_mock_record(
            MesgNum::Session,
            vec![
                ("start_time", Value::UInt32(1000000)),
                ("sport_index", Value::UInt8(0)),
            ],
        );
        let sport = pwf_core::Sport::Running;
        let started_at = "2024-01-01T10:00:00Z";
        let duration_sec = Some(3600);
        let distance_m = Some(10000.0);
        let exercises = vec![];
        let telemetry = WorkoutTelemetry::default();

        let segments = create_sport_segment(
            &session,
            sport,
            started_at,
            duration_sec,
            distance_m,
            &exercises,
            &telemetry,
        );

        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].sport, sport);
        assert_eq!(segments[0].segment_id, "segment-1000000");
        assert_eq!(segments[0].segment_index, 0);
        assert_eq!(segments[0].duration_sec, Some(3600));
        assert_eq!(segments[0].distance_m, Some(10000.0));
    }

    #[test]
    fn test_create_sport_segment_with_exercises() {
        let session = create_mock_record(
            MesgNum::Session,
            vec![("start_time", Value::UInt32(2000000))],
        );
        let exercises = vec![
            CompletedExercise {
                id: Some("ex1".to_string()),
                name: "Exercise 1".to_string(),
                modality: None,
                sets: vec![],
                notes: None,
                sport: None,
                pool_config: None,
            },
            CompletedExercise {
                id: Some("ex2".to_string()),
                name: "Exercise 2".to_string(),
                modality: None,
                sets: vec![],
                notes: None,
                sport: None,
                pool_config: None,
            },
        ];

        let segments = create_sport_segment(
            &session,
            pwf_core::Sport::Cycling,
            "2024-01-01T10:00:00Z",
            None,
            None,
            &exercises,
            &WorkoutTelemetry::default(),
        );

        assert_eq!(segments[0].exercise_ids.len(), 2);
        assert_eq!(segments[0].exercise_ids[0], "ex1");
        assert_eq!(segments[0].exercise_ids[1], "ex2");
    }

    // Tests for extract_swimming_data
    #[test]
    fn test_extract_swimming_data_empty_lengths() {
        let lengths: Vec<FitDataRecord> = vec![];
        let result = extract_swimming_data(&lengths, Some(1000000));
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_swimming_data_50m_pool() {
        let lengths = vec![create_mock_record(
            MesgNum::Length,
            vec![
                ("pool_length", Value::Float64(50.0)),
                ("total_elapsed_time", Value::Float64(45.0)),
                ("swim_stroke", Value::UInt8(0)), // Freestyle
                ("total_strokes", Value::UInt16(30)),
                ("length_type", Value::UInt8(1)), // Active
                ("timestamp", Value::UInt32(1000000)),
            ],
        )];

        let result = extract_swimming_data(&lengths, Some(1000000));
        assert!(result.is_some());
        let (data, config) = result.unwrap();
        assert_eq!(config.pool_length, 50.0);
        assert_eq!(config.pool_length_unit, PoolLengthUnit::Meters);
        assert_eq!(data.lengths.len(), 1);
        assert_eq!(data.lengths[0].stroke_type, StrokeType::Freestyle);
        assert_eq!(data.lengths[0].stroke_count, Some(30));
        assert_eq!(data.lengths[0].swolf, Some(75)); // 30 strokes + 45 seconds
        assert_eq!(data.lengths[0].active, Some(true));
    }

    #[test]
    fn test_extract_swimming_data_25m_pool() {
        let lengths = vec![create_mock_record(
            MesgNum::Length,
            vec![
                ("pool_length", Value::Float64(25.0)),
                ("total_elapsed_time", Value::Float64(20.0)),
                ("swim_stroke", Value::UInt8(1)), // Backstroke
                ("total_strokes", Value::UInt16(15)),
                ("timestamp", Value::UInt32(1000000)),
            ],
        )];

        let result = extract_swimming_data(&lengths, Some(1000000));
        assert!(result.is_some());
        let (data, config) = result.unwrap();
        assert_eq!(config.pool_length, 25.0);
        assert_eq!(config.pool_length_unit, PoolLengthUnit::Meters);
        assert_eq!(data.lengths[0].stroke_type, StrokeType::Backstroke);
    }

    #[test]
    fn test_extract_swimming_data_yards_pool() {
        let lengths = vec![create_mock_record(
            MesgNum::Length,
            vec![
                ("pool_length", Value::Float64(33.0)), // ~33 yards
                ("total_elapsed_time", Value::Float64(30.0)),
                ("swim_stroke", Value::UInt8(2)), // Breaststroke
                ("total_strokes", Value::UInt16(20)),
            ],
        )];

        let result = extract_swimming_data(&lengths, Some(1000000));
        assert!(result.is_some());
        let (_, config) = result.unwrap();
        assert_eq!(config.pool_length, 33.0);
        assert_eq!(config.pool_length_unit, PoolLengthUnit::Yards);
    }

    #[test]
    fn test_extract_swimming_data_multiple_lengths() {
        let lengths = vec![
            create_mock_record(
                MesgNum::Length,
                vec![
                    ("pool_length", Value::Float64(50.0)),
                    ("total_elapsed_time", Value::Float64(45.0)),
                    ("swim_stroke", Value::UInt8(0)), // Freestyle
                    ("total_strokes", Value::UInt16(30)),
                    ("length_type", Value::UInt8(1)), // Active
                ],
            ),
            create_mock_record(
                MesgNum::Length,
                vec![
                    ("pool_length", Value::Float64(50.0)),
                    ("total_elapsed_time", Value::Float64(50.0)),
                    ("swim_stroke", Value::UInt8(0)), // Freestyle
                    ("total_strokes", Value::UInt16(32)),
                    ("length_type", Value::UInt8(1)), // Active
                ],
            ),
            create_mock_record(
                MesgNum::Length,
                vec![
                    ("pool_length", Value::Float64(50.0)),
                    ("total_elapsed_time", Value::Float64(60.0)),
                    ("swim_stroke", Value::UInt8(0)), // Freestyle
                    ("total_strokes", Value::UInt16(35)),
                    ("length_type", Value::UInt8(0)), // Rest
                ],
            ),
        ];

        let result = extract_swimming_data(&lengths, Some(1000000));
        assert!(result.is_some());
        let (data, _) = result.unwrap();
        assert_eq!(data.lengths.len(), 3);
        assert_eq!(data.total_lengths, Some(3));
        assert_eq!(data.active_lengths, Some(2)); // Only 2 are active
        assert_eq!(data.stroke_type, Some(StrokeType::Freestyle)); // All same stroke

        // Average SWOLF calculation: (75 + 82 + 95) / 3 = 84
        assert_eq!(data.swolf_avg, Some(84));
    }

    #[test]
    fn test_extract_swimming_data_mixed_strokes() {
        let lengths = vec![
            create_mock_record(
                MesgNum::Length,
                vec![
                    ("pool_length", Value::Float64(50.0)),
                    ("total_elapsed_time", Value::Float64(45.0)),
                    ("swim_stroke", Value::UInt8(0)), // Freestyle
                    ("total_strokes", Value::UInt16(30)),
                ],
            ),
            create_mock_record(
                MesgNum::Length,
                vec![
                    ("pool_length", Value::Float64(50.0)),
                    ("total_elapsed_time", Value::Float64(55.0)),
                    ("swim_stroke", Value::UInt8(1)), // Backstroke
                    ("total_strokes", Value::UInt16(35)),
                ],
            ),
        ];

        let result = extract_swimming_data(&lengths, Some(1000000));
        assert!(result.is_some());
        let (data, _) = result.unwrap();
        assert_eq!(data.stroke_type, None); // Mixed strokes, so None
    }

    #[test]
    fn test_extract_swimming_data_all_stroke_types() {
        // Test each stroke type mapping
        let stroke_codes = vec![
            (0, StrokeType::Freestyle),
            (1, StrokeType::Backstroke),
            (2, StrokeType::Breaststroke),
            (3, StrokeType::Butterfly),
            (4, StrokeType::Drill),
            (5, StrokeType::Mixed),
            (6, StrokeType::IndividualMedley),
        ];

        for (code, expected_stroke) in stroke_codes {
            let lengths = vec![create_mock_record(
                MesgNum::Length,
                vec![
                    ("pool_length", Value::Float64(50.0)),
                    ("total_elapsed_time", Value::Float64(45.0)),
                    ("swim_stroke", Value::UInt8(code)),
                    ("total_strokes", Value::UInt16(30)),
                ],
            )];

            let result = extract_swimming_data(&lengths, Some(1000000));
            assert!(result.is_some());
            let (data, _) = result.unwrap();
            assert_eq!(data.lengths[0].stroke_type, expected_stroke);
        }
    }

    #[test]
    fn test_extract_swimming_data_missing_pool_length() {
        // If pool_length is missing, should return None
        let lengths = vec![create_mock_record(
            MesgNum::Length,
            vec![
                ("total_elapsed_time", Value::Float64(45.0)),
                ("swim_stroke", Value::UInt8(0)),
            ],
        )];

        let result = extract_swimming_data(&lengths, Some(1000000));
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_swimming_data_missing_duration() {
        // If duration is missing, that length is filtered out
        let lengths = vec![
            create_mock_record(
                MesgNum::Length,
                vec![
                    ("pool_length", Value::Float64(50.0)),
                    ("swim_stroke", Value::UInt8(0)),
                    // Missing total_elapsed_time
                ],
            ),
            create_mock_record(
                MesgNum::Length,
                vec![
                    ("pool_length", Value::Float64(50.0)),
                    ("total_elapsed_time", Value::Float64(45.0)),
                    ("swim_stroke", Value::UInt8(0)),
                    ("total_strokes", Value::UInt16(30)),
                ],
            ),
        ];

        let result = extract_swimming_data(&lengths, Some(1000000));
        assert!(result.is_some());
        let (data, _) = result.unwrap();
        assert_eq!(data.lengths.len(), 1); // Only the valid length
    }

    #[test]
    fn test_extract_swimming_data_with_uint32_duration() {
        // Test that uint32 duration works too
        let lengths = vec![create_mock_record(
            MesgNum::Length,
            vec![
                ("pool_length", Value::Float64(50.0)),
                ("total_elapsed_time", Value::UInt32(45)),
                ("swim_stroke", Value::UInt8(0)),
            ],
        )];

        let result = extract_swimming_data(&lengths, Some(1000000));
        assert!(result.is_some());
        let (data, _) = result.unwrap();
        assert_eq!(data.lengths[0].duration_sec, 45);
    }

    #[test]
    fn test_extract_swimming_data_swolf_calculation() {
        // Test SWOLF calculation: strokes + seconds
        let lengths = vec![create_mock_record(
            MesgNum::Length,
            vec![
                ("pool_length", Value::Float64(50.0)),
                ("total_elapsed_time", Value::Float64(42.5)),
                ("swim_stroke", Value::UInt8(0)),
                ("total_strokes", Value::UInt16(28)),
            ],
        )];

        let result = extract_swimming_data(&lengths, Some(1000000));
        assert!(result.is_some());
        let (data, _) = result.unwrap();
        assert_eq!(data.lengths[0].swolf, Some(70)); // 28 + 42 (truncated)
    }

    #[test]
    fn test_extract_swimming_data_no_strokes() {
        // If no stroke count, SWOLF should be None
        let lengths = vec![create_mock_record(
            MesgNum::Length,
            vec![
                ("pool_length", Value::Float64(50.0)),
                ("total_elapsed_time", Value::Float64(45.0)),
                ("swim_stroke", Value::UInt8(0)),
                // Missing total_strokes
            ],
        )];

        let result = extract_swimming_data(&lengths, Some(1000000));
        assert!(result.is_some());
        let (data, _) = result.unwrap();
        assert_eq!(data.lengths[0].stroke_count, None);
        assert_eq!(data.lengths[0].swolf, None);
    }

    // Tests for convert_laps_to_exercises
    #[test]
    fn test_convert_laps_to_exercises_no_laps() {
        let session = create_mock_record(
            MesgNum::Session,
            vec![
                ("start_time", Value::UInt32(1000000)),
                ("total_elapsed_time", Value::UInt32(3600)),
                ("total_distance", Value::Float64(5000.0)),
                ("sport", Value::UInt8(0)), // Running
            ],
        );
        let laps: Vec<FitDataRecord> = vec![];
        let lengths: Vec<FitDataRecord> = vec![];
        let mut result = ConversionResult::new(String::new());

        let exercises = convert_laps_to_exercises(&session, &laps, &lengths, &mut result);
        assert!(exercises.is_ok());
        let exercises = exercises.unwrap();
        assert_eq!(exercises.len(), 1);
        assert_eq!(exercises[0].sets.len(), 1);
        assert_eq!(exercises[0].sets[0].duration_sec, Some(3600));
        assert_eq!(exercises[0].sets[0].distance_meters, Some(5000.0));
        assert!(result.has_warnings()); // Should warn about no laps
    }

    #[test]
    fn test_convert_laps_to_exercises_multiple_laps() {
        let session = create_mock_record(
            MesgNum::Session,
            vec![
                ("start_time", Value::UInt32(1000000)),
                ("sport", Value::UInt8(1)), // Cycling
            ],
        );
        let laps = vec![
            create_mock_record(
                MesgNum::Lap,
                vec![
                    ("total_elapsed_time", Value::UInt32(1200)),
                    ("total_distance", Value::Float64(5000.0)),
                ],
            ),
            create_mock_record(
                MesgNum::Lap,
                vec![
                    ("total_timer_time", Value::UInt32(1100)),
                    ("total_distance", Value::Float64(4800.0)),
                ],
            ),
        ];
        let lengths: Vec<FitDataRecord> = vec![];
        let mut result = ConversionResult::new(String::new());

        let exercises = convert_laps_to_exercises(&session, &laps, &lengths, &mut result);
        assert!(exercises.is_ok());
        let exercises = exercises.unwrap();
        assert_eq!(exercises.len(), 1);
        assert_eq!(exercises[0].sets.len(), 2);
        assert_eq!(exercises[0].sets[0].set_number, Some(1));
        assert_eq!(exercises[0].sets[0].duration_sec, Some(1200));
        assert_eq!(exercises[0].sets[0].distance_meters, Some(5000.0));
        assert_eq!(exercises[0].sets[1].set_number, Some(2));
        assert_eq!(exercises[0].sets[1].duration_sec, Some(1100));
    }

    #[test]
    fn test_convert_laps_to_exercises_swimming() {
        let session = create_mock_record(
            MesgNum::Session,
            vec![
                ("start_time", Value::UInt32(1000000)),
                ("sport", Value::UInt8(5)), // Swimming
            ],
        );
        let laps = vec![create_mock_record(
            MesgNum::Lap,
            vec![
                ("total_elapsed_time", Value::UInt32(600)),
                ("total_distance", Value::Float64(500.0)),
            ],
        )];
        let lengths = vec![
            create_mock_record(
                MesgNum::Length,
                vec![
                    ("pool_length", Value::Float64(50.0)),
                    ("total_elapsed_time", Value::Float64(45.0)),
                    ("swim_stroke", Value::UInt8(0)),
                    ("total_strokes", Value::UInt16(30)),
                ],
            ),
            create_mock_record(
                MesgNum::Length,
                vec![
                    ("pool_length", Value::Float64(50.0)),
                    ("total_elapsed_time", Value::Float64(46.0)),
                    ("swim_stroke", Value::UInt8(0)),
                    ("total_strokes", Value::UInt16(31)),
                ],
            ),
        ];
        let mut result = ConversionResult::new(String::new());

        let exercises = convert_laps_to_exercises(&session, &laps, &lengths, &mut result);
        assert!(exercises.is_ok());
        let exercises = exercises.unwrap();
        assert_eq!(exercises.len(), 1);
        assert!(exercises[0].pool_config.is_some());
        let pool_config = exercises[0].pool_config.as_ref().unwrap();
        assert_eq!(pool_config.pool_length, 50.0);

        // Swimming data should be attached to first set
        assert!(exercises[0].sets[0].swimming.is_some());
        let swimming = exercises[0].sets[0].swimming.as_ref().unwrap();
        assert_eq!(swimming.lengths.len(), 2);
    }
}
