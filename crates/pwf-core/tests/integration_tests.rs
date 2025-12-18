//! Integration tests for PWF example files
//!
//! This test suite validates all example files in the examples/ directory,
//! ensuring they pass validation and testing round-trip serialization.

use pwf_core::{history, plan};
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

// ============================================================================
// Valid Plan Tests
// ============================================================================

#[test]
fn test_beginner_strength_valid() {
    let yaml = read_example("beginner-strength.yaml");
    let result = plan::validate(&yaml);

    assert!(
        result.is_valid(),
        "beginner-strength.yaml should be valid. Errors: {:?}",
        result.errors
    );
    assert!(result.plan.is_some(), "Plan should be parsed successfully");

    // Verify basic structure
    let plan_data = result.plan.unwrap();
    assert_eq!(plan_data.plan_version, 1);
    assert!(plan_data.meta.is_some());
    assert_eq!(plan_data.cycle.days.len(), 3);
    assert!(!plan_data.glossary.is_empty());

    // Verify glossary entries
    assert!(plan_data.glossary.contains_key("AMRAP"));
    assert!(plan_data.glossary.contains_key("1RM"));
}

#[test]
fn test_complete_v11_features_valid() {
    let yaml = read_example("complete-v1.1-features.yaml");
    let result = plan::validate(&yaml);

    assert!(
        result.is_valid(),
        "complete-v1.1-features.yaml should be valid. Errors: {:?}",
        result.errors
    );
    assert!(result.plan.is_some());

    let plan_data = result.plan.unwrap();
    let meta = plan_data.meta.as_ref().unwrap();

    // Verify v1.1 features
    assert!(meta.activated_at.is_some());
    assert!(meta.completed_at.is_some());
    assert!(!plan_data.glossary.is_empty());
}

#[test]
fn test_minimal_plan_valid() {
    let yaml = read_example("minimal.yaml");
    let result = plan::validate(&yaml);

    assert!(
        result.is_valid(),
        "minimal.yaml should be valid. Errors: {:?}",
        result.errors
    );
    assert!(result.plan.is_some());
}

#[test]
fn test_mixed_modalities_valid() {
    let yaml = read_example("mixed-modalities.yaml");
    let result = plan::validate(&yaml);

    assert!(
        result.is_valid(),
        "mixed-modalities.yaml should be valid. Errors: {:?}",
        result.errors
    );
    assert!(result.plan.is_some());
}

// ============================================================================
// Valid History Tests
// ============================================================================

#[test]
fn test_history_export_valid() {
    let yaml = read_example("history-export.yaml");
    let result = history::validate(&yaml);

    assert!(
        result.is_valid(),
        "history-export.yaml should be valid. Errors: {:?}",
        result.errors
    );
    assert!(result.history.is_some());

    let history_data = result.history.unwrap();
    assert_eq!(history_data.history_version, 1);
    assert_eq!(history_data.workouts.len(), 3);
    assert_eq!(history_data.personal_records.len(), 2);
    assert_eq!(history_data.body_measurements.len(), 2);
}

#[test]
fn test_history_minimal_valid() {
    let yaml = read_example("history-minimal.yaml");
    let result = history::validate(&yaml);

    assert!(
        result.is_valid(),
        "history-minimal.yaml should be valid. Errors: {:?}",
        result.errors
    );
    assert!(result.history.is_some());
}

// ============================================================================
// Invalid Plan Tests - Expected Error Codes
// ============================================================================

#[test]
fn test_invalid_timestamp_format() {
    let yaml = read_example("invalid/invalid-timestamp-format.yaml");
    let result = plan::validate(&yaml);

    assert!(
        !result.is_valid(),
        "invalid-timestamp-format.yaml should be invalid"
    );
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code.as_deref() == Some("PWF-P001")),
        "Should have PWF-P001 error. Errors: {:?}",
        result.errors
    );
}

#[test]
fn test_invalid_timestamp_order() {
    let yaml = read_example("invalid/invalid-timestamp-order.yaml");
    let result = plan::validate(&yaml);

    assert!(
        !result.is_valid(),
        "invalid-timestamp-order.yaml should be invalid"
    );
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code.as_deref() == Some("PWF-P005")),
        "Should have PWF-P005 error. Errors: {:?}",
        result.errors
    );
}

#[test]
fn test_invalid_glossary_term() {
    let yaml = read_example("invalid/invalid-glossary-term.yaml");
    let result = plan::validate(&yaml);

    assert!(
        !result.is_valid(),
        "invalid-glossary-term.yaml should be invalid"
    );
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code.as_deref() == Some("PWF-P008")),
        "Should have PWF-P008 error. Errors: {:?}",
        result.errors
    );
}

#[test]
fn test_invalid_glossary_empty() {
    let yaml = read_example("invalid/invalid-glossary-empty.yaml");
    let result = plan::validate(&yaml);

    assert!(
        !result.is_valid(),
        "invalid-glossary-empty.yaml should be invalid"
    );
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code.as_deref() == Some("PWF-P009")),
        "Should have PWF-P009 error. Errors: {:?}",
        result.errors
    );
}

#[test]
fn test_invalid_missing_version() {
    let yaml = read_example("invalid/missing-version.yaml");
    let result = plan::validate(&yaml);

    assert!(!result.is_valid(), "missing-version.yaml should be invalid");
}

#[test]
fn test_invalid_empty_days() {
    let yaml = read_example("invalid/empty-days.yaml");
    let result = plan::validate(&yaml);

    assert!(!result.is_valid(), "empty-days.yaml should be invalid");
}

#[test]
fn test_invalid_modality() {
    let yaml = read_example("invalid/invalid-modality.yaml");
    let result = plan::validate(&yaml);

    assert!(
        !result.is_valid(),
        "invalid-modality.yaml should be invalid"
    );
}

// ============================================================================
// Invalid History Tests - Expected Error Codes
// ============================================================================

#[test]
fn test_invalid_history_rir_high() {
    let yaml = read_example("invalid/invalid-history-rir-high.yaml");
    let result = history::validate(&yaml);

    // This should be valid but have a warning
    assert!(
        result.is_valid(),
        "invalid-history-rir-high.yaml should be valid with warnings"
    );
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.code.as_deref() == Some("PWF-H303")),
        "Should have PWF-H303 warning. Warnings: {:?}",
        result.warnings
    );
}

#[test]
fn test_invalid_history_rpe_rir_both() {
    let yaml = read_example("invalid/invalid-history-rpe-rir-both.yaml");
    let result = history::validate(&yaml);

    assert!(
        result.is_valid(),
        "invalid-history-rpe-rir-both.yaml should be valid with warnings"
    );
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.code.as_deref() == Some("PWF-H304")),
        "Should have PWF-H304 warning. Warnings: {:?}",
        result.warnings
    );
}

#[test]
fn test_invalid_history_pr_no_unit() {
    let yaml = read_example("invalid/invalid-history-pr-no-unit.yaml");
    let result = history::validate(&yaml);

    assert!(
        result.is_valid(),
        "invalid-history-pr-no-unit.yaml should be valid with warnings"
    );
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.code.as_deref() == Some("PWF-H403")),
        "Should have PWF-H403 warning. Warnings: {:?}",
        result.warnings
    );
}

#[test]
fn test_invalid_history_preferred_units_mismatch() {
    let yaml = read_example("invalid/invalid-history-preferred-units-mismatch.yaml");
    let result = history::validate(&yaml);

    assert!(
        result.is_valid(),
        "invalid-history-preferred-units-mismatch.yaml should be valid with warnings"
    );
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.code.as_deref() == Some("PWF-H601")),
        "Should have PWF-H601 warning. Warnings: {:?}",
        result.warnings
    );
}

#[test]
fn test_invalid_history_rpe_out_of_range() {
    let yaml = read_example("invalid/invalid-history-rpe-out-of-range.yaml");
    let result = history::validate(&yaml);

    assert!(
        result.is_valid(),
        "invalid-history-rpe-out-of-range.yaml should be valid with warnings"
    );
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.code.as_deref() == Some("PWF-H302")),
        "Should have PWF-H302 warning. Warnings: {:?}",
        result.warnings
    );
}

// ============================================================================
// Round-Trip Serialization Tests
// ============================================================================

#[test]
fn test_plan_round_trip_beginner_strength() {
    let yaml = read_example("beginner-strength.yaml");

    // Parse original
    let result1 = plan::validate(&yaml);
    assert!(result1.is_valid());
    let plan1 = result1.plan.unwrap();

    // Serialize to YAML
    let yaml2 = serde_yaml::to_string(&plan1).unwrap();

    // Parse again
    let result2 = plan::validate(&yaml2);
    assert!(result2.is_valid());
    let plan2 = result2.plan.unwrap();

    // Verify key fields match
    assert_eq!(plan1.plan_version, plan2.plan_version);
    assert_eq!(plan1.cycle.days.len(), plan2.cycle.days.len());
    assert_eq!(plan1.glossary.len(), plan2.glossary.len());

    if let (Some(meta1), Some(meta2)) = (&plan1.meta, &plan2.meta) {
        assert_eq!(meta1.title, meta2.title);
        assert_eq!(meta1.status, meta2.status);
    }
}

#[test]
fn test_plan_round_trip_complete_v11() {
    let yaml = read_example("complete-v1.1-features.yaml");

    let result1 = plan::validate(&yaml);
    assert!(result1.is_valid());
    let plan1 = result1.plan.unwrap();

    let yaml2 = serde_yaml::to_string(&plan1).unwrap();
    let result2 = plan::validate(&yaml2);
    assert!(result2.is_valid());
    let plan2 = result2.plan.unwrap();

    // Verify v1.1 features preserved
    if let (Some(meta1), Some(meta2)) = (&plan1.meta, &plan2.meta) {
        assert_eq!(meta1.activated_at, meta2.activated_at);
        assert_eq!(meta1.completed_at, meta2.completed_at);
    }

    assert_eq!(plan1.glossary, plan2.glossary);
}

#[test]
fn test_history_round_trip() {
    let yaml = read_example("history-export.yaml");

    let result1 = history::validate(&yaml);
    assert!(result1.is_valid());
    let history1 = result1.history.unwrap();

    let yaml2 = serde_yaml::to_string(&history1).unwrap();
    let result2 = history::validate(&yaml2);
    assert!(result2.is_valid());
    let history2 = result2.history.unwrap();

    // Verify key fields match
    assert_eq!(history1.history_version, history2.history_version);
    assert_eq!(history1.workouts.len(), history2.workouts.len());
    assert_eq!(
        history1.personal_records.len(),
        history2.personal_records.len()
    );
    assert_eq!(
        history1.body_measurements.len(),
        history2.body_measurements.len()
    );
}

// ============================================================================
// File Reading Error Tests
// ============================================================================

#[test]
fn test_nonexistent_file() {
    let path = examples_dir().join("nonexistent-file.yaml");
    let result = fs::read_to_string(&path);
    assert!(result.is_err(), "Reading nonexistent file should fail");
}

#[test]
fn test_invalid_yaml_syntax() {
    let invalid_yaml = r#"
plan_version: 1
cycle:
  days:
    - exercises: [this is not properly closed
"#;

    let result = plan::validate(invalid_yaml);
    assert!(!result.is_valid(), "Invalid YAML syntax should fail");
    assert!(!result.errors.is_empty());
}

#[test]
fn test_malformed_utf8() {
    // Test with valid UTF-8 that contains special characters
    let yaml_with_special_chars = r#"
plan_version: 1
meta:
  title: "Plan with émojis 🏋️"
  description: "Testing spëcial chàracters"
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
"#;

    let result = plan::validate(yaml_with_special_chars);
    assert!(
        result.is_valid(),
        "Valid UTF-8 with special chars should parse"
    );
}

#[test]
fn test_large_file_handling() {
    // Create a synthetic large plan with many days and exercises
    let mut yaml = String::from("plan_version: 1\ncycle:\n  days:\n");

    // Generate 100 days with 10 exercises each
    for day_num in 0..100 {
        yaml.push_str(&format!("    - id: \"day-{}\"\n", day_num));
        yaml.push_str("      exercises:\n");

        for ex_num in 0..10 {
            yaml.push_str(&format!("        - name: \"Exercise {}\"\n", ex_num));
            yaml.push_str("          modality: strength\n");
            yaml.push_str("          target_sets: 3\n");
            yaml.push_str("          target_reps: 10\n");
        }
    }

    // Should handle large files without issues
    let result = plan::validate(&yaml);
    assert!(
        result.is_valid(),
        "Large generated plan should be valid. Errors: {:?}",
        result.errors
    );
    assert_eq!(result.plan.as_ref().unwrap().cycle.days.len(), 100);
}

#[test]
fn test_very_large_file_handling() {
    // Create a synthetic very large history with many workouts
    let mut yaml = String::from(
        r#"history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
"#,
    );

    // Generate 1000 workouts
    for i in 0..1000 {
        yaml.push_str(&format!(
            r#"  - date: "2025-01-15"
    exercises:
      - name: "Exercise {}"
        sets:
          - reps: 10
            weight_kg: 100
"#,
            i
        ));
    }

    let result = history::validate(&yaml);
    assert!(
        result.is_valid(),
        "Large history file should be valid. Errors: {:?}",
        result.errors
    );
    assert_eq!(result.history.as_ref().unwrap().workouts.len(), 1000);

    // Verify the serialized size is reasonable (> 10KB)
    let serialized = serde_yaml::to_string(&result.history.unwrap()).unwrap();
    assert!(
        serialized.len() > 10_000,
        "Large file should serialize to > 10KB"
    );
}

// ============================================================================
// Cross-Validation Tests
// ============================================================================

#[test]
fn test_history_statistics_calculation() {
    let yaml = read_example("history-export.yaml");
    let result = history::validate(&yaml);

    assert!(result.is_valid());
    assert!(result.statistics.is_some());

    let stats = result.statistics.unwrap();

    // Verify statistics match the example file
    assert_eq!(stats.total_workouts, 3);
    assert_eq!(stats.total_exercises, 8); // Sum of exercises across all workouts

    // Verify date ranges
    assert!(stats.date_range_start.is_some());
    assert!(stats.date_range_end.is_some());
    assert_eq!(stats.date_range_start.as_deref(), Some("2025-01-15"));
    assert_eq!(stats.date_range_end.as_deref(), Some("2025-01-19"));

    // Verify personal records count
    assert_eq!(stats.personal_records_count, 2);
    assert_eq!(stats.body_measurements_count, 2);
}

#[test]
fn test_plan_statistics_calculation() {
    let yaml = read_example("beginner-strength.yaml");
    let result = plan::validate(&yaml);

    assert!(result.is_valid());
    assert!(result.statistics.is_some());

    let stats = result.statistics.unwrap();

    // Verify day count matches
    assert_eq!(stats.total_days, 3);

    // Verify total exercises across all days
    assert_eq!(stats.total_exercises, 12); // 4 + 4 + 4 exercises per day

    // Verify modality distribution
    assert!(stats.strength_count > 0);
    assert!(stats.countdown_count > 0);
}

#[test]
fn test_personal_records_accuracy() {
    let yaml = read_example("history-export.yaml");
    let result = history::validate(&yaml);

    assert!(result.is_valid());
    let history_data = result.history.unwrap();

    // Verify PR details match expected values from file
    assert_eq!(history_data.personal_records.len(), 2);

    let bench_pr = history_data
        .personal_records
        .iter()
        .find(|pr| pr.exercise_name == "Bench Press")
        .expect("Should have bench press PR");

    assert_eq!(bench_pr.value, 100.0);
    assert_eq!(bench_pr.achieved_at, "2025-01-15");
}

#[test]
fn test_workout_date_chronology() {
    let yaml = read_example("history-export.yaml");
    let result = history::validate(&yaml);

    assert!(result.is_valid());
    let history_data = result.history.unwrap();

    // Verify workouts are in chronological order (or at least valid dates)
    let dates: Vec<&str> = history_data
        .workouts
        .iter()
        .map(|w| w.date.as_str())
        .collect();

    assert_eq!(dates, vec!["2025-01-15", "2025-01-17", "2025-01-19"]);
}

#[test]
fn test_exercise_set_counts() {
    let yaml = read_example("history-export.yaml");
    let result = history::validate(&yaml);

    assert!(result.is_valid());
    let history_data = result.history.unwrap();

    // First workout, first exercise (Bench Press) should have 6 sets
    let bench_sets = &history_data.workouts[0].exercises[0].sets;
    assert_eq!(bench_sets.len(), 6);

    // Verify warmup vs working set types
    let warmup_count = bench_sets
        .iter()
        .filter(|s| matches!(s.set_type, Some(pwf_core::history::SetType::Warmup)))
        .count();
    assert_eq!(warmup_count, 3);
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_empty_plan_cycle() {
    let yaml = r#"
plan_version: 1
cycle:
  days: []
"#;

    let result = plan::validate(yaml);
    assert!(!result.is_valid(), "Empty days should be invalid");
}

#[test]
fn test_empty_history_workouts() {
    let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
"#;

    let result = history::validate(yaml);
    // Should be valid but might have warnings
    assert!(result.is_valid(), "Empty workouts list should be valid");
}

#[test]
fn test_plan_with_unicode_content() {
    let yaml = r#"
plan_version: 1
meta:
  title: "健身计划"
  description: "Тренировка с весом тела"
glossary:
  "スクワット": "A fundamental lower body exercise"
cycle:
  days:
    - exercises:
        - name: "Приседания"
          modality: strength
"#;

    let result = plan::validate(yaml);
    assert!(
        result.is_valid(),
        "Unicode content should be handled correctly"
    );
}

#[test]
fn test_history_with_fractional_rpe() {
    let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Squat"
        sets:
          - reps: 5
            weight_kg: 140
            rpe: 7.5
"#;

    let result = history::validate(yaml);
    assert!(result.is_valid(), "Fractional RPE should be valid");
}

#[test]
fn test_all_example_files_exist() {
    let examples_path = examples_dir();

    // Ensure the examples directory exists
    assert!(
        examples_path.exists(),
        "Examples directory should exist at {:?}",
        examples_path
    );

    // List of files that should exist
    let required_files = vec![
        "beginner-strength.yaml",
        "complete-v1.1-features.yaml",
        "history-export.yaml",
        "history-minimal.yaml",
        "minimal.yaml",
        "mixed-modalities.yaml",
    ];

    for file in required_files {
        let file_path = examples_path.join(file);
        assert!(
            file_path.exists(),
            "Example file should exist: {:?}",
            file_path
        );
    }
}
