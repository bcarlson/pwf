//! Tests for WASM utility functions

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use pwf_wasm::*;
use serde_json::Value;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_get_version() {
    let version = get_version();
    assert!(!version.is_empty());
    assert!(version.contains('.'));  // Should be in format "X.Y.Z"
}

#[wasm_bindgen_test]
fn test_get_version_matches_cargo_version() {
    let version = get_version();
    // Version should be 1.3.0
    assert_eq!(version, "1.3.0");
}

#[wasm_bindgen_test]
fn test_get_supported_modalities() {
    let result = get_supported_modalities();
    let modalities: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(modalities.is_array());
    let array = modalities.as_array().unwrap();
    assert!(array.len() > 0);

    // Check for expected modalities
    let modality_strings: Vec<&str> = array
        .iter()
        .filter_map(|v| v.as_str())
        .collect();

    assert!(modality_strings.contains(&"strength"));
    assert!(modality_strings.contains(&"countdown"));
    assert!(modality_strings.contains(&"stopwatch"));
    assert!(modality_strings.contains(&"interval"));
    assert!(modality_strings.contains(&"cycling"));
    assert!(modality_strings.contains(&"running"));
    assert!(modality_strings.contains(&"rowing"));
    assert!(modality_strings.contains(&"swimming"));
}

#[wasm_bindgen_test]
fn test_get_supported_sports() {
    let result = get_supported_sports();
    let sports: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(sports.is_array());
    let array = sports.as_array().unwrap();
    assert!(array.len() > 0);

    let sport_strings: Vec<&str> = array
        .iter()
        .filter_map(|v| v.as_str())
        .collect();

    // Check for expected sports
    assert!(sport_strings.contains(&"Swimming"));
    assert!(sport_strings.contains(&"Cycling"));
    assert!(sport_strings.contains(&"Running"));
    assert!(sport_strings.contains(&"Rowing"));
    assert!(sport_strings.contains(&"Strength"));
    assert!(sport_strings.contains(&"Hiking"));
    assert!(sport_strings.contains(&"Walking"));
    assert!(sport_strings.contains(&"Yoga"));
    assert!(sport_strings.contains(&"CrossFit"));
    assert!(sport_strings.contains(&"Other"));
}

#[wasm_bindgen_test]
fn test_get_supported_sports_count() {
    let result = get_supported_sports();
    let sports: Value = serde_wasm_bindgen::from_value(result).unwrap();
    let array = sports.as_array().unwrap();

    // Should have 22 sport types as per the expanded sport enum
    assert_eq!(array.len(), 22);
}

#[wasm_bindgen_test]
fn test_get_supported_equipment() {
    let result = get_supported_equipment();
    let equipment: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(equipment.is_array());
    let array = equipment.as_array().unwrap();
    assert!(array.len() > 0);

    let equipment_strings: Vec<&str> = array
        .iter()
        .filter_map(|v| v.as_str())
        .collect();

    // Check for expected equipment
    assert!(equipment_strings.contains(&"barbell"));
    assert!(equipment_strings.contains(&"dumbbells"));
    assert!(equipment_strings.contains(&"kettlebell"));
    assert!(equipment_strings.contains(&"pullup_bar"));
    assert!(equipment_strings.contains(&"bench"));
    assert!(equipment_strings.contains(&"cables"));
    assert!(equipment_strings.contains(&"bands"));
    assert!(equipment_strings.contains(&"bodyweight"));
    assert!(equipment_strings.contains(&"machine"));
}

#[wasm_bindgen_test]
fn test_modalities_are_lowercase() {
    let result = get_supported_modalities();
    let modalities: Value = serde_wasm_bindgen::from_value(result).unwrap();
    let array = modalities.as_array().unwrap();

    for modality in array {
        let s = modality.as_str().unwrap();
        // All modalities should be lowercase
        assert_eq!(s, s.to_lowercase());
    }
}

#[wasm_bindgen_test]
fn test_equipment_uses_underscores() {
    let result = get_supported_equipment();
    let equipment: Value = serde_wasm_bindgen::from_value(result).unwrap();
    let array = equipment.as_array().unwrap();

    for item in array {
        let s = item.as_str().unwrap();
        // Equipment tags should use underscores, not spaces
        assert!(!s.contains(' '));
    }
}
