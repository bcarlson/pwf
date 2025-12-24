//! PWF to GPX conversion logic

use crate::error::{ConversionError, ConversionWarning, GpxExportResult};
use gpx::{Gpx, GpxVersion, Track, TrackSegment, Waypoint};
use pwf_core::history::{GpsRoute, Workout, WpsHistory};

/// Convert PWF history to GPX XML format
///
/// # Arguments
/// * `history` - PWF history structure to export
///
/// # Returns
/// GpxExportResult with GPX XML and any warnings
///
/// # Example
/// ```no_run
/// use pwf_converters::gpx::pwf_to_gpx;
/// use pwf_core::history::WpsHistory;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Parse PWF history from YAML
/// let yaml_content = std::fs::read_to_string("workout.yaml")?;
/// let history: WpsHistory = pwf_core::history::parse(&yaml_content)?;
///
/// // Export to GPX
/// let result = pwf_to_gpx(&history)?;
/// println!("GPX: {}", result.gpx_xml);
/// # Ok(())
/// # }
/// ```
pub fn pwf_to_gpx(history: &WpsHistory) -> Result<GpxExportResult, ConversionError> {
    let mut result = GpxExportResult::new(String::new());

    // Create GPX structure
    let mut gpx = Gpx {
        version: GpxVersion::Gpx11,
        creator: Some("PWF Converters".to_string()),
        ..Default::default()
    };

    // Convert workouts with GPS routes to GPX tracks
    let mut tracks = Vec::new();
    for workout in &history.workouts {
        // Only export workouts that have GPS data
        if let Some(ref telemetry) = workout.telemetry {
            if let Some(ref gps_route) = telemetry.gps_route {
                match convert_workout_to_track(workout, gps_route, &mut result) {
                    Ok(track) => tracks.push(track),
                    Err(e) => {
                        result.add_warning(ConversionWarning::DataQualityIssue {
                            issue: format!("Failed to convert workout to track: {}", e),
                        });
                    }
                }
            } else {
                // Workout has no GPS route
                result.add_warning(ConversionWarning::MissingField {
                    source_field: "gps_route".to_string(),
                    reason: format!(
                        "Workout '{}' has no GPS route data",
                        workout.title.as_ref().unwrap_or(&workout.date)
                    ),
                });
            }
        } else {
            // Workout has no telemetry
            result.add_warning(ConversionWarning::MissingField {
                source_field: "telemetry".to_string(),
                reason: format!(
                    "Workout '{}' has no telemetry data",
                    workout.title.as_ref().unwrap_or(&workout.date)
                ),
            });
        }
    }

    if tracks.is_empty() {
        result.add_warning(ConversionWarning::DataQualityIssue {
            issue: "No GPS routes found in any workouts".to_string(),
        });
    }

    gpx.tracks = tracks;

    // Serialize to GPX XML
    result.gpx_xml = serialize_gpx(&gpx)?;

    Ok(result)
}

/// Convert a PWF Workout with GPS route to a GPX Track
fn convert_workout_to_track(
    workout: &Workout,
    gps_route: &GpsRoute,
    result: &mut GpxExportResult,
) -> Result<Track, ConversionError> {
    // Create track with workout metadata
    let mut track = Track {
        name: Some(
            workout
                .title
                .clone()
                .unwrap_or_else(|| format!("Workout {}", workout.date)),
        ),
        description: workout.notes.clone(),
        ..Default::default()
    };

    // Convert GPS positions to trackpoints
    if gps_route.positions.is_empty() {
        result.add_warning(ConversionWarning::DataQualityIssue {
            issue: format!(
                "GPS route '{}' has no positions",
                gps_route.name.as_ref().unwrap_or(&gps_route.route_id)
            ),
        });
        return Err(ConversionError::InvalidGpxData(
            "GPS route has no positions".to_string(),
        ));
    }

    let mut segment = TrackSegment::new();
    for position in &gps_route.positions {
        let waypoint = convert_position_to_waypoint(position, result)?;
        segment.points.push(waypoint);
    }

    track.segments.push(segment);

    Ok(track)
}

/// Convert a PWF GPS position to a GPX Waypoint
fn convert_position_to_waypoint(
    position: &pwf_core::history::GpsPosition,
    _result: &mut GpxExportResult,
) -> Result<Waypoint, ConversionError> {
    let mut waypoint = Waypoint::new(geo_types::Point::new(
        position.longitude_deg,
        position.latitude_deg,
    ));

    // Set elevation if present
    if let Some(elevation) = position.elevation_m {
        waypoint.elevation = Some(elevation);
    }

    // Parse and set timestamp
    // The timestamp string is in ISO 8601 format (e.g., "2024-01-15T14:30:00Z")
    // gpx::Time implements From<time::OffsetDateTime>
    if let Ok(time_str) = time::OffsetDateTime::parse(
        &position.timestamp,
        &time::format_description::well_known::Iso8601::DEFAULT,
    ) {
        waypoint.time = Some(gpx::Time::from(time_str));
    }

    // Note: GPX 1.1 standard doesn't directly support heart rate, power, cadence
    // These would require Garmin TrackPointExtension v2
    // For now, we'll just include basic GPS data (lat/lon/ele/time)
    // TODO: Add support for Garmin GPX extensions for heart rate, power, cadence

    Ok(waypoint)
}

/// Serialize GPX structure to XML string
fn serialize_gpx(gpx: &Gpx) -> Result<String, ConversionError> {
    let mut buffer = Vec::new();
    gpx::write(gpx, &mut buffer).map_err(|e| ConversionError::GpxWriteError(e.to_string()))?;

    String::from_utf8(buffer)
        .map_err(|e| ConversionError::GpxWriteError(format!("UTF-8 encoding error: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pwf_core::history::{GpsPosition, WorkoutTelemetry};

    #[test]
    fn test_pwf_to_gpx_empty_history() {
        let history = WpsHistory {
            history_version: 1,
            exported_at: "2024-01-15T16:00:00Z".to_string(),
            export_source: None,
            units: Default::default(),
            personal_records: vec![],
            body_measurements: vec![],
            workouts: vec![],
        };

        let result = pwf_to_gpx(&history).unwrap();
        assert!(result.has_warnings());
        assert!(result.gpx_xml.contains("<?xml"));
    }

    #[test]
    fn test_pwf_to_gpx_with_gps_route() {
        let history = WpsHistory {
            history_version: 1,
            exported_at: "2024-01-15T16:00:00Z".to_string(),
            export_source: None,
            units: Default::default(),
            personal_records: vec![],
            body_measurements: vec![],
            workouts: vec![Workout {
                id: Some("workout1".to_string()),
                date: "2024-01-15".to_string(),
                started_at: Some("2024-01-15T14:30:00Z".to_string()),
                ended_at: None,
                duration_sec: Some(3600),
                title: Some("Morning Run".to_string()),
                notes: Some("Beautiful day!".to_string()),
                plan_id: None,
                plan_day_id: None,
                exercises: vec![],
                telemetry: Some(WorkoutTelemetry {
                    gps_route: Some(GpsRoute {
                        route_id: "route1".to_string(),
                        name: Some("Run Route".to_string()),
                        positions: vec![
                            GpsPosition {
                                latitude_deg: 37.7749,
                                longitude_deg: -122.4194,
                                timestamp: "2024-01-15T14:30:00Z".to_string(),
                                elevation_m: Some(100.0),
                                accuracy_m: None,
                                speed_mps: Some(2.5),
                                heading_deg: None,
                                heart_rate_bpm: Some(140),
                                power_watts: None,
                                cadence: Some(85),
                                temperature_c: Some(15.0),
                            },
                            GpsPosition {
                                latitude_deg: 37.7750,
                                longitude_deg: -122.4195,
                                timestamp: "2024-01-15T14:31:00Z".to_string(),
                                elevation_m: Some(102.0),
                                accuracy_m: None,
                                speed_mps: Some(2.6),
                                heading_deg: None,
                                heart_rate_bpm: Some(142),
                                power_watts: None,
                                cadence: Some(86),
                                temperature_c: Some(15.5),
                            },
                        ],
                        total_distance_m: Some(1000.0),
                        total_ascent_m: Some(50.0),
                        total_descent_m: Some(20.0),
                        min_elevation_m: Some(90.0),
                        max_elevation_m: Some(120.0),
                        bbox_sw_lat: None,
                        bbox_sw_lng: None,
                        bbox_ne_lat: None,
                        bbox_ne_lng: None,
                        recording_mode: None,
                        gps_fix: None,
                    }),
                    ..Default::default()
                }),
                devices: vec![],
                sport: None,
                sport_segments: None,
            }],
        };

        let result = pwf_to_gpx(&history).unwrap();
        assert!(!result.has_warnings() || result.warnings.is_empty());
        assert!(result.gpx_xml.contains("<?xml"));
        assert!(result.gpx_xml.contains("Morning Run"));
        assert!(result.gpx_xml.contains("37.7749"));
        assert!(result.gpx_xml.contains("-122.4194"));
    }

    #[test]
    fn test_pwf_to_gpx_no_gps_data() {
        let history = WpsHistory {
            history_version: 1,
            exported_at: "2024-01-15T16:00:00Z".to_string(),
            export_source: None,
            units: Default::default(),
            personal_records: vec![],
            body_measurements: vec![],
            workouts: vec![Workout {
                id: Some("workout1".to_string()),
                date: "2024-01-15".to_string(),
                started_at: Some("2024-01-15T14:30:00Z".to_string()),
                ended_at: None,
                duration_sec: Some(3600),
                title: Some("Strength Training".to_string()),
                notes: None,
                plan_id: None,
                plan_day_id: None,
                exercises: vec![],
                telemetry: None,
                devices: vec![],
                sport: None,
                sport_segments: None,
            }],
        };

        let result = pwf_to_gpx(&history).unwrap();
        assert!(result.has_warnings());
        // Should warn about missing telemetry
        assert!(result
            .warnings
            .iter()
            .any(|w| matches!(w, ConversionWarning::MissingField { .. })));
    }

    #[test]
    fn test_convert_position_to_waypoint() {
        let position = GpsPosition {
            latitude_deg: 37.7749,
            longitude_deg: -122.4194,
            timestamp: "2024-01-15T14:30:00Z".to_string(),
            elevation_m: Some(100.5),
            accuracy_m: Some(5.0),
            speed_mps: Some(2.5),
            heading_deg: Some(180.0),
            heart_rate_bpm: Some(140),
            power_watts: Some(200),
            cadence: Some(85),
            temperature_c: Some(15.0),
        };

        let mut result = GpxExportResult::new(String::new());
        let waypoint = convert_position_to_waypoint(&position, &mut result).unwrap();

        assert_eq!(waypoint.point().x(), -122.4194);
        assert_eq!(waypoint.point().y(), 37.7749);
        assert_eq!(waypoint.elevation, Some(100.5));
        assert!(waypoint.time.is_some());
    }
}
