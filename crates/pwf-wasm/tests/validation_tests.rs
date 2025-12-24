//! Comprehensive validation tests for WASM bindings

#![cfg(target_arch = "wasm32")]

use pwf_wasm::*;
use serde_json::Value;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// ============================================================================
// PLAN VALIDATION TESTS
// ============================================================================

#[wasm_bindgen_test]
fn test_validate_plan_minimal_valid() {
    let yaml = r#"
plan_version: 1
cycle:
  days:
    - exercises:
        - name: Squat
          modality: strength
          target_sets: 3
          target_reps: 5
"#;

    let result = validate_plan(yaml);
    assert!(result.is_object());

    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();
    assert_eq!(json["valid"], true);
    assert!(json["errors"].as_array().unwrap().is_empty());
}

#[wasm_bindgen_test]
fn test_validate_plan_complete_valid() {
    let yaml = r#"
plan_version: 1
meta:
  title: "Beginner Strength Program"
  description: "A comprehensive beginner strength training program"
  author: "Test Author"
  equipment:
    - barbell
    - bench
cycle:
  days:
    - exercises:
        - name: Squat
          modality: strength
          target_sets: 3
          target_reps: 5
          target_load: "100 kg"
        - name: Bench Press
          modality: strength
          target_sets: 3
          target_reps: 5
    - exercises:
        - name: Deadlift
          modality: strength
          target_sets: 1
          target_reps: 5
"#;

    let result = validate_plan(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert_eq!(json["valid"], true);
    assert!(json["plan"].is_object());
    assert_eq!(json["plan"]["meta"]["title"], "Beginner Strength Program");
}

#[wasm_bindgen_test]
fn test_validate_plan_with_warnings() {
    let yaml = r#"
plan_version: 1
meta:
  title: "Test Plan"
  unknown_field: "this will generate a warning"
cycle:
  days:
    - exercises:
        - name: Squat
          modality: strength
          target_sets: 3
          target_reps: 5
"#;

    let result = validate_plan(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert_eq!(json["valid"], true);
    // May have warnings for unknown fields
}

#[wasm_bindgen_test]
fn test_validate_plan_invalid_missing_required() {
    let yaml = r#"
plan_version: 1
cycle:
  days:
    - exercises:
        - name: Squat
          modality: strength
          # Missing target_sets and target_reps for strength modality
"#;

    let result = validate_plan(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert_eq!(json["valid"], false);
    assert!(!json["errors"].as_array().unwrap().is_empty());
}

#[wasm_bindgen_test]
fn test_validate_plan_invalid_yaml_syntax() {
    let yaml = r#"
plan_version: 1
cycle:
  days: [
    - exercises:
"#;

    let result = validate_plan(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert_eq!(json["valid"], false);
    assert!(!json["errors"].as_array().unwrap().is_empty());
}

#[wasm_bindgen_test]
fn test_validate_plan_invalid_modality() {
    let yaml = r#"
plan_version: 1
cycle:
  days:
    - exercises:
        - name: Squat
          modality: invalid_modality
          target_sets: 3
          target_reps: 5
"#;

    let result = validate_plan(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert_eq!(json["valid"], false);
}

#[wasm_bindgen_test]
fn test_validate_plan_multiple_exercises() {
    let yaml = r#"
plan_version: 1
cycle:
  days:
    - exercises:
        - name: Squat
          modality: strength
          target_sets: 3
          target_reps: 5
        - name: Bench Press
          modality: strength
          target_sets: 3
          target_reps: 5
        - name: Rowing
          modality: countdown
          target_duration_sec: 600
        - name: Plank
          modality: stopwatch
"#;

    let result = validate_plan(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert_eq!(json["valid"], true);
}

#[wasm_bindgen_test]
fn test_validate_plan_with_library() {
    let yaml = r#"
plan_version: 1
exercise_library:
  - id: squat
    name: Barbell Back Squat
    modality: strength
    equipment:
      - barbell
cycle:
  days:
    - exercises:
        - library_exercise_id: squat
          target_sets: 3
          target_reps: 5
"#;

    let result = validate_plan(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert_eq!(json["valid"], true);
}

#[wasm_bindgen_test]
fn test_validate_plan_empty_yaml() {
    let yaml = "";

    let result = validate_plan(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert_eq!(json["valid"], false);
}

#[wasm_bindgen_test]
fn test_validate_plan_interval_modality() {
    let yaml = r#"
plan_version: 1
cycle:
  days:
    - exercises:
        - name: HIIT Sprints
          modality: interval
          target_sets: 8
          target_duration_sec: 30
"#;

    let result = validate_plan(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert_eq!(json["valid"], true);
}

// ============================================================================
// HISTORY VALIDATION TESTS
// ============================================================================

#[wasm_bindgen_test]
fn test_validate_history_minimal_valid() {
    let yaml = r#"
history_version: 1
exported_at: "2025-12-24T12:00:00Z"
workouts:
  - date: "2025-12-24"
    exercises:
      - name: Squat
        sets:
          - reps: 5
            weight_kg: 100
"#;

    let result = validate_history(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert_eq!(json["valid"], true);
    assert!(json["errors"].as_array().unwrap().is_empty());
}

#[wasm_bindgen_test]
fn test_validate_history_complete_valid() {
    let yaml = r#"
history_version: 1
exported_at: "2025-12-24T12:00:00Z"
units:
  weight: kg
  distance: meters
workouts:
  - date: "2025-12-24"
    started_at: "2025-12-24T08:00:00Z"
    ended_at: "2025-12-24T09:00:00Z"
    duration_sec: 3600
    title: "Morning Strength Session"
    sport: StrengthTraining
    exercises:
      - name: Squat
        modality: strength
        sets:
          - set_number: 1
            reps: 5
            weight_kg: 100
            completed_at: "2025-12-24T08:05:00Z"
          - set_number: 2
            reps: 5
            weight_kg: 100
          - set_number: 3
            reps: 5
            weight_kg: 100
"#;

    let result = validate_history(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert_eq!(json["valid"], true);
    assert!(json["history"].is_object());
}

#[wasm_bindgen_test]
fn test_validate_history_with_telemetry() {
    let yaml = r#"
history_version: 1
exported_at: "2025-12-24T12:00:00Z"
workouts:
  - date: "2025-12-24"
    exercises:
      - name: Running
        sets:
          - duration_sec: 3600
            distance_meters: 10000
    telemetry:
      calories: 500
      avg_heart_rate: 150
      max_heart_rate: 170
"#;

    let result = validate_history(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert_eq!(json["valid"], true);
}

#[wasm_bindgen_test]
fn test_validate_history_with_gps_route() {
    let yaml = r#"
history_version: 1
exported_at: "2025-12-24T12:00:00Z"
workouts:
  - date: "2025-12-24"
    exercises:
      - name: Running
        sets:
          - duration_sec: 3600
    telemetry:
      gps_route:
        positions:
          - latitude: 37.7749
            longitude: -122.4194
            elevation_meters: 10.0
            timestamp: "2025-12-24T08:00:00Z"
          - latitude: 37.7750
            longitude: -122.4195
            elevation_meters: 11.0
            timestamp: "2025-12-24T08:01:00Z"
"#;

    let result = validate_history(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert_eq!(json["valid"], true);
}

#[wasm_bindgen_test]
fn test_validate_history_invalid_missing_date() {
    let yaml = r#"
history_version: 1
exported_at: "2025-12-24T12:00:00Z"
workouts:
  - exercises:
      - name: Squat
        sets:
          - reps: 5
"#;

    let result = validate_history(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert_eq!(json["valid"], false);
}

#[wasm_bindgen_test]
fn test_validate_history_empty_workouts() {
    let yaml = r#"
history_version: 1
exported_at: "2025-12-24T12:00:00Z"
workouts: []
"#;

    let result = validate_history(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    // Empty workouts list should be valid
    assert_eq!(json["valid"], true);
}

#[wasm_bindgen_test]
fn test_validate_history_multiple_workouts() {
    let yaml = r#"
history_version: 1
exported_at: "2025-12-24T12:00:00Z"
workouts:
  - date: "2025-12-24"
    exercises:
      - name: Squat
        sets:
          - reps: 5
            weight_kg: 100
  - date: "2025-12-23"
    exercises:
      - name: Bench Press
        sets:
          - reps: 5
            weight_kg: 80
  - date: "2025-12-22"
    exercises:
      - name: Deadlift
        sets:
          - reps: 5
            weight_kg: 120
"#;

    let result = validate_history(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert_eq!(json["valid"], true);
    assert_eq!(json["history"]["workouts"].as_array().unwrap().len(), 3);
}

#[wasm_bindgen_test]
fn test_validate_history_with_swimming_data() {
    let yaml = r#"
history_version: 1
exported_at: "2025-12-24T12:00:00Z"
workouts:
  - date: "2025-12-24"
    sport: Swimming
    exercises:
      - name: Swimming
        sets:
          - duration_sec: 1800
            distance_meters: 1000
            swimming:
              pool_length_meters: 25
              lengths: 40
              stroke_type: freestyle
"#;

    let result = validate_history(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert_eq!(json["valid"], true);
}

#[wasm_bindgen_test]
fn test_validate_history_invalid_yaml_syntax() {
    let yaml = r#"
history_version: 1
exported_at: "2025-12-24T12:00:00Z"
workouts: [
  - date: "2025-12-24
"#;

    let result = validate_history(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert_eq!(json["valid"], false);
}
