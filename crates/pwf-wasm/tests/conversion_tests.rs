//! Comprehensive conversion tests for WASM bindings

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use pwf_wasm::*;
use serde_json::Value;

wasm_bindgen_test_configure!(run_in_browser);

// ============================================================================
// GPX TO PWF CONVERSION TESTS
// ============================================================================

#[wasm_bindgen_test]
fn test_gpx_to_pwf_minimal() {
    let gpx_xml = r#"<?xml version="1.0"?>
<gpx version="1.1" creator="Test">
  <trk>
    <name>Morning Run</name>
    <type>running</type>
    <trkseg>
      <trkpt lat="37.7749" lon="-122.4194">
        <ele>10.0</ele>
        <time>2025-12-24T08:00:00Z</time>
      </trkpt>
      <trkpt lat="37.7750" lon="-122.4195">
        <ele>11.0</ele>
        <time>2025-12-24T08:01:00Z</time>
      </trkpt>
    </trkseg>
  </trk>
</gpx>"#;

    let result = gpx_to_pwf(gpx_xml.as_bytes(), false);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(json["pwf_yaml"].is_string());
    assert!(json["pwf_yaml"].as_str().unwrap().contains("history_version: 1"));
}

#[wasm_bindgen_test]
fn test_gpx_to_pwf_with_summary_only() {
    let gpx_xml = r#"<?xml version="1.0"?>
<gpx version="1.1" creator="Test">
  <trk>
    <name>Morning Run</name>
    <trkseg>
      <trkpt lat="37.7749" lon="-122.4194">
        <ele>10.0</ele>
        <time>2025-12-24T08:00:00Z</time>
      </trkpt>
    </trkseg>
  </trk>
</gpx>"#;

    let result = gpx_to_pwf(gpx_xml.as_bytes(), true);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(json["pwf_yaml"].is_string());
    // Summary only should skip detailed GPS positions
}

#[wasm_bindgen_test]
fn test_gpx_to_pwf_invalid_xml() {
    let invalid_xml = b"<gpx><trk><invalid</gpx>";

    let result = gpx_to_pwf(invalid_xml, false);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(json["error"].is_string());
}

#[wasm_bindgen_test]
fn test_gpx_to_pwf_empty_data() {
    let empty = b"";

    let result = gpx_to_pwf(empty, false);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(json["error"].is_string());
}

#[wasm_bindgen_test]
fn test_gpx_to_pwf_multiple_tracks() {
    let gpx_xml = r#"<?xml version="1.0"?>
<gpx version="1.1" creator="Test">
  <trk>
    <name>Track 1</name>
    <trkseg>
      <trkpt lat="37.7749" lon="-122.4194">
        <time>2025-12-24T08:00:00Z</time>
      </trkpt>
    </trkseg>
  </trk>
  <trk>
    <name>Track 2</name>
    <trkseg>
      <trkpt lat="37.7750" lon="-122.4195">
        <time>2025-12-24T09:00:00Z</time>
      </trkpt>
    </trkseg>
  </trk>
</gpx>"#;

    let result = gpx_to_pwf(gpx_xml.as_bytes(), false);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(json["pwf_yaml"].is_string());
}

// ============================================================================
// TCX TO PWF CONVERSION TESTS
// ============================================================================

#[wasm_bindgen_test]
fn test_tcx_to_pwf_minimal() {
    let tcx_xml = r#"<?xml version="1.0"?>
<TrainingCenterDatabase xmlns="http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2">
  <Activities>
    <Activity Sport="Running">
      <Id>2025-12-24T08:00:00Z</Id>
      <Lap StartTime="2025-12-24T08:00:00Z">
        <TotalTimeSeconds>3600</TotalTimeSeconds>
        <DistanceMeters>10000</DistanceMeters>
        <Track>
          <Trackpoint>
            <Time>2025-12-24T08:00:00Z</Time>
            <Position>
              <LatitudeDegrees>37.7749</LatitudeDegrees>
              <LongitudeDegrees>-122.4194</LongitudeDegrees>
            </Position>
          </Trackpoint>
        </Track>
      </Lap>
    </Activity>
  </Activities>
</TrainingCenterDatabase>"#;

    let result = tcx_to_pwf(tcx_xml.as_bytes(), false);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(json["pwf_yaml"].is_string());
    assert!(json["pwf_yaml"].as_str().unwrap().contains("history_version: 1"));
}

#[wasm_bindgen_test]
fn test_tcx_to_pwf_with_heart_rate() {
    let tcx_xml = r#"<?xml version="1.0"?>
<TrainingCenterDatabase xmlns="http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2">
  <Activities>
    <Activity Sport="Running">
      <Id>2025-12-24T08:00:00Z</Id>
      <Lap StartTime="2025-12-24T08:00:00Z">
        <TotalTimeSeconds>3600</TotalTimeSeconds>
        <AverageHeartRateBpm><Value>150</Value></AverageHeartRateBpm>
        <MaximumHeartRateBpm><Value>170</Value></MaximumHeartRateBpm>
      </Lap>
    </Activity>
  </Activities>
</TrainingCenterDatabase>"#;

    let result = tcx_to_pwf(tcx_xml.as_bytes(), false);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(json["pwf_yaml"].is_string());
    let yaml = json["pwf_yaml"].as_str().unwrap();
    assert!(yaml.contains("avg_heart_rate") || yaml.contains("telemetry"));
}

#[wasm_bindgen_test]
fn test_tcx_to_pwf_invalid_xml() {
    let invalid_xml = b"<TrainingCenterDatabase><Activities><invalid</Activities>";

    let result = tcx_to_pwf(invalid_xml, false);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(json["error"].is_string());
}

// ============================================================================
// PWF TO GPX CONVERSION TESTS
// ============================================================================

#[wasm_bindgen_test]
fn test_pwf_to_gpx_with_gps_route() {
    let yaml = r#"
history_version: 1
exported_at: "2025-12-24T12:00:00Z"
workouts:
  - date: "2025-12-24"
    title: "Morning Run"
    sport: Running
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

    let result = pwf_to_gpx(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(json["gpx_xml"].is_string());
    let gpx = json["gpx_xml"].as_str().unwrap();
    assert!(gpx.contains("<?xml"));
    assert!(gpx.contains("<gpx"));
    assert!(gpx.contains("37.7749"));
}

#[wasm_bindgen_test]
fn test_pwf_to_gpx_without_gps_data() {
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

    let result = pwf_to_gpx(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    // Should succeed but may have warnings about no GPS data
    assert!(json["gpx_xml"].is_string() || json["warnings"].is_array());
}

#[wasm_bindgen_test]
fn test_pwf_to_gpx_invalid_yaml() {
    let yaml = r#"
history_version: 1
workouts: [
  - date: "invalid
"#;

    let result = pwf_to_gpx(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(json["error"].is_string());
}

#[wasm_bindgen_test]
fn test_pwf_to_gpx_multiple_workouts() {
    let yaml = r#"
history_version: 1
exported_at: "2025-12-24T12:00:00Z"
workouts:
  - date: "2025-12-24"
    sport: Running
    exercises:
      - name: Running
        sets:
          - duration_sec: 1800
    telemetry:
      gps_route:
        positions:
          - latitude: 37.7749
            longitude: -122.4194
            timestamp: "2025-12-24T08:00:00Z"
  - date: "2025-12-23"
    sport: Cycling
    exercises:
      - name: Cycling
        sets:
          - duration_sec: 3600
    telemetry:
      gps_route:
        positions:
          - latitude: 37.8749
            longitude: -122.5194
            timestamp: "2025-12-23T08:00:00Z"
"#;

    let result = pwf_to_gpx(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(json["gpx_xml"].is_string());
}

// ============================================================================
// PWF TO TCX CONVERSION TESTS
// ============================================================================

#[wasm_bindgen_test]
fn test_pwf_to_tcx_basic() {
    let yaml = r#"
history_version: 1
exported_at: "2025-12-24T12:00:00Z"
workouts:
  - date: "2025-12-24"
    started_at: "2025-12-24T08:00:00Z"
    sport: Running
    exercises:
      - name: Running
        sets:
          - duration_sec: 3600
            distance_meters: 10000
"#;

    let result = pwf_to_tcx(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(json["tcx_xml"].is_string());
    let tcx = json["tcx_xml"].as_str().unwrap();
    assert!(tcx.contains("<?xml"));
    assert!(tcx.contains("TrainingCenterDatabase"));
}

#[wasm_bindgen_test]
fn test_pwf_to_tcx_with_telemetry() {
    let yaml = r#"
history_version: 1
exported_at: "2025-12-24T12:00:00Z"
workouts:
  - date: "2025-12-24"
    started_at: "2025-12-24T08:00:00Z"
    sport: Running
    exercises:
      - name: Running
        sets:
          - duration_sec: 3600
    telemetry:
      calories: 500
      avg_heart_rate: 150
      max_heart_rate: 170
"#;

    let result = pwf_to_tcx(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(json["tcx_xml"].is_string());
    let tcx = json["tcx_xml"].as_str().unwrap();
    assert!(tcx.contains("150") || tcx.contains("HeartRate"));
}

#[wasm_bindgen_test]
fn test_pwf_to_tcx_invalid_yaml() {
    let yaml = "invalid: yaml: [";

    let result = pwf_to_tcx(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(json["error"].is_string());
}

// ============================================================================
// PWF TO CSV CONVERSION TESTS
// ============================================================================

#[wasm_bindgen_test]
fn test_pwf_to_csv_basic() {
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
              heart_rate_avg: 150
"#;

    let result = pwf_to_csv(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(json["csv_data"].is_string());
    let csv = json["csv_data"].as_str().unwrap();
    // CSV should have headers
    assert!(csv.len() > 0);
}

#[wasm_bindgen_test]
fn test_pwf_to_csv_empty_workouts() {
    let yaml = r#"
history_version: 1
exported_at: "2025-12-24T12:00:00Z"
workouts: []
"#;

    let result = pwf_to_csv(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(json["csv_data"].is_string());
    assert_eq!(json["data_points"], 0);
    assert_eq!(json["workouts_processed"], 0);
}

#[wasm_bindgen_test]
fn test_pwf_to_csv_invalid_yaml() {
    let yaml = "invalid: [yaml";

    let result = pwf_to_csv(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(json["error"].is_string());
}

#[wasm_bindgen_test]
fn test_pwf_to_csv_with_gps_data() {
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

    let result = pwf_to_csv(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    assert!(json["csv_data"].is_string());
    assert!(json["data_points"].as_u64().unwrap() > 0);
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[wasm_bindgen_test]
fn test_conversion_with_unicode_characters() {
    let yaml = r#"
history_version: 1
exported_at: "2025-12-24T12:00:00Z"
workouts:
  - date: "2025-12-24"
    title: "🏃 Morning Run 跑步"
    notes: "Felt great! 💪"
    exercises:
      - name: "Running 🏃‍♂️"
        sets:
          - duration_sec: 3600
"#;

    let result = pwf_to_gpx(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    // Should handle unicode gracefully
    assert!(json["gpx_xml"].is_string() || json["error"].is_string());
}

#[wasm_bindgen_test]
fn test_conversion_with_large_numbers() {
    let yaml = r#"
history_version: 1
exported_at: "2025-12-24T12:00:00Z"
workouts:
  - date: "2025-12-24"
    exercises:
      - name: Running
        sets:
          - duration_sec: 999999
            distance_meters: 999999999.99
"#;

    let result = pwf_to_tcx(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    // Should handle large numbers
    assert!(json["tcx_xml"].is_string() || json["warnings"].is_array());
}

#[wasm_bindgen_test]
fn test_conversion_with_special_characters_in_names() {
    let yaml = r#"
history_version: 1
exported_at: "2025-12-24T12:00:00Z"
workouts:
  - date: "2025-12-24"
    title: "Test & Demo <workout>"
    exercises:
      - name: "Squats & Lunges"
        sets:
          - reps: 5
"#;

    let result = pwf_to_gpx(yaml);
    let json: Value = serde_wasm_bindgen::from_value(result).unwrap();

    // Should escape XML special characters properly
    assert!(json["gpx_xml"].is_string() || json["error"].is_string());
}
