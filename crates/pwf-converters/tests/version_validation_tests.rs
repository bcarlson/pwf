//! Version validation tests for all converter formats
//!
//! These tests ensure that all converters generate PWF history files with
//! the correct history_version field. This catches bugs where converters
//! might accidentally set version 2 when they should use version 1.

use pwf_core::history::WpsHistory;
use std::io::Cursor;

/// Helper to parse YAML and extract history_version
fn get_history_version_from_yaml(yaml: &str) -> u32 {
    let history: WpsHistory = serde_yaml::from_str(yaml).expect("Failed to parse YAML");
    history.history_version
}

/// Test that GPX import sets correct history_version
#[test]
fn test_gpx_import_history_version() {
    let gpx_data = r#"<?xml version="1.0" encoding="UTF-8"?>
<gpx version="1.1" creator="Test" xmlns="http://www.topografix.com/GPX/1/1">
  <metadata>
    <name>Test Route</name>
  </metadata>
  <trk>
    <name>Morning Run</name>
    <trkseg>
      <trkpt lat="37.7749" lon="-122.4194">
        <ele>100.0</ele>
        <time>2024-01-15T14:30:00Z</time>
      </trkpt>
      <trkpt lat="37.7750" lon="-122.4195">
        <ele>102.0</ele>
        <time>2024-01-15T14:31:00Z</time>
      </trkpt>
    </trkseg>
  </trk>
</gpx>"#;

    let reader = Cursor::new(gpx_data);
    let result = pwf_converters::gpx_to_pwf(reader, false).unwrap();

    // Parse the YAML to check version
    let version = get_history_version_from_yaml(&result.pwf_yaml);

    // GPX import should always use history_version: 1
    assert_eq!(
        version, 1,
        "GPX import must set history_version to 1 (not {})",
        version
    );
}

/// Test that GPX import with summary-only mode still sets correct version
#[test]
fn test_gpx_import_summary_only_history_version() {
    let gpx_data = r#"<?xml version="1.0" encoding="UTF-8"?>
<gpx version="1.1" creator="Test" xmlns="http://www.topografix.com/GPX/1/1">
  <trk>
    <name>Test</name>
    <trkseg>
      <trkpt lat="37.7749" lon="-122.4194">
        <time>2024-01-15T14:30:00Z</time>
      </trkpt>
    </trkseg>
  </trk>
</gpx>"#;

    let reader = Cursor::new(gpx_data);
    let result = pwf_converters::gpx_to_pwf(reader, true).unwrap();
    let version = get_history_version_from_yaml(&result.pwf_yaml);

    assert_eq!(version, 1);
}

/// Test that TCX import sets correct history_version
#[test]
fn test_tcx_import_history_version() {
    let tcx_data = r#"<?xml version="1.0" encoding="UTF-8"?>
<TrainingCenterDatabase xmlns="http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2">
  <Activities>
    <Activity Sport="Running">
      <Id>2024-01-15T14:30:00Z</Id>
      <Lap StartTime="2024-01-15T14:30:00Z">
        <TotalTimeSeconds>1800</TotalTimeSeconds>
        <DistanceMeters>5000</DistanceMeters>
        <Calories>400</Calories>
        <Track>
          <Trackpoint>
            <Time>2024-01-15T14:30:00Z</Time>
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

    let reader = Cursor::new(tcx_data);
    let result = pwf_converters::tcx_to_pwf(reader, false).unwrap();
    let version = get_history_version_from_yaml(&result.pwf_yaml);

    // TCX import uses history_version: 2 because it includes GPS/telemetry data (v2.1 features)
    assert_eq!(
        version, 2,
        "TCX import must set history_version to 2 (not {})",
        version
    );
}

/// Test that all formats correctly validate with PWF history validator
/// after conversion (ensuring version compatibility)
#[test]
fn test_converted_files_validate() {
    // GPX conversion
    let gpx_data = r#"<?xml version="1.0" encoding="UTF-8"?>
<gpx version="1.1" creator="Test" xmlns="http://www.topografix.com/GPX/1/1">
  <trk>
    <name>Test</name>
    <trkseg>
      <trkpt lat="37.7749" lon="-122.4194">
        <time>2024-01-15T14:30:00Z</time>
      </trkpt>
    </trkseg>
  </trk>
</gpx>"#;

    let reader = Cursor::new(gpx_data);
    let gpx_result = pwf_converters::gpx_to_pwf(reader, false).unwrap();

    // Validate the generated YAML
    let validation = pwf_core::history::validate(&gpx_result.pwf_yaml);
    assert!(
        validation.valid,
        "GPX-converted PWF must pass validation. Errors: {:?}",
        validation.errors
    );

    // TCX conversion
    let tcx_data = r#"<?xml version="1.0" encoding="UTF-8"?>
<TrainingCenterDatabase xmlns="http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2">
  <Activities>
    <Activity Sport="Running">
      <Id>2024-01-15T14:30:00Z</Id>
      <Lap StartTime="2024-01-15T14:30:00Z">
        <TotalTimeSeconds>1800</TotalTimeSeconds>
        <DistanceMeters>5000</DistanceMeters>
        <Calories>400</Calories>
        <Track>
          <Trackpoint>
            <Time>2024-01-15T14:30:00Z</Time>
          </Trackpoint>
        </Track>
      </Lap>
    </Activity>
  </Activities>
</TrainingCenterDatabase>"#;

    let reader = Cursor::new(tcx_data);
    let tcx_result = pwf_converters::tcx_to_pwf(reader, false).unwrap();

    let validation = pwf_core::history::validate(&tcx_result.pwf_yaml);
    assert!(
        validation.valid,
        "TCX-converted PWF must pass validation. Errors: {:?}",
        validation.errors
    );
}

/// Test version field parsing and validation
#[test]
fn test_history_version_field_validation() {
    // Valid: version 1
    let yaml_v1 = r#"
history_version: 1
exported_at: "2024-01-15T16:00:00Z"
workouts: []
"#;
    let result = pwf_core::history::validate(yaml_v1);
    assert!(result.valid, "history_version: 1 should be valid");

    // Valid: version 2 (for advanced features)
    let yaml_v2 = r#"
history_version: 2
exported_at: "2024-01-15T16:00:00Z"
workouts: []
"#;
    let result = pwf_core::history::validate(yaml_v2);
    assert!(result.valid, "history_version: 2 should be valid");

    // Invalid: version 0
    let yaml_v0 = r#"
history_version: 0
exported_at: "2024-01-15T16:00:00Z"
workouts: []
"#;
    let result = pwf_core::history::validate(yaml_v0);
    assert!(!result.valid, "history_version: 0 should be invalid");
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.message.contains("history_version")),
        "Should have error about invalid history_version"
    );

    // Invalid: version 3
    let yaml_v3 = r#"
history_version: 3
exported_at: "2024-01-15T16:00:00Z"
workouts: []
"#;
    let result = pwf_core::history::validate(yaml_v3);
    assert!(!result.valid, "history_version: 3 should be invalid");
}
