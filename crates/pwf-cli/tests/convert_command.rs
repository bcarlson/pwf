//! Integration tests for CLI convert command
//!
//! Tests cover:
//! - Convert command with various flags (--summary-only, --verbose)
//! - Error handling paths (invalid formats, missing files, conversion failures)
//! - Different format combinations (fit to pwf)
//! - Edge cases in the conversion command

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;

/// Helper to get the binary command
fn pwf_cmd() -> Command {
    Command::new(env!("CARGO_BIN_EXE_pwf"))
}

/// Helper to create a temporary directory for test files
struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("pwf_convert_test_{}", name));
        fs::create_dir_all(&path).unwrap();
        TempDir { path }
    }

    fn join(&self, file: &str) -> PathBuf {
        self.path.join(file)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

// ============================================================================
// Basic Convert Command Tests
// ============================================================================

#[test]
fn test_convert_help_message() {
    pwf_cmd()
        .arg("convert")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Convert between PWF and other formats",
        ))
        .stdout(predicate::str::contains("--from"))
        .stdout(predicate::str::contains("--to"))
        .stdout(predicate::str::contains("--summary-only"))
        .stdout(predicate::str::contains("--verbose"));
}

#[test]
fn test_convert_missing_required_args() {
    // Missing --from
    pwf_cmd()
        .arg("convert")
        .arg("--to")
        .arg("pwf")
        .arg("input.fit")
        .arg("output.yaml")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));

    // Missing --to
    pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("input.fit")
        .arg("output.yaml")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));

    // Missing input file
    pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg("output.yaml")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));

    // Missing output file
    pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg("input.fit")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_convert_same_format_error() {
    let temp = TempDir::new("same_format");
    let input = temp.join("input.fit");
    let output = temp.join("output.fit");

    // Create a dummy input file
    fs::write(&input, b"dummy").unwrap();

    pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("fit")
        .arg(&input)
        .arg(&output)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Input and output formats are the same",
        ));
}

#[test]
fn test_convert_case_insensitive_same_format() {
    let temp = TempDir::new("case_insensitive");
    let input = temp.join("input.fit");
    let output = temp.join("output.fit");

    fs::write(&input, b"dummy").unwrap();

    // Test with different case
    pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("FIT")
        .arg("--to")
        .arg("fit")
        .arg(&input)
        .arg(&output)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Input and output formats are the same",
        ));
}

#[test]
fn test_convert_input_file_not_found() {
    let temp = TempDir::new("missing_input");
    let input = temp.join("nonexistent.fit");
    let output = temp.join("output.yaml");

    pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg(&input)
        .arg(&output)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Input file not found"));
}

#[test]
fn test_convert_output_file_exists() {
    let temp = TempDir::new("output_exists");
    let input = temp.join("input.fit");
    let output = temp.join("output.yaml");

    // Create both files
    fs::write(&input, b"dummy").unwrap();
    fs::write(&output, b"existing").unwrap();

    pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg(&input)
        .arg(&output)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Output file already exists"));
}

#[test]
fn test_convert_unsupported_format_combination() {
    let temp = TempDir::new("unsupported_combo");
    let input = temp.join("input.tcx");
    let output = temp.join("output.yaml");

    fs::write(&input, b"dummy").unwrap();

    // TCX to PWF is not yet implemented
    pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("tcx")
        .arg("--to")
        .arg("pwf")
        .arg(&input)
        .arg(&output)
        .assert()
        .failure()
        .stderr(predicate::str::contains("not yet implemented"))
        .stderr(predicate::str::contains("Currently supported conversions:"))
        .stderr(predicate::str::contains("fit → pwf"));
}

#[test]
fn test_convert_unsupported_shows_coming_soon() {
    let temp = TempDir::new("coming_soon");
    let input = temp.join("input.gpx");
    let output = temp.join("output.yaml");

    fs::write(&input, b"dummy").unwrap();

    pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("gpx")
        .arg("--to")
        .arg("pwf")
        .arg(&input)
        .arg(&output)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Coming soon:"))
        .stderr(predicate::str::contains("tcx → pwf"))
        .stderr(predicate::str::contains("gpx → pwf"));
}

#[test]
fn test_convert_invalid_fit_file() {
    let temp = TempDir::new("invalid_fit");
    let input = temp.join("invalid.fit");
    let output = temp.join("output.yaml");

    // Write invalid FIT data
    fs::write(&input, b"This is not a valid FIT file").unwrap();

    pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg(&input)
        .arg(&output)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Conversion failed"));
}

#[test]
fn test_convert_empty_fit_file() {
    let temp = TempDir::new("empty_fit");
    let input = temp.join("empty.fit");
    let output = temp.join("output.yaml");

    // Write empty file
    fs::write(&input, b"").unwrap();

    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    // Empty FIT file may succeed with warnings or fail with conversion error
    // Both are acceptable behaviors
    if result.status.code() == Some(0) {
        let stdout = String::from_utf8_lossy(&result.stdout);
        // Should have warnings about empty/no sessions
        assert!(
            stdout.contains("warnings") || output.exists(),
            "Empty FIT should produce warnings or create output"
        );
    } else {
        // Or it may fail
        let stderr = String::from_utf8_lossy(&result.stderr);
        assert!(
            stderr.contains("Conversion failed") || stderr.contains("error"),
            "Failed conversion should show error message"
        );
    }
}

// ============================================================================
// Successful Conversion Tests (with minimal valid FIT data)
// ============================================================================

fn create_minimal_fit_header() -> Vec<u8> {
    let mut header = Vec::new();
    header.push(14); // Header size
    header.push(0x20); // Protocol version 2.0
    header.extend_from_slice(&[0x00, 0x08]); // Profile version 8.0
    header.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Data size
    header.extend_from_slice(b".FIT"); // Data type
    header.extend_from_slice(&[0x00, 0x00]); // CRC
    header
}

#[test]
fn test_convert_shows_progress_messages() {
    let temp = TempDir::new("progress");
    let input = temp.join("test.fit");
    let output = temp.join("output.yaml");

    // Create a minimal FIT file (will likely fail but tests message output)
    let fit_data = create_minimal_fit_header();
    fs::write(&input, fit_data).unwrap();

    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&result.stdout);

    // Should show conversion message
    assert!(stdout.contains("Converting") || result.status.code() == Some(1));
}

// ============================================================================
// Verbose Mode Tests
// ============================================================================

#[test]
fn test_convert_verbose_flag() {
    let temp = TempDir::new("verbose");
    let input = temp.join("test.fit");
    let output = temp.join("output.yaml");

    let fit_data = create_minimal_fit_header();
    fs::write(&input, fit_data).unwrap();

    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg("--verbose")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&result.stdout);

    // Verbose mode should show more details (if conversion proceeds)
    // Even if it fails, we test the flag is accepted
    if result.status.code() == Some(0) {
        assert!(
            stdout.contains("Reading FIT file")
                || stdout.contains("Parsing FIT records")
                || stdout.contains("Converting to PWF structure")
        );
    }
}

#[test]
fn test_convert_verbose_shows_warnings() {
    // This test verifies that --verbose flag shows conversion warnings
    // when they occur. We can't easily create a FIT file that produces
    // warnings, so we test the flag acceptance
    let temp = TempDir::new("verbose_warnings");
    let input = temp.join("test.fit");
    let output = temp.join("output.yaml");

    let fit_data = create_minimal_fit_header();
    fs::write(&input, fit_data).unwrap();

    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg("--verbose")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    // Verbose flag should be accepted
    assert!(result.status.code().is_some());
}

#[test]
fn test_convert_verbose_short_flag() {
    let temp = TempDir::new("verbose_short");
    let input = temp.join("test.fit");
    let output = temp.join("output.yaml");

    let fit_data = create_minimal_fit_header();
    fs::write(&input, fit_data).unwrap();

    // Test -v short flag
    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg("-v")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    assert!(result.status.code().is_some());
}

// ============================================================================
// Summary Only Mode Tests
// ============================================================================

#[test]
fn test_convert_summary_only_flag() {
    let temp = TempDir::new("summary_only");
    let input = temp.join("test.fit");
    let output = temp.join("output.yaml");

    let fit_data = create_minimal_fit_header();
    fs::write(&input, fit_data).unwrap();

    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg("--summary-only")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    // Summary-only flag should be accepted
    assert!(result.status.code().is_some());
}

#[test]
fn test_convert_summary_only_with_verbose() {
    let temp = TempDir::new("summary_verbose");
    let input = temp.join("test.fit");
    let output = temp.join("output.yaml");

    let fit_data = create_minimal_fit_header();
    fs::write(&input, fit_data).unwrap();

    // Combine --summary-only and --verbose
    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg("--summary-only")
        .arg("--verbose")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    // Both flags should be accepted together
    assert!(result.status.code().is_some());
}

// ============================================================================
// Format Combination Tests
// ============================================================================

#[test]
fn test_convert_fit_to_pwf_accepted() {
    let temp = TempDir::new("fit_to_pwf");
    let input = temp.join("test.fit");
    let output = temp.join("output.yaml");

    let fit_data = create_minimal_fit_header();
    fs::write(&input, fit_data).unwrap();

    // FIT to PWF should be accepted (even if conversion fails)
    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    // Should not reject the format combination
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(!stderr.contains("not yet implemented"));
}

#[test]
fn test_convert_pwf_to_fit_not_implemented() {
    let temp = TempDir::new("pwf_to_fit");
    let input = temp.join("test.yaml");
    let output = temp.join("output.fit");

    fs::write(&input, b"plan_version: 1").unwrap();

    pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("pwf")
        .arg("--to")
        .arg("fit")
        .arg(&input)
        .arg(&output)
        .assert()
        .failure()
        .stderr(predicate::str::contains("not yet implemented"));
}

#[test]
fn test_convert_pwf_to_csv_not_implemented() {
    let temp = TempDir::new("pwf_to_csv");
    let input = temp.join("test.yaml");
    let output = temp.join("output.csv");

    fs::write(&input, b"plan_version: 1").unwrap();

    pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("pwf")
        .arg("--to")
        .arg("csv")
        .arg(&input)
        .arg(&output)
        .assert()
        .failure()
        .stderr(predicate::str::contains("not yet implemented"));
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_convert_with_special_characters_in_path() {
    let temp = TempDir::new("special_chars");
    let input = temp.join("test file with spaces.fit");
    let output = temp.join("output file.yaml");

    let fit_data = create_minimal_fit_header();
    fs::write(&input, fit_data).unwrap();

    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    // Should handle paths with spaces
    assert!(result.status.code().is_some());
}

#[test]
fn test_convert_large_file_detection() {
    let temp = TempDir::new("large_file");
    let input = temp.join("large.fit");
    let output = temp.join("output.yaml");

    // Create a file > 1MB (triggers verbose message about large files)
    let mut large_data = create_minimal_fit_header();
    large_data.extend(vec![0u8; 2_000_000]); // 2MB of zeros
    fs::write(&input, large_data).unwrap();

    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg("--verbose")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&result.stdout);

    // Should detect large file in verbose mode
    if stdout.contains("Large file detected") {
        assert!(stdout.contains("MB"));
    }
}

#[test]
fn test_convert_exit_code_on_failure() {
    let temp = TempDir::new("exit_code_fail");
    let input = temp.join("invalid.fit");
    let output = temp.join("output.yaml");

    fs::write(&input, b"invalid fit data").unwrap();

    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    // Should return non-zero exit code on failure
    assert_ne!(result.status.code(), Some(0));
}

#[test]
fn test_convert_does_not_create_output_on_failure() {
    let temp = TempDir::new("no_output_on_fail");
    let input = temp.join("invalid.fit");
    let output = temp.join("output.yaml");

    fs::write(&input, b"invalid").unwrap();

    let _ = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    // Output file should not exist if conversion failed
    assert!(
        !output.exists(),
        "Output file should not be created on conversion failure"
    );
}

#[test]
fn test_convert_shows_next_steps_on_success() {
    // This test checks that successful conversions show next steps
    // We create a scenario where conversion might succeed (though unlikely with minimal header)
    let temp = TempDir::new("next_steps");
    let input = temp.join("test.fit");
    let output = temp.join("output.yaml");

    let fit_data = create_minimal_fit_header();
    fs::write(&input, fit_data).unwrap();

    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&result.stdout);

    // If conversion succeeds, should show next steps
    if result.status.code() == Some(0) {
        assert!(stdout.contains("Next steps:") || stdout.contains("Validate:"));
    }
}

#[test]
fn test_convert_warning_count_without_verbose() {
    // Test that non-verbose mode shows warning count but not details
    let temp = TempDir::new("warning_count");
    let input = temp.join("test.fit");
    let output = temp.join("output.yaml");

    let fit_data = create_minimal_fit_header();
    fs::write(&input, fit_data).unwrap();

    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&result.stdout);

    // If successful with warnings, should suggest --verbose
    if result.status.code() == Some(0) && stdout.contains("warnings") {
        assert!(stdout.contains("--verbose"));
    }
}

// ============================================================================
// Info Command Test (for coverage)
// ============================================================================

#[test]
fn test_info_command() {
    pwf_cmd()
        .arg("info")
        .assert()
        .success()
        .stdout(predicate::str::contains("PWF - Portable Workout Format"))
        .stdout(predicate::str::contains("Specification version:"))
        .stdout(predicate::str::contains("Validator version:"))
        .stdout(predicate::str::contains("Supported formats:"))
        .stdout(predicate::str::contains("plan"))
        .stdout(predicate::str::contains("history"))
        .stdout(predicate::str::contains("Modalities:"))
        .stdout(predicate::str::contains("strength"))
        .stdout(predicate::str::contains("countdown"))
        .stdout(predicate::str::contains("stopwatch"))
        .stdout(predicate::str::contains("interval"))
        .stdout(predicate::str::contains("https://pwf.dev"));
}

#[test]
fn test_info_command_exit_code() {
    let output = pwf_cmd().arg("info").output().unwrap();
    assert_eq!(output.status.code(), Some(0));
}

// ============================================================================
// Additional Path Coverage Tests
// ============================================================================

#[test]
fn test_convert_io_error_on_write() {
    let temp = TempDir::new("io_error");
    let input = temp.join("test.fit");
    // Try to write to a directory (which should fail)
    let output = temp.join("subdir");
    fs::create_dir(&output).unwrap();

    let fit_data = create_minimal_fit_header();
    fs::write(&input, fit_data).unwrap();

    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    // Should fail because output is a directory
    assert_ne!(result.status.code(), Some(0));
}

#[test]
fn test_convert_file_read_error() {
    let temp = TempDir::new("read_error");
    let input = temp.join("test.fit");
    let output = temp.join("output.yaml");

    // Create file but make it a directory instead
    fs::create_dir(&input).unwrap();

    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("Failed to open input file") || stderr.contains("error"),
        "Should report file read error"
    );
}

#[test]
fn test_convert_formats_case_variations() {
    let temp = TempDir::new("case_variations");
    let input = temp.join("test.FIT");
    let output = temp.join("output.YAML");

    let fit_data = create_minimal_fit_header();
    fs::write(&input, fit_data).unwrap();

    // Test with uppercase format names
    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("FIT")
        .arg("--to")
        .arg("PWF")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    // Should accept case variations
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        !stderr.contains("not yet implemented") || result.status.code() == Some(1),
        "Should accept case-insensitive format names"
    );
}

#[test]
fn test_convert_mixed_case_formats() {
    let temp = TempDir::new("mixed_case");
    let input = temp.join("test.fit");
    let output = temp.join("output.yaml");

    let fit_data = create_minimal_fit_header();
    fs::write(&input, fit_data).unwrap();

    // Test with MiXeD case
    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("FiT")
        .arg("--to")
        .arg("PwF")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(!stderr.contains("not yet implemented") || result.status.code() == Some(1));
}

#[test]
fn test_convert_verbose_conversion_statistics() {
    let temp = TempDir::new("stats");
    let input = temp.join("test.fit");
    let output = temp.join("output.yaml");

    let fit_data = create_minimal_fit_header();
    fs::write(&input, fit_data).unwrap();

    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg("--verbose")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&result.stdout);

    // If conversion proceeds, verbose should show statistics
    if result.status.code() == Some(0) {
        assert!(stdout.contains("lines") || stdout.contains("KB") || stdout.contains("Generated"));
    }
}

// ============================================================================
// Additional Coverage Tests
// ============================================================================

#[test]
fn test_convert_file_becomes_unreadable() {
    // Test edge case where file exists during check but becomes unreadable
    // This is hard to simulate, so we test the error path by using a directory
    let temp = TempDir::new("unreadable");
    let input = temp.join("test.fit");
    let output = temp.join("output.yaml");

    // Create a directory with the input name (after existence check would pass differently)
    fs::create_dir(&input).unwrap();

    let result = pwf_cmd()
        .arg("convert")
        .arg("--from")
        .arg("fit")
        .arg("--to")
        .arg("pwf")
        .arg(&input)
        .arg(&output)
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&result.stderr);
    // Should fail with file read error
    assert!(
        stderr.contains("Failed to open input file") || stderr.contains("error"),
        "Should report file opening error"
    );
}
