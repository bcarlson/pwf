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

    // Validate history_version
    if history.history_version != 1 {
        errors.push(ValidationIssue::error_with_code(
            "history_version",
            format!(
                "Unsupported history_version: {}. Only version 1 is supported.",
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
            }
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
        assert!(result.warnings.iter().any(|w| w.code == Some(error_codes::RIR_OUT_OF_RANGE.to_string())));
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
        assert!(!result.warnings.iter().any(|w| w.code == Some(error_codes::RIR_OUT_OF_RANGE.to_string())));
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
        assert!(result.warnings.iter().any(|w| w.code == Some(error_codes::RPE_RIR_BOTH_SET.to_string())));
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
        assert!(result.warnings.iter().any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
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
        assert!(!result.warnings.iter().any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
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
        assert!(result.warnings.iter().any(|w| w.code == Some(error_codes::PREFERRED_UNITS_MISMATCH.to_string())));
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
        assert!(result.warnings.iter().any(|w| w.code == Some(error_codes::PREFERRED_UNITS_MISMATCH.to_string())));
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
        assert!(!result.warnings.iter().any(|w| w.code == Some(error_codes::PREFERRED_UNITS_MISMATCH.to_string())));
    }
}
