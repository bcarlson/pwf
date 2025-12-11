//! Validation rules for PWF history exports

use super::parser::parse;
use super::types::{HistoryStatistics, WpsHistory};
use crate::error::ValidationIssue;

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
        errors.push(ValidationIssue::error(
            "history_version",
            format!(
                "Unsupported history_version: {}. Only version 1 is supported.",
                history.history_version
            ),
        ));
    }

    // Validate exported_at
    if history.exported_at.is_empty() {
        errors.push(ValidationIssue::error(
            "exported_at",
            "exported_at timestamp is required",
        ));
    }

    // Validate workouts
    for (workout_idx, workout) in history.workouts.iter().enumerate() {
        let workout_path = format!("workouts[{}]", workout_idx);

        // Validate date
        if workout.date.is_empty() {
            errors.push(ValidationIssue::error(
                format!("{}.date", workout_path),
                "Workout date is required",
            ));
        }

        // Validate exercises
        if workout.exercises.is_empty() {
            warnings.push(ValidationIssue::warning(
                format!("{}.exercises", workout_path),
                "Workout has no exercises",
            ));
        }

        for (ex_idx, exercise) in workout.exercises.iter().enumerate() {
            let ex_path = format!("{}.exercises[{}]", workout_path, ex_idx);

            // Validate exercise name
            if exercise.name.is_empty() {
                errors.push(ValidationIssue::error(
                    format!("{}.name", ex_path),
                    "Exercise name is required",
                ));
            }

            // Validate sets
            if exercise.sets.is_empty() {
                warnings.push(ValidationIssue::warning(
                    format!("{}.sets", ex_path),
                    "Exercise has no recorded sets",
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
                    warnings.push(ValidationIssue::warning(
                        &set_path,
                        "Set has no recorded metrics (reps, weight, duration, or distance)",
                    ));
                }

                // Validate RPE range
                if let Some(rpe) = set.rpe {
                    if !(0.0..=10.0).contains(&rpe) {
                        warnings.push(ValidationIssue::warning(
                            format!("{}.rpe", set_path),
                            format!("RPE should be between 0 and 10, got {}", rpe),
                        ));
                    }
                }
            }
        }
    }

    // Validate personal records
    for (pr_idx, pr) in history.personal_records.iter().enumerate() {
        let pr_path = format!("personal_records[{}]", pr_idx);

        if pr.exercise_name.is_empty() {
            errors.push(ValidationIssue::error(
                format!("{}.exercise_name", pr_path),
                "Personal record must have exercise_name",
            ));
        }

        if pr.achieved_at.is_empty() {
            errors.push(ValidationIssue::error(
                format!("{}.achieved_at", pr_path),
                "Personal record must have achieved_at date",
            ));
        }
    }

    // Validate body measurements
    for (bm_idx, bm) in history.body_measurements.iter().enumerate() {
        let bm_path = format!("body_measurements[{}]", bm_idx);

        if bm.date.is_empty() {
            errors.push(ValidationIssue::error(
                format!("{}.date", bm_path),
                "Body measurement must have date",
            ));
        }

        // Check for at least one measurement
        let has_measurement = bm.weight_kg.is_some()
            || bm.weight_lb.is_some()
            || bm.body_fat_percent.is_some()
            || bm.measurements.is_some();

        if !has_measurement {
            warnings.push(ValidationIssue::warning(
                &bm_path,
                "Body measurement entry has no recorded values",
            ));
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
}
