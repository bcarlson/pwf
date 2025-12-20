//! Integration tests for PWF v2.0 exercise library feature

use pwf_core::plan::{resolve_exercise, validate};

#[test]
fn test_library_with_valid_references() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Library Test"
exercise_library:
  - id: squat
    name: "Squat"
    modality: strength
    default_sets: 3
    default_reps: 8
cycle:
  days:
    - exercises:
        - exercise_ref: squat
"#;

    let result = validate(yaml);
    assert!(result.is_valid(), "Errors: {:?}", result.errors);
    assert!(result.warnings.is_empty());
}

#[test]
fn test_library_invalid_reference() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Library Test"
exercise_library:
  - id: squat
    name: "Squat"
    modality: strength
cycle:
  days:
    - exercises:
        - exercise_ref: nonexistent
"#;

    let result = validate(yaml);
    assert!(!result.is_valid());
    assert!(result
        .errors
        .iter()
        .any(|e| e.code == Some("PWF-P032".to_string())));
}

#[test]
fn test_library_duplicate_ids() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Library Test"
exercise_library:
  - id: squat
    name: "Squat 1"
    modality: strength
  - id: squat
    name: "Squat 2"
    modality: strength
cycle:
  days:
    - exercises:
        - exercise_ref: squat
"#;

    let result = validate(yaml);
    assert!(!result.is_valid());
    assert!(result
        .errors
        .iter()
        .any(|e| e.code == Some("PWF-P034".to_string())));
}

#[test]
fn test_library_invalid_id_format() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Library Test"
exercise_library:
  - id: "invalid@id"
    name: "Squat"
    modality: strength
cycle:
  days:
    - exercises:
        - name: "Push-up"
          modality: strength
"#;

    let result = validate(yaml);
    assert!(!result.is_valid());
    assert!(result
        .errors
        .iter()
        .any(|e| e.code == Some("PWF-P035".to_string())));
}

#[test]
fn test_library_name_too_long() {
    let long_name = "a".repeat(101);
    let yaml = format!(
        r#"
plan_version: 2
meta:
  title: "Library Test"
exercise_library:
  - id: squat
    name: "{}"
    modality: strength
cycle:
  days:
    - exercises:
        - name: "Push-up"
          modality: strength
"#,
        long_name
    );

    let result = validate(&yaml);
    assert!(!result.is_valid());
    assert!(result
        .errors
        .iter()
        .any(|e| e.code == Some("PWF-P036".to_string())));
}

#[test]
fn test_library_description_too_long() {
    let long_desc = "a".repeat(501);
    let yaml = format!(
        r#"
plan_version: 2
meta:
  title: "Library Test"
exercise_library:
  - id: squat
    name: "Squat"
    description: "{}"
    modality: strength
cycle:
  days:
    - exercises:
        - name: "Push-up"
          modality: strength
"#,
        long_desc
    );

    let result = validate(&yaml);
    assert!(!result.is_valid());
    assert!(result
        .errors
        .iter()
        .any(|e| e.code == Some("PWF-P037".to_string())));
}

#[test]
fn test_library_too_many_entries() {
    let mut library_entries = Vec::new();
    for i in 0..501 {
        library_entries.push(format!(
            "  - id: ex{}\n    name: \"Exercise {}\"\n    modality: strength",
            i, i
        ));
    }

    let yaml = format!(
        r#"
plan_version: 2
meta:
  title: "Library Test"
exercise_library:
{}
cycle:
  days:
    - exercises:
        - name: "Push-up"
          modality: strength
"#,
        library_entries.join("\n")
    );

    let result = validate(&yaml);
    assert!(!result.is_valid());
    assert!(result
        .errors
        .iter()
        .any(|e| e.code == Some("PWF-P033".to_string())));
}

#[test]
fn test_v1_with_exercise_library_warning() {
    let yaml = r#"
plan_version: 1
meta:
  title: "V1 with Library"
exercise_library:
  - id: squat
    name: "Squat"
    modality: strength
cycle:
  days:
    - exercises:
        - name: "Push-up"
          modality: strength
"#;

    let result = validate(yaml);
    assert!(result.is_valid());
    assert!(result.warnings.iter().any(|w| w.path == "exercise_library"));
}

#[test]
fn test_v1_with_exercise_ref_warning() {
    let yaml = r#"
plan_version: 1
meta:
  title: "V1 with Ref"
cycle:
  days:
    - exercises:
        - exercise_ref: squat
          modality: strength
"#;

    let result = validate(yaml);
    assert!(result.is_valid());
    assert!(result
        .warnings
        .iter()
        .any(|w| w.message.contains("exercise_ref")));
}

#[test]
fn test_v2_requires_modality_or_ref() {
    let yaml = r#"
plan_version: 2
meta:
  title: "V2 Missing Both"
cycle:
  days:
    - exercises:
        - name: "Push-up"
"#;

    let result = validate(yaml);
    assert!(!result.is_valid());
    assert!(result
        .errors
        .iter()
        .any(|e| e.code == Some("PWF-P030".to_string())));
}

#[test]
fn test_v2_both_modality_and_ref_warning() {
    let yaml = r#"
plan_version: 2
meta:
  title: "V2 Both Fields"
exercise_library:
  - id: squat
    name: "Squat"
    modality: strength
cycle:
  days:
    - exercises:
        - exercise_ref: squat
          modality: countdown
"#;

    let result = validate(yaml);
    assert!(result.is_valid());
    assert!(result
        .warnings
        .iter()
        .any(|w| w.code == Some("PWF-P031".to_string())));
}

#[test]
fn test_library_with_all_difficulty_levels() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Difficulty Levels"
exercise_library:
  - id: ex1
    name: "Beginner Exercise"
    modality: strength
    difficulty: beginner
  - id: ex2
    name: "Intermediate Exercise"
    modality: strength
    difficulty: intermediate
  - id: ex3
    name: "Advanced Exercise"
    modality: strength
    difficulty: advanced
cycle:
  days:
    - exercises:
        - exercise_ref: ex1
        - exercise_ref: ex2
        - exercise_ref: ex3
"#;

    let result = validate(yaml);
    assert!(result.is_valid());
}

#[test]
fn test_library_with_equipment_and_muscle_groups() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Equipment Test"
exercise_library:
  - id: squat
    name: "Barbell Squat"
    modality: strength
    equipment:
      - barbell
      - squat_rack
    muscle_groups:
      - quads
      - glutes
      - hamstrings
cycle:
  days:
    - exercises:
        - exercise_ref: squat
"#;

    let result = validate(yaml);
    assert!(result.is_valid());
}

#[test]
fn test_library_with_default_values() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Defaults Test"
exercise_library:
  - id: squat
    name: "Squat"
    modality: strength
    default_sets: 3
    default_reps: 8
  - id: plank
    name: "Plank"
    modality: countdown
    default_duration_sec: 30
  - id: run
    name: "Run"
    modality: running
    default_distance_meters: 5000
cycle:
  days:
    - exercises:
        - exercise_ref: squat
        - exercise_ref: plank
        - exercise_ref: run
"#;

    let result = validate(yaml);
    assert!(result.is_valid());
}

#[test]
fn test_library_with_cues_and_links() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Media Test"
exercise_library:
  - id: squat
    name: "Squat"
    modality: strength
    cues: "Keep chest up, break at hips"
    link: "https://example.com/squat"
    image: "https://example.com/squat.jpg"
cycle:
  days:
    - exercises:
        - exercise_ref: squat
"#;

    let result = validate(yaml);
    assert!(result.is_valid());
}

#[test]
fn test_resolve_exercise_uses_library_defaults() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Resolution Test"
exercise_library:
  - id: squat
    name: "Barbell Squat"
    modality: strength
    default_sets: 3
    default_reps: 8
    cues: "Keep chest up"
cycle:
  days:
    - exercises:
        - exercise_ref: squat
          target_sets: 5
"#;

    let result = validate(yaml);
    assert!(result.is_valid());

    let plan = result.plan.unwrap();
    let exercise = &plan.cycle.days[0].exercises[0];

    let resolved = resolve_exercise(exercise, &plan.exercise_library).unwrap();
    assert_eq!(resolved.name, "Barbell Squat");
    assert_eq!(resolved.target_sets, Some(5)); // Overridden
    assert_eq!(resolved.target_reps, Some(8)); // From library
}

#[test]
fn test_library_empty_is_valid() {
    let yaml = r#"
plan_version: 2
meta:
  title: "Empty Library"
exercise_library: []
cycle:
  days:
    - exercises:
        - name: "Push-up"
          modality: strength
"#;

    let result = validate(yaml);
    assert!(result.is_valid());
}

#[test]
fn test_library_with_all_modalities() {
    let yaml = r#"
plan_version: 2
meta:
  title: "All Modalities"
exercise_library:
  - id: squat
    name: "Squat"
    modality: strength
  - id: plank
    name: "Plank"
    modality: countdown
  - id: row
    name: "Row"
    modality: stopwatch
  - id: hiit
    name: "HIIT"
    modality: interval
  - id: bike
    name: "Bike"
    modality: cycling
  - id: run
    name: "Run"
    modality: running
  - id: row2
    name: "Row Machine"
    modality: rowing
  - id: swim
    name: "Swim"
    modality: swimming
cycle:
  days:
    - exercises:
        - exercise_ref: squat
        - exercise_ref: plank
        - exercise_ref: row
        - exercise_ref: hiit
        - exercise_ref: bike
        - exercise_ref: run
        - exercise_ref: row2
        - exercise_ref: swim
"#;

    let result = validate(yaml);
    assert!(result.is_valid());
}
