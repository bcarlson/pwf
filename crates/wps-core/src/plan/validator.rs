//! Validation rules for WPS plans

use super::parser::parse;
use super::types::{PlanStatistics, WpsPlan};
use crate::error::ValidationIssue;
use crate::Modality;
use std::collections::HashSet;

/// Result of plan validation
#[derive(Debug, Clone, serde::Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<WpsPlan>,
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<PlanStatistics>,
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

/// Validate a YAML string as a WPS plan
pub fn validate(yaml: &str) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Parse YAML
    let plan = match parse(yaml) {
        Ok(p) => p,
        Err(e) => {
            errors.push(ValidationIssue::error("", e.to_string()));
            return ValidationResult {
                valid: false,
                plan: None,
                errors,
                warnings,
                statistics: None,
            };
        }
    };

    // Validate plan_version
    if plan.plan_version != 1 {
        errors.push(ValidationIssue::error(
            "plan_version",
            format!(
                "Unsupported plan_version: {}. Only version 1 is supported.",
                plan.plan_version
            ),
        ));
    }

    // Validate meta
    if let Some(ref meta) = plan.meta {
        if meta.title.is_empty() {
            errors.push(ValidationIssue::error(
                "meta.title",
                "title cannot be empty",
            ));
        }
        if meta.title.len() > 80 {
            errors.push(ValidationIssue::error(
                "meta.title",
                format!("title exceeds 80 characters ({} chars)", meta.title.len()),
            ));
        }
    } else {
        warnings.push(ValidationIssue::warning(
            "meta",
            "Missing meta section - plan will have no title",
        ));
    }

    // Validate days
    if plan.cycle.days.is_empty() {
        errors.push(ValidationIssue::error(
            "cycle.days",
            "Must have at least 1 day",
        ));
    }

    // Track day orders for duplicate detection
    let mut seen_orders = HashSet::new();

    // Validate each day
    for (day_idx, day) in plan.cycle.days.iter().enumerate() {
        let day_path = format!("cycle.days[{}]", day_idx);

        // Check order
        if let Some(order) = day.order {
            if seen_orders.contains(&order) {
                errors.push(ValidationIssue::error(
                    format!("{}.order", day_path),
                    format!("Duplicate day order: {}", order),
                ));
            }
            seen_orders.insert(order);
        }

        // Check exercises
        if day.exercises.is_empty() {
            errors.push(ValidationIssue::error(
                format!("{}.exercises", day_path),
                "Day must have at least 1 exercise",
            ));
        }

        // Validate each exercise
        for (ex_idx, exercise) in day.exercises.iter().enumerate() {
            let ex_path = format!("{}.exercises[{}]", day_path, ex_idx);

            // Check name
            if exercise.name.is_none() {
                warnings.push(ValidationIssue::warning(
                    format!("{}.name", ex_path),
                    "Missing exercise name",
                ));
            }

            // Modality-specific validation
            match exercise.modality {
                Modality::Strength => {
                    if exercise.target_sets.is_none() && exercise.target_reps.is_none() {
                        warnings.push(ValidationIssue::warning(
                            &ex_path,
                            "Strength exercise missing target_sets/target_reps",
                        ));
                    }
                }
                Modality::Countdown => {
                    if exercise.target_duration_sec.is_none() {
                        warnings.push(ValidationIssue::warning(
                            &ex_path,
                            "Countdown exercise missing target_duration_sec",
                        ));
                    }
                }
                Modality::Interval => {
                    if exercise.target_sets.is_none() {
                        warnings.push(ValidationIssue::warning(
                            &ex_path,
                            "Interval exercise missing target_sets",
                        ));
                    }
                }
                Modality::Stopwatch => {
                    // No required fields
                }
            }

            // Validate URLs
            if let Some(ref link) = exercise.link {
                if !link.starts_with("https://") {
                    if link.starts_with("http://") {
                        warnings.push(ValidationIssue::warning(
                            format!("{}.link", ex_path),
                            "HTTP URLs not allowed, use HTTPS",
                        ));
                    } else {
                        errors.push(ValidationIssue::error(
                            format!("{}.link", ex_path),
                            "Invalid URL format",
                        ));
                    }
                }
            }

            if let Some(ref image) = exercise.image {
                if !image.starts_with("https://") {
                    warnings.push(ValidationIssue::warning(
                        format!("{}.image", ex_path),
                        "Image URL should use HTTPS",
                    ));
                }
            }
        }
    }

    // Calculate statistics if valid
    let statistics = if errors.is_empty() {
        Some(calculate_statistics(&plan))
    } else {
        None
    };

    let valid = errors.is_empty();

    ValidationResult {
        valid,
        plan: if valid { Some(plan) } else { None },
        errors,
        warnings,
        statistics,
    }
}

fn calculate_statistics(plan: &WpsPlan) -> PlanStatistics {
    let mut stats = PlanStatistics {
        total_days: plan.cycle.days.len(),
        ..Default::default()
    };

    for day in &plan.cycle.days {
        stats.total_exercises += day.exercises.len();

        for exercise in &day.exercises {
            match exercise.modality {
                Modality::Strength => stats.strength_count += 1,
                Modality::Countdown => stats.countdown_count += 1,
                Modality::Stopwatch => stats.stopwatch_count += 1,
                Modality::Interval => stats.interval_count += 1,
            }
        }
    }

    if let Some(ref meta) = plan.meta {
        stats.equipment = meta.equipment.clone();
    }

    stats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_minimal_valid_plan() {
        let yaml = r#"
plan_version: 1
cycle:
  days:
    - exercises:
        - name: Push-ups
          modality: strength
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
    }

    #[test]
    fn validate_missing_days() {
        let yaml = r#"
plan_version: 1
cycle:
  days: []
"#;
        let result = validate(yaml);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| e.message.contains("at least 1 day")));
    }

    #[test]
    fn validate_unsupported_version() {
        let yaml = r#"
plan_version: 99
cycle:
  days:
    - exercises:
        - modality: strength
"#;
        let result = validate(yaml);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| e.message.contains("Unsupported")));
    }

    #[test]
    fn validate_statistics() {
        let yaml = r#"
plan_version: 1
meta:
  title: "Test"
  equipment: [barbell]
cycle:
  days:
    - exercises:
        - name: Squat
          modality: strength
        - name: Plank
          modality: countdown
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        let stats = result.statistics.unwrap();
        assert_eq!(stats.total_days, 1);
        assert_eq!(stats.total_exercises, 2);
        assert_eq!(stats.strength_count, 1);
        assert_eq!(stats.countdown_count, 1);
    }
}
