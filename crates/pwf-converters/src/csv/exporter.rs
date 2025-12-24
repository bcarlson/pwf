//! CSV export implementation for PWF time-series telemetry data

use crate::error::{ConversionError, ConversionWarning, CsvExportResult};
use csv::Writer;
use pwf_core::history::{TimeSeriesData, Workout, WpsHistory};
use std::io::Write as IoWrite;

/// Options for CSV export
#[derive(Debug, Clone, Default)]
pub struct CsvExportOptions {
    /// Export only specific fields (empty = all fields)
    pub fields: Vec<String>,
    /// Include workout metadata as comments
    pub include_metadata: bool,
    /// Combine all workouts into one CSV (vs separate per workout)
    pub combine_workouts: bool,
}

/// Export time-series telemetry data from PWF history to CSV format
///
/// This function extracts all time-series telemetry data from workouts and
/// exports it to CSV format suitable for analysis in spreadsheet applications.
///
/// # Arguments
///
/// * `history` - The PWF history export to process
/// * `options` - Export options (fields to include, metadata, etc.)
///
/// # Returns
///
/// A `CsvExportResult` containing the CSV data and any warnings
///
/// # Errors
///
/// Returns `ConversionError` if:
/// - No time-series data is found in the history
/// - CSV writing fails
/// - Data validation fails
pub fn export_telemetry_to_csv(
    history: &WpsHistory,
    _options: &CsvExportOptions,
) -> Result<CsvExportResult, ConversionError> {
    let mut csv_buffer = Vec::new();
    let mut result = CsvExportResult::new(String::new());

    // Collect all time-series data from all workouts
    let mut all_time_series: Vec<(String, &TimeSeriesData)> = Vec::new();
    let mut has_any_telemetry = false;

    for workout in &history.workouts {
        for exercise in &workout.exercises {
            for set in &exercise.sets {
                if let Some(ref telemetry) = set.telemetry {
                    has_any_telemetry = true;
                    if let Some(ref ts) = telemetry.time_series {
                        // Validate time series data
                        if let Err(e) = ts.validate_lengths() {
                            result.add_warning(ConversionWarning::DataQualityIssue {
                                issue: format!("Time series validation error: {}", e),
                            });
                            continue;
                        }

                        // Create a label for this time series
                        let label = create_time_series_label(workout, exercise, set);
                        all_time_series.push((label, ts));
                    }
                }
            }
        }
    }

    if all_time_series.is_empty() {
        if !has_any_telemetry {
            return Err(ConversionError::MissingRequiredField(
                "No telemetry data found in history export. \
                 CSV export requires workouts with time-series telemetry data."
                    .to_string(),
            ));
        } else {
            return Err(ConversionError::MissingRequiredField(
                "No time-series data found in telemetry. \
                 Only summary metrics are available. \
                 CSV export requires second-by-second time-series data."
                    .to_string(),
            ));
        }
    }

    // Write CSV in a scope to drop the writer before using csv_buffer
    {
        let mut writer = Writer::from_writer(&mut csv_buffer);

        // Write CSV header
        write_csv_header(&mut writer)?;

        // Export time-series data
        for (label, ts) in &all_time_series {
            export_time_series(&mut writer, ts, label, &mut result)?;
        }

        writer
            .flush()
            .map_err(|e| ConversionError::IoError(std::io::Error::other(e)))?;
    } // writer dropped here

    result.csv_data = String::from_utf8(csv_buffer).map_err(|e| {
        ConversionError::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Invalid UTF-8 in CSV output: {}", e),
        ))
    })?;

    result.workouts_processed = all_time_series.len();

    Ok(result)
}

/// Write CSV header row with all telemetry fields
fn write_csv_header<W: IoWrite>(writer: &mut Writer<W>) -> Result<(), ConversionError> {
    writer
        .write_record([
            "workout_label",
            "timestamp",
            "elapsed_sec",
            "heart_rate",
            "power",
            "cadence",
            "speed_mps",
            "elevation_m",
            "latitude",
            "longitude",
            "distance_m",
            "temperature_c",
            "grade_percent",
            "respiration_rate",
            "core_temperature_c",
            "muscle_oxygen_percent",
            "power_balance",
            "left_pedal_smoothness",
            "right_pedal_smoothness",
            "left_torque_effectiveness",
            "right_torque_effectiveness",
            "stride_length_m",
            "vertical_oscillation_cm",
            "ground_contact_time_ms",
            "ground_contact_balance",
            "stroke_rate",
            "stroke_count",
            "swolf",
        ])
        .map_err(|e| ConversionError::IoError(std::io::Error::other(e)))?;

    Ok(())
}

/// Export a single time-series dataset to CSV
#[allow(clippy::vec_init_then_push)]
fn export_time_series<W: IoWrite>(
    writer: &mut Writer<W>,
    ts: &TimeSeriesData,
    label: &str,
    result: &mut CsvExportResult,
) -> Result<(), ConversionError> {
    let num_points = ts.len();

    for i in 0..num_points {
        let mut record = Vec::new();

        // Workout label
        record.push(label.to_string());

        // Timestamp (required)
        record.push(ts.timestamps[i].clone());

        // Elapsed time
        record.push(format_optional_u32(&ts.elapsed_sec, i));

        // Heart rate
        record.push(format_optional_u32(&ts.heart_rate, i));

        // Power
        record.push(format_optional_u32(&ts.power, i));

        // Cadence
        record.push(format_optional_u32(&ts.cadence, i));

        // Speed
        record.push(format_optional_f64(&ts.speed_mps, i));

        // Elevation
        record.push(format_optional_f64(&ts.elevation_m, i));

        // Latitude
        record.push(format_optional_f64(&ts.latitude, i));

        // Longitude
        record.push(format_optional_f64(&ts.longitude, i));

        // Distance
        record.push(format_optional_f64(&ts.distance_m, i));

        // Temperature
        record.push(format_optional_f64(&ts.temperature_c, i));

        // Grade
        record.push(format_optional_f64(&ts.grade_percent, i));

        // Respiration rate
        record.push(format_optional_u32(&ts.respiration_rate, i));

        // Core temperature
        record.push(format_optional_f64(&ts.core_temperature_c, i));

        // Muscle oxygen
        record.push(format_optional_f64(&ts.muscle_oxygen_percent, i));

        // Power balance
        record.push(format_optional_f64(&ts.power_balance, i));

        // Left pedal smoothness
        record.push(format_optional_f64(&ts.left_pedal_smoothness, i));

        // Right pedal smoothness
        record.push(format_optional_f64(&ts.right_pedal_smoothness, i));

        // Left torque effectiveness
        record.push(format_optional_f64(&ts.left_torque_effectiveness, i));

        // Right torque effectiveness
        record.push(format_optional_f64(&ts.right_torque_effectiveness, i));

        // Stride length
        record.push(format_optional_f64(&ts.stride_length_m, i));

        // Vertical oscillation
        record.push(format_optional_f64(&ts.vertical_oscillation_cm, i));

        // Ground contact time
        record.push(format_optional_u32(&ts.ground_contact_time_ms, i));

        // Ground contact balance
        record.push(format_optional_f64(&ts.ground_contact_balance, i));

        // Stroke rate
        record.push(format_optional_u32(&ts.stroke_rate, i));

        // Stroke count
        record.push(format_optional_u32(&ts.stroke_count, i));

        // SWOLF
        record.push(format_optional_u32(&ts.swolf, i));

        writer
            .write_record(&record)
            .map_err(|e| ConversionError::IoError(std::io::Error::other(e)))?;

        result.data_points += 1;
    }

    Ok(())
}

/// Create a descriptive label for a time series from workout/exercise/set info
fn create_time_series_label(
    workout: &Workout,
    exercise: &pwf_core::history::CompletedExercise,
    set: &pwf_core::history::CompletedSet,
) -> String {
    let workout_id = workout
        .id
        .as_ref()
        .or(workout.title.as_ref())
        .map(|s| s.as_str())
        .unwrap_or("workout");

    let exercise_name = &exercise.name;

    let set_num = set
        .set_number
        .map(|n| format!("set{}", n))
        .unwrap_or_else(|| "set".to_string());

    format!("{}-{}-{}", workout_id, exercise_name, set_num)
}

/// Format optional u32 field for CSV output
fn format_optional_u32(field: &Option<Vec<u32>>, index: usize) -> String {
    field
        .as_ref()
        .and_then(|v| v.get(index))
        .map(|val| val.to_string())
        .unwrap_or_default()
}

/// Format optional f64 field for CSV output
fn format_optional_f64(field: &Option<Vec<f64>>, index: usize) -> String {
    field
        .as_ref()
        .and_then(|v| v.get(index))
        .map(|val| val.to_string())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pwf_core::history::{
        CompletedExercise, CompletedSet, SetTelemetry, TimeSeriesData, Units, WpsHistory,
    };

    fn create_test_history_with_time_series() -> WpsHistory {
        let time_series = TimeSeriesData {
            timestamps: vec![
                "2025-01-15T14:30:00Z".to_string(),
                "2025-01-15T14:30:01Z".to_string(),
                "2025-01-15T14:30:02Z".to_string(),
            ],
            elapsed_sec: Some(vec![0, 1, 2]),
            heart_rate: Some(vec![145, 147, 149]),
            power: Some(vec![200, 205, 210]),
            cadence: Some(vec![90, 91, 92]),
            speed_mps: Some(vec![5.2, 5.3, 5.4]),
            elevation_m: Some(vec![100.5, 100.8, 101.0]),
            latitude: Some(vec![37.7749, 37.7750, 37.7751]),
            longitude: Some(vec![-122.4194, -122.4195, -122.4196]),
            distance_m: Some(vec![0.0, 5.3, 10.7]),
            temperature_c: Some(vec![20.0, 20.0, 20.0]),
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
            duration_sec: None,
            distance_meters: None,
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
            name: "Cycling".to_string(),
            modality: None,
            notes: None,
            sets: vec![set],
            pool_config: None,
            sport: None,
        };

        let workout = pwf_core::history::Workout {
            id: Some("test-workout".to_string()),
            date: "2025-01-15".to_string(),
            started_at: None,
            ended_at: None,
            duration_sec: None,
            title: None,
            notes: None,
            plan_id: None,
            plan_day_id: None,
            exercises: vec![exercise],
            telemetry: None,
            devices: vec![],
            sport: None,
            sport_segments: None,
        };

        WpsHistory {
            history_version: 2,
            exported_at: "2025-01-15T15:00:00Z".to_string(),
            export_source: None,
            units: Units::default(),
            workouts: vec![workout],
            personal_records: vec![],
            body_measurements: vec![],
        }
    }

    #[test]
    fn test_export_telemetry_to_csv_basic() {
        let history = create_test_history_with_time_series();
        let options = CsvExportOptions::default();

        let result = export_telemetry_to_csv(&history, &options).unwrap();

        assert!(!result.csv_data.is_empty());
        assert_eq!(result.data_points, 3);
        assert_eq!(result.workouts_processed, 1);

        // Verify CSV contains expected data
        assert!(result.csv_data.contains("timestamp"));
        assert!(result.csv_data.contains("heart_rate"));
        assert!(result.csv_data.contains("power"));
        assert!(result.csv_data.contains("2025-01-15T14:30:00Z"));
        assert!(result.csv_data.contains("145"));
        assert!(result.csv_data.contains("200"));
    }

    #[test]
    fn test_export_telemetry_to_csv_no_time_series() {
        let mut history = create_test_history_with_time_series();
        // Remove time series data
        history.workouts[0].exercises[0].sets[0].telemetry = None;

        let options = CsvExportOptions::default();
        let result = export_telemetry_to_csv(&history, &options);

        assert!(result.is_err());
        match result {
            Err(ConversionError::MissingRequiredField(msg)) => {
                assert!(msg.contains("No telemetry data found"));
            }
            _ => panic!("Expected MissingRequiredField error"),
        }
    }

    #[test]
    fn test_export_telemetry_to_csv_missing_fields() {
        let mut history = create_test_history_with_time_series();

        // Create time series with only basic fields
        let time_series = TimeSeriesData {
            timestamps: vec![
                "2025-01-15T14:30:00Z".to_string(),
                "2025-01-15T14:30:01Z".to_string(),
            ],
            heart_rate: Some(vec![145, 147]),
            // No power, cadence, etc.
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

        // Verify empty fields are handled correctly
        let lines: Vec<&str> = result.csv_data.lines().collect();
        assert!(lines.len() >= 2); // Header + data rows

        // Check that empty fields are present but empty
        for line in &lines[1..] {
            // Skip header
            let fields: Vec<&str> = line.split(',').collect();
            // Should have all columns even if empty
            assert!(fields.len() > 10);
        }
    }

    #[test]
    fn test_export_telemetry_to_csv_invalid_time_series() {
        let mut history = create_test_history_with_time_series();

        // Create invalid time series with mismatched lengths - should be skipped
        let invalid_time_series = TimeSeriesData {
            timestamps: vec!["2025-01-15T14:30:00Z".to_string()],
            heart_rate: Some(vec![145, 147]), // Length mismatch!
            ..Default::default()
        };

        // Also create a valid time series so export can succeed
        let valid_time_series = TimeSeriesData {
            timestamps: vec![
                "2025-01-15T14:30:00Z".to_string(),
                "2025-01-15T14:30:01Z".to_string(),
            ],
            heart_rate: Some(vec![150, 152]),
            ..Default::default()
        };

        // Set first set to invalid
        history.workouts[0].exercises[0].sets[0].telemetry = Some(SetTelemetry {
            time_series: Some(invalid_time_series),
            ..Default::default()
        });

        // Add second set with valid data
        let mut valid_set = history.workouts[0].exercises[0].sets[0].clone();
        valid_set.set_number = Some(2);
        valid_set.telemetry = Some(SetTelemetry {
            time_series: Some(valid_time_series),
            ..Default::default()
        });
        history.workouts[0].exercises[0].sets.push(valid_set);

        let options = CsvExportOptions::default();
        let result = export_telemetry_to_csv(&history, &options).unwrap();

        // Should have warnings about data quality from invalid time series
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| matches!(w, ConversionWarning::DataQualityIssue { .. })));

        // Should have exported only the valid set
        assert_eq!(result.workouts_processed, 1);
        assert_eq!(result.data_points, 2);
    }

    #[test]
    fn test_create_time_series_label() {
        let mut test_history = create_test_history_with_time_series();
        test_history.workouts[0].id = Some("workout-123".to_string());
        test_history.workouts[0].exercises[0].name = "Bench Press".to_string();
        test_history.workouts[0].exercises[0].sets[0].set_number = Some(3);

        let label = create_time_series_label(
            &test_history.workouts[0],
            &test_history.workouts[0].exercises[0],
            &test_history.workouts[0].exercises[0].sets[0],
        );
        assert_eq!(label, "workout-123-Bench Press-set3");
    }

    #[test]
    fn test_create_time_series_label_no_id() {
        let mut test_history = create_test_history_with_time_series();
        test_history.workouts[0].id = None;
        test_history.workouts[0].title = Some("Morning Workout".to_string());
        test_history.workouts[0].exercises[0].name = "Squats".to_string();
        test_history.workouts[0].exercises[0].sets[0].set_number = None;

        let label = create_time_series_label(
            &test_history.workouts[0],
            &test_history.workouts[0].exercises[0],
            &test_history.workouts[0].exercises[0].sets[0],
        );
        assert_eq!(label, "Morning Workout-Squats-set");
    }

    #[test]
    fn test_format_optional_u32() {
        let data = Some(vec![100, 200, 300]);
        assert_eq!(format_optional_u32(&data, 0), "100");
        assert_eq!(format_optional_u32(&data, 1), "200");
        assert_eq!(format_optional_u32(&data, 2), "300");
        assert_eq!(format_optional_u32(&data, 99), ""); // Out of bounds

        let empty: Option<Vec<u32>> = None;
        assert_eq!(format_optional_u32(&empty, 0), "");
    }

    #[test]
    fn test_format_optional_f64() {
        let data = Some(vec![1.5, 2.7, 3.2]);
        assert_eq!(format_optional_f64(&data, 0), "1.5");
        assert_eq!(format_optional_f64(&data, 1), "2.7");
        assert_eq!(format_optional_f64(&data, 2), "3.2");
        assert_eq!(format_optional_f64(&data, 99), ""); // Out of bounds

        let empty: Option<Vec<f64>> = None;
        assert_eq!(format_optional_f64(&empty, 0), "");
    }

    #[test]
    fn test_csv_header_all_fields() {
        let history = create_test_history_with_time_series();
        let options = CsvExportOptions::default();
        let result = export_telemetry_to_csv(&history, &options).unwrap();

        let header_line = result.csv_data.lines().next().unwrap();
        let fields: Vec<&str> = header_line.split(',').collect();

        // Verify all expected fields are present
        assert!(fields.contains(&"workout_label"));
        assert!(fields.contains(&"timestamp"));
        assert!(fields.contains(&"elapsed_sec"));
        assert!(fields.contains(&"heart_rate"));
        assert!(fields.contains(&"power"));
        assert!(fields.contains(&"cadence"));
        assert!(fields.contains(&"speed_mps"));
        assert!(fields.contains(&"elevation_m"));
        assert!(fields.contains(&"latitude"));
        assert!(fields.contains(&"longitude"));
        assert!(fields.contains(&"distance_m"));
        assert!(fields.contains(&"temperature_c"));
    }

    #[test]
    fn test_multiple_workouts() {
        let mut history = create_test_history_with_time_series();

        // Add a second workout
        let mut workout2 = history.workouts[0].clone();
        workout2.id = Some("workout-2".to_string());
        history.workouts.push(workout2);

        let options = CsvExportOptions::default();
        let result = export_telemetry_to_csv(&history, &options).unwrap();

        assert_eq!(result.data_points, 6); // 3 points × 2 workouts
        assert_eq!(result.workouts_processed, 2);

        // Verify both workout labels are present
        assert!(result.csv_data.contains("test-workout"));
        assert!(result.csv_data.contains("workout-2"));
    }
}
