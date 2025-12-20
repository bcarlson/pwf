//! Validation rules for PWF history exports

use super::error_codes;
use super::parser::parse;
use super::types::{HistoryStatistics, RecordType, WpsHistory};
use crate::error::ValidationIssue;
use crate::types::WeightUnit;

/// Result of history validation
#[derive(Debug, Clone, serde::Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<WpsHistory>,
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<HistoryStatistics>,
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

/// Validate a YAML string as a PWF history export
pub fn validate(yaml: &str) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Parse YAML
    let history = match parse(yaml) {
        Ok(h) => h,
        Err(e) => {
            errors.push(ValidationIssue::error("", e.to_string()));
            return ValidationResult {
                valid: false,
                history: None,
                errors,
                warnings,
                statistics: None,
            };
        }
    };

    // Validate history_version (support v1 and v2)
    if history.history_version != 1 && history.history_version != 2 {
        errors.push(ValidationIssue::error_with_code(
            "history_version",
            format!(
                "Unsupported history_version: {}. Supported versions: 1, 2",
                history.history_version
            ),
            error_codes::INVALID_VERSION,
        ));
    }

    // Validate exported_at
    if history.exported_at.is_empty() {
        errors.push(ValidationIssue::error_with_code(
            "exported_at",
            "exported_at timestamp is required",
            error_codes::MISSING_EXPORTED_AT,
        ));
    }

    // Validate workouts
    for (workout_idx, workout) in history.workouts.iter().enumerate() {
        let workout_path = format!("workouts[{}]", workout_idx);

        // Validate date
        if workout.date.is_empty() {
            errors.push(ValidationIssue::error_with_code(
                format!("{}.date", workout_path),
                "Workout date is required",
                error_codes::MISSING_WORKOUT_DATE,
            ));
        }

        // Validate exercises
        if workout.exercises.is_empty() {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.exercises", workout_path),
                "Workout has no exercises",
                error_codes::NO_EXERCISES,
            ));
        }

        for (ex_idx, exercise) in workout.exercises.iter().enumerate() {
            let ex_path = format!("{}.exercises[{}]", workout_path, ex_idx);

            // Validate exercise name
            if exercise.name.is_empty() {
                errors.push(ValidationIssue::error_with_code(
                    format!("{}.name", ex_path),
                    "Exercise name is required",
                    error_codes::MISSING_EXERCISE_NAME,
                ));
            }

            // Validate sets
            if exercise.sets.is_empty() {
                warnings.push(ValidationIssue::warning_with_code(
                    format!("{}.sets", ex_path),
                    "Exercise has no recorded sets",
                    error_codes::NO_SETS,
                ));
            }

            for (set_idx, set) in exercise.sets.iter().enumerate() {
                let set_path = format!("{}.sets[{}]", ex_path, set_idx);

                // Check for at least one metric
                let has_metric = set.reps.is_some()
                    || set.weight_kg.is_some()
                    || set.weight_lb.is_some()
                    || set.duration_sec.is_some()
                    || set.distance_meters.is_some();

                if !has_metric {
                    warnings.push(ValidationIssue::warning_with_code(
                        &set_path,
                        "Set has no recorded metrics (reps, weight, duration, or distance)",
                        error_codes::NO_METRICS,
                    ));
                }

                // Validate RPE range
                if let Some(rpe) = set.rpe {
                    if !(0.0..=10.0).contains(&rpe) {
                        warnings.push(ValidationIssue::warning_with_code(
                            format!("{}.rpe", set_path),
                            format!("RPE should be between 0 and 10, got {}", rpe),
                            error_codes::RPE_OUT_OF_RANGE,
                        ));
                    }
                }

                // Validate RIR range
                if let Some(rir) = set.rir {
                    if rir > 10 {
                        warnings.push(ValidationIssue::warning_with_code(
                            format!("{}.rir", set_path),
                            format!("RIR typically ranges 0-10, got {}", rir),
                            error_codes::RIR_OUT_OF_RANGE,
                        ));
                    }
                }

                // Warn if both RPE and RIR are set
                if set.rpe.is_some() && set.rir.is_some() {
                    warnings.push(ValidationIssue::warning_with_code(
                        &set_path,
                        "Both RPE and RIR are set. Typically only one should be used.",
                        error_codes::RPE_RIR_BOTH_SET,
                    ));
                }

                // Validate telemetry (v2 feature)
                if let Some(telemetry) = &set.telemetry {
                    validate_set_telemetry(&set_path, telemetry, &mut warnings);
                }
            }
        }

        // Validate workout-level telemetry (v2 feature)
        if let Some(telemetry) = &workout.telemetry {
            validate_workout_telemetry(&workout_path, telemetry, &mut warnings);
        }
    }

    // Validate personal records
    for (pr_idx, pr) in history.personal_records.iter().enumerate() {
        let pr_path = format!("personal_records[{}]", pr_idx);

        if pr.exercise_name.is_empty() {
            errors.push(ValidationIssue::error_with_code(
                format!("{}.exercise_name", pr_path),
                "Personal record must have exercise_name",
                error_codes::MISSING_PR_EXERCISE,
            ));
        }

        if pr.achieved_at.is_empty() {
            errors.push(ValidationIssue::error_with_code(
                format!("{}.achieved_at", pr_path),
                "Personal record must have achieved_at date",
                error_codes::MISSING_PR_DATE,
            ));
        }

        // Validate rep-specific PRs have units
        match pr.record_type {
            RecordType::OneRepMax
            | RecordType::MaxWeight3rm
            | RecordType::MaxWeight5rm
            | RecordType::MaxWeight8rm
            | RecordType::MaxWeight10rm
            | RecordType::MaxWeight => {
                if pr.unit.is_none() {
                    warnings.push(ValidationIssue::warning_with_code(
                        format!("{}.unit", pr_path),
                        "Weight-based personal records should specify a unit (kg or lb)",
                        error_codes::PR_MISSING_UNIT,
                    ));
                }
            }
            RecordType::MaxDistance | RecordType::FastestTime => {
                if pr.unit.is_none() {
                    warnings.push(ValidationIssue::warning_with_code(
                        format!("{}.unit", pr_path),
                        "Distance/time personal records should specify appropriate units",
                        error_codes::PR_MISSING_UNIT,
                    ));
                }
            }
            _ => {}
        }
    }

    // Validate body measurements
    for (bm_idx, bm) in history.body_measurements.iter().enumerate() {
        let bm_path = format!("body_measurements[{}]", bm_idx);

        if bm.date.is_empty() {
            errors.push(ValidationIssue::error_with_code(
                format!("{}.date", bm_path),
                "Body measurement must have date",
                error_codes::MISSING_BM_DATE,
            ));
        }

        // Check for at least one measurement
        let has_measurement = bm.weight_kg.is_some()
            || bm.weight_lb.is_some()
            || bm.body_fat_percent.is_some()
            || bm.measurements.is_some();

        if !has_measurement {
            warnings.push(ValidationIssue::warning_with_code(
                &bm_path,
                "Body measurement entry has no recorded values",
                error_codes::NO_BM_VALUES,
            ));
        }
    }

    // Validate preferred_units consistency
    if let Some(export_source) = &history.export_source {
        if let Some(preferred) = &export_source.preferred_units {
            // Check if preferred units match actual usage in workouts
            let mut uses_kg = false;
            let mut uses_lb = false;

            for workout in &history.workouts {
                for exercise in &workout.exercises {
                    for set in &exercise.sets {
                        if set.weight_kg.is_some() {
                            uses_kg = true;
                        }
                        if set.weight_lb.is_some() {
                            uses_lb = true;
                        }
                    }
                }
            }

            // Warn if preferred unit doesn't match actual data
            if uses_kg && !uses_lb {
                // Only kg data present
                if preferred.weight == WeightUnit::Lb {
                    warnings.push(ValidationIssue::warning_with_code(
                        "export_source.preferred_units.weight",
                        "Preferred weight unit is 'lb', but only kg values are present in workouts",
                        error_codes::PREFERRED_UNITS_MISMATCH,
                    ));
                }
            } else if uses_lb && !uses_kg {
                // Only lb data present
                if preferred.weight == WeightUnit::Kg {
                    warnings.push(ValidationIssue::warning_with_code(
                        "export_source.preferred_units.weight",
                        "Preferred weight unit is 'kg', but only lb values are present in workouts",
                        error_codes::PREFERRED_UNITS_MISMATCH,
                    ));
                }
            }
        }
    }

    // Calculate statistics
    let statistics = if errors.is_empty() {
        Some(calculate_statistics(&history))
    } else {
        None
    };

    let valid = errors.is_empty();

    ValidationResult {
        valid,
        history: if valid { Some(history) } else { None },
        errors,
        warnings,
        statistics,
    }
}

fn calculate_statistics(history: &WpsHistory) -> HistoryStatistics {
    let mut stats = HistoryStatistics {
        total_workouts: history.workouts.len(),
        personal_records_count: history.personal_records.len(),
        body_measurements_count: history.body_measurements.len(),
        ..Default::default()
    };

    let mut dates: Vec<&str> = Vec::new();

    for workout in &history.workouts {
        dates.push(&workout.date);
        stats.total_exercises += workout.exercises.len();

        for exercise in &workout.exercises {
            stats.total_sets += exercise.sets.len();

            for set in &exercise.sets {
                if let (Some(reps), Some(weight)) = (set.reps, set.weight_kg) {
                    stats.total_volume_kg += reps as f64 * weight;
                }
            }
        }
    }

    // Get date range
    dates.sort();
    if let Some(first) = dates.first() {
        stats.date_range_start = Some(first.to_string());
    }
    if let Some(last) = dates.last() {
        stats.date_range_end = Some(last.to_string());
    }

    stats
}

fn validate_set_telemetry(
    path: &str,
    telemetry: &super::types::SetTelemetry,
    warnings: &mut Vec<ValidationIssue>,
) {
    // Validate heart rate
    if let Some(hr) = telemetry.heart_rate_avg {
        if hr > 250 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.heart_rate_avg", path),
                format!("Heart rate {} seems unusually high", hr),
                error_codes::HEART_RATE_OUT_OF_RANGE,
            ));
        }
    }
    if let Some(hr) = telemetry.heart_rate_max {
        if hr > 250 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.heart_rate_max", path),
                format!("Max heart rate {} seems unusually high", hr),
                error_codes::HEART_RATE_OUT_OF_RANGE,
            ));
        }
    }
    if let Some(hr) = telemetry.heart_rate_min {
        if hr < 30 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.heart_rate_min", path),
                format!("Min heart rate {} seems unusually low", hr),
                error_codes::HEART_RATE_OUT_OF_RANGE,
            ));
        }
    }

    // Validate power (watts)
    if let Some(power) = telemetry.power_avg {
        if power > 2000 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.power_avg", path),
                format!("Average power {} watts seems unusually high", power),
                error_codes::POWER_NEGATIVE,
            ));
        }
    }

    // Validate elevation
    if let Some(elev) = telemetry.elevation_gain_m {
        if elev < 0.0 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.elevation_gain_m", path),
                "Elevation gain cannot be negative",
                error_codes::ELEVATION_NEGATIVE,
            ));
        }
    }
    if let Some(elev) = telemetry.elevation_gain_ft {
        if elev < 0.0 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.elevation_gain_ft", path),
                "Elevation gain cannot be negative",
                error_codes::ELEVATION_NEGATIVE,
            ));
        }
    }

    // Validate speed
    if let Some(speed) = telemetry.speed_avg_mps {
        if speed < 0.0 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.speed_avg_mps", path),
                "Speed cannot be negative",
                error_codes::SPEED_NEGATIVE,
            ));
        }
    }

    // Validate cadence
    if let Some(cadence) = telemetry.cadence_avg {
        if cadence > 300 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.cadence_avg", path),
                format!("Cadence {} seems unusually high", cadence),
                error_codes::CADENCE_OUT_OF_RANGE,
            ));
        }
    }

    // Validate humidity
    if let Some(humidity) = telemetry.humidity_percent {
        if !(0.0..=100.0).contains(&humidity) {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.humidity_percent", path),
                format!("Humidity must be between 0 and 100, got {}", humidity),
                error_codes::HUMIDITY_OUT_OF_RANGE,
            ));
        }
    }

    // Validate pace
    if let Some(pace) = telemetry.pace_avg_sec_per_km {
        if pace == 0 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.pace_avg_sec_per_km", path),
                "Pace cannot be zero",
                error_codes::PACE_NEGATIVE,
            ));
        }
    }

    // Warn if conflicting unit systems are used
    let has_metric_elevation =
        telemetry.elevation_gain_m.is_some() || telemetry.elevation_loss_m.is_some();
    let has_imperial_elevation =
        telemetry.elevation_gain_ft.is_some() || telemetry.elevation_loss_ft.is_some();

    if has_metric_elevation && has_imperial_elevation {
        warnings.push(ValidationIssue::warning_with_code(
            format!("{}.telemetry", path),
            "Both metric and imperial elevation units provided. Consider using only one unit system.",
            error_codes::TELEMETRY_UNIT_MISMATCH,
        ));
    }

    let has_metric_speed = telemetry.speed_avg_kph.is_some() || telemetry.speed_max_kph.is_some();
    let has_imperial_speed = telemetry.speed_avg_mph.is_some() || telemetry.speed_max_mph.is_some();

    if has_metric_speed && has_imperial_speed {
        warnings.push(ValidationIssue::warning_with_code(
            format!("{}.telemetry", path),
            "Both metric (km/h) and imperial (mph) speed units provided. Consider using only one unit system.",
            error_codes::TELEMETRY_UNIT_MISMATCH,
        ));
    }
}

fn validate_workout_telemetry(
    path: &str,
    telemetry: &super::types::WorkoutTelemetry,
    warnings: &mut Vec<ValidationIssue>,
) {
    // Validate heart rate
    if let Some(hr) = telemetry.heart_rate_avg {
        if hr > 250 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.heart_rate_avg", path),
                format!("Average heart rate {} seems unusually high", hr),
                error_codes::HEART_RATE_OUT_OF_RANGE,
            ));
        }
    }

    // Validate power
    if let Some(power) = telemetry.power_avg {
        if power > 2000 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.power_avg", path),
                format!("Average power {} watts seems unusually high", power),
                error_codes::POWER_NEGATIVE,
            ));
        }
    }

    // Validate total distance
    if let Some(dist) = telemetry.total_distance_m {
        if dist < 0.0 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.total_distance_m", path),
                "Total distance cannot be negative",
                error_codes::SPEED_NEGATIVE,
            ));
        }
    }

    // Validate elevation
    if let Some(elev) = telemetry.total_elevation_gain_m {
        if elev < 0.0 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.total_elevation_gain_m", path),
                "Total elevation gain cannot be negative",
                error_codes::ELEVATION_NEGATIVE,
            ));
        }
    }

    // Validate humidity
    if let Some(humidity) = telemetry.humidity_percent {
        if !(0.0..=100.0).contains(&humidity) {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.humidity_percent", path),
                format!("Humidity must be between 0 and 100, got {}", humidity),
                error_codes::HUMIDITY_OUT_OF_RANGE,
            ));
        }
    }

    // Warn about conflicting units
    let has_metric_distance =
        telemetry.total_distance_m.is_some() || telemetry.total_distance_km.is_some();
    let has_imperial_distance = telemetry.total_distance_mi.is_some();

    if has_metric_distance && has_imperial_distance {
        warnings.push(ValidationIssue::warning_with_code(
            format!("{}.telemetry", path),
            "Both metric and imperial distance units provided. Consider using only one unit system.",
            error_codes::TELEMETRY_UNIT_MISMATCH,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_minimal_history() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: Squat
        sets:
          - reps: 5
            weight_kg: 100
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
    }

    #[test]
    fn validate_missing_date() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - exercises:
      - name: Squat
        sets:
          - reps: 5
"#;
        let result = validate(yaml);
        assert!(!result.is_valid());
        assert!(result.errors.iter().any(|e| e.message.contains("date")));
    }

    #[test]
    fn validate_statistics() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: Squat
        sets:
          - reps: 5
            weight_kg: 100
          - reps: 5
            weight_kg: 100
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        let stats = result.statistics.unwrap();
        assert_eq!(stats.total_workouts, 1);
        assert_eq!(stats.total_sets, 2);
        assert_eq!(stats.total_volume_kg, 1000.0);
    }

    #[test]
    fn validate_rir_out_of_range() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: Squat
        sets:
          - reps: 5
            weight_kg: 100
            rir: 15
"#;
        let result = validate(yaml);
        assert!(result.is_valid()); // Still valid, just a warning
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::RIR_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_rir_valid_range() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: Squat
        sets:
          - reps: 5
            weight_kg: 100
            rir: 2
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::RIR_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_rpe_rir_both_set() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: Squat
        sets:
          - reps: 5
            weight_kg: 100
            rpe: 8.0
            rir: 2
"#;
        let result = validate(yaml);
        assert!(result.is_valid()); // Still valid, just a warning
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::RPE_RIR_BOTH_SET.to_string())));
    }

    #[test]
    fn validate_pr_missing_unit_one_rep_max() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "Bench Press"
    record_type: 1rm
    value: 225.0
    achieved_at: "2025-01-10"
"#;
        let result = validate(yaml);
        assert!(result.is_valid()); // Still valid, just a warning
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    #[test]
    fn validate_pr_with_unit_three_rm() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "Squat"
    record_type: max_weight_3rm
    value: 315
    unit: lb
    achieved_at: "2025-01-12"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    #[test]
    fn validate_preferred_units_mismatch_kg_data_lb_preferred() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
export_source:
  app_name: "Test App"
  preferred_units:
    weight: lb
workouts:
  - date: "2025-01-15"
    exercises:
      - name: Squat
        sets:
          - reps: 5
            weight_kg: 100
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PREFERRED_UNITS_MISMATCH.to_string())));
    }

    #[test]
    fn validate_preferred_units_mismatch_lb_data_kg_preferred() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
export_source:
  app_name: "Test App"
  preferred_units:
    weight: kg
workouts:
  - date: "2025-01-15"
    exercises:
      - name: Bench Press
        sets:
          - reps: 5
            weight_lb: 225
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PREFERRED_UNITS_MISMATCH.to_string())));
    }

    #[test]
    fn validate_preferred_units_match() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
export_source:
  app_name: "Test App"
  preferred_units:
    weight: kg
workouts:
  - date: "2025-01-15"
    exercises:
      - name: Squat
        sets:
          - reps: 5
            weight_kg: 100
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PREFERRED_UNITS_MISMATCH.to_string())));
    }

    // RecordType Tests - Testing all variants
    #[test]
    fn validate_pr_max_weight_missing_unit() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "Deadlift"
    record_type: max_weight
    value: 500.0
    achieved_at: "2025-01-10"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    #[test]
    fn validate_pr_max_reps_no_unit_warning() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "Pull-ups"
    record_type: max_reps
    value: 25.0
    achieved_at: "2025-01-10"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        // max_reps should NOT warn about missing unit
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    #[test]
    fn validate_pr_max_volume_missing_unit() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "Squat"
    record_type: max_volume
    value: 5000.0
    achieved_at: "2025-01-10"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        // max_volume should NOT warn about missing unit (currently not implemented)
        // This test documents current behavior
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    #[test]
    fn validate_pr_max_duration_missing_unit() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "Plank"
    record_type: max_duration
    value: 300.0
    achieved_at: "2025-01-10"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        // max_duration should NOT warn about missing unit (currently not implemented)
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    #[test]
    fn validate_pr_max_distance_missing_unit() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "Run"
    record_type: max_distance
    value: 10000.0
    achieved_at: "2025-01-10"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    #[test]
    fn validate_pr_fastest_time_missing_unit() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "5K Run"
    record_type: fastest_time
    value: 1200.0
    achieved_at: "2025-01-10"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    #[test]
    fn validate_pr_max_weight_8rm_missing_unit() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "Bench Press"
    record_type: max_weight_8rm
    value: 185.0
    achieved_at: "2025-01-10"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    #[test]
    fn validate_pr_max_weight_10rm_with_unit() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "Overhead Press"
    record_type: max_weight_10rm
    value: 135.0
    unit: lb
    achieved_at: "2025-01-10"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    // Body Measurement Tests
    #[test]
    fn validate_body_measurement_missing_date() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
body_measurements:
  - date: ""
    weight_kg: 75.5
"#;
        let result = validate(yaml);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| e.code == Some(error_codes::MISSING_BM_DATE.to_string())));
    }

    #[test]
    fn validate_body_measurement_no_values() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
body_measurements:
  - date: "2025-01-15"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::NO_BM_VALUES.to_string())));
    }

    #[test]
    fn validate_body_measurement_valid() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
body_measurements:
  - date: "2025-01-15"
    weight_kg: 75.5
    body_fat_percent: 15.2
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::NO_BM_VALUES.to_string())));
    }

    #[test]
    fn validate_body_measurement_with_dimensions() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
body_measurements:
  - date: "2025-01-15"
    measurements:
      chest_cm: 100.0
      waist_cm: 85.0
      bicep_left_cm: 38.5
      bicep_right_cm: 38.0
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::NO_BM_VALUES.to_string())));
    }

    // Empty Array Tests
    #[test]
    fn validate_empty_exercises() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises: []
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::NO_EXERCISES.to_string())));
    }

    #[test]
    fn validate_empty_sets() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Bench Press"
        sets: []
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::NO_SETS.to_string())));
    }

    #[test]
    fn validate_set_no_metrics() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Bench Press"
        sets:
          - set_number: 1
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::NO_METRICS.to_string())));
    }

    // RPE Edge Cases
    #[test]
    fn validate_rpe_exactly_zero() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Warmup"
        sets:
          - reps: 10
            rpe: 0.0
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::RPE_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_rpe_exactly_ten() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Max Effort"
        sets:
          - reps: 1
            weight_kg: 200
            rpe: 10.0
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::RPE_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_rpe_negative() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Squat"
        sets:
          - reps: 5
            weight_kg: 100
            rpe: -1.0
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::RPE_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_rpe_above_ten() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Squat"
        sets:
          - reps: 5
            weight_kg: 100
            rpe: 10.5
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::RPE_OUT_OF_RANGE.to_string())));
    }

    // Export Source Tests
    #[test]
    fn validate_export_source_missing_app_name() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
export_source:
  platform: "iOS"
  app_version: "1.0.0"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Squat"
        sets:
          - reps: 5
            weight_kg: 100
"#;
        let result = validate(yaml);
        // app_name is optional, should still be valid
        assert!(result.is_valid());
    }

    #[test]
    fn validate_export_source_different_platforms() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
export_source:
  app_name: "Fitness Tracker"
  platform: "Android"
  app_version: "2.1.0"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Deadlift"
        sets:
          - reps: 5
            weight_kg: 150
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
    }

    // Statistics Edge Cases
    #[test]
    fn validate_statistics_empty_workouts() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        let stats = result.statistics.unwrap();
        assert_eq!(stats.total_workouts, 0);
        assert_eq!(stats.total_exercises, 0);
        assert_eq!(stats.total_sets, 0);
        assert_eq!(stats.total_volume_kg, 0.0);
        assert!(stats.date_range_start.is_none());
        assert!(stats.date_range_end.is_none());
    }

    #[test]
    fn validate_statistics_workout_no_sets() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Stretching"
        sets: []
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        let stats = result.statistics.unwrap();
        assert_eq!(stats.total_workouts, 1);
        assert_eq!(stats.total_exercises, 1);
        assert_eq!(stats.total_sets, 0);
        assert_eq!(stats.total_volume_kg, 0.0);
    }

    #[test]
    fn validate_statistics_single_workout_date_range() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-10"
    exercises:
      - name: "Squat"
        sets:
          - reps: 5
            weight_kg: 100
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        let stats = result.statistics.unwrap();
        assert_eq!(stats.date_range_start, Some("2025-01-10".to_string()));
        assert_eq!(stats.date_range_end, Some("2025-01-10".to_string()));
    }

    #[test]
    fn validate_statistics_multiple_workouts_date_range() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-05"
    exercises:
      - name: "Bench Press"
        sets:
          - reps: 5
            weight_kg: 80
  - date: "2025-01-10"
    exercises:
      - name: "Squat"
        sets:
          - reps: 5
            weight_kg: 100
  - date: "2025-01-15"
    exercises:
      - name: "Deadlift"
        sets:
          - reps: 3
            weight_kg: 150
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        let stats = result.statistics.unwrap();
        assert_eq!(stats.total_workouts, 3);
        assert_eq!(stats.date_range_start, Some("2025-01-05".to_string()));
        assert_eq!(stats.date_range_end, Some("2025-01-15".to_string()));
    }

    // ===== History v2 Tests =====

    #[test]
    fn validate_history_v2() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            distance_meters: 5000
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
    }

    #[test]
    fn validate_history_unsupported_version() {
        let yaml = r#"
history_version: 3
exported_at: "2025-12-20T10:30:00Z"
workouts: []
"#;
        let result = validate(yaml);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| e.code == Some(error_codes::INVALID_VERSION.to_string())));
    }

    // ===== Telemetry Tests =====

    #[test]
    fn validate_set_telemetry_heart_rate() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Cycling"
        sets:
          - duration_sec: 600
            telemetry:
              heart_rate_avg: 145
              heart_rate_max: 165
              heart_rate_min: 120
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_set_telemetry_heart_rate_high() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Cycling"
        sets:
          - duration_sec: 600
            telemetry:
              heart_rate_avg: 255
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::HEART_RATE_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_set_telemetry_heart_rate_low() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Cycling"
        sets:
          - duration_sec: 600
            telemetry:
              heart_rate_min: 25
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::HEART_RATE_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_set_telemetry_power() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Cycling"
        sets:
          - duration_sec: 600
            telemetry:
              power_avg: 250
              power_max: 420
              power_min: 180
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_set_telemetry_power_very_high() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Cycling"
        sets:
          - duration_sec: 600
            telemetry:
              power_avg: 2500
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::POWER_NEGATIVE.to_string())));
    }

    #[test]
    fn validate_set_telemetry_elevation() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              elevation_gain_m: 150.5
              elevation_loss_m: 125.2
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_set_telemetry_elevation_negative() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              elevation_gain_m: -50.0
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::ELEVATION_NEGATIVE.to_string())));
    }

    #[test]
    fn validate_set_telemetry_speed() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              speed_avg_kph: 12.5
              speed_max_kph: 15.8
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_set_telemetry_speed_negative() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              speed_avg_mps: -2.5
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::SPEED_NEGATIVE.to_string())));
    }

    #[test]
    fn validate_set_telemetry_cadence() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              cadence_avg: 172
              cadence_max: 185
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_set_telemetry_cadence_very_high() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              cadence_avg: 350
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::CADENCE_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_set_telemetry_humidity() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              humidity_percent: 65.5
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_set_telemetry_humidity_out_of_range() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              humidity_percent: 150.0
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::HUMIDITY_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_set_telemetry_pace_zero() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              pace_avg_sec_per_km: 0
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PACE_NEGATIVE.to_string())));
    }

    #[test]
    fn validate_set_telemetry_unit_mismatch_elevation() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              elevation_gain_m: 150.0
              elevation_gain_ft: 492.0
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::TELEMETRY_UNIT_MISMATCH.to_string())));
    }

    #[test]
    fn validate_set_telemetry_unit_mismatch_speed() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              speed_avg_kph: 12.5
              speed_avg_mph: 7.8
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::TELEMETRY_UNIT_MISMATCH.to_string())));
    }

    #[test]
    fn validate_workout_telemetry() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    telemetry:
      heart_rate_avg: 155
      heart_rate_max: 182
      power_avg: 245
      total_distance_km: 45.2
      total_elevation_gain_m: 250
      speed_avg_kph: 30.1
      cadence_avg: 88
      total_calories: 680
      temperature_c: 22.5
      humidity_percent: 65
    exercises:
      - name: "Cycling"
        sets:
          - duration_sec: 5400
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_workout_telemetry_unit_mismatch() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    telemetry:
      total_distance_km: 10.0
      total_distance_mi: 6.2
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::TELEMETRY_UNIT_MISMATCH.to_string())));
    }

    #[test]
    fn validate_swimming_with_stroke_rate() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Swimming"
        modality: swimming
        sets:
          - duration_sec: 105
            distance_meters: 100
            telemetry:
              heart_rate_avg: 145
              pace_avg_sec_per_km: 1050
              stroke_rate: 34
              calories: 19
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_complete_telemetry_example() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    title: "Morning Run"
    telemetry:
      heart_rate_avg: 158
      heart_rate_max: 185
      total_distance_km: 10.94
      total_elevation_gain_m: 250
      speed_avg_kph: 11.9
      pace_avg_sec_per_km: 303
      cadence_avg: 172
      temperature_c: 14.4
      humidity_percent: 72
      total_calories: 785
    exercises:
      - name: "Trail Run"
        modality: running
        sets:
          - duration_sec: 1980
            distance_meters: 5500
            telemetry:
              heart_rate_avg: 165
              heart_rate_max: 185
              elevation_gain_m: 198
              speed_avg_kph: 10.1
              pace_avg_sec_per_km: 355
              cadence_avg: 168
              temperature_c: 13.3
              humidity_percent: 75
              calories: 425
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }
}
