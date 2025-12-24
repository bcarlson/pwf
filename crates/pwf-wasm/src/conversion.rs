//! Format conversion bindings for FIT, TCX, GPX, and CSV formats

use std::io::Cursor;
use wasm_bindgen::prelude::*;

/// Convert a FIT file to PWF YAML format.
///
/// # Parameters
/// - `bytes`: The FIT file content as a byte array
/// - `summary_only`: If true, skip time-series telemetry data
///
/// # Returns
/// JSON object with:
/// ```json
/// {
///   "pwf_yaml": "...",
///   "warnings": [...]
/// }
/// ```
#[wasm_bindgen]
pub fn fit_to_pwf(bytes: &[u8], summary_only: bool) -> JsValue {
    let cursor = Cursor::new(bytes);
    let result = pwf_converters::fit_to_pwf(cursor, summary_only);

    match result {
        Ok(conversion_result) => {
            serde_wasm_bindgen::to_value(&conversion_result).unwrap_or_else(|err| {
                JsValue::from_str(&format!("Failed to serialize conversion result: {}", err))
            })
        }
        Err(err) => {
            let error_obj = serde_json::json!({
                "error": err.to_string(),
                "pwf_yaml": null,
                "warnings": []
            });
            serde_wasm_bindgen::to_value(&error_obj).unwrap()
        }
    }
}

/// Convert a TCX file to PWF YAML format.
///
/// # Parameters
/// - `bytes`: The TCX file content as a byte array
/// - `summary_only`: If true, skip time-series telemetry data
///
/// # Returns
/// JSON object with PWF YAML and warnings
#[wasm_bindgen]
pub fn tcx_to_pwf(bytes: &[u8], summary_only: bool) -> JsValue {
    let cursor = Cursor::new(bytes);
    let result = pwf_converters::tcx_to_pwf(cursor, summary_only);

    match result {
        Ok(conversion_result) => {
            serde_wasm_bindgen::to_value(&conversion_result).unwrap_or_else(|err| {
                JsValue::from_str(&format!("Failed to serialize conversion result: {}", err))
            })
        }
        Err(err) => {
            let error_obj = serde_json::json!({
                "error": err.to_string(),
                "pwf_yaml": null,
                "warnings": []
            });
            serde_wasm_bindgen::to_value(&error_obj).unwrap()
        }
    }
}

/// Convert a GPX file to PWF YAML format.
///
/// # Parameters
/// - `bytes`: The GPX file content as a byte array
/// - `summary_only`: If true, skip time-series telemetry data
///
/// # Returns
/// JSON object with PWF YAML and warnings
#[wasm_bindgen]
pub fn gpx_to_pwf(bytes: &[u8], summary_only: bool) -> JsValue {
    let cursor = Cursor::new(bytes);
    let result = pwf_converters::gpx_to_pwf(cursor, summary_only);

    match result {
        Ok(conversion_result) => {
            serde_wasm_bindgen::to_value(&conversion_result).unwrap_or_else(|err| {
                JsValue::from_str(&format!("Failed to serialize conversion result: {}", err))
            })
        }
        Err(err) => {
            let error_obj = serde_json::json!({
                "error": err.to_string(),
                "pwf_yaml": null,
                "warnings": []
            });
            serde_wasm_bindgen::to_value(&error_obj).unwrap()
        }
    }
}

/// Convert PWF YAML to TCX format.
///
/// # Parameters
/// - `yaml`: The PWF YAML content as a string
///
/// # Returns
/// JSON object with:
/// ```json
/// {
///   "tcx_xml": "...",
///   "warnings": [...]
/// }
/// ```
#[wasm_bindgen]
pub fn pwf_to_tcx(yaml: &str) -> JsValue {
    // First parse the PWF YAML
    let history_result = pwf_core::history::parse(yaml);

    match history_result {
        Ok(history) => {
            let result = pwf_converters::pwf_to_tcx(&history);

            match result {
                Ok(export_result) => {
                    serde_wasm_bindgen::to_value(&export_result).unwrap_or_else(|err| {
                        JsValue::from_str(&format!("Failed to serialize export result: {}", err))
                    })
                }
                Err(err) => {
                    let error_obj = serde_json::json!({
                        "error": err.to_string(),
                        "tcx_xml": null,
                        "warnings": []
                    });
                    serde_wasm_bindgen::to_value(&error_obj).unwrap()
                }
            }
        }
        Err(err) => {
            let error_obj = serde_json::json!({
                "error": format!("Failed to parse PWF YAML: {}", err),
                "tcx_xml": null,
                "warnings": []
            });
            serde_wasm_bindgen::to_value(&error_obj).unwrap()
        }
    }
}

/// Convert PWF YAML to GPX format.
///
/// # Parameters
/// - `yaml`: The PWF YAML content as a string
///
/// # Returns
/// JSON object with:
/// ```json
/// {
///   "gpx_xml": "...",
///   "warnings": [...]
/// }
/// ```
#[wasm_bindgen]
pub fn pwf_to_gpx(yaml: &str) -> JsValue {
    // First parse the PWF YAML
    let history_result = pwf_core::history::parse(yaml);

    match history_result {
        Ok(history) => {
            let result = pwf_converters::pwf_to_gpx(&history);

            match result {
                Ok(export_result) => {
                    serde_wasm_bindgen::to_value(&export_result).unwrap_or_else(|err| {
                        JsValue::from_str(&format!("Failed to serialize export result: {}", err))
                    })
                }
                Err(err) => {
                    let error_obj = serde_json::json!({
                        "error": err.to_string(),
                        "gpx_xml": null,
                        "warnings": []
                    });
                    serde_wasm_bindgen::to_value(&error_obj).unwrap()
                }
            }
        }
        Err(err) => {
            let error_obj = serde_json::json!({
                "error": format!("Failed to parse PWF YAML: {}", err),
                "gpx_xml": null,
                "warnings": []
            });
            serde_wasm_bindgen::to_value(&error_obj).unwrap()
        }
    }
}

/// Convert PWF YAML to CSV format for telemetry data.
///
/// # Parameters
/// - `yaml`: The PWF YAML content as a string
///
/// # Returns
/// JSON object with:
/// ```json
/// {
///   "csv_data": "...",
///   "warnings": [...],
///   "data_points": 1234,
///   "workouts_processed": 5
/// }
/// ```
#[wasm_bindgen]
pub fn pwf_to_csv(yaml: &str) -> JsValue {
    // First parse the PWF YAML
    let history_result = pwf_core::history::parse(yaml);

    match history_result {
        Ok(history) => {
            // Use default CSV export options
            let options = pwf_converters::CsvExportOptions::default();
            let result = pwf_converters::export_telemetry_to_csv(&history, &options);

            match result {
                Ok(export_result) => {
                    serde_wasm_bindgen::to_value(&export_result).unwrap_or_else(|err| {
                        JsValue::from_str(&format!("Failed to serialize export result: {}", err))
                    })
                }
                Err(err) => {
                    let error_obj = serde_json::json!({
                        "error": err.to_string(),
                        "csv_data": null,
                        "warnings": [],
                        "data_points": 0,
                        "workouts_processed": 0
                    });
                    serde_wasm_bindgen::to_value(&error_obj).unwrap()
                }
            }
        }
        Err(err) => {
            let error_obj = serde_json::json!({
                "error": format!("Failed to parse PWF YAML: {}", err),
                "csv_data": null,
                "warnings": [],
                "data_points": 0,
                "workouts_processed": 0
            });
            serde_wasm_bindgen::to_value(&error_obj).unwrap()
        }
    }
}
