//! Integration tests for TCX to PWF conversion

use pwf_converters::{tcx_to_pwf, ConversionWarning};
use std::io::Cursor;

/// Helper to create a minimal valid TCX XML
fn create_minimal_tcx() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<TrainingCenterDatabase xmlns="http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2">
  <Activities>
    <Activity Sport="Running">
      <Id>2024-01-15T10:00:00Z</Id>
      <Lap StartTime="2024-01-15T10:00:00Z">
        <TotalTimeSeconds>1800</TotalTimeSeconds>
        <DistanceMeters>5000</DistanceMeters>
        <Calories>300</Calories>
        <Intensity>Active</Intensity>
        <TriggerMethod>Manual</TriggerMethod>
        <Track>
          <Trackpoint>
            <Time>2024-01-15T10:00:00Z</Time>
            <Position>
              <LatitudeDegrees>37.7749</LatitudeDegrees>
              <LongitudeDegrees>-122.4194</LongitudeDegrees>
            </Position>
            <AltitudeMeters>50.0</AltitudeMeters>
            <HeartRateBpm>
              <Value>140</Value>
            </HeartRateBpm>
          </Trackpoint>
        </Track>
      </Lap>
    </Activity>
  </Activities>
</TrainingCenterDatabase>"#
        .to_string()
}

#[test]
fn test_tcx_to_pwf_minimal() {
    let tcx_xml = create_minimal_tcx();
    let cursor = Cursor::new(tcx_xml.as_bytes());

    let result = tcx_to_pwf(cursor, false);
    assert!(result.is_ok(), "TCX parsing should succeed");

    let conversion = result.unwrap();
    assert!(!conversion.pwf_yaml.is_empty(), "Should generate PWF YAML");

    // Parse the generated YAML
    let history: serde_yaml::Value = serde_yaml::from_str(&conversion.pwf_yaml).unwrap();

    // Check basic structure
    assert_eq!(history["history_version"], 2);
    assert!(history["workouts"].is_sequence());

    let workouts = history["workouts"].as_sequence().unwrap();
    assert_eq!(workouts.len(), 1, "Should have one workout");

    let workout = &workouts[0];
    assert_eq!(workout["date"], "2024-01-15");
    assert_eq!(workout["started_at"], "2024-01-15T10:00:00Z");
    assert_eq!(workout["duration_sec"], 1800);
    assert_eq!(workout["sport"], "running");
}

#[test]
fn test_tcx_to_pwf_with_telemetry() {
    let tcx_xml = create_minimal_tcx();
    let cursor = Cursor::new(tcx_xml.as_bytes());

    let result = tcx_to_pwf(cursor, false).unwrap();
    let history: serde_yaml::Value = serde_yaml::from_str(&result.pwf_yaml).unwrap();

    let workout = &history["workouts"][0];
    let telemetry = &workout["telemetry"];

    assert!(telemetry["total_distance_km"].is_number());
    assert_eq!(telemetry["total_distance_km"].as_f64().unwrap(), 5.0);
    assert_eq!(telemetry["total_calories"], 300);
}

#[test]
fn test_tcx_to_pwf_with_gps() {
    let tcx_xml = create_minimal_tcx();
    let cursor = Cursor::new(tcx_xml.as_bytes());

    let result = tcx_to_pwf(cursor, false).unwrap();
    let history: serde_yaml::Value = serde_yaml::from_str(&result.pwf_yaml).unwrap();

    let workout = &history["workouts"][0];
    let telemetry = &workout["telemetry"];

    assert!(telemetry["gps_route"].is_mapping());
    let gps_route = &telemetry["gps_route"];

    assert!(gps_route["positions"].is_sequence());
    let positions = gps_route["positions"].as_sequence().unwrap();
    assert_eq!(positions.len(), 1);

    let position = &positions[0];
    assert_eq!(position["latitude_deg"].as_f64().unwrap(), 37.7749);
    assert_eq!(position["longitude_deg"].as_f64().unwrap(), -122.4194);
    assert_eq!(position["elevation_m"], 50.0);
    assert_eq!(position["heart_rate_bpm"], 140);
}

#[test]
fn test_tcx_to_pwf_summary_only() {
    let tcx_xml = create_minimal_tcx();
    let cursor = Cursor::new(tcx_xml.as_bytes());

    // With summary_only=true, GPS data should be excluded
    let result = tcx_to_pwf(cursor, true).unwrap();
    let history: serde_yaml::Value = serde_yaml::from_str(&result.pwf_yaml).unwrap();

    let workout = &history["workouts"][0];
    let telemetry = &workout["telemetry"];

    // GPS route should not be present when summary_only is true
    assert!(
        telemetry["gps_route"].is_null()
            || !telemetry.as_mapping().unwrap().contains_key("gps_route")
    );
}

#[test]
fn test_tcx_to_pwf_no_activities() {
    // TCX with no Activities element
    let tcx_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<TrainingCenterDatabase xmlns="http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2">
</TrainingCenterDatabase>"#;

    let cursor = Cursor::new(tcx_xml.as_bytes());
    let result = tcx_to_pwf(cursor, false).unwrap();

    // Should have warning about no activities
    assert!(result.has_warnings());
    assert!(result
        .warnings
        .iter()
        .any(|w| matches!(w, ConversionWarning::DataQualityIssue { .. })));

    let history: serde_yaml::Value = serde_yaml::from_str(&result.pwf_yaml).unwrap();
    let workouts = history["workouts"].as_sequence().unwrap();
    assert_eq!(workouts.len(), 0);
}

#[test]
fn test_tcx_to_pwf_multiple_laps() {
    let tcx_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<TrainingCenterDatabase xmlns="http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2">
  <Activities>
    <Activity Sport="Cycling">
      <Id>2024-01-15T10:00:00Z</Id>
      <Lap StartTime="2024-01-15T10:00:00Z">
        <TotalTimeSeconds>600</TotalTimeSeconds>
        <DistanceMeters>3000</DistanceMeters>
        <Calories>100</Calories>
        <Intensity>Active</Intensity>
        <TriggerMethod>Manual</TriggerMethod>
        <Track>
          <Trackpoint>
            <Time>2024-01-15T10:00:00Z</Time>
          </Trackpoint>
        </Track>
      </Lap>
      <Lap StartTime="2024-01-15T10:10:00Z">
        <TotalTimeSeconds>700</TotalTimeSeconds>
        <DistanceMeters>3500</DistanceMeters>
        <Calories>120</Calories>
        <Intensity>Active</Intensity>
        <TriggerMethod>Manual</TriggerMethod>
        <Track>
          <Trackpoint>
            <Time>2024-01-15T10:00:00Z</Time>
          </Trackpoint>
        </Track>
      </Lap>
    </Activity>
  </Activities>
</TrainingCenterDatabase>"#;

    let cursor = Cursor::new(tcx_xml.as_bytes());
    let result = tcx_to_pwf(cursor, false).unwrap();
    let history: serde_yaml::Value = serde_yaml::from_str(&result.pwf_yaml).unwrap();

    let workout = &history["workouts"][0];
    assert_eq!(workout["duration_sec"], 1300); // 600 + 700
    assert_eq!(workout["sport"], "cycling");

    let telemetry = &workout["telemetry"];
    assert_eq!(telemetry["total_distance_km"].as_f64().unwrap(), 6.5); // 3000 + 3500 meters
    assert_eq!(telemetry["total_calories"], 220); // 100 + 120

    // Check exercises (each lap becomes an exercise)
    let exercises = workout["exercises"].as_sequence().unwrap();
    assert_eq!(exercises.len(), 2);

    assert_eq!(exercises[0]["name"], "Lap 1");
    assert_eq!(exercises[1]["name"], "Lap 2");
}

#[test]
fn test_tcx_to_pwf_invalid_xml() {
    let invalid_tcx = "This is not XML";
    let cursor = Cursor::new(invalid_tcx.as_bytes());

    let result = tcx_to_pwf(cursor, false);
    assert!(result.is_err(), "Should fail on invalid XML");
}

#[test]
fn test_tcx_sport_mapping() {
    let sports = vec![
        ("Running", "running"),
        ("Biking", "cycling"),
        ("Swimming", "swimming"),
        ("Rowing", "rowing"),
        ("Other", "other"),
    ];

    for (tcx_sport, expected_pwf_sport) in sports {
        let tcx_xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<TrainingCenterDatabase xmlns="http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2">
  <Activities>
    <Activity Sport="{}">
      <Id>2024-01-15T10:00:00Z</Id>
      <Lap StartTime="2024-01-15T10:00:00Z">
        <TotalTimeSeconds>1800</TotalTimeSeconds>
        <DistanceMeters>5000</DistanceMeters>
        <Calories>300</Calories>
        <Intensity>Active</Intensity>
        <TriggerMethod>Manual</TriggerMethod>
        <Track>
          <Trackpoint>
            <Time>2024-01-15T10:00:00Z</Time>
          </Trackpoint>
        </Track>
      </Lap>
    </Activity>
  </Activities>
</TrainingCenterDatabase>"#,
            tcx_sport
        );

        let cursor = Cursor::new(tcx_xml.as_bytes());
        let result = tcx_to_pwf(cursor, false).unwrap();
        let history: serde_yaml::Value = serde_yaml::from_str(&result.pwf_yaml).unwrap();
        let workout = &history["workouts"][0];

        assert_eq!(
            workout["sport"].as_str().unwrap(),
            expected_pwf_sport,
            "TCX sport '{}' should map to PWF sport '{}'",
            tcx_sport,
            expected_pwf_sport
        );
    }
}

#[test]
fn test_tcx_to_pwf_export_source() {
    let tcx_xml = create_minimal_tcx();
    let cursor = Cursor::new(tcx_xml.as_bytes());

    let result = tcx_to_pwf(cursor, false).unwrap();
    let history: serde_yaml::Value = serde_yaml::from_str(&result.pwf_yaml).unwrap();

    let export_source = &history["export_source"];
    assert_eq!(export_source["app_name"], "PWF TCX Converter");
    assert!(export_source["app_version"].is_string());
    assert_eq!(export_source["platform"], "TCX file");
}

#[test]
fn test_tcx_with_heart_rate() {
    let tcx_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<TrainingCenterDatabase xmlns="http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2">
  <Activities>
    <Activity Sport="Running">
      <Id>2024-01-15T10:00:00Z</Id>
      <Lap StartTime="2024-01-15T10:00:00Z">
        <TotalTimeSeconds>1800</TotalTimeSeconds>
        <DistanceMeters>5000</DistanceMeters>
        <Calories>300</Calories>
        <AverageHeartRateBpm>
          <Value>150</Value>
        </AverageHeartRateBpm>
        <MaximumHeartRateBpm>
          <Value>180</Value>
        </MaximumHeartRateBpm>
        <Intensity>Active</Intensity>
        <TriggerMethod>Manual</TriggerMethod>
        <Track>
          <Trackpoint>
            <Time>2024-01-15T10:00:00Z</Time>
            <HeartRateBpm>
              <Value>140</Value>
            </HeartRateBpm>
          </Trackpoint>
          <Trackpoint>
            <Time>2024-01-15T10:15:00Z</Time>
            <HeartRateBpm>
              <Value>160</Value>
            </HeartRateBpm>
          </Trackpoint>
        </Track>
      </Lap>
    </Activity>
  </Activities>
</TrainingCenterDatabase>"#;

    let cursor = Cursor::new(tcx_xml.as_bytes());
    let result = tcx_to_pwf(cursor, false).unwrap();
    let history: serde_yaml::Value = serde_yaml::from_str(&result.pwf_yaml).unwrap();

    let telemetry = &history["workouts"][0]["telemetry"];
    // The lap has avg=150, max=180, and trackpoints with HR data should be captured
    // Check if either lap-level or trackpoint-level HR is captured
    assert!(
        telemetry.get("heart_rate_avg").is_some() || telemetry.get("heart_rate_max").is_some(),
        "Should have some heart rate data"
    );
}

#[test]
fn test_tcx_export_workout_with_invalid_gps_timestamp() {
    use pwf_converters::pwf_to_tcx;
    use pwf_core::history::{
        CompletedExercise, GpsPosition, GpsRoute, Workout, WorkoutTelemetry, WpsHistory,
    };

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
            ended_at: Some("2024-01-15T15:30:00Z".to_string()),
            duration_sec: Some(3600),
            title: Some("Run with exercises".to_string()),
            notes: None,
            plan_id: None,
            plan_day_id: None,
            exercises: vec![CompletedExercise {
                id: Some("ex1".to_string()),
                name: "Running".to_string(),
                notes: None,
                sets: vec![],
                modality: None,
                pool_config: None,
                sport: None,
            }],
            telemetry: Some(WorkoutTelemetry {
                gps_route: Some(GpsRoute {
                    route_id: "route1".to_string(),
                    name: None,
                    positions: vec![GpsPosition {
                        latitude_deg: 37.7749,
                        longitude_deg: -122.4194,
                        timestamp: "invalid-timestamp".to_string(), // Invalid timestamp
                        elevation_m: Some(100.0),
                        accuracy_m: None,
                        speed_mps: None,
                        heading_deg: None,
                        heart_rate_bpm: None,
                        power_watts: None,
                        cadence: None,
                        temperature_c: None,
                    }],
                    total_distance_m: None,
                    total_ascent_m: None,
                    total_descent_m: None,
                    min_elevation_m: None,
                    max_elevation_m: None,
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
            sport: Some(pwf_core::Sport::Running),
            sport_segments: None,
        }],
    };

    let result = pwf_to_tcx(&history);

    // Conversion should succeed but with warnings about bad timestamp
    assert!(result.is_ok());
    let tcx_result = result.unwrap();

    // Should still generate valid XML structure
    assert!(tcx_result.tcx_xml.contains("<?xml"));
    assert!(tcx_result.tcx_xml.contains("<TrainingCenterDatabase"));
}

#[test]
fn test_tcx_export_workout_without_started_at() {
    use pwf_converters::pwf_to_tcx;
    use pwf_core::history::{Workout, WpsHistory};

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
            started_at: None, // No started_at - should use date fallback
            ended_at: None,
            duration_sec: Some(3600),
            title: Some("Workout without start time".to_string()),
            notes: None,
            plan_id: None,
            plan_day_id: None,
            exercises: vec![],
            telemetry: None,
            devices: vec![],
            sport: Some(pwf_core::Sport::Running),
            sport_segments: None,
        }],
    };

    let result = pwf_to_tcx(&history).unwrap();

    // Should use date as fallback for ID
    assert!(result.tcx_xml.contains("2024-01-15T00:00:00Z"));
    assert!(result.tcx_xml.contains("<Activity"));
}

#[test]
fn test_tcx_export_with_set_heart_rate_telemetry() {
    use pwf_converters::pwf_to_tcx;
    use pwf_core::history::{CompletedExercise, CompletedSet, SetTelemetry, Workout, WpsHistory};

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
            ended_at: Some("2024-01-15T15:30:00Z".to_string()),
            duration_sec: Some(3600),
            title: Some("Workout with set HR data".to_string()),
            notes: None,
            plan_id: None,
            plan_day_id: None,
            exercises: vec![CompletedExercise {
                id: Some("ex1".to_string()),
                name: "Cardio".to_string(),
                notes: None,
                sets: vec![
                    CompletedSet {
                        set_number: Some(1),
                        set_type: None,
                        reps: Some(10),
                        weight_kg: None,
                        weight_lb: None,
                        duration_sec: Some(60),
                        distance_meters: None,
                        rpe: None,
                        rir: None,
                        notes: None,
                        completed_at: None,
                        is_pr: None,
                        swimming: None,
                        telemetry: Some(SetTelemetry {
                            heart_rate_avg: Some(150),
                            heart_rate_max: Some(170),
                            ..Default::default()
                        }),
                    },
                    CompletedSet {
                        set_number: Some(2),
                        set_type: None,
                        reps: Some(10),
                        weight_kg: None,
                        weight_lb: None,
                        duration_sec: Some(60),
                        distance_meters: None,
                        rpe: None,
                        rir: None,
                        notes: None,
                        completed_at: None,
                        is_pr: None,
                        swimming: None,
                        telemetry: Some(SetTelemetry {
                            heart_rate_avg: Some(160),
                            heart_rate_max: Some(180),
                            ..Default::default()
                        }),
                    },
                ],
                modality: Some(pwf_core::Modality::Interval),
                pool_config: None,
                sport: None,
            }],
            telemetry: None,
            devices: vec![],
            sport: Some(pwf_core::Sport::Running),
            sport_segments: None,
        }],
    };

    let result = pwf_to_tcx(&history).unwrap();

    // Should include heart rate data from sets
    assert!(result.tcx_xml.contains("<AverageHeartRateBpm>"));
    assert!(result.tcx_xml.contains("<MaximumHeartRateBpm>"));
}

#[test]
fn test_tcx_export_strength_exercise_warning() {
    use pwf_converters::pwf_to_tcx;
    use pwf_core::history::{CompletedExercise, CompletedSet, Workout, WpsHistory};

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
            ended_at: Some("2024-01-15T15:30:00Z".to_string()),
            duration_sec: Some(3600),
            title: Some("Strength workout".to_string()),
            notes: None,
            plan_id: None,
            plan_day_id: None,
            exercises: vec![CompletedExercise {
                id: Some("ex1".to_string()),
                name: "Bench Press".to_string(),
                notes: None,
                sets: vec![CompletedSet {
                    set_number: Some(1),
                    set_type: None,
                    reps: Some(10),
                    weight_kg: Some(80.0),
                    weight_lb: None,
                    duration_sec: None,
                    distance_meters: None,
                    rpe: None,
                    rir: None,
                    notes: None,
                    completed_at: None,
                    is_pr: None,
                    swimming: None,
                    telemetry: None,
                }],
                modality: Some(pwf_core::Modality::Strength),
                pool_config: None,
                sport: None,
            }],
            telemetry: None,
            devices: vec![],
            sport: Some(pwf_core::Sport::StrengthTraining),
            sport_segments: None,
        }],
    };

    let result = pwf_to_tcx(&history).unwrap();

    // Should warn about strength exercises not mapping well to TCX
    assert!(result.has_warnings());
    assert!(result.warnings.iter().any(|w| match w {
        pwf_converters::ConversionWarning::UnsupportedFeature { feature } =>
            feature.contains("Strength") && feature.contains("TCX"),
        _ => false,
    }));
}

#[test]
fn test_tcx_export_workout_level_calories() {
    use pwf_converters::pwf_to_tcx;
    use pwf_core::history::{
        CompletedExercise, CompletedSet, Workout, WorkoutTelemetry, WpsHistory,
    };

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
            ended_at: Some("2024-01-15T15:30:00Z".to_string()),
            duration_sec: Some(3600),
            title: Some("Workout with calories".to_string()),
            notes: None,
            plan_id: None,
            plan_day_id: None,
            exercises: vec![CompletedExercise {
                id: Some("ex1".to_string()),
                name: "Running".to_string(),
                notes: None,
                sets: vec![CompletedSet {
                    set_number: Some(1),
                    set_type: None,
                    reps: Some(1),
                    weight_kg: None,
                    weight_lb: None,
                    duration_sec: Some(3600),
                    distance_meters: None,
                    rpe: None,
                    rir: None,
                    notes: None,
                    completed_at: None,
                    is_pr: None,
                    swimming: None,
                    telemetry: None,
                }],
                modality: Some(pwf_core::Modality::Stopwatch),
                pool_config: None,
                sport: None,
            }],
            telemetry: Some(WorkoutTelemetry {
                total_calories: Some(450),
                ..Default::default()
            }),
            devices: vec![],
            sport: Some(pwf_core::Sport::Running),
            sport_segments: None,
        }],
    };

    let result = pwf_to_tcx(&history).unwrap();

    // Should include calories from workout-level telemetry
    assert!(result.tcx_xml.contains("<Calories>450</Calories>"));
}

#[test]
fn test_tcx_export_with_gps_power_cadence_extensions() {
    use pwf_converters::pwf_to_tcx;
    use pwf_core::history::{
        CompletedExercise, CompletedSet, GpsPosition, GpsRoute, Workout, WorkoutTelemetry,
        WpsHistory,
    };

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
            ended_at: Some("2024-01-15T15:30:00Z".to_string()),
            duration_sec: Some(3600),
            title: Some("Cycling with power".to_string()),
            notes: None,
            plan_id: None,
            plan_day_id: None,
            exercises: vec![CompletedExercise {
                id: Some("ex1".to_string()),
                name: "Cycling".to_string(),
                notes: None,
                sets: vec![CompletedSet {
                    set_number: Some(1),
                    set_type: None,
                    reps: Some(1),
                    weight_kg: None,
                    weight_lb: None,
                    duration_sec: Some(3600),
                    distance_meters: Some(20000.0),
                    rpe: None,
                    rir: None,
                    notes: None,
                    completed_at: None,
                    is_pr: None,
                    swimming: None,
                    telemetry: None,
                }],
                modality: Some(pwf_core::Modality::Stopwatch),
                pool_config: None,
                sport: None,
            }],
            telemetry: Some(WorkoutTelemetry {
                gps_route: Some(GpsRoute {
                    route_id: "route1".to_string(),
                    name: None,
                    positions: vec![
                        GpsPosition {
                            latitude_deg: 37.7749,
                            longitude_deg: -122.4194,
                            timestamp: "2024-01-15T14:30:00Z".to_string(),
                            elevation_m: Some(100.0),
                            accuracy_m: None,
                            speed_mps: Some(8.5),
                            heading_deg: None,
                            heart_rate_bpm: Some(150),
                            power_watts: Some(250), // Power data for extensions
                            cadence: Some(90),      // Cadence data for extensions
                            temperature_c: None,
                        },
                        GpsPosition {
                            latitude_deg: 37.7750,
                            longitude_deg: -122.4195,
                            timestamp: "2024-01-15T14:31:00Z".to_string(),
                            elevation_m: Some(102.0),
                            accuracy_m: None,
                            speed_mps: Some(9.0),
                            heading_deg: None,
                            heart_rate_bpm: Some(155),
                            power_watts: Some(260),
                            cadence: Some(92),
                            temperature_c: None,
                        },
                    ],
                    total_distance_m: Some(20000.0),
                    total_ascent_m: Some(50.0),
                    total_descent_m: Some(30.0),
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
            sport: Some(pwf_core::Sport::Cycling),
            sport_segments: None,
        }],
    };

    let result = pwf_to_tcx(&history).unwrap();

    // Should generate valid TCX with trackpoints and extensions
    assert!(result.tcx_xml.contains("<Trackpoint>"));
    assert!(result.tcx_xml.contains("<Extensions>"));
    // Cadence should be in trackpoint or extensions
    assert!(result.tcx_xml.contains("90") || result.tcx_xml.contains("92"));
    // Should have heart rate
    assert!(result.tcx_xml.contains("150") || result.tcx_xml.contains("155"));
}

#[test]
fn test_tcx_export_exercise_with_bad_gps_timestamp() {
    use pwf_converters::pwf_to_tcx;
    use pwf_core::history::{
        CompletedExercise, CompletedSet, GpsPosition, GpsRoute, Workout, WorkoutTelemetry,
        WpsHistory,
    };

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
            ended_at: Some("2024-01-15T15:30:00Z".to_string()),
            duration_sec: Some(3600),
            title: Some("Run with bad GPS".to_string()),
            notes: None,
            plan_id: None,
            plan_day_id: None,
            exercises: vec![CompletedExercise {
                id: Some("ex1".to_string()),
                name: "Running".to_string(),
                notes: None,
                sets: vec![CompletedSet {
                    set_number: Some(1),
                    set_type: None,
                    reps: Some(1),
                    weight_kg: None,
                    weight_lb: None,
                    duration_sec: Some(3600),
                    distance_meters: Some(5000.0),
                    rpe: None,
                    rir: None,
                    notes: None,
                    completed_at: None,
                    is_pr: None,
                    swimming: None,
                    telemetry: None,
                }],
                modality: Some(pwf_core::Modality::Stopwatch),
                pool_config: None,
                sport: None,
            }],
            telemetry: Some(WorkoutTelemetry {
                gps_route: Some(GpsRoute {
                    route_id: "route1".to_string(),
                    name: None,
                    positions: vec![GpsPosition {
                        latitude_deg: 37.7749,
                        longitude_deg: -122.4194,
                        timestamp: "not-a-valid-timestamp".to_string(), // Invalid!
                        elevation_m: Some(100.0),
                        accuracy_m: None,
                        speed_mps: None,
                        heading_deg: None,
                        heart_rate_bpm: None,
                        power_watts: None,
                        cadence: None,
                        temperature_c: None,
                    }],
                    total_distance_m: Some(5000.0),
                    total_ascent_m: None,
                    total_descent_m: None,
                    min_elevation_m: None,
                    max_elevation_m: None,
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
            sport: Some(pwf_core::Sport::Running),
            sport_segments: None,
        }],
    };

    let result = pwf_to_tcx(&history);

    // Should succeed but with warnings about failed exercise conversion
    assert!(result.is_ok());
    let tcx_result = result.unwrap();
    assert!(tcx_result.has_warnings());
    assert!(tcx_result.warnings.iter().any(|w| match w {
        pwf_converters::ConversionWarning::DataQualityIssue { issue } =>
            issue.contains("Failed to convert exercise"),
        _ => false,
    }));
}

#[test]
fn test_tcx_export_workout_without_sport() {
    use pwf_converters::pwf_to_tcx;
    use pwf_core::history::{Workout, WpsHistory};

    let history = WpsHistory {
        history_version: 2,
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
            title: Some("Generic Workout".to_string()),
            notes: None,
            plan_id: None,
            plan_day_id: None,
            exercises: vec![],
            telemetry: None,
            devices: vec![],
            sport: None, // No sport specified
            sport_segments: None,
        }],
    };

    let result = pwf_to_tcx(&history).unwrap();

    // Should warn about missing sport field
    assert!(result.has_warnings());
    assert!(result.warnings.iter().any(|w| matches!(
        w,
        pwf_converters::ConversionWarning::MissingField { source_field, .. }
        if source_field == "sport"
    )));

    // Should default to "Other" in TCX
    assert!(result.tcx_xml.contains("Sport=\"Other\""));
}

#[test]
fn test_tcx_export_empty_workout_list() {
    use pwf_converters::pwf_to_tcx;
    use pwf_core::history::WpsHistory;

    let history = WpsHistory {
        history_version: 2,
        exported_at: "2024-01-15T16:00:00Z".to_string(),
        export_source: None,
        units: Default::default(),
        personal_records: vec![],
        body_measurements: vec![],
        workouts: vec![], // No workouts
    };

    let result = pwf_to_tcx(&history).unwrap();

    // Should warn about no workouts
    assert!(result.has_warnings());
    assert!(result.warnings.iter().any(|w| matches!(
        w,
        pwf_converters::ConversionWarning::DataQualityIssue { .. }
    )));

    // Should still generate valid TCX XML
    assert!(result.tcx_xml.contains("<?xml"));
    assert!(result.tcx_xml.contains("<TrainingCenterDatabase"));
}

#[test]
fn test_tcx_export_workout_without_telemetry() {
    use pwf_converters::pwf_to_tcx;
    use pwf_core::history::{Workout, WpsHistory};
    use pwf_core::Sport;

    let history = WpsHistory {
        history_version: 2,
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
            title: Some("Strength Training".to_string()),
            notes: None,
            plan_id: None,
            plan_day_id: None,
            exercises: vec![],
            telemetry: None, // No telemetry
            devices: vec![],
            sport: Some(Sport::StrengthTraining),
            sport_segments: None,
        }],
    };

    let result = pwf_to_tcx(&history).unwrap();

    // Should generate valid TCX even though workout might not have laps
    assert!(result.tcx_xml.contains("<?xml"));
    assert!(result.tcx_xml.contains("<TrainingCenterDatabase"));

    // Workout without exercises/laps might generate warnings or be skipped
    // This is acceptable behavior for edge case
}
