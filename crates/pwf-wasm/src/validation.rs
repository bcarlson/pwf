//! Validation bindings for PWF plans and history files

use wasm_bindgen::prelude::*;

/// Validate a PWF plan from YAML string.
///
/// Returns a JSON object with the validation result:
/// ```json
/// {
///   "valid": true,
///   "plan": { /* WpsPlan object */ },
///   "errors": [],
///   "warnings": [],
///   "statistics": { /* PlanStatistics */ }
/// }
/// ```
///
/// # Example
///
/// ```javascript
/// const yaml = `
/// plan_version: 1
/// cycle:
///   days:
///     - exercises:
///         - name: Squat
///           modality: strength
///           target_sets: 3
///           target_reps: 5
/// `;
/// const result = validate_plan(yaml);
/// console.log(result.valid); // true
/// ```
#[wasm_bindgen]
pub fn validate_plan(yaml: &str) -> JsValue {
    let result = pwf_core::plan::validate(yaml);
    serde_wasm_bindgen::to_value(&result).unwrap_or_else(|err| {
        JsValue::from_str(&format!("Failed to serialize validation result: {}", err))
    })
}

/// Validate a PWF history file from YAML string.
///
/// Returns a JSON object with the validation result:
/// ```json
/// {
///   "valid": true,
///   "history": { /* WpsHistory object */ },
///   "errors": [],
///   "warnings": [],
///   "statistics": { /* HistoryStatistics */ }
/// }
/// ```
///
/// # Example
///
/// ```javascript
/// const yaml = `
/// history_version: 1
/// exported_at: "2025-12-24T12:00:00Z"
/// workouts:
///   - date: "2025-12-24"
///     exercises:
///       - name: Squat
///         sets:
///           - reps: 5
///             weight_kg: 100
/// `;
/// const result = validate_history(yaml);
/// console.log(result.valid); // true
/// ```
#[wasm_bindgen]
pub fn validate_history(yaml: &str) -> JsValue {
    let result = pwf_core::history::validate(yaml);
    serde_wasm_bindgen::to_value(&result).unwrap_or_else(|err| {
        JsValue::from_str(&format!("Failed to serialize validation result: {}", err))
    })
}
