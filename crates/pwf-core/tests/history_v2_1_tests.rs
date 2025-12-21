//! Integration tests for PWF v2.1 history examples

use pwf_core::history;
use std::fs;
use std::path::PathBuf;

/// Helper function to get the path to the examples directory
fn examples_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("examples")
}

/// Helper function to read a file from the examples directory
fn read_example(filename: &str) -> String {
    let path = examples_dir().join(filename);
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read example file {}: {}", path.display(), e))
}

#[test]
fn test_history_swimming_v2_1_example() {
    let yaml = read_example("history-swimming-v2.1.yaml");

    let result = history::validate(&yaml);

    // Should validate successfully (warnings are OK - they indicate data quality issues but not invalid data)
    assert!(
        result.is_valid(),
        "Swimming v2.1 example should be valid. Errors: {:?}",
        result.errors
    );

    // Parse and verify structure
    let parsed = history::parse(&yaml).expect("Should parse successfully");

    assert_eq!(parsed.history_version, 2);
    assert_eq!(parsed.workouts.len(), 1);

    let workout = &parsed.workouts[0];
    assert_eq!(workout.id, Some("swim-technique-20251220".to_string()));
    assert_eq!(workout.sport.as_ref().unwrap(), &pwf_core::Sport::Swimming);

    // Verify swimming-specific data
    let exercise = &workout.exercises[0];
    assert!(exercise.pool_config.is_some());
    assert_eq!(exercise.pool_config.as_ref().unwrap().pool_length, 25.0);
    assert_eq!(
        exercise.pool_config.as_ref().unwrap().pool_length_unit,
        pwf_core::history::PoolLengthUnit::Meters
    );

    // Verify swimming set data
    let set = &exercise.sets[0];
    assert!(set.swimming.is_some());

    let swimming_data = set.swimming.as_ref().unwrap();
    assert!(swimming_data.total_lengths.is_some());
    assert!(!swimming_data.lengths.is_empty());

    // Verify SWOLF calculation on first length
    let first_length = &swimming_data.lengths[0];
    assert_eq!(
        first_length.stroke_type,
        pwf_core::history::StrokeType::Freestyle
    );
    assert!(first_length.stroke_count.is_some());
    assert!(first_length.swolf.is_some());
    assert!(first_length.validate_swolf());
}

#[test]
fn test_history_cycling_power_v2_1_example() {
    let yaml = read_example("history-cycling-power-v2.1.yaml");

    let result = history::validate(&yaml);

    // Should validate successfully
    assert!(
        result.is_valid(),
        "Cycling power v2.1 example should be valid. Errors: {:?}",
        result.errors
    );
    assert!(
        result.warnings.is_empty(),
        "Cycling power v2.1 example should have no warnings. Warnings: {:?}",
        result.warnings
    );

    // Parse and verify structure
    let parsed = history::parse(&yaml).expect("Should parse successfully");

    assert_eq!(parsed.history_version, 2);
    assert_eq!(parsed.workouts.len(), 1);

    let workout = &parsed.workouts[0];
    assert_eq!(workout.id, Some("ride-20251220-interval".to_string()));
    assert_eq!(workout.sport.as_ref().unwrap(), &pwf_core::Sport::Cycling);

    // Verify advanced metrics
    let telemetry = workout.telemetry.as_ref().unwrap();
    assert!(telemetry.advanced_metrics.is_some());

    let advanced = telemetry.advanced_metrics.as_ref().unwrap();
    assert_eq!(advanced.training_effect, Some(3.8));
    assert_eq!(advanced.vo2_max_estimate, Some(52.3));
    assert_eq!(advanced.recovery_time_hours, Some(36));

    // Verify power metrics
    assert!(telemetry.power_metrics.is_some());

    let power = telemetry.power_metrics.as_ref().unwrap();
    assert_eq!(power.normalized_power, Some(268));
    assert_eq!(power.training_stress_score, Some(142.5));
    assert_eq!(power.intensity_factor, Some(0.89));
    assert_eq!(power.variability_index, Some(1.09));
    assert_eq!(power.ftp_watts, Some(300));

    // Verify time in zones
    assert!(telemetry.time_in_zones.is_some());

    let zones = telemetry.time_in_zones.as_ref().unwrap();
    assert!(zones.hr_zones_sec.is_some());
    assert!(zones.power_zones_sec.is_some());
    assert_eq!(zones.hr_zones_sec.as_ref().unwrap().len(), 6);
    assert_eq!(zones.power_zones_sec.as_ref().unwrap().len(), 6);

    // Verify GPS route
    assert!(telemetry.gps_route.is_some());

    let gps = telemetry.gps_route.as_ref().unwrap();
    assert_eq!(gps.route_id, "route-20251220-intervals");
    assert!(!gps.positions.is_empty());
    assert!(gps.total_distance_m.is_some());
    assert!(gps.total_ascent_m.is_some());

    // Verify device tracking
    assert!(!workout.devices.is_empty());
    assert_eq!(workout.devices.len(), 3); // Bike computer, power meter, HRM
}

#[test]
fn test_history_triathlon_v2_1_example() {
    let yaml = read_example("history-triathlon-v2.1.yaml");

    let result = history::validate(&yaml);

    // Should validate successfully
    assert!(
        result.is_valid(),
        "Triathlon v2.1 example should be valid. Errors: {:?}",
        result.errors
    );
    assert!(
        result.warnings.is_empty(),
        "Triathlon v2.1 example should have no warnings. Warnings: {:?}",
        result.warnings
    );

    // Parse and verify structure
    let parsed = history::parse(&yaml).expect("Should parse successfully");

    assert_eq!(parsed.history_version, 2);
    assert_eq!(parsed.workouts.len(), 1);

    let workout = &parsed.workouts[0];
    assert_eq!(workout.id, Some("tri-sprint-20251220".to_string()));

    // Verify multi-sport segments
    assert!(workout.sport_segments.is_some());

    let segments = workout.sport_segments.as_ref().unwrap();
    assert_eq!(segments.len(), 3); // Swim, bike, run

    // Verify swim segment
    let swim = &segments[0];
    assert_eq!(swim.segment_id, "swim");
    assert_eq!(swim.sport, pwf_core::Sport::Swimming);
    assert_eq!(swim.segment_index, 0);
    assert_eq!(swim.duration_sec, Some(980));
    assert_eq!(swim.distance_m, Some(750.0));

    // Verify T1 transition
    assert!(swim.transition.is_some());
    let t1 = swim.transition.as_ref().unwrap();
    assert_eq!(t1.transition_id, "T1");
    assert_eq!(t1.from_sport, pwf_core::Sport::Swimming);
    assert_eq!(t1.to_sport, pwf_core::Sport::Cycling);
    assert_eq!(t1.duration_sec, Some(95));

    // Verify bike segment
    let bike = &segments[1];
    assert_eq!(bike.segment_id, "bike");
    assert_eq!(bike.sport, pwf_core::Sport::Cycling);
    assert_eq!(bike.segment_index, 1);
    assert_eq!(bike.duration_sec, Some(2640));
    assert_eq!(bike.distance_m, Some(20000.0));

    // Verify bike segment has power metrics
    assert!(bike.telemetry.is_some());
    let bike_telemetry = bike.telemetry.as_ref().unwrap();
    assert!(bike_telemetry.power_avg.is_some());
    assert!(bike_telemetry.power_metrics.is_some());

    // Verify T2 transition
    assert!(bike.transition.is_some());
    let t2 = bike.transition.as_ref().unwrap();
    assert_eq!(t2.transition_id, "T2");
    assert_eq!(t2.from_sport, pwf_core::Sport::Cycling);
    assert_eq!(t2.to_sport, pwf_core::Sport::Running);
    assert_eq!(t2.duration_sec, Some(42));

    // Verify run segment
    let run = &segments[2];
    assert_eq!(run.segment_id, "run");
    assert_eq!(run.sport, pwf_core::Sport::Running);
    assert_eq!(run.segment_index, 2);
    assert_eq!(run.duration_sec, Some(1380));
    assert_eq!(run.distance_m, Some(5000.0));

    // No transition after final segment
    assert!(run.transition.is_none());

    // Verify exercises reference segments via IDs
    assert_eq!(workout.exercises.len(), 3);
    assert_eq!(
        workout.exercises[0].sport.as_ref().unwrap(),
        &pwf_core::Sport::Swimming
    );
    assert_eq!(
        workout.exercises[1].sport.as_ref().unwrap(),
        &pwf_core::Sport::Cycling
    );
    assert_eq!(
        workout.exercises[2].sport.as_ref().unwrap(),
        &pwf_core::Sport::Running
    );
}

#[test]
fn test_v2_1_backward_compatibility() {
    // Verify that a v2.0 history file (without v2.1 features) still validates
    let yaml = r#"
history_version: 2
exported_at: "2025-01-15T12:00:00Z"

units:
  weight: kg
  distance: meters

workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Squat"
        modality: strength
        sets:
          - reps: 10
            weight_kg: 100

personal_records: []
body_measurements: []
"#;

    let result = history::validate(yaml);
    assert!(
        result.is_valid(),
        "V2.0 history (without v2.1 features) should still validate. Errors: {:?}",
        result.errors
    );

    // Verify it parses correctly
    let parsed = history::parse(yaml).expect("Should parse v2.0 history");
    assert_eq!(parsed.history_version, 2);
    assert_eq!(parsed.workouts.len(), 1);

    // Verify v2.1 fields are None/empty
    let workout = &parsed.workouts[0];
    assert!(workout.sport.is_none());
    assert!(workout.sport_segments.is_none());
    assert!(workout.devices.is_empty());
    assert!(workout.telemetry.is_none());

    let exercise = &workout.exercises[0];
    assert!(exercise.pool_config.is_none());
    assert!(exercise.sport.is_none());

    let set = &exercise.sets[0];
    assert!(set.swimming.is_none());
}
