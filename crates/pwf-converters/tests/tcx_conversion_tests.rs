//! Integration tests for TCX to PWF conversion

use pwf_converters::{tcx_to_pwf, ConversionWarning};
use std::io::Cursor;

/// Helper to create a minimal valid TCX XML
fn create_minimal_tcx() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<TrainingCenterDatabase xmlns="http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2">
  <Activities>
    <Activity Sport="Running">
      <Id>2024-01-15T10:00:00Z</Id>
      <Lap StartTime="2024-01-15T10:00:00Z">
        <TotalTimeSeconds>1800</TotalTimeSeconds>
        <DistanceMeters>5000</DistanceMeters>
        <Calories>300</Calories>
        <Intensity>Active</Intensity>
        <TriggerMethod>Manual</TriggerMethod>
        <Track>
          <Trackpoint>
            <Time>2024-01-15T10:00:00Z</Time>
            <Position>
              <LatitudeDegrees>37.7749</LatitudeDegrees>
              <LongitudeDegrees>-122.4194</LongitudeDegrees>
            </Position>
            <AltitudeMeters>50.0</AltitudeMeters>
            <HeartRateBpm>
              <Value>140</Value>
            </HeartRateBpm>
          </Trackpoint>
        </Track>
      </Lap>
    </Activity>
  </Activities>
</TrainingCenterDatabase>"#
        .to_string()
}

#[test]
fn test_tcx_to_pwf_minimal() {
    let tcx_xml = create_minimal_tcx();
    let cursor = Cursor::new(tcx_xml.as_bytes());

    let result = tcx_to_pwf(cursor, false);
    assert!(result.is_ok(), "TCX parsing should succeed");

    let conversion = result.unwrap();
    assert!(!conversion.pwf_yaml.is_empty(), "Should generate PWF YAML");

    // Parse the generated YAML
    let history: serde_yaml::Value = serde_yaml::from_str(&conversion.pwf_yaml).unwrap();

    // Check basic structure
    assert_eq!(history["history_version"], 2);
    assert!(history["workouts"].is_sequence());

    let workouts = history["workouts"].as_sequence().unwrap();
    assert_eq!(workouts.len(), 1, "Should have one workout");

    let workout = &workouts[0];
    assert_eq!(workout["date"], "2024-01-15");
    assert_eq!(workout["started_at"], "2024-01-15T10:00:00Z");
    assert_eq!(workout["duration_sec"], 1800);
    assert_eq!(workout["sport"], "running");
}

#[test]
fn test_tcx_to_pwf_with_telemetry() {
    let tcx_xml = create_minimal_tcx();
    let cursor = Cursor::new(tcx_xml.as_bytes());

    let result = tcx_to_pwf(cursor, false).unwrap();
    let history: serde_yaml::Value = serde_yaml::from_str(&result.pwf_yaml).unwrap();

    let workout = &history["workouts"][0];
    let telemetry = &workout["telemetry"];

    assert!(telemetry["total_distance_km"].is_number());
    assert_eq!(telemetry["total_distance_km"].as_f64().unwrap(), 5.0);
    assert_eq!(telemetry["total_calories"], 300);
}

#[test]
fn test_tcx_to_pwf_with_gps() {
    let tcx_xml = create_minimal_tcx();
    let cursor = Cursor::new(tcx_xml.as_bytes());

    let result = tcx_to_pwf(cursor, false).unwrap();
    let history: serde_yaml::Value = serde_yaml::from_str(&result.pwf_yaml).unwrap();

    let workout = &history["workouts"][0];
    let telemetry = &workout["telemetry"];

    assert!(telemetry["gps_route"].is_mapping());
    let gps_route = &telemetry["gps_route"];

    assert!(gps_route["positions"].is_sequence());
    let positions = gps_route["positions"].as_sequence().unwrap();
    assert_eq!(positions.len(), 1);

    let position = &positions[0];
    assert_eq!(position["latitude_deg"].as_f64().unwrap(), 37.7749);
    assert_eq!(position["longitude_deg"].as_f64().unwrap(), -122.4194);
    assert_eq!(position["elevation_m"], 50.0);
    assert_eq!(position["heart_rate_bpm"], 140);
}

#[test]
fn test_tcx_to_pwf_summary_only() {
    let tcx_xml = create_minimal_tcx();
    let cursor = Cursor::new(tcx_xml.as_bytes());

    // With summary_only=true, GPS data should be excluded
    let result = tcx_to_pwf(cursor, true).unwrap();
    let history: serde_yaml::Value = serde_yaml::from_str(&result.pwf_yaml).unwrap();

    let workout = &history["workouts"][0];
    let telemetry = &workout["telemetry"];

    // GPS route should not be present when summary_only is true
    assert!(
        telemetry["gps_route"].is_null()
            || !telemetry.as_mapping().unwrap().contains_key("gps_route")
    );
}

#[test]
fn test_tcx_to_pwf_no_activities() {
    // TCX with no Activities element
    let tcx_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<TrainingCenterDatabase xmlns="http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2">
</TrainingCenterDatabase>"#;

    let cursor = Cursor::new(tcx_xml.as_bytes());
    let result = tcx_to_pwf(cursor, false).unwrap();

    // Should have warning about no activities
    assert!(result.has_warnings());
    assert!(result
        .warnings
        .iter()
        .any(|w| matches!(w, ConversionWarning::DataQualityIssue { .. })));

    let history: serde_yaml::Value = serde_yaml::from_str(&result.pwf_yaml).unwrap();
    let workouts = history["workouts"].as_sequence().unwrap();
    assert_eq!(workouts.len(), 0);
}

#[test]
fn test_tcx_to_pwf_multiple_laps() {
    let tcx_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<TrainingCenterDatabase xmlns="http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2">
  <Activities>
    <Activity Sport="Cycling">
      <Id>2024-01-15T10:00:00Z</Id>
      <Lap StartTime="2024-01-15T10:00:00Z">
        <TotalTimeSeconds>600</TotalTimeSeconds>
        <DistanceMeters>3000</DistanceMeters>
        <Calories>100</Calories>
        <Intensity>Active</Intensity>
        <TriggerMethod>Manual</TriggerMethod>
        <Track>
          <Trackpoint>
            <Time>2024-01-15T10:00:00Z</Time>
          </Trackpoint>
        </Track>
      </Lap>
      <Lap StartTime="2024-01-15T10:10:00Z">
        <TotalTimeSeconds>700</TotalTimeSeconds>
        <DistanceMeters>3500</DistanceMeters>
        <Calories>120</Calories>
        <Intensity>Active</Intensity>
        <TriggerMethod>Manual</TriggerMethod>
        <Track>
          <Trackpoint>
            <Time>2024-01-15T10:00:00Z</Time>
          </Trackpoint>
        </Track>
      </Lap>
    </Activity>
  </Activities>
</TrainingCenterDatabase>"#;

    let cursor = Cursor::new(tcx_xml.as_bytes());
    let result = tcx_to_pwf(cursor, false).unwrap();
    let history: serde_yaml::Value = serde_yaml::from_str(&result.pwf_yaml).unwrap();

    let workout = &history["workouts"][0];
    assert_eq!(workout["duration_sec"], 1300); // 600 + 700
    assert_eq!(workout["sport"], "cycling");

    let telemetry = &workout["telemetry"];
    assert_eq!(telemetry["total_distance_km"].as_f64().unwrap(), 6.5); // 3000 + 3500 meters
    assert_eq!(telemetry["total_calories"], 220); // 100 + 120

    // Check exercises (each lap becomes an exercise)
    let exercises = workout["exercises"].as_sequence().unwrap();
    assert_eq!(exercises.len(), 2);

    assert_eq!(exercises[0]["name"], "Lap 1");
    assert_eq!(exercises[1]["name"], "Lap 2");
}

#[test]
fn test_tcx_to_pwf_invalid_xml() {
    let invalid_tcx = "This is not XML";
    let cursor = Cursor::new(invalid_tcx.as_bytes());

    let result = tcx_to_pwf(cursor, false);
    assert!(result.is_err(), "Should fail on invalid XML");
}

#[test]
fn test_tcx_sport_mapping() {
    let sports = vec![
        ("Running", "running"),
        ("Biking", "cycling"),
        ("Swimming", "swimming"),
        ("Rowing", "rowing"),
        ("Other", "other"),
    ];

    for (tcx_sport, expected_pwf_sport) in sports {
        let tcx_xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<TrainingCenterDatabase xmlns="http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2">
  <Activities>
    <Activity Sport="{}">
      <Id>2024-01-15T10:00:00Z</Id>
      <Lap StartTime="2024-01-15T10:00:00Z">
        <TotalTimeSeconds>1800</TotalTimeSeconds>
        <DistanceMeters>5000</DistanceMeters>
        <Calories>300</Calories>
        <Intensity>Active</Intensity>
        <TriggerMethod>Manual</TriggerMethod>
        <Track>
          <Trackpoint>
            <Time>2024-01-15T10:00:00Z</Time>
          </Trackpoint>
        </Track>
      </Lap>
    </Activity>
  </Activities>
</TrainingCenterDatabase>"#,
            tcx_sport
        );

        let cursor = Cursor::new(tcx_xml.as_bytes());
        let result = tcx_to_pwf(cursor, false).unwrap();
        let history: serde_yaml::Value = serde_yaml::from_str(&result.pwf_yaml).unwrap();
        let workout = &history["workouts"][0];

        assert_eq!(
            workout["sport"].as_str().unwrap(),
            expected_pwf_sport,
            "TCX sport '{}' should map to PWF sport '{}'",
            tcx_sport,
            expected_pwf_sport
        );
    }
}

#[test]
fn test_tcx_to_pwf_export_source() {
    let tcx_xml = create_minimal_tcx();
    let cursor = Cursor::new(tcx_xml.as_bytes());

    let result = tcx_to_pwf(cursor, false).unwrap();
    let history: serde_yaml::Value = serde_yaml::from_str(&result.pwf_yaml).unwrap();

    let export_source = &history["export_source"];
    assert_eq!(export_source["app_name"], "PWF TCX Converter");
    assert!(export_source["app_version"].is_string());
    assert_eq!(export_source["platform"], "TCX file");
}

#[test]
fn test_tcx_with_heart_rate() {
    let tcx_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<TrainingCenterDatabase xmlns="http://www.garmin.com/xmlschemas/TrainingCenterDatabase/v2">
  <Activities>
    <Activity Sport="Running">
      <Id>2024-01-15T10:00:00Z</Id>
      <Lap StartTime="2024-01-15T10:00:00Z">
        <TotalTimeSeconds>1800</TotalTimeSeconds>
        <DistanceMeters>5000</DistanceMeters>
        <Calories>300</Calories>
        <AverageHeartRateBpm>
          <Value>150</Value>
        </AverageHeartRateBpm>
        <MaximumHeartRateBpm>
          <Value>180</Value>
        </MaximumHeartRateBpm>
        <Intensity>Active</Intensity>
        <TriggerMethod>Manual</TriggerMethod>
        <Track>
          <Trackpoint>
            <Time>2024-01-15T10:00:00Z</Time>
            <HeartRateBpm>
              <Value>140</Value>
            </HeartRateBpm>
          </Trackpoint>
          <Trackpoint>
            <Time>2024-01-15T10:15:00Z</Time>
            <HeartRateBpm>
              <Value>160</Value>
            </HeartRateBpm>
          </Trackpoint>
        </Track>
      </Lap>
    </Activity>
  </Activities>
</TrainingCenterDatabase>"#;

    let cursor = Cursor::new(tcx_xml.as_bytes());
    let result = tcx_to_pwf(cursor, false).unwrap();
    let history: serde_yaml::Value = serde_yaml::from_str(&result.pwf_yaml).unwrap();

    let telemetry = &history["workouts"][0]["telemetry"];
    // The lap has avg=150, max=180, and trackpoints with HR data should be captured
    // Check if either lap-level or trackpoint-level HR is captured
    assert!(
        telemetry.get("heart_rate_avg").is_some() || telemetry.get("heart_rate_max").is_some(),
        "Should have some heart rate data"
    );
}
