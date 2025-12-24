//! GPX to PWF conversion logic
//!
//! Converts GPX (GPS Exchange Format) files to PWF history format.

use crate::error::{ConversionError, ConversionResult, ConversionWarning};
use crate::gpx::mappings::{infer_sport_from_metadata, map_gpx_type_to_sport};
use gpx::{Gpx, Waypoint};
use pwf_core::history::{
    CompletedExercise, CompletedSet, ExportSource, GpsPosition, GpsRoute, SetTelemetry,
    TimeSeriesData, Units, Workout, WorkoutTelemetry, WpsHistory,
};
use std::io::Read;

/// Convert GPX file to PWF history format
///
/// # Arguments
/// * `reader` - GPX file reader
/// * `summary_only` - If true, skip time-series position data (only summary metrics)
///
/// # Returns
/// ConversionResult with PWF YAML and any warnings
///
/// # Example
/// ```no_run
/// use pwf_converters::gpx::parser::gpx_to_pwf;
/// use std::fs::File;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let gpx_file = File::open("route.gpx")?;
/// let result = gpx_to_pwf(gpx_file, false)?;
///
/// // Check warnings
/// for warning in &result.warnings {
///     println!("Warning: {}", warning);
/// }
///
/// // Save PWF YAML
/// std::fs::write("workout.yaml", &result.pwf_yaml)?;
/// # Ok(())
/// # }
/// ```
pub fn gpx_to_pwf<R: Read>(
    reader: R,
    summary_only: bool,
) -> Result<ConversionResult, ConversionError> {
    let mut result = ConversionResult::new(String::new());

    // Parse GPX file
    let gpx: Gpx = gpx::read(reader).map_err(|e| {
        ConversionError::GpxReadError(format!("Failed to parse GPX file: {}", e))
    })?;

    // Create PWF history structure
    let mut workouts = Vec::new();

    // Convert tracks to workouts
    for (track_idx, track) in gpx.tracks.iter().enumerate() {
        match convert_track_to_workout(track_idx, track, &gpx, summary_only, &mut result) {
            Ok(workout) => workouts.push(workout),
            Err(e) => {
                result.add_warning(ConversionWarning::DataQualityIssue {
                    issue: format!("Failed to convert track {}: {}", track_idx, e),
                });
            }
        }
    }

    // If no tracks found, check for routes
    if workouts.is_empty() && !gpx.routes.is_empty() {
        result.add_warning(ConversionWarning::UnsupportedFeature {
            feature: "GPX routes are not fully supported. Use tracks instead.".to_string(),
        });
    }

    // If still no workouts, check for waypoints only
    if workouts.is_empty() && !gpx.waypoints.is_empty() {
        result.add_warning(ConversionWarning::UnsupportedFeature {
            feature: "GPX file contains only waypoints. Waypoint-only files cannot be converted to workouts.".to_string(),
        });
    }

    if workouts.is_empty() {
        return Err(ConversionError::InvalidGpxData(
            "No valid tracks found in GPX file".to_string(),
        ));
    }

    // Create history export
    let history = WpsHistory {
        history_version: 2,
        exported_at: chrono::Utc::now().to_rfc3339(),
        export_source: Some(ExportSource {
            app_name: Some(format!(
                "GPX Import ({})",
                gpx.creator.as_deref().unwrap_or("Unknown")
            )),
            app_version: None,
            platform: None,
            preferred_units: None,
        }),
        units: Units::default(),
        workouts,
        personal_records: vec![],
        body_measurements: vec![],
    };

    // Serialize to YAML
    let pwf_yaml = serde_yaml::to_string(&history).map_err(ConversionError::YamlError)?;
    result.pwf_yaml = pwf_yaml;

    Ok(result)
}

/// Convert a GPX track to a PWF workout
fn convert_track_to_workout(
    track_idx: usize,
    track: &gpx::Track,
    gpx: &Gpx,
    summary_only: bool,
    result: &mut ConversionResult,
) -> Result<Workout, ConversionError> {
    // Extract track metadata
    let title = track
        .name
        .clone()
        .or_else(|| Some(format!("GPX Track {}", track_idx + 1)));

    let notes = track.description.clone().or_else(|| track.comment.clone());

    // Determine sport type from track type or metadata
    let sport_string = track
        .type_
        .as_ref()
        .map(|t| map_gpx_type_to_sport(Some(t.as_str())))
        .unwrap_or_else(|| infer_sport_from_metadata(gpx));

    // Convert sport string to Sport enum
    let sport = sport_string.parse().unwrap_or(pwf_core::Sport::Other);

    // Combine all track segments
    let mut all_positions = Vec::new();
    let mut all_waypoints = Vec::new();

    for segment in &track.segments {
        for point in &segment.points {
            all_waypoints.push(point);
            if !summary_only {
                all_positions.push(convert_waypoint_to_gps_position(point));
            }
        }
    }

    if all_waypoints.is_empty() {
        return Err(ConversionError::InvalidGpxData(
            "Track has no points".to_string(),
        ));
    }

    // Calculate basic statistics
    let (start_time, end_time, duration_sec) = extract_time_info(&all_waypoints);
    let distance_m = calculate_total_distance(&all_waypoints);

    // Calculate elevation statistics
    let (min_elev, max_elev, ascent, descent) = calculate_elevation_stats(&all_waypoints);

    // Calculate bounding box
    let bbox = calculate_bounding_box(&all_waypoints);

    // Create GPS route
    let gps_route = if !summary_only && !all_positions.is_empty() {
        Some(GpsRoute {
            route_id: format!("gpx-track-{}", track_idx),
            name: track.name.clone(),
            positions: all_positions.clone(),
            total_distance_m: Some(distance_m),
            total_ascent_m: ascent,
            total_descent_m: descent,
            min_elevation_m: min_elev,
            max_elevation_m: max_elev,
            bbox_sw_lat: bbox.as_ref().map(|b| b.0),
            bbox_sw_lng: bbox.as_ref().map(|b| b.1),
            bbox_ne_lat: bbox.as_ref().map(|b| b.2),
            bbox_ne_lng: bbox.as_ref().map(|b| b.3),
            recording_mode: None,
            gps_fix: None, // GPX doesn't specify fix quality
        })
    } else {
        None
    };

    // Create time series data from waypoints
    let time_series = if !summary_only && !all_waypoints.is_empty() {
        Some(create_time_series_from_waypoints(&all_waypoints, result))
    } else {
        None
    };

    // Calculate average heart rate if available
    let avg_heart_rate = calculate_average_heart_rate(&all_waypoints);

    // Create workout telemetry
    let telemetry = Some(WorkoutTelemetry {
        gps_route,
        heart_rate_avg: avg_heart_rate,
        total_distance_m: Some(distance_m),
        ..Default::default()
    });

    // Create a single exercise representing the entire track
    let exercise = CompletedExercise {
        id: None,
        name: track
            .name
            .clone()
            .unwrap_or_else(|| "GPS Activity".to_string()),
        modality: None,
        sport: Some(sport),
        notes: track.comment.clone(),
        sets: vec![CompletedSet {
            set_number: Some(1),
            set_type: None,
            reps: None,
            weight_kg: None,
            weight_lb: None,
            duration_sec,
            distance_meters: Some(distance_m),
            rpe: None,
            rir: None,
            notes: None,
            is_pr: None,
            completed_at: start_time.clone(),
            telemetry: Some(SetTelemetry {
                time_series,
                ..Default::default()
            }),
            swimming: None,
        }],
        pool_config: None,
    };

    // Create workout
    let workout = Workout {
        id: None,
        date: extract_date_from_time(&start_time),
        started_at: start_time.clone(),
        ended_at: end_time,
        duration_sec,
        title,
        notes,
        plan_id: None,
        plan_day_id: None,
        exercises: vec![exercise],
        telemetry,
        devices: vec![],
        sport: Some(sport),
        sport_segments: None,
    };

    Ok(workout)
}

/// Convert GPX waypoint to PWF GPS position
fn convert_waypoint_to_gps_position(waypoint: &Waypoint) -> GpsPosition {
    GpsPosition {
        latitude_deg: waypoint.point().y(),
        longitude_deg: waypoint.point().x(),
        elevation_m: waypoint.elevation,
        timestamp: waypoint
            .time
            .as_ref()
            .and_then(|t| t.format().ok())
            .unwrap_or_default(),
        accuracy_m: None,
        speed_mps: None,
        heading_deg: None,
        heart_rate_bpm: None,
        cadence: None,
        power_watts: None,
        temperature_c: None,
    }
}

/// Create time series data from GPX waypoints
fn create_time_series_from_waypoints(
    waypoints: &[&Waypoint],
    result: &mut ConversionResult,
) -> TimeSeriesData {
    let mut timestamps = Vec::new();
    let mut elevations = Vec::new();
    let mut latitudes = Vec::new();
    let mut longitudes = Vec::new();
    let mut distances = Vec::new();
    let mut heart_rates = Vec::new();

    let mut cumulative_distance = 0.0;
    let mut prev_point: Option<&Waypoint> = None;

    let mut has_heart_rate = false;

    for waypoint in waypoints {
        // Timestamp (required for time series)
        if let Some(time) = &waypoint.time {
            if let Ok(formatted) = time.format() {
                timestamps.push(formatted);
            } else {
                result.add_warning(ConversionWarning::DataQualityIssue {
                    issue: "Failed to format timestamp".to_string(),
                });
                continue;
            }
        } else {
            // Skip points without timestamps
            result.add_warning(ConversionWarning::DataQualityIssue {
                issue: "Some track points missing timestamps - skipped".to_string(),
            });
            continue;
        }

        // Elevation
        if let Some(elev) = waypoint.elevation {
            elevations.push(elev);
        }

        // Position
        latitudes.push(waypoint.point().y());
        longitudes.push(waypoint.point().x());

        // Calculate distance
        if let Some(prev) = prev_point {
            let dist = calculate_distance(prev, waypoint);
            cumulative_distance += dist;
        }
        distances.push(cumulative_distance);
        prev_point = Some(waypoint);

        // Heart rate (from GPX extensions if available)
        // Note: Standard GPX doesn't have HR, but Garmin extensions do
        // For now, we'll leave this empty unless we parse extensions
        if let Some(hr) = extract_heart_rate_from_extensions(waypoint) {
            heart_rates.push(hr);
            has_heart_rate = true;
        }
    }

    if timestamps.is_empty() {
        result.add_warning(ConversionWarning::DataQualityIssue {
            issue: "No valid timestamps found in track points".to_string(),
        });
    }

    TimeSeriesData {
        timestamps,
        elevation_m: if !elevations.is_empty() {
            Some(elevations)
        } else {
            None
        },
        latitude: if !latitudes.is_empty() {
            Some(latitudes)
        } else {
            None
        },
        longitude: if !longitudes.is_empty() {
            Some(longitudes)
        } else {
            None
        },
        distance_m: if !distances.is_empty() {
            Some(distances)
        } else {
            None
        },
        heart_rate: if has_heart_rate {
            Some(heart_rates)
        } else {
            None
        },
        ..Default::default()
    }
}

/// Extract time information from waypoints
fn extract_time_info(waypoints: &[&Waypoint]) -> (Option<String>, Option<String>, Option<u32>) {
    let start_time = waypoints
        .first()
        .and_then(|wp| wp.time.as_ref())
        .and_then(|t| t.format().ok());

    let end_time = waypoints
        .last()
        .and_then(|wp| wp.time.as_ref())
        .and_then(|t| t.format().ok());

    let duration_sec = if let (Some(start_wp), Some(end_wp)) = (waypoints.first(), waypoints.last())
    {
        if let (Some(start_time), Some(end_time)) = (&start_wp.time, &end_wp.time) {
            // Calculate duration by parsing the timestamps
            if let (Ok(start_str), Ok(end_str)) = (start_time.format(), end_time.format()) {
                // Parse ISO 8601 timestamps and calculate difference
                if let (Ok(start_dt), Ok(end_dt)) = (
                    chrono::DateTime::parse_from_rfc3339(&start_str),
                    chrono::DateTime::parse_from_rfc3339(&end_str),
                ) {
                    let duration = end_dt.signed_duration_since(start_dt);
                    Some(duration.num_seconds().max(0) as u32)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    (start_time, end_time, duration_sec)
}

/// Extract date from ISO 8601 timestamp
fn extract_date_from_time(timestamp: &Option<String>) -> String {
    timestamp
        .as_ref()
        .and_then(|t| t.split('T').next())
        .unwrap_or("2025-01-01")
        .to_string()
}

/// Calculate total distance along the track
fn calculate_total_distance(waypoints: &[&Waypoint]) -> f64 {
    let mut total = 0.0;
    for i in 1..waypoints.len() {
        total += calculate_distance(waypoints[i - 1], waypoints[i]);
    }
    total
}

/// Calculate distance between two waypoints using Haversine formula
fn calculate_distance(wp1: &Waypoint, wp2: &Waypoint) -> f64 {
    let lat1 = wp1.point().y().to_radians();
    let lat2 = wp2.point().y().to_radians();
    let lon1 = wp1.point().x().to_radians();
    let lon2 = wp2.point().x().to_radians();

    let dlat = lat2 - lat1;
    let dlon = lon2 - lon1;

    let a = (dlat / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    const EARTH_RADIUS_M: f64 = 6371000.0; // meters
    EARTH_RADIUS_M * c
}

/// Calculate elevation statistics from waypoints
fn calculate_elevation_stats(
    waypoints: &[&Waypoint],
) -> (Option<f64>, Option<f64>, Option<f64>, Option<f64>) {
    let elevations: Vec<f64> = waypoints.iter().filter_map(|wp| wp.elevation).collect();

    if elevations.is_empty() {
        return (None, None, None, None);
    }

    let min_elev = elevations.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_elev = elevations.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    // Calculate total ascent and descent
    let mut total_ascent = 0.0;
    let mut total_descent = 0.0;

    for i in 1..elevations.len() {
        let diff = elevations[i] - elevations[i - 1];
        if diff > 0.0 {
            total_ascent += diff;
        } else if diff < 0.0 {
            total_descent += diff.abs();
        }
    }

    (
        Some(min_elev),
        Some(max_elev),
        Some(total_ascent),
        Some(total_descent),
    )
}

/// Calculate bounding box from waypoints
/// Returns (sw_lat, sw_lng, ne_lat, ne_lng)
fn calculate_bounding_box(waypoints: &[&Waypoint]) -> Option<(f64, f64, f64, f64)> {
    if waypoints.is_empty() {
        return None;
    }

    let mut min_lat = f64::INFINITY;
    let mut max_lat = f64::NEG_INFINITY;
    let mut min_lng = f64::INFINITY;
    let mut max_lng = f64::NEG_INFINITY;

    for wp in waypoints {
        let lat = wp.point().y();
        let lng = wp.point().x();

        min_lat = min_lat.min(lat);
        max_lat = max_lat.max(lat);
        min_lng = min_lng.min(lng);
        max_lng = max_lng.max(lng);
    }

    Some((min_lat, min_lng, max_lat, max_lng))
}

/// Calculate average heart rate from waypoints
fn calculate_average_heart_rate(waypoints: &[&Waypoint]) -> Option<u32> {
    let heart_rates: Vec<u32> = waypoints
        .iter()
        .filter_map(|wp| extract_heart_rate_from_extensions(wp))
        .collect();

    if heart_rates.is_empty() {
        None
    } else {
        let sum: u32 = heart_rates.iter().sum();
        Some(sum / heart_rates.len() as u32)
    }
}

/// Extract heart rate from Garmin GPX extensions (if available)
/// Returns None for standard GPX without extensions
fn extract_heart_rate_from_extensions(_waypoint: &Waypoint) -> Option<u32> {
    // TODO: Parse Garmin TrackPointExtension for heart rate
    // For now, standard GPX doesn't include HR data
    // This would require parsing the extensions XML
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn create_test_gpx() -> String {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<gpx version="1.1" creator="Test">
  <metadata>
    <name>Test Route</name>
  </metadata>
  <trk>
    <name>Morning Run</name>
    <type>running</type>
    <trkseg>
      <trkpt lat="37.7749" lon="-122.4194">
        <ele>10.0</ele>
        <time>2025-01-15T14:30:00Z</time>
      </trkpt>
      <trkpt lat="37.7750" lon="-122.4195">
        <ele>11.0</ele>
        <time>2025-01-15T14:30:10Z</time>
      </trkpt>
      <trkpt lat="37.7751" lon="-122.4196">
        <ele>12.0</ele>
        <time>2025-01-15T14:30:20Z</time>
      </trkpt>
    </trkseg>
  </trk>
</gpx>"#
            .to_string()
    }

    #[test]
    fn test_gpx_to_pwf_basic() {
        let gpx_data = create_test_gpx();
        let cursor = Cursor::new(gpx_data.as_bytes());

        let result = gpx_to_pwf(cursor, false).unwrap();

        assert!(!result.pwf_yaml.is_empty());
        assert!(result.pwf_yaml.contains("Morning Run"));
        assert!(result.pwf_yaml.contains("running"));
    }

    #[test]
    fn test_gpx_to_pwf_summary_only() {
        let gpx_data = create_test_gpx();
        let cursor = Cursor::new(gpx_data.as_bytes());

        let result = gpx_to_pwf(cursor, true).unwrap();

        assert!(!result.pwf_yaml.is_empty());
        // Summary only should not include full position arrays
        assert!(result.pwf_yaml.contains("Morning Run"));
    }

    #[test]
    fn test_convert_waypoint_to_gps_position() {
        let gpx_data = create_test_gpx();
        let gpx: Gpx = gpx::read(Cursor::new(gpx_data.as_bytes())).unwrap();

        let track = &gpx.tracks[0];
        let segment = &track.segments[0];
        let waypoint = &segment.points[0];

        let pos = convert_waypoint_to_gps_position(waypoint);

        assert_eq!(pos.latitude_deg, 37.7749);
        assert_eq!(pos.longitude_deg, -122.4194);
        assert_eq!(pos.elevation_m, Some(10.0));
    }

    #[test]
    fn test_calculate_distance() {
        let gpx_data = create_test_gpx();
        let gpx: Gpx = gpx::read(Cursor::new(gpx_data.as_bytes())).unwrap();

        let track = &gpx.tracks[0];
        let segment = &track.segments[0];
        let wp1 = &segment.points[0];
        let wp2 = &segment.points[1];

        let distance = calculate_distance(wp1, wp2);

        // Should be a small distance (roughly 10-20 meters)
        assert!(distance > 0.0);
        assert!(distance < 100.0);
    }
}
