use pwf_core::plan;

#[test]
fn test_linear_progression_valid() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Linear Progression Test"
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          target_sets: 3
          target_reps: 5
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
            deload_condition: failed_twice_consecutive
            deload_percent: 90
"#;
    let result = plan::validate(yaml);
    assert!(result.is_valid(), "Errors: {:?}", result.errors);
}

#[test]
fn test_double_progression_valid() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Double Progression Test"
cycle:
  days:
    - exercises:
        - name: "Bench Press"
          modality: strength
          target_sets: 3
          target_reps: 8
          progression_rules:
            type: double_progression
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
            reps_range_min: 8
            reps_range_max: 12
            deload_condition: failed_twice_consecutive
            deload_percent: 85
"#;
    let result = plan::validate(yaml);
    assert!(result.is_valid(), "Errors: {:?}", result.errors);
}

#[test]
fn test_progression_rules_v1_warning() {
    let yaml = r#"
plan_version: 1
meta:
  title: "V1 Test"
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
"#;
    let result = plan::validate(yaml);
    assert!(result.is_valid());
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.code == Some("PWF-P040".to_string())),
        "Should warn about progression_rules in v1"
    );
}

#[test]
fn test_progression_rules_non_strength_warning() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Non-strength Test"
cycle:
  days:
    - exercises:
        - name: "Plank"
          modality: countdown
          target_duration_sec: 60
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
"#;
    let result = plan::validate(yaml);
    assert!(result.is_valid());
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.code == Some("PWF-P041".to_string())),
        "Should warn about progression_rules for non-strength exercise"
    );
}

#[test]
fn test_linear_progression_missing_weight_increment() {
    let yaml = r#"
plan_version: 2
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
"#;
    let result = plan::validate(yaml);
    assert!(!result.is_valid());
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code == Some("PWF-P042".to_string())),
        "Should error when linear progression lacks weight increment"
    );
}

#[test]
fn test_progression_both_kg_and_lbs_error() {
    let yaml = r#"
plan_version: 2
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
            weight_increment_lbs: 5
"#;
    let result = plan::validate(yaml);
    assert!(!result.is_valid());
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code == Some("PWF-P043".to_string())),
        "Should error when both kg and lbs specified"
    );
}

#[test]
fn test_linear_progression_with_reps_range_warning() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Test"
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
            reps_range_min: 5
            reps_range_max: 8
"#;
    let result = plan::validate(yaml);
    assert!(result.is_valid());
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.code == Some("PWF-P044".to_string())),
        "Should warn about reps_range in linear progression"
    );
}

#[test]
fn test_double_progression_missing_reps_range() {
    let yaml = r#"
plan_version: 2
cycle:
  days:
    - exercises:
        - name: "Bench"
          modality: strength
          progression_rules:
            type: double_progression
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
"#;
    let result = plan::validate(yaml);
    assert!(!result.is_valid());
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code == Some("PWF-P045".to_string())),
        "Should error when double progression lacks reps_range"
    );
}

#[test]
fn test_double_progression_missing_weight_increment() {
    let yaml = r#"
plan_version: 2
cycle:
  days:
    - exercises:
        - name: "Bench"
          modality: strength
          progression_rules:
            type: double_progression
            success_condition: all_sets_completed
            reps_range_min: 8
            reps_range_max: 12
"#;
    let result = plan::validate(yaml);
    assert!(!result.is_valid());
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code == Some("PWF-P046".to_string())),
        "Should error when double progression lacks weight increment"
    );
}

#[test]
fn test_reps_range_invalid() {
    let yaml = r#"
plan_version: 2
cycle:
  days:
    - exercises:
        - name: "Bench"
          modality: strength
          progression_rules:
            type: double_progression
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
            reps_range_min: 12
            reps_range_max: 8
"#;
    let result = plan::validate(yaml);
    assert!(!result.is_valid());
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code == Some("PWF-P047".to_string())),
        "Should error when reps_range_min >= reps_range_max"
    );
}

#[test]
fn test_reps_range_min_zero() {
    let yaml = r#"
plan_version: 2
cycle:
  days:
    - exercises:
        - name: "Bench"
          modality: strength
          progression_rules:
            type: double_progression
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
            reps_range_min: 0
            reps_range_max: 12
"#;
    let result = plan::validate(yaml);
    assert!(!result.is_valid());
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code == Some("PWF-P048".to_string())),
        "Should error when reps_range_min is 0"
    );
}

#[test]
fn test_reps_range_max_too_high_warning() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Test"
cycle:
  days:
    - exercises:
        - name: "Bench"
          modality: strength
          progression_rules:
            type: double_progression
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
            reps_range_min: 50
            reps_range_max: 150
"#;
    let result = plan::validate(yaml);
    assert!(result.is_valid());
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.code == Some("PWF-P049".to_string())),
        "Should warn when reps_range_max is unusually high"
    );
}

#[test]
fn test_weight_increment_kg_negative() {
    let yaml = r#"
plan_version: 2
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: -2.5
"#;
    let result = plan::validate(yaml);
    assert!(!result.is_valid());
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code == Some("PWF-P050".to_string())),
        "Should error when weight_increment_kg is negative"
    );
}

#[test]
fn test_weight_increment_kg_too_large_warning() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Test"
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: 60
"#;
    let result = plan::validate(yaml);
    assert!(result.is_valid());
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.code == Some("PWF-P051".to_string())),
        "Should warn when weight_increment_kg is very large"
    );
}

#[test]
fn test_weight_increment_lbs_negative() {
    let yaml = r#"
plan_version: 2
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_lbs: -5
"#;
    let result = plan::validate(yaml);
    assert!(!result.is_valid());
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code == Some("PWF-P052".to_string())),
        "Should error when weight_increment_lbs is negative"
    );
}

#[test]
fn test_weight_increment_lbs_too_large_warning() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Test"
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_lbs: 120
"#;
    let result = plan::validate(yaml);
    assert!(result.is_valid());
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.code == Some("PWF-P053".to_string())),
        "Should warn when weight_increment_lbs is very large"
    );
}

#[test]
fn test_deload_percent_out_of_range() {
    let yaml = r#"
plan_version: 2
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
            deload_percent: 40
"#;
    let result = plan::validate(yaml);
    assert!(!result.is_valid());
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code == Some("PWF-P054".to_string())),
        "Should error when deload_percent is out of range"
    );
}

#[test]
fn test_deload_weeks_zero() {
    let yaml = r#"
plan_version: 2
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
            deload_weeks: 0
"#;
    let result = plan::validate(yaml);
    assert!(!result.is_valid());
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code == Some("PWF-P055".to_string())),
        "Should error when deload_weeks is 0"
    );
}

#[test]
fn test_deload_weeks_too_long_warning() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Test"
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
            deload_weeks: 10
"#;
    let result = plan::validate(yaml);
    assert!(result.is_valid());
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.code == Some("PWF-P056".to_string())),
        "Should warn when deload_weeks is unusually long"
    );
}

#[test]
fn test_max_weight_both_kg_and_lbs() {
    let yaml = r#"
plan_version: 2
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
            max_weight_kg: 200
            max_weight_lbs: 440
"#;
    let result = plan::validate(yaml);
    assert!(!result.is_valid());
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code == Some("PWF-P057".to_string())),
        "Should error when both max_weight_kg and max_weight_lbs specified"
    );
}

#[test]
fn test_max_weight_kg_negative() {
    let yaml = r#"
plan_version: 2
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
            max_weight_kg: -100
"#;
    let result = plan::validate(yaml);
    assert!(!result.is_valid());
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code == Some("PWF-P058".to_string())),
        "Should error when max_weight_kg is negative"
    );
}

#[test]
fn test_max_weight_lbs_negative() {
    let yaml = r#"
plan_version: 2
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_lbs: 5
            max_weight_lbs: -220
"#;
    let result = plan::validate(yaml);
    assert!(!result.is_valid());
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code == Some("PWF-P059".to_string())),
        "Should error when max_weight_lbs is negative"
    );
}

#[test]
fn test_reps_increment_zero() {
    let yaml = r#"
plan_version: 2
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
            reps_increment: 0
"#;
    let result = plan::validate(yaml);
    assert!(!result.is_valid());
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code == Some("PWF-P060".to_string())),
        "Should error when reps_increment is 0"
    );
}

#[test]
fn test_reps_increment_too_large_warning() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Test"
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
            reps_increment: 15
"#;
    let result = plan::validate(yaml);
    assert!(result.is_valid());
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.code == Some("PWF-P061".to_string())),
        "Should warn when reps_increment is very large"
    );
}

#[test]
fn test_deload_condition_without_percent_warning() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Test"
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
            deload_condition: failed_twice_consecutive
"#;
    let result = plan::validate(yaml);
    assert!(result.is_valid());
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.code == Some("PWF-P062".to_string())),
        "Should warn when deload_condition without deload_percent"
    );
}

#[test]
fn test_progression_rules_all_success_conditions() {
    let conditions = vec![
        "all_sets_completed",
        "last_set_completed",
        "average_reps_reached",
    ];

    for condition in conditions {
        let yaml = format!(
            r#"
plan_version: 2
meta:
  title: "Test"
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: {}
            weight_increment_kg: 2.5
"#,
            condition
        );
        let result = plan::validate(&yaml);
        assert!(
            result.is_valid(),
            "success_condition '{}' should be valid. Errors: {:?}",
            condition,
            result.errors
        );
    }
}

#[test]
fn test_progression_rules_all_deload_conditions() {
    let conditions = vec![
        "failed_once_consecutive",
        "failed_twice_consecutive",
        "failed_three_consecutive",
        "rir_above_target",
    ];

    for condition in conditions {
        let yaml = format!(
            r#"
plan_version: 2
meta:
  title: "Test"
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
            deload_condition: {}
            deload_percent: 90
"#,
            condition
        );
        let result = plan::validate(&yaml);
        assert!(
            result.is_valid(),
            "deload_condition '{}' should be valid. Errors: {:?}",
            condition,
            result.errors
        );
    }
}

#[test]
fn test_progression_rules_with_notes() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Test"
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
            notes: "This is a custom progression note"
"#;
    let result = plan::validate(yaml);
    assert!(result.is_valid(), "Errors: {:?}", result.errors);
}

#[test]
fn test_example_linear_progression_file() {
    let yaml = std::fs::read_to_string("/home/nitro/Projects/pwf/examples/progression-linear.yaml")
        .expect("Failed to read example file");
    let result = plan::validate(&yaml);
    assert!(
        result.is_valid(),
        "progression-linear.yaml should be valid. Errors: {:?}",
        result.errors
    );
}

#[test]
fn test_example_double_progression_file() {
    let yaml = std::fs::read_to_string("/home/nitro/Projects/pwf/examples/progression-double.yaml")
        .expect("Failed to read example file");
    let result = plan::validate(&yaml);
    assert!(
        result.is_valid(),
        "progression-double.yaml should be valid. Errors: {:?}",
        result.errors
    );
}

#[test]
fn test_progression_rules_deload_percent_boundaries() {
    // Test boundary values
    let test_cases = vec![
        (50.0, true),   // minimum valid
        (100.0, true),  // maximum valid
        (75.0, true),   // middle value
        (49.9, false),  // just below minimum
        (100.1, false), // just above maximum
    ];

    for (percent, should_be_valid) in test_cases {
        let yaml = format!(
            r#"
plan_version: 2
meta:
  title: "Test"
cycle:
  days:
    - exercises:
        - name: "Squat"
          modality: strength
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
            deload_percent: {}
"#,
            percent
        );
        let result = plan::validate(&yaml);
        assert_eq!(
            result.is_valid(),
            should_be_valid,
            "deload_percent {} should be {}. Errors: {:?}",
            percent,
            if should_be_valid { "valid" } else { "invalid" },
            result.errors
        );
    }
}

#[test]
fn test_progression_rules_comprehensive_double_progression() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Comprehensive Double Progression Test"
cycle:
  days:
    - exercises:
        - name: "Dumbbell Bench Press"
          modality: strength
          target_sets: 3
          target_reps: 8
          target_load: "30kg DBs"
          progression_rules:
            type: double_progression
            success_condition: all_sets_completed
            weight_increment_kg: 2.0
            reps_range_min: 8
            reps_range_max: 12
            reps_increment: 1
            deload_condition: failed_twice_consecutive
            deload_percent: 85
            deload_weeks: 1
            max_weight_kg: 50
            notes: "Progress reps to 12, then increase weight and return to 8 reps"
"#;
    let result = plan::validate(yaml);
    assert!(result.is_valid(), "Errors: {:?}", result.errors);
    assert!(
        result.warnings.is_empty(),
        "Should have no warnings. Warnings: {:?}",
        result.warnings
    );
}
