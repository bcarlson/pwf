//! Integration tests for CSV export functionality

use pwf_converters::{export_telemetry_to_csv, CsvExportOptions};
use pwf_core::history::{CompletedExercise, CompletedSet, ExportSource, SetTelemetry, TimeSeriesData, Units, Workout, WpsHistory};
use pwf_core::Sport;

/// Create a test history with comprehensive telemetry data
fn create_test_history_with_full_telemetry() -> WpsHistory {
    let time_series = TimeSeriesData {
        timestamps: vec![
            "2025-01-15T14:30:00Z".to_string(),
            "2025-01-15T14:30:01Z".to_string(),
            "2025-01-15T14:30:02Z".to_string(),
            "2025-01-15T14:30:03Z".to_string(),
            "2025-01-15T14:30:04Z".to_string(),
        ],
        elapsed_sec: Some(vec![0, 1, 2, 3, 4]),
        heart_rate: Some(vec![145, 147, 149, 151, 153]),
        power: Some(vec![200, 205, 210, 215, 220]),
        cadence: Some(vec![90, 91, 92, 93, 94]),
        speed_mps: Some(vec![5.2, 5.3, 5.4, 5.5, 5.6]),
        elevation_m: Some(vec![100.5, 100.8, 101.0, 101.2, 101.5]),
        latitude: Some(vec![37.7749, 37.7750, 37.7751, 37.7752, 37.7753]),
        longitude: Some(vec![-122.4194, -122.4195, -122.4196, -122.4197, -122.4198]),
        distance_m: Some(vec![0.0, 5.3, 10.7, 16.2, 21.8]),
        temperature_c: Some(vec![20.0, 20.1, 20.1, 20.2, 20.2]),
        grade_percent: Some(vec![0.0, 0.5, 1.0, 1.5, 2.0]),
        respiration_rate: Some(vec![16, 17, 18, 19, 20]),
        core_temperature_c: Some(vec![37.0, 37.1, 37.1, 37.2, 37.2]),
        muscle_oxygen_percent: Some(vec![75.0, 74.5, 74.0, 73.5, 73.0]),
        ..Default::default()
    };

    let telemetry = SetTelemetry {
        time_series: Some(time_series),
        ..Default::default()
    };

    let set = CompletedSet {
        set_number: Some(1),
        set_type: None,
        reps: None,
        weight_kg: None,
        weight_lb: None,
        duration_sec: Some(5),
        distance_meters: Some(27.0),
        rpe: None,
        rir: None,
        notes: None,
        is_pr: None,
        completed_at: None,
        telemetry: Some(telemetry),
        swimming: None,
    };

    let exercise = CompletedExercise {
        id: None,
        name: "Cycling Intervals".to_string(),
        modality: None,
        notes: None,
        sets: vec![set],
        pool_config: None,
        sport: Some(Sport::Cycling),
    };

    let workout = Workout {
        id: Some("test-cycling-workout".to_string()),
        date: "2025-01-15".to_string(),
        started_at: Some("2025-01-15T14:30:00Z".to_string()),
        ended_at: Some("2025-01-15T14:30:05Z".to_string()),
        duration_sec: Some(5),
        title: Some("Cycling Test Workout".to_string()),
        notes: Some("Test workout with full telemetry".to_string()),
        plan_id: None,
        plan_day_id: None,
        exercises: vec![exercise],
        telemetry: None,
        devices: vec![],
        sport: Some(Sport::Cycling),
        sport_segments: None,
    };

    WpsHistory {
        history_version: 2,
        exported_at: "2025-01-15T15:00:00Z".to_string(),
        export_source: Some(ExportSource {
            app_name: Some("PWF Test Suite".to_string()),
            app_version: None,
            platform: None,
            preferred_units: None,
        }),
        units: Units::default(),
        workouts: vec![workout],
        personal_records: vec![],
        body_measurements: vec![],
    }
}

#[test]
fn test_csv_export_comprehensive_data() {
    let history = create_test_history_with_full_telemetry();
    let options = CsvExportOptions::default();

    let result = export_telemetry_to_csv(&history, &options).unwrap();

    assert!(!result.csv_data.is_empty());
    assert_eq!(result.data_points, 5);
    assert_eq!(result.workouts_processed, 1);

    // Verify CSV structure
    let lines: Vec<&str> = result.csv_data.lines().collect();
    assert_eq!(lines.len(), 6); // Header + 5 data rows

    // Verify header contains all expected fields
    let header = lines[0];
    assert!(header.contains("workout_label"));
    assert!(header.contains("timestamp"));
    assert!(header.contains("heart_rate"));
    assert!(header.contains("power"));
    assert!(header.contains("cadence"));
    assert!(header.contains("speed_mps"));
    assert!(header.contains("elevation_m"));
    assert!(header.contains("latitude"));
    assert!(header.contains("longitude"));
    assert!(header.contains("distance_m"));
    assert!(header.contains("temperature_c"));

    // Verify data rows contain expected values
    let first_data_row = lines[1];
    assert!(first_data_row.contains("test-cycling-workout"));
    assert!(first_data_row.contains("2025-01-15T14:30:00Z"));
    assert!(first_data_row.contains("145")); // heart_rate
    assert!(first_data_row.contains("200")); // power
    assert!(first_data_row.contains("90")); // cadence
}

#[test]
fn test_csv_export_partial_telemetry() {
    let mut history = create_test_history_with_full_telemetry();

    // Create time series with only basic fields
    let time_series = TimeSeriesData {
        timestamps: vec![
            "2025-01-15T14:30:00Z".to_string(),
            "2025-01-15T14:30:01Z".to_string(),
        ],
        heart_rate: Some(vec![145, 147]),
        power: Some(vec![200, 205]),
        // No other fields
        ..Default::default()
    };

    history.workouts[0].exercises[0].sets[0].telemetry = Some(SetTelemetry {
        time_series: Some(time_series),
        ..Default::default()
    });

    let options = CsvExportOptions::default();
    let result = export_telemetry_to_csv(&history, &options).unwrap();

    assert!(!result.csv_data.is_empty());
    assert_eq!(result.data_points, 2);

    // Verify CSV has all columns but many are empty
    let lines: Vec<&str> = result.csv_data.lines().collect();
    assert_eq!(lines.len(), 3); // Header + 2 data rows

    // Check that rows have all columns even if some are empty
    for line in &lines[1..] {
        let fields: Vec<&str> = line.split(',').collect();
        // Should have 27 fields (see header in exporter.rs)
        assert!(fields.len() >= 10);
    }
}

#[test]
fn test_csv_export_multiple_workouts() {
    let mut history = create_test_history_with_full_telemetry();

    // Add a second workout
    let mut workout2 = history.workouts[0].clone();
    workout2.id = Some("test-workout-2".to_string());
    workout2.title = Some("Second Workout".to_string());
    history.workouts.push(workout2);

    let options = CsvExportOptions::default();
    let result = export_telemetry_to_csv(&history, &options).unwrap();

    assert_eq!(result.data_points, 10); // 5 points × 2 workouts
    assert_eq!(result.workouts_processed, 2);

    // Verify both workout labels are present in CSV
    assert!(result.csv_data.contains("test-cycling-workout"));
    assert!(result.csv_data.contains("test-workout-2"));
}

#[test]
fn test_csv_export_no_telemetry() {
    let mut history = create_test_history_with_full_telemetry();

    // Remove telemetry
    history.workouts[0].exercises[0].sets[0].telemetry = None;

    let options = CsvExportOptions::default();
    let result = export_telemetry_to_csv(&history, &options);

    assert!(result.is_err());
    match result {
        Err(e) => {
            let error_msg = e.to_string();
            assert!(error_msg.contains("No telemetry data found"));
        }
        Ok(_) => panic!("Expected error when no telemetry data present"),
    }
}

#[test]
fn test_csv_export_no_time_series() {
    let mut history = create_test_history_with_full_telemetry();

    // Keep telemetry but remove time series
    history.workouts[0].exercises[0].sets[0].telemetry = Some(SetTelemetry {
        time_series: None,
        heart_rate_avg: Some(150),
        heart_rate_max: Some(180),
        ..Default::default()
    });

    let options = CsvExportOptions::default();
    let result = export_telemetry_to_csv(&history, &options);

    assert!(result.is_err());
    match result {
        Err(e) => {
            let error_msg = e.to_string();
            assert!(error_msg.contains("No time-series data found"));
        }
        Ok(_) => panic!("Expected error when no time-series data present"),
    }
}

#[test]
fn test_csv_export_validates_time_series() {
    let mut history = create_test_history_with_full_telemetry();

    // Create invalid time series with mismatched array lengths
    let invalid_time_series = TimeSeriesData {
        timestamps: vec!["2025-01-15T14:30:00Z".to_string()],
        heart_rate: Some(vec![145, 147]), // Length mismatch!
        ..Default::default()
    };

    // Create a valid time series
    let valid_time_series = TimeSeriesData {
        timestamps: vec!["2025-01-15T14:30:00Z".to_string(), "2025-01-15T14:30:01Z".to_string()],
        heart_rate: Some(vec![150, 152]),
        ..Default::default()
    };

    // Set first set to invalid
    history.workouts[0].exercises[0].sets[0].telemetry = Some(SetTelemetry {
        time_series: Some(invalid_time_series),
        ..Default::default()
    });

    // Add a second set with valid time series
    let mut valid_set = history.workouts[0].exercises[0].sets[0].clone();
    valid_set.set_number = Some(2);
    valid_set.telemetry = Some(SetTelemetry {
        time_series: Some(valid_time_series),
        ..Default::default()
    });
    history.workouts[0].exercises[0].sets.push(valid_set);

    let options = CsvExportOptions::default();
    let result = export_telemetry_to_csv(&history, &options).unwrap();

    // Should have warnings about data quality from the invalid time series
    assert!(result.has_warnings());
    assert!(!result.warnings.is_empty());

    // But should still export the valid time series
    assert_eq!(result.workouts_processed, 1); // Only the valid set
    assert_eq!(result.data_points, 2); // 2 points from valid set
}

#[test]
fn test_csv_export_gps_data() {
    let history = create_test_history_with_full_telemetry();
    let options = CsvExportOptions::default();

    let result = export_telemetry_to_csv(&history, &options).unwrap();

    // Verify GPS coordinates are exported
    assert!(result.csv_data.contains("latitude"));
    assert!(result.csv_data.contains("longitude"));
    assert!(result.csv_data.contains("37.7749")); // Sample latitude
    assert!(result.csv_data.contains("-122.4194")); // Sample longitude
}

#[test]
fn test_csv_export_cycling_metrics() {
    let history = create_test_history_with_full_telemetry();
    let options = CsvExportOptions::default();

    let result = export_telemetry_to_csv(&history, &options).unwrap();

    // Verify cycling-specific metrics
    assert!(result.csv_data.contains("power"));
    assert!(result.csv_data.contains("cadence"));
    assert!(result.csv_data.contains("speed_mps"));
}

#[test]
fn test_csv_export_advanced_metrics() {
    let history = create_test_history_with_full_telemetry();
    let options = CsvExportOptions::default();

    let result = export_telemetry_to_csv(&history, &options).unwrap();

    // Verify advanced metrics are in header
    let header = result.csv_data.lines().next().unwrap();
    assert!(header.contains("respiration_rate"));
    assert!(header.contains("core_temperature_c"));
    assert!(header.contains("muscle_oxygen_percent"));
    assert!(header.contains("grade_percent"));
}

#[test]
fn test_csv_export_workout_label_generation() {
    let history = create_test_history_with_full_telemetry();
    let options = CsvExportOptions::default();

    let result = export_telemetry_to_csv(&history, &options).unwrap();

    // Verify workout label is correctly generated
    let lines: Vec<&str> = result.csv_data.lines().collect();
    let first_data_row = lines[1];

    // Label format: workout_id-exercise_name-set_number
    assert!(first_data_row.contains("test-cycling-workout-Cycling Intervals-set1"));
}

#[test]
fn test_csv_export_empty_history() {
    let history = WpsHistory {
        history_version: 2,
        exported_at: "2025-01-15T15:00:00Z".to_string(),
        export_source: None,
        units: Units::default(),
        workouts: vec![],
        personal_records: vec![],
        body_measurements: vec![],
    };

    let options = CsvExportOptions::default();
    let result = export_telemetry_to_csv(&history, &options);

    assert!(result.is_err());
}

#[test]
fn test_csv_export_multiple_sets() {
    let mut history = create_test_history_with_full_telemetry();

    // Add a second set to the same exercise
    let mut set2 = history.workouts[0].exercises[0].sets[0].clone();
    set2.set_number = Some(2);
    history.workouts[0].exercises[0].sets.push(set2);

    let options = CsvExportOptions::default();
    let result = export_telemetry_to_csv(&history, &options).unwrap();

    assert_eq!(result.data_points, 10); // 5 points × 2 sets
    assert_eq!(result.workouts_processed, 2);

    // Verify both sets are labeled correctly
    assert!(result.csv_data.contains("set1"));
    assert!(result.csv_data.contains("set2"));
}
