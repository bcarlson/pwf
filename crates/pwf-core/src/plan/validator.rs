//! Validation rules for PWF plans

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

/// Validate a YAML string as a PWF plan
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

        // Validate activated_at timestamp format
        if let Some(ref activated_at) = meta.activated_at {
            if !is_valid_iso8601_datetime(activated_at) {
                errors.push(ValidationIssue::error_with_code(
                    "meta.activated_at",
                    format!("Invalid ISO 8601 datetime format: {}", activated_at),
                    "PWF-P001",
                ));
            }
        }

        // Validate completed_at timestamp format
        if let Some(ref completed_at) = meta.completed_at {
            if !is_valid_iso8601_datetime(completed_at) {
                errors.push(ValidationIssue::error_with_code(
                    "meta.completed_at",
                    format!("Invalid ISO 8601 datetime format: {}", completed_at),
                    "PWF-P002",
                ));
            }
        }

        // Validate status consistency with timestamps
        if let Some(status) = meta.status {
            use super::types::PlanStatus;
            if status == PlanStatus::Active && meta.activated_at.is_none() {
                warnings.push(ValidationIssue::warning_with_code(
                    "meta.activated_at",
                    "Plan status is 'active' but activated_at timestamp is missing",
                    "PWF-P003",
                ));
            }
            if status == PlanStatus::Completed && meta.completed_at.is_none() {
                warnings.push(ValidationIssue::warning_with_code(
                    "meta.completed_at",
                    "Plan status is 'completed' but completed_at timestamp is missing",
                    "PWF-P004",
                ));
            }
        }

        // Validate temporal order (activated_at must be before completed_at)
        if let (Some(ref activated), Some(ref completed)) = (&meta.activated_at, &meta.completed_at)
        {
            if is_valid_iso8601_datetime(activated)
                && is_valid_iso8601_datetime(completed)
                && !is_timestamp_before(activated, completed)
            {
                errors.push(ValidationIssue::error_with_code(
                    "meta",
                    "activated_at must be before completed_at",
                    "PWF-P005",
                ));
            }
        }
    } else {
        warnings.push(ValidationIssue::warning(
            "meta",
            "Missing meta section - plan will have no title",
        ));
    }

    // Validate glossary
    if plan.glossary.len() > 100 {
        errors.push(ValidationIssue::error_with_code(
            "glossary",
            format!(
                "Glossary has {} entries but maximum is 100",
                plan.glossary.len()
            ),
            "PWF-P006",
        ));
    }

    for (term, definition) in &plan.glossary {
        // Validate term format
        if term.is_empty() || term.len() > 50 {
            errors.push(ValidationIssue::error_with_code(
                format!("glossary.{}", term),
                format!("Term '{}' must be 1-50 characters", term),
                "PWF-P007",
            ));
        }

        // Validate term characters (alphanumeric, space, hyphen, apostrophe)
        if !term
            .chars()
            .all(|c| c.is_alphanumeric() || c == ' ' || c == '-' || c == '\'')
        {
            errors.push(ValidationIssue::error_with_code(
                format!("glossary.{}", term),
                format!(
                    "Term '{}' contains invalid characters (use alphanumeric, space, -, or ')",
                    term
                ),
                "PWF-P008",
            ));
        }

        // Validate definition length
        if definition.is_empty() {
            errors.push(ValidationIssue::error_with_code(
                format!("glossary.{}", term),
                format!("Definition for '{}' cannot be empty", term),
                "PWF-P009",
            ));
        }

        if definition.len() > 500 {
            errors.push(ValidationIssue::error_with_code(
                format!("glossary.{}", term),
                format!(
                    "Definition for '{}' exceeds 500 characters ({} chars)",
                    term,
                    definition.len()
                ),
                "PWF-P010",
            ));
        }
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

fn is_valid_iso8601_datetime(s: &str) -> bool {
    // Basic ISO 8601 format check: YYYY-MM-DDTHH:MM:SSZ or YYYY-MM-DDTHH:MM:SS+HH:MM
    // Minimum length check
    if s.len() < 20 {
        return false;
    }

    // Check basic structure
    let parts: Vec<&str> = s.split('T').collect();
    if parts.len() != 2 {
        return false;
    }

    // Check date part (YYYY-MM-DD)
    let date_parts: Vec<&str> = parts[0].split('-').collect();
    if date_parts.len() != 3 {
        return false;
    }
    if date_parts[0].len() != 4 || date_parts[1].len() != 2 || date_parts[2].len() != 2 {
        return false;
    }

    // Check time part has colons
    if !parts[1].contains(':') {
        return false;
    }

    // Check for timezone indicator (Z or +/-HH:MM)
    let has_z = parts[1].ends_with('Z');
    let has_offset = parts[1].contains('+') || parts[1].matches('-').count() > 0;

    has_z || has_offset
}

fn is_timestamp_before(earlier: &str, later: &str) -> bool {
    // Simple string comparison works for ISO 8601 timestamps
    earlier < later
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

    #[test]
    fn validate_activated_at_valid() {
        let yaml = r#"
plan_version: 1
meta:
  title: "Test"
  status: active
  activated_at: "2025-01-15T10:30:00Z"
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        // Should not have PWF-P003 warning since activated_at is present
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some("PWF-P003".to_string())));
    }

    #[test]
    fn validate_activated_at_invalid_format() {
        let yaml = r#"
plan_version: 1
meta:
  title: "Test"
  activated_at: "2025-01-15 10:30:00"
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
            .any(|e| e.code == Some("PWF-P001".to_string())));
    }

    #[test]
    fn validate_completed_at_invalid_format() {
        let yaml = r#"
plan_version: 1
meta:
  title: "Test"
  completed_at: "2025-01-15"
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
            .any(|e| e.code == Some("PWF-P002".to_string())));
    }

    #[test]
    fn validate_status_active_without_activated_at() {
        let yaml = r#"
plan_version: 1
meta:
  title: "Test"
  status: active
cycle:
  days:
    - exercises:
        - modality: strength
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some("PWF-P003".to_string())));
    }

    #[test]
    fn validate_status_completed_without_completed_at() {
        let yaml = r#"
plan_version: 1
meta:
  title: "Test"
  status: completed
cycle:
  days:
    - exercises:
        - modality: strength
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some("PWF-P004".to_string())));
    }

    #[test]
    fn validate_temporal_order_invalid() {
        let yaml = r#"
plan_version: 1
meta:
  title: "Test"
  activated_at: "2025-03-31T20:00:00Z"
  completed_at: "2025-01-01T08:00:00Z"
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
            .any(|e| e.code == Some("PWF-P005".to_string())));
    }

    #[test]
    fn validate_glossary_valid() {
        let yaml = r#"
plan_version: 1
glossary:
  RPE: "Rate of Perceived Exertion, scale 1-10"
  RIR: "Reps in Reserve"
  "1RM": "One Rep Max"
cycle:
  days:
    - exercises:
        - modality: strength
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
    }

    #[test]
    fn validate_glossary_too_many_entries() {
        let mut glossary_lines = Vec::new();
        for i in 0..101 {
            glossary_lines.push(format!("  term{}: \"Definition {}\"", i, i));
        }
        let yaml = format!(
            "plan_version: 1\nglossary:\n{}\ncycle:\n  days:\n    - exercises:\n        - modality: strength",
            glossary_lines.join("\n")
        );
        let result = validate(&yaml);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| e.code == Some("PWF-P006".to_string())));
    }

    #[test]
    fn validate_glossary_invalid_term() {
        let yaml = r#"
plan_version: 1
glossary:
  "RPE@Max": "Invalid term with @ character"
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
            .any(|e| e.code == Some("PWF-P008".to_string())));
    }

    #[test]
    fn validate_glossary_empty_definition() {
        let yaml = r#"
plan_version: 1
glossary:
  RPE: ""
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
            .any(|e| e.code == Some("PWF-P009".to_string())));
    }

    #[test]
    fn validate_glossary_definition_too_long() {
        let long_def = "a".repeat(501);
        let yaml = format!(
            "plan_version: 1\nglossary:\n  RPE: \"{}\"\ncycle:\n  days:\n    - exercises:\n        - modality: strength",
            long_def
        );
        let result = validate(&yaml);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| e.code == Some("PWF-P010".to_string())));
    }
}
