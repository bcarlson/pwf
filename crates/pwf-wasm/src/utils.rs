//! Utility functions for the WASM interface

use wasm_bindgen::prelude::*;

/// Get the PWF version.
///
/// Returns the current version of the PWF library.
///
/// # Example
///
/// ```javascript
/// const version = get_version();
/// console.log(version); // "1.3.0"
/// ```
#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Get a list of supported workout modalities.
///
/// Returns an array of modality names that PWF supports.
///
/// # Example
///
/// ```javascript
/// const modalities = get_supported_modalities();
/// console.log(modalities); // ["strength", "countdown", "stopwatch", "interval", ...]
/// ```
#[wasm_bindgen]
pub fn get_supported_modalities() -> JsValue {
    let modalities = vec![
        "strength",
        "countdown",
        "stopwatch",
        "interval",
        "cycling",
        "running",
        "rowing",
        "swimming",
    ];

    serde_wasm_bindgen::to_value(&modalities).unwrap_or(JsValue::NULL)
}

/// Get a list of supported sport types.
///
/// Returns an array of sport type names that PWF supports.
///
/// # Example
///
/// ```javascript
/// const sports = get_supported_sports();
/// console.log(sports); // ["Swimming", "Cycling", "Running", ...]
/// ```
#[wasm_bindgen]
pub fn get_supported_sports() -> JsValue {
    let sports = vec![
        "Swimming",
        "Cycling",
        "Running",
        "Rowing",
        "Transition",
        "Strength",
        "StrengthTraining",
        "Hiking",
        "Walking",
        "Yoga",
        "Pilates",
        "FunctionalFitness",
        "Calisthenics",
        "Cardio",
        "CrossCountrySkiing",
        "DownhillSkiing",
        "Snowboarding",
        "StandUpPaddling",
        "Kayaking",
        "Elliptical",
        "StairClimbing",
        "Other",
    ];

    serde_wasm_bindgen::to_value(&sports).unwrap_or(JsValue::NULL)
}

/// Get a list of supported equipment tags.
///
/// Returns an array of equipment tag names that PWF supports.
///
/// # Example
///
/// ```javascript
/// const equipment = get_supported_equipment();
/// console.log(equipment); // ["barbell", "dumbbells", ...]
/// ```
#[wasm_bindgen]
pub fn get_supported_equipment() -> JsValue {
    let equipment = vec![
        "barbell",
        "dumbbells",
        "kettlebell",
        "pullup_bar",
        "bench",
        "cables",
        "bands",
        "bodyweight",
        "machine",
    ];

    serde_wasm_bindgen::to_value(&equipment).unwrap_or(JsValue::NULL)
}
