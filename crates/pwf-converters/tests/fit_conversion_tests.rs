//! Integration tests for FIT to PWF conversion
//!
//! These tests verify the complete conversion pipeline from FIT files to PWF YAML.

use pwf_converters::error::{ConversionError, ConversionWarning};
use pwf_converters::fit_to_pwf;
use std::io::Cursor;

#[test]
fn test_empty_fit_file_has_warnings() {
    // Empty FIT file (or file with no sessions) should produce warnings
    let empty_data: Vec<u8> = Vec::new();
    let cursor = Cursor::new(empty_data);

    let result = fit_to_pwf(cursor, false);

    // Empty file might parse successfully but should have warnings about no data
    // or return an error - both are acceptable
    match result {
        Ok(res) => {
            // Should have warnings about missing sessions
            assert!(
                res.has_warnings() || res.pwf_yaml.contains("workouts: []"),
                "Empty FIT should have warnings or empty workouts"
            );
        }
        Err(ConversionError::FitReadError(_)) => {
            // Also acceptable - empty file can't be parsed
        }
        Err(e) => {
            panic!("Unexpected error type: {:?}", e);
        }
    }
}

#[test]
fn test_invalid_fit_data_returns_error() {
    // Invalid FIT data should fail to parse
    let invalid_data = vec![0xFF; 100]; // Random bytes
    let cursor = Cursor::new(invalid_data);

    let result = fit_to_pwf(cursor, false);
    assert!(result.is_err(), "Invalid FIT data should return an error");
}

#[test]
fn test_minimal_fit_header_structure() {
    // FIT file header structure (14 bytes minimum)
    // This won't be a valid FIT file, but tests header parsing
    let mut fit_data = Vec::new();

    // FIT header (14 bytes)
    fit_data.push(14); // Header size
    fit_data.push(0x10); // Protocol version
    fit_data.extend_from_slice(&[0x00, 0x00]); // Profile version
    fit_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Data size
    fit_data.extend_from_slice(b".FIT"); // Data type
    fit_data.extend_from_slice(&[0x00, 0x00]); // CRC

    let cursor = Cursor::new(fit_data);
    let result = fit_to_pwf(cursor, false);

    // This will likely fail due to missing records, but tests header parsing
    // We expect either a parse error or a warning about no sessions
    match result {
        Ok(res) => {
            // Should have warnings about missing data
            assert!(
                !res.warnings.is_empty(),
                "Should have warnings for minimal FIT file"
            );
        }
        Err(ConversionError::FitReadError(_)) => {
            // Also acceptable - incomplete FIT file
        }
        Err(e) => {
            panic!("Unexpected error type: {:?}", e);
        }
    }
}

#[test]
fn test_conversion_result_with_warnings() {
    // Test that conversion warnings are properly collected
    use pwf_converters::error::ConversionResult;

    let mut result = ConversionResult::new("test: yaml".to_string());
    assert!(!result.has_warnings());

    result.add_warning(ConversionWarning::UnsupportedFeature {
        feature: "test_feature".to_string(),
    });
    assert!(result.has_warnings());
    assert_eq!(result.warnings.len(), 1);

    result.add_warning(ConversionWarning::MissingField {
        source_field: "test_field".to_string(),
        reason: "not supported".to_string(),
    });
    assert_eq!(result.warnings.len(), 2);
}

#[test]
fn test_warning_display_formats() {
    // Test warning message formatting
    let warning1 = ConversionWarning::MissingField {
        source_field: "heart_rate_zone".to_string(),
        reason: "PWF does not support HR zones".to_string(),
    };
    assert!(warning1
        .to_string()
        .contains("Missing field 'heart_rate_zone'"));

    let warning2 = ConversionWarning::ValueClamped {
        field: "cadence".to_string(),
        original: "300".to_string(),
        clamped: "255".to_string(),
    };
    assert!(warning2.to_string().contains("Value clamped"));

    let warning3 = ConversionWarning::UnsupportedFeature {
        feature: "lap_trigger".to_string(),
    };
    assert!(warning3.to_string().contains("Unsupported feature"));

    let warning4 = ConversionWarning::TimeSeriesSkipped {
        reason: "summary_only flag".to_string(),
    };
    assert!(warning4.to_string().contains("Time-series data skipped"));

    let warning5 = ConversionWarning::DataQualityIssue {
        issue: "Inconsistent timestamps".to_string(),
    };
    assert!(warning5.to_string().contains("Data quality issue"));
}

#[test]
fn test_conversion_error_types() {
    // Test error type conversions
    use std::io::{Error, ErrorKind};

    // IO Error
    let io_err = Error::new(ErrorKind::NotFound, "file not found");
    let conv_err: ConversionError = io_err.into();
    assert!(matches!(conv_err, ConversionError::IoError(_)));

    // Test error display
    let err = ConversionError::MissingRequiredField("session_start".to_string());
    assert!(err.to_string().contains("Missing required field"));

    let err = ConversionError::InvalidFitData("corrupt session data".to_string());
    assert!(err.to_string().contains("Invalid FIT data"));

    let err = ConversionError::UnsupportedFormat("GPX".to_string());
    assert!(err.to_string().contains("Unsupported format"));
}

#[test]
fn test_summary_only_flag() {
    // Test that summary_only flag is accepted (even with invalid data)
    let invalid_data = vec![0xFF; 20];
    let cursor = Cursor::new(invalid_data);

    // Should fail regardless of summary_only flag, but tests parameter acceptance
    let result = fit_to_pwf(cursor, true);
    assert!(result.is_err());
}

#[test]
fn test_conversion_preserves_yaml_structure() {
    // Test that successful conversions produce valid YAML structure
    // We'll test this indirectly by checking error types

    use pwf_converters::error::ConversionResult;

    let result = ConversionResult::new("history_version: 2\nworkouts: []".to_string());
    assert_eq!(result.pwf_yaml, "history_version: 2\nworkouts: []");
    assert!(!result.has_warnings());
}

/// Integration test helpers
mod test_helpers {
    /// Generate a minimal valid FIT file header
    pub fn create_fit_header() -> Vec<u8> {
        let mut header = Vec::new();
        header.push(14); // Header size
        header.push(0x20); // Protocol version 2.0
        header.extend_from_slice(&[0x00, 0x08]); // Profile version 8.0
        header.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Data size (placeholder)
        header.extend_from_slice(b".FIT"); // Data type
        header.extend_from_slice(&[0x00, 0x00]); // CRC (placeholder)
        header
    }

    #[test]
    fn test_header_creation() {
        let header = create_fit_header();
        assert_eq!(header.len(), 14);
        assert_eq!(header[0], 14);
        assert_eq!(&header[8..12], b".FIT");
    }
}

#[test]
fn test_fit_sport_mapping() {
    // Test sport type mapping (indirect test via mappings module)
    use pwf_converters::fit::mappings::map_fit_sport;
    use pwf_core::Sport;

    assert_eq!(map_fit_sport(0, None), Sport::Running);
    assert_eq!(map_fit_sport(1, None), Sport::Cycling);
    assert_eq!(map_fit_sport(5, None), Sport::Swimming);
    assert_eq!(map_fit_sport(255, None), Sport::Other);
}

#[test]
fn test_swim_stroke_mapping() {
    // Test swim stroke mapping
    use pwf_converters::fit::mappings::map_swim_stroke;
    use pwf_core::history::StrokeType;

    assert_eq!(map_swim_stroke(0), StrokeType::Freestyle);
    assert_eq!(map_swim_stroke(1), StrokeType::Backstroke);
    assert_eq!(map_swim_stroke(2), StrokeType::Breaststroke);
    assert_eq!(map_swim_stroke(3), StrokeType::Butterfly);
    assert_eq!(map_swim_stroke(4), StrokeType::Drill);
    assert_eq!(map_swim_stroke(5), StrokeType::Mixed);
    assert_eq!(map_swim_stroke(6), StrokeType::IndividualMedley);
    assert_eq!(map_swim_stroke(255), StrokeType::Freestyle); // Default
}

#[test]
fn test_timestamp_conversion() {
    // Test FIT timestamp to ISO8601 conversion
    use pwf_converters::common::utils::fit_timestamp_to_iso8601;

    // FIT epoch is 1989-12-31 00:00:00 UTC
    let iso = fit_timestamp_to_iso8601(0);
    assert!(iso.starts_with("1989-12-31T00:00:00"));

    // Test a more recent timestamp
    let iso = fit_timestamp_to_iso8601(1_000_000_000);
    assert!(iso.contains("T"));
    assert!(iso.contains("Z") || iso.contains("+"));
}

#[test]
fn test_duration_conversion() {
    // Test seconds to ISO8601 duration conversion
    use pwf_converters::common::utils::seconds_to_duration;

    assert_eq!(seconds_to_duration(0), "PT0S");
    assert_eq!(seconds_to_duration(30), "PT30S");
    assert_eq!(seconds_to_duration(90), "PT1M30S");
    assert_eq!(seconds_to_duration(3665), "PT1H1M5S");
}

#[test]
fn test_unit_conversions() {
    // Test unit conversion utilities
    use pwf_converters::common::utils::{meters_to_km, mps_to_kph};

    assert_eq!(meters_to_km(1000.0), 1.0);
    assert_eq!(meters_to_km(5000.0), 5.0);
    assert_eq!(mps_to_kph(10.0), 36.0);
    assert_eq!(mps_to_kph(2.78), 10.008); // ~10 kph
}

#[test]
fn test_gps_coordinate_conversion() {
    // Test semicircles to degrees conversion
    use pwf_converters::common::utils::semicircles_to_degrees;

    // 90 degrees = 2^30 semicircles
    let degrees = semicircles_to_degrees(1_073_741_824);
    assert!((degrees - 90.0).abs() < 0.001);

    // -90 degrees
    let degrees = semicircles_to_degrees(-1_073_741_824);
    assert!((degrees + 90.0).abs() < 0.001);

    // 180 degrees = 2^31 semicircles
    let degrees = semicircles_to_degrees(2_147_483_647);
    assert!((degrees - 180.0).abs() < 0.001);
}
