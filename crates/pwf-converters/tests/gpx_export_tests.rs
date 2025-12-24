//! Integration tests for PWF to GPX export

use pwf_converters::{pwf_to_gpx, ConversionWarning};
use pwf_core::history::{GpsPosition, GpsRoute, Workout, WorkoutTelemetry, WpsHistory};
use std::io::Cursor;

/// Helper to create a PWF history with GPS data
fn create_pwf_with_gps() -> WpsHistory {
    WpsHistory {
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
            ended_at: Some("2024-01-15T15:30:00Z".to_string()),
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
    }
}

#[test]
fn test_pwf_to_gpx_basic_export() {
    let history = create_pwf_with_gps();
    let result = pwf_to_gpx(&history).unwrap();

    // Check that GPX XML was generated
    assert!(!result.gpx_xml.is_empty());
    assert!(result.gpx_xml.contains("<?xml"));
    assert!(result.gpx_xml.contains("<gpx"));

    // Check for track data
    assert!(result.gpx_xml.contains("<trk>"));
    assert!(result.gpx_xml.contains("Morning Run"));

    // Check for trackpoints
    assert!(result.gpx_xml.contains("<trkpt"));
    assert!(result.gpx_xml.contains("37.7749"));
    assert!(result.gpx_xml.contains("-122.4194"));

    // Check for elevation
    assert!(result.gpx_xml.contains("<ele>100"));

    // No warnings for valid data
    assert!(!result.has_warnings());
}

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

    // Should still generate valid GPX
    assert!(result.gpx_xml.contains("<?xml"));
    assert!(result.gpx_xml.contains("<gpx"));

    // Should warn about no GPS routes
    assert!(result.has_warnings());
    assert!(result
        .warnings
        .iter()
        .any(|w| matches!(w, ConversionWarning::DataQualityIssue { .. })));
}

#[test]
fn test_pwf_to_gpx_workout_without_gps() {
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

    // Should warn about missing telemetry
    assert!(result.has_warnings());
    assert!(result
        .warnings
        .iter()
        .any(|w| matches!(w, ConversionWarning::MissingField { .. })));
}

#[test]
fn test_pwf_to_gpx_multiple_workouts() {
    let mut history = create_pwf_with_gps();

    // Add a second workout with GPS
    history.workouts.push(Workout {
        id: Some("workout2".to_string()),
        date: "2024-01-16".to_string(),
        started_at: Some("2024-01-16T08:00:00Z".to_string()),
        ended_at: Some("2024-01-16T09:00:00Z".to_string()),
        duration_sec: Some(3600),
        title: Some("Evening Ride".to_string()),
        notes: None,
        plan_id: None,
        plan_day_id: None,
        exercises: vec![],
        telemetry: Some(WorkoutTelemetry {
            gps_route: Some(GpsRoute {
                route_id: "route2".to_string(),
                name: Some("Bike Route".to_string()),
                positions: vec![GpsPosition {
                    latitude_deg: 37.8049,
                    longitude_deg: -122.4294,
                    timestamp: "2024-01-16T08:00:00Z".to_string(),
                    elevation_m: Some(50.0),
                    accuracy_m: None,
                    speed_mps: Some(8.0),
                    heading_deg: None,
                    heart_rate_bpm: Some(150),
                    power_watts: Some(200),
                    cadence: Some(90),
                    temperature_c: Some(18.0),
                }],
                total_distance_m: Some(5000.0),
                total_ascent_m: Some(100.0),
                total_descent_m: Some(80.0),
                min_elevation_m: Some(40.0),
                max_elevation_m: Some(110.0),
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
    });

    let result = pwf_to_gpx(&history).unwrap();

    // Should contain both tracks
    assert!(result.gpx_xml.contains("Morning Run"));
    assert!(result.gpx_xml.contains("Evening Ride"));

    // Check both coordinate sets
    assert!(result.gpx_xml.contains("37.7749"));
    assert!(result.gpx_xml.contains("37.8049"));
}

#[test]
fn test_pwf_to_gpx_preserves_timestamps() {
    let history = create_pwf_with_gps();
    let result = pwf_to_gpx(&history).unwrap();

    // GPX should contain timestamp elements
    assert!(result.gpx_xml.contains("<time>"));

    // Should contain our timestamps (converted to proper format)
    assert!(result.gpx_xml.contains("2024-01-15"));
}

#[test]
fn test_pwf_to_gpx_valid_xml() {
    let history = create_pwf_with_gps();
    let result = pwf_to_gpx(&history).unwrap();

    // Try to parse the GPX XML back
    let cursor = Cursor::new(result.gpx_xml.as_bytes());
    let gpx_result = gpx::read(cursor);

    // Should parse successfully
    assert!(gpx_result.is_ok(), "Generated GPX should be valid XML");

    let gpx = gpx_result.unwrap();
    assert_eq!(gpx.version, gpx::GpxVersion::Gpx11);
    assert_eq!(gpx.tracks.len(), 1);
    assert_eq!(gpx.tracks[0].segments.len(), 1);
    assert_eq!(gpx.tracks[0].segments[0].points.len(), 2);
}
