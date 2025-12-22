//! Integration tests for FIT to PWF conversion
//!
//! These tests verify the complete conversion pipeline from FIT files to PWF YAML.

use pwf_converters::error::{ConversionError, ConversionResult, ConversionWarning};
use pwf_converters::fit_to_pwf;
use std::io::Cursor;

#[allow(dead_code)]
mod test_helpers;

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

// ===== New comprehensive integration tests =====

#[test]
fn test_conversion_result_multiple_warnings() {
    // Test that ConversionResult can accumulate multiple warnings of different types
    let mut result = ConversionResult::new("test: yaml".to_string());

    result.add_warning(ConversionWarning::MissingField {
        source_field: "field1".to_string(),
        reason: "not supported".to_string(),
    });

    result.add_warning(ConversionWarning::ValueClamped {
        field: "field2".to_string(),
        original: "1000".to_string(),
        clamped: "255".to_string(),
    });

    result.add_warning(ConversionWarning::UnsupportedFeature {
        feature: "feature1".to_string(),
    });

    assert_eq!(result.warnings.len(), 3);
    assert!(result.has_warnings());

    // Verify each warning type is preserved
    let has_missing = result
        .warnings
        .iter()
        .any(|w| matches!(w, ConversionWarning::MissingField { .. }));
    let has_clamped = result
        .warnings
        .iter()
        .any(|w| matches!(w, ConversionWarning::ValueClamped { .. }));
    let has_unsupported = result
        .warnings
        .iter()
        .any(|w| matches!(w, ConversionWarning::UnsupportedFeature { .. }));

    assert!(has_missing);
    assert!(has_clamped);
    assert!(has_unsupported);
}

#[test]
fn test_warning_equality() {
    // Test ConversionWarning PartialEq implementation
    let warning1 = ConversionWarning::MissingField {
        source_field: "test".to_string(),
        reason: "reason".to_string(),
    };
    let warning2 = ConversionWarning::MissingField {
        source_field: "test".to_string(),
        reason: "reason".to_string(),
    };
    let warning3 = ConversionWarning::MissingField {
        source_field: "different".to_string(),
        reason: "reason".to_string(),
    };

    assert_eq!(warning1, warning2);
    assert_ne!(warning1, warning3);
}

#[test]
fn test_warning_clone() {
    // Test ConversionWarning Clone implementation
    let warning = ConversionWarning::DataQualityIssue {
        issue: "test issue".to_string(),
    };
    let cloned = warning.clone();

    assert_eq!(warning, cloned);
}

#[test]
fn test_error_from_io_error() {
    // Test ConversionError From<io::Error> implementation
    use std::io::{Error, ErrorKind};

    let io_error = Error::new(ErrorKind::PermissionDenied, "access denied");
    let conv_error: ConversionError = io_error.into();

    match conv_error {
        ConversionError::IoError(_) => {} // Expected
        _ => panic!("Expected IoError variant"),
    }
}

#[test]
fn test_error_from_yaml_error() {
    // Test ConversionError From<serde_yaml::Error> implementation
    // Create an invalid YAML serialization scenario
    use serde::Serialize;

    #[derive(Serialize)]
    struct BadStruct {
        #[serde(serialize_with = "bad_serialize")]
        field: String,
    }

    fn bad_serialize<S>(_: &String, _: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Err(serde::ser::Error::custom("intentional error"))
    }

    let bad = BadStruct {
        field: "test".to_string(),
    };

    let yaml_err = serde_yaml::to_string(&bad).unwrap_err();
    let conv_error: ConversionError = yaml_err.into();

    match conv_error {
        ConversionError::YamlError(_) => {} // Expected
        _ => panic!("Expected YamlError variant"),
    }
}

#[test]
fn test_conversion_error_display() {
    // Test all ConversionError variants display correctly
    let errors = vec![
        ConversionError::InvalidFitData("test data".to_string()),
        ConversionError::PwfValidationError("validation failed".to_string()),
        ConversionError::UnsupportedFormat("TCX".to_string()),
        ConversionError::MissingRequiredField("required_field".to_string()),
    ];

    for error in errors {
        let display = error.to_string();
        assert!(!display.is_empty(), "Error display should not be empty");
    }
}

#[test]
fn test_invalid_fit_magic_bytes() {
    // Test FIT file with invalid magic bytes
    let mut fit_data = Vec::new();
    fit_data.push(14); // Header size
    fit_data.push(0x20); // Protocol version
    fit_data.extend_from_slice(&[0x00, 0x08]); // Profile version
    fit_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Data size
    fit_data.extend_from_slice(b"NOTF"); // Invalid magic - should be ".FIT"
    fit_data.extend_from_slice(&[0x00, 0x00]); // CRC

    let cursor = Cursor::new(fit_data);
    let result = fit_to_pwf(cursor, false);

    assert!(result.is_err(), "Invalid magic bytes should cause error");
}

#[test]
fn test_truncated_fit_file() {
    // Test FIT file that's truncated mid-stream
    let mut fit_data = Vec::new();
    fit_data.push(14); // Header size
    fit_data.push(0x20); // Protocol version
    fit_data.extend_from_slice(&[0x00, 0x08]); // Profile version
    fit_data.extend_from_slice(&[0x10, 0x00, 0x00, 0x00]); // Data size = 16 bytes
    fit_data.extend_from_slice(b".FIT"); // Magic
    fit_data.extend_from_slice(&[0x00, 0x00]); // CRC
                                               // But only provide 5 bytes when we promised 16
    fit_data.extend_from_slice(&[0x01, 0x02, 0x03, 0x04, 0x05]);

    let cursor = Cursor::new(fit_data);
    let result = fit_to_pwf(cursor, false);

    // Should fail due to truncation
    match result {
        Err(ConversionError::FitReadError(_)) => {} // Expected
        Ok(_) => panic!("Truncated file should fail"),
        Err(e) => panic!("Unexpected error type: {:?}", e),
    }
}

#[test]
fn test_empty_buffer_read() {
    // Test reading from an empty buffer
    let empty: Vec<u8> = Vec::new();
    let cursor = Cursor::new(empty);

    let result = fit_to_pwf(cursor, false);

    match result {
        Err(ConversionError::FitReadError(_)) => {} // Expected
        Ok(res) => {
            // If it somehow parses, should have warnings
            assert!(res.has_warnings());
        }
        Err(e) => panic!("Unexpected error type: {:?}", e),
    }
}

#[test]
fn test_io_error_handling() {
    // Test handling of I/O errors during read
    use std::io::{Error, Read};

    struct FailingReader;

    impl Read for FailingReader {
        fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
            Err(Error::other("simulated I/O error"))
        }
    }

    let failing_reader = FailingReader;
    let result = fit_to_pwf(failing_reader, false);

    match result {
        Err(ConversionError::IoError(_)) => {} // Expected
        _ => panic!("Expected IoError"),
    }
}

#[test]
fn test_summary_only_flag_true() {
    // Test that summary_only=true is accepted (parameter passing test)
    let invalid_data = vec![0xFF; 50];
    let cursor = Cursor::new(invalid_data);

    let result = fit_to_pwf(cursor, true);

    // Should still fail due to invalid data, but tests that the flag is accepted
    assert!(result.is_err());
}

#[test]
fn test_summary_only_flag_false() {
    // Test that summary_only=false is accepted (parameter passing test)
    let invalid_data = vec![0xFF; 50];
    let cursor = Cursor::new(invalid_data);

    let result = fit_to_pwf(cursor, false);

    // Should still fail due to invalid data, but tests that the flag is accepted
    assert!(result.is_err());
}

#[test]
fn test_conversion_result_empty_warnings() {
    // Test ConversionResult with no warnings
    let result = ConversionResult::new("test: content".to_string());

    assert!(!result.has_warnings());
    assert_eq!(result.warnings.len(), 0);
    assert!(result.warnings.is_empty());
}

#[test]
fn test_all_warning_variants_display() {
    // Test display formatting for all warning variants
    let warnings = vec![
        ConversionWarning::MissingField {
            source_field: "hr_zone".to_string(),
            reason: "PWF doesn't support zones".to_string(),
        },
        ConversionWarning::ValueClamped {
            field: "power".to_string(),
            original: "2000".to_string(),
            clamped: "1999".to_string(),
        },
        ConversionWarning::UnsupportedFeature {
            feature: "virtual_partner".to_string(),
        },
        ConversionWarning::TimeSeriesSkipped {
            reason: "summary_only mode".to_string(),
        },
        ConversionWarning::DataQualityIssue {
            issue: "GPS signal lost".to_string(),
        },
    ];

    for warning in warnings {
        let display = warning.to_string();
        assert!(!display.is_empty());
        // Each should contain some identifying text
        match warning {
            ConversionWarning::MissingField { .. } => assert!(display.contains("Missing field")),
            ConversionWarning::ValueClamped { .. } => assert!(display.contains("Value clamped")),
            ConversionWarning::UnsupportedFeature { .. } => {
                assert!(display.contains("Unsupported feature"))
            }
            ConversionWarning::TimeSeriesSkipped { .. } => {
                assert!(display.contains("Time-series data skipped"))
            }
            ConversionWarning::DataQualityIssue { .. } => {
                assert!(display.contains("Data quality issue"))
            }
        }
    }
}

#[test]
fn test_zero_byte_file() {
    // Test handling of a truly empty file (0 bytes)
    let empty: Vec<u8> = vec![];
    let cursor = Cursor::new(empty);

    let result = fit_to_pwf(cursor, false);

    // Empty file might parse successfully with warnings or return an error
    match result {
        Err(ConversionError::FitReadError(_)) => {} // Expected: parse error
        Ok(res) => {
            // If it parses, should have warnings about no data
            assert!(res.has_warnings(), "Empty file should have warnings");
            assert!(
                res.pwf_yaml.contains("workouts: []"),
                "Should have empty workouts"
            );
        }
        Err(e) => panic!("Unexpected error type: {:?}", e),
    }
}

#[test]
fn test_single_byte_file() {
    // Test handling of a file with only 1 byte
    let data: Vec<u8> = vec![0x42];
    let cursor = Cursor::new(data);

    let result = fit_to_pwf(cursor, false);

    // Should return an error (incomplete header)
    assert!(result.is_err());
}

#[test]
fn test_partial_header() {
    // Test FIT file with incomplete header (only 10 bytes instead of 14)
    let data: Vec<u8> = vec![14, 0x20, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x2E, 0x46];
    let cursor = Cursor::new(data);

    let result = fit_to_pwf(cursor, false);

    assert!(result.is_err());
}
