//! TCX to PWF conversion logic

use crate::common::utils::meters_to_km;
use crate::error::{ConversionError, ConversionResult, ConversionWarning};
use crate::tcx::mappings::map_tcx_sport;
use chrono::Utc;
use pwf_core::history::{
    CompletedExercise, CompletedSet, ExportSource, GpsPosition, GpsRoute, Units, Workout,
    WorkoutTelemetry, WpsHistory,
};
use std::io::{BufReader, Read};

/// Convert TCX file data to PWF YAML format
///
/// # Arguments
/// * `reader` - Reader containing TCX file data
/// * `summary_only` - If true, skip time-series data for smaller output
///
/// # Returns
/// ConversionResult with PWF YAML and any warnings
pub fn tcx_to_pwf<R: Read>(
    reader: R,
    summary_only: bool,
) -> Result<ConversionResult, ConversionError> {
    // Parse TCX file using tcx crate
    // The tcx::read function requires a mutable BufReader
    let mut buf_reader = BufReader::new(reader);
    let tcx_data: tcx::TrainingCenterDatabase = match tcx::read(&mut buf_reader) {
        Ok(data) => data,
        Err(e) => {
            return Err(ConversionError::TcxReadError(format!(
                "Failed to parse TCX: {}",
                e
            )));
        }
    };

    let mut result = ConversionResult::new(String::new());

    // Extract activities
    let activities = match tcx_data.activities {
        Some(activities_wrapper) => activities_wrapper.activities,
        None => {
            result.add_warning(ConversionWarning::DataQualityIssue {
                issue: "No activities found in TCX file".to_string(),
            });
            // Return minimal valid PWF
            let history = create_empty_history();
            result.pwf_yaml = serde_yaml::to_string(&history)?;
            return Ok(result);
        }
    };

    // Convert each TCX activity to a PWF workout
    let mut workouts = Vec::new();
    for activity in activities {
        match convert_activity_to_workout(&activity, summary_only, &mut result) {
            Ok(workout) => workouts.push(workout),
            Err(e) => {
                result.add_warning(ConversionWarning::DataQualityIssue {
                    issue: format!("Failed to convert activity: {}", e),
                });
            }
        }
    }

    // Build WpsHistory structure
    let history = WpsHistory {
        history_version: 2,
        exported_at: Utc::now().to_rfc3339(),
        export_source: Some(ExportSource {
            app_name: Some("PWF TCX Converter".to_string()),
            app_version: Some(env!("CARGO_PKG_VERSION").to_string()),
            platform: Some("TCX file".to_string()),
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

/// Create an empty PWF history structure
fn create_empty_history() -> WpsHistory {
    WpsHistory {
        history_version: 2,
        exported_at: Utc::now().to_rfc3339(),
        export_source: Some(ExportSource {
            app_name: Some("PWF TCX Converter".to_string()),
            app_version: Some(env!("CARGO_PKG_VERSION").to_string()),
            platform: None,
            preferred_units: None,
        }),
        units: Units::default(),
        workouts: Vec::new(),
        personal_records: Vec::new(),
        body_measurements: Vec::new(),
    }
}

/// Convert a TCX Activity to a PWF Workout
fn convert_activity_to_workout(
    activity: &tcx::Activity,
    summary_only: bool,
    _result: &mut ConversionResult,
) -> Result<Workout, ConversionError> {
    // Extract sport type
    let sport = map_tcx_sport(&activity.sport);

    // Extract activity ID (timestamp)
    let started_at = activity.id.clone();
    let date = started_at.split('T').next().unwrap_or("").to_string();

    // Process laps
    let mut total_duration_sec: Option<u32> = None;
    let mut total_distance_m = 0.0;
    let mut exercises = Vec::new();
    let mut all_trackpoints: Vec<&tcx::Trackpoint> = Vec::new();

    // Aggregate telemetry data
    let mut total_calories = 0u32;
    let mut max_hr = 0u8;
    let mut hr_sum = 0u32;
    let mut hr_count = 0u32;
    let mut cadence_sum = 0u32;
    let mut cadence_count = 0u32;

    for (lap_index, lap) in activity.laps.iter().enumerate() {
        // Aggregate lap-level data
        let lap_duration = lap.total_time_seconds as u32;
        total_duration_sec = Some(total_duration_sec.unwrap_or(0) + lap_duration);

        total_distance_m += lap.distance_meters;

        total_calories += lap.calories as u32;

        if let Some(max_heart_rate) = lap.maximum_heart_rate {
            let max_hr_val = max_heart_rate as u8;
            if max_hr_val > max_hr {
                max_hr = max_hr_val;
            }
        }

        if let Some(avg_hr) = lap.average_heart_rate {
            hr_sum += avg_hr as u32;
            hr_count += 1;
        }

        // Collect trackpoints for GPS route
        for track in &lap.tracks {
            for trackpoint in &track.trackpoints {
                all_trackpoints.push(trackpoint);

                // Aggregate trackpoint-level telemetry
                if let Some(heart_rate) = &trackpoint.heart_rate {
                    let hr = heart_rate.value as u8;
                    if hr > max_hr {
                        max_hr = hr;
                    }
                }

                if let Some(cadence) = trackpoint.cadence {
                    cadence_sum += cadence as u32;
                    cadence_count += 1;
                }

                // Check extensions for power data (if present)
                // Note: The tcx crate's Extensions type may not have power field directly
                // This would need checking the actual Extensions struct definition
            }
        }

        // Convert lap to an exercise set
        let set = CompletedSet {
            set_number: Some((lap_index + 1) as u32),
            reps: None,
            weight_kg: None,
            weight_lb: None,
            duration_sec: Some(lap.total_time_seconds as u32),
            distance_meters: Some(lap.distance_meters),
            rpe: None,
            rir: None,
            set_type: None,
            is_pr: None,
            notes: None,
            completed_at: None,
            telemetry: None,
            swimming: None,
        };

        exercises.push(CompletedExercise {
            id: None,
            name: format!("Lap {}", lap_index + 1),
            modality: Some(pwf_core::Modality::Stopwatch),
            sets: vec![set],
            notes: None,
            sport: None,
            pool_config: None,
        });
    }

    // Build workout telemetry
    let mut telemetry = WorkoutTelemetry::default();

    if hr_count > 0 {
        telemetry.heart_rate_avg = Some(hr_sum / hr_count);
    }
    if max_hr > 0 {
        telemetry.heart_rate_max = Some(max_hr as u32);
    }
    if total_calories > 0 {
        telemetry.total_calories = Some(total_calories);
    }
    if total_distance_m > 0.0 {
        telemetry.total_distance_km = Some(meters_to_km(total_distance_m));
    }
    if cadence_count > 0 {
        telemetry.cadence_avg = Some(cadence_sum / cadence_count);
    }

    // Extract GPS route from trackpoints
    if !summary_only && !all_trackpoints.is_empty() {
        if let Some(gps_route) = extract_gps_route(&all_trackpoints, &started_at, total_distance_m)
        {
            telemetry.gps_route = Some(gps_route);
        }
    }

    // TCX doesn't have a standardized Creator field in Activity
    // Device info would be in extensions if available
    let devices = Vec::new();

    // Calculate end time
    let ended_at = if let (Some(duration), Ok(start_time)) = (
        total_duration_sec,
        chrono::DateTime::parse_from_rfc3339(&started_at),
    ) {
        Some(
            (start_time + chrono::Duration::seconds(duration as i64))
                .to_rfc3339()
                .to_string(),
        )
    } else {
        None
    };

    Ok(Workout {
        id: None,
        date,
        started_at: Some(started_at),
        ended_at,
        duration_sec: total_duration_sec,
        title: Some(format!("{:?} Workout", sport)),
        notes: None,
        plan_id: None,
        plan_day_id: None,
        exercises,
        telemetry: Some(telemetry),
        devices,
        sport: Some(sport),
        sport_segments: None,
    })
}

/// Extract GPS route from TCX trackpoints
fn extract_gps_route(
    trackpoints: &[&tcx::Trackpoint],
    activity_id: &str,
    total_distance_m: f64,
) -> Option<GpsRoute> {
    let positions: Vec<GpsPosition> = trackpoints
        .iter()
        .filter_map(|tp| {
            let position = tp.position.as_ref()?;
            let latitude_deg = position.latitude;
            let longitude_deg = position.longitude;

            // Skip invalid coordinates (0,0)
            if latitude_deg.abs() < 0.001 && longitude_deg.abs() < 0.001 {
                return None;
            }

            Some(GpsPosition {
                latitude_deg,
                longitude_deg,
                timestamp: tp.time.to_rfc3339(),
                elevation_m: tp.altitude_meters,
                accuracy_m: None,  // TCX doesn't include GPS accuracy
                speed_mps: None,   // TCX includes speed in extensions, but not standard
                heading_deg: None, // TCX doesn't include heading
                heart_rate_bpm: tp.heart_rate.as_ref().map(|hr| hr.value as u32),
                power_watts: None, // Power data would be in extensions if available
                cadence: tp.cadence.map(|c| c as u32),
                temperature_c: None, // TCX doesn't include temperature in standard schema
            })
        })
        .collect();

    if positions.is_empty() {
        return None;
    }

    // Calculate elevation metrics
    let mut total_ascent_m = 0.0;
    let mut total_descent_m = 0.0;
    let mut min_elevation_m = f64::MAX;
    let mut max_elevation_m = f64::MIN;
    let mut prev_elevation: Option<f64> = None;

    // Calculate bounding box
    let mut min_lat = f64::MAX;
    let mut max_lat = f64::MIN;
    let mut min_lng = f64::MAX;
    let mut max_lng = f64::MIN;

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

    // Generate route ID from activity timestamp
    let route_id = format!("route-{}", activity_id.replace([':', '-', 'T', 'Z'], ""));

    Some(GpsRoute {
        route_id,
        name: None,
        total_distance_m: if total_distance_m > 0.0 {
            Some(total_distance_m)
        } else {
            None
        },
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
        recording_mode: Some("every_second".to_string()), // TCX typically uses per-second recording
        gps_fix: None,                                    // TCX doesn't include GPS fix quality
        positions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcx_to_pwf_compiles() {
        // Placeholder test to ensure function signature compiles
        let _: fn(std::io::Cursor<Vec<u8>>, bool) -> Result<ConversionResult, ConversionError> =
            tcx_to_pwf;
    }

    #[test]
    fn test_create_empty_history() {
        let history = create_empty_history();
        assert_eq!(history.history_version, 2);
        assert_eq!(history.workouts.len(), 0);
        assert_eq!(history.personal_records.len(), 0);
        assert_eq!(history.body_measurements.len(), 0);
    }
}
