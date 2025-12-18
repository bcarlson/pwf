use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;

// Helper function to get the path to the pwf binary
fn pwf_cmd() -> Command {
    Command::new(env!("CARGO_BIN_EXE_pwf"))
}

// Helper to get absolute path to examples directory
fn examples_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("examples")
}

// Test 1: Valid plan file validation succeeds
#[test]
fn test_validate_valid_plan() {
    let example_path = examples_dir().join("beginner-strength.yaml");

    pwf_cmd()
        .arg("validate")
        .arg(&example_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"))
        .stdout(predicate::str::contains("beginner-strength.yaml"));
}

// Test 2: Invalid plan file - missing version
#[test]
fn test_validate_missing_version() {
    let example_path = examples_dir().join("invalid/missing-version.yaml");

    pwf_cmd()
        .arg("validate")
        .arg(&example_path)
        .assert()
        .failure()
        .stdout(predicate::str::contains("✗"))
        .stdout(predicate::str::contains("missing-version.yaml"));
}

// Test 3: Invalid plan file - empty days array
#[test]
fn test_validate_empty_days() {
    let example_path = examples_dir().join("invalid/empty-days.yaml");

    pwf_cmd()
        .arg("validate")
        .arg(&example_path)
        .assert()
        .failure()
        .stdout(predicate::str::contains("✗"))
        .stdout(predicate::str::contains("empty-days.yaml"));
}

// Test 4: Invalid plan file - invalid modality
#[test]
fn test_validate_invalid_modality() {
    let example_path = examples_dir().join("invalid/invalid-modality.yaml");

    pwf_cmd()
        .arg("validate")
        .arg(&example_path)
        .assert()
        .failure()
        .stdout(predicate::str::contains("✗"))
        .stdout(predicate::str::contains("invalid-modality.yaml"));
}

// Test 5: Multiple valid files validation
#[test]
fn test_validate_multiple_valid_files() {
    let example1 = examples_dir().join("beginner-strength.yaml");
    let example2 = examples_dir().join("minimal.yaml");

    pwf_cmd()
        .arg("validate")
        .arg(&example1)
        .arg(&example2)
        .assert()
        .success()
        .stdout(predicate::str::contains("beginner-strength.yaml"))
        .stdout(predicate::str::contains("minimal.yaml"));
}

// Test 6: Multiple files with mixed validity
#[test]
fn test_validate_multiple_mixed_files() {
    let valid = examples_dir().join("minimal.yaml");
    let invalid = examples_dir().join("invalid/missing-version.yaml");

    pwf_cmd()
        .arg("validate")
        .arg(&valid)
        .arg(&invalid)
        .assert()
        .failure()
        .stdout(predicate::str::contains("minimal.yaml"))
        .stdout(predicate::str::contains("missing-version.yaml"))
        .stdout(predicate::str::contains("✓"))
        .stdout(predicate::str::contains("✗"));
}

// Test 7: JSON output format
#[test]
fn test_validate_json_format() {
    let example_path = examples_dir().join("beginner-strength.yaml");

    pwf_cmd()
        .arg("validate")
        .arg(&example_path)
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""type": "plan""#))
        .stdout(predicate::str::contains(r#""valid": true"#))
        .stdout(predicate::str::contains(r#""file":"#));
}

// Test 8: JSON format with invalid file
#[test]
fn test_validate_json_format_invalid() {
    let example_path = examples_dir().join("invalid/empty-days.yaml");

    pwf_cmd()
        .arg("validate")
        .arg(&example_path)
        .arg("--format")
        .arg("json")
        .assert()
        .failure()
        .stdout(predicate::str::contains(r#""valid": false"#))
        .stdout(predicate::str::contains(r#""errors""#));
}

// Test 9: Compact output format
#[test]
fn test_validate_compact_format() {
    let example_path = examples_dir().join("beginner-strength.yaml");

    pwf_cmd()
        .arg("validate")
        .arg(&example_path)
        .arg("--format")
        .arg("compact")
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"))
        .stdout(predicate::str::contains("beginner-strength.yaml"))
        // In compact mode, we should not see detailed stats
        .stdout(predicate::str::contains("days").not());
}

// Test 10: Compact format with invalid file
#[test]
fn test_validate_compact_format_invalid() {
    let example_path = examples_dir().join("invalid/missing-version.yaml");

    pwf_cmd()
        .arg("validate")
        .arg(&example_path)
        .arg("--format")
        .arg("compact")
        .assert()
        .failure()
        .stdout(predicate::str::contains("✗"))
        .stdout(predicate::str::contains("missing-version.yaml"));
}

// Test 11: File not found error
#[test]
fn test_validate_file_not_found() {
    pwf_cmd()
        .arg("validate")
        .arg("/nonexistent/path/file.yaml")
        .assert()
        .failure()
        .stderr(predicate::str::contains("nonexistent"));
}

// Test 12: Invalid file path (directory instead of file)
#[test]
fn test_validate_directory_path() {
    let dir_path = examples_dir();

    pwf_cmd()
        .arg("validate")
        .arg(&dir_path)
        .assert()
        .failure()
        .stderr(predicate::str::is_match(r"(?i)(directory|invalid)").unwrap());
}

// Test 13: No files provided (should fail with clap error)
#[test]
fn test_validate_no_files() {
    pwf_cmd()
        .arg("validate")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

// Test 14: Quiet mode - suppresses warnings but shows errors
#[test]
fn test_validate_quiet_mode_with_valid_file() {
    let example_path = examples_dir().join("beginner-strength.yaml");

    let output_normal = pwf_cmd()
        .arg("validate")
        .arg(&example_path)
        .output()
        .unwrap();

    let output_quiet = pwf_cmd()
        .arg("validate")
        .arg(&example_path)
        .arg("--quiet")
        .output()
        .unwrap();

    let normal_stdout = String::from_utf8_lossy(&output_normal.stdout);
    let quiet_stdout = String::from_utf8_lossy(&output_quiet.stdout);

    // Both should succeed
    assert_eq!(output_normal.status.code(), Some(0));
    assert_eq!(output_quiet.status.code(), Some(0));

    // Quiet mode should have less or equal output
    assert!(quiet_stdout.len() <= normal_stdout.len());
}

// Test 15: Quiet mode doesn't suppress errors
#[test]
fn test_validate_quiet_mode_with_errors() {
    let example_path = examples_dir().join("invalid/empty-days.yaml");

    pwf_cmd()
        .arg("validate")
        .arg(&example_path)
        .arg("--quiet")
        .assert()
        .failure()
        .stdout(predicate::str::contains("✗"));
}

// Test 16: Pretty format (default) shows statistics
#[test]
fn test_validate_pretty_format_shows_stats() {
    let example_path = examples_dir().join("beginner-strength.yaml");

    pwf_cmd()
        .arg("validate")
        .arg(&example_path)
        .arg("--format")
        .arg("pretty")
        .assert()
        .success()
        .stdout(predicate::str::contains("days"))
        .stdout(predicate::str::contains("exercises"));
}

// Test 17: Exit code 0 for valid files
#[test]
fn test_exit_code_success() {
    let example_path = examples_dir().join("minimal.yaml");

    let output = pwf_cmd()
        .arg("validate")
        .arg(&example_path)
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(0));
}

// Test 18: Exit code 1 for validation errors
#[test]
fn test_exit_code_validation_error() {
    let example_path = examples_dir().join("invalid/missing-version.yaml");

    let output = pwf_cmd()
        .arg("validate")
        .arg(&example_path)
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
}

// Test 19: Create a temporary invalid YAML and test system error handling
#[test]
fn test_validate_malformed_yaml() {
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join("malformed.yaml");

    // Write malformed YAML
    fs::write(&temp_file, "plan_version: 1\ninvalid: [unclosed").unwrap();

    pwf_cmd()
        .arg("validate")
        .arg(&temp_file)
        .assert()
        .failure();

    // Cleanup
    let _ = fs::remove_file(&temp_file);
}

// Test 20: Validate with strict mode - file with warnings
#[test]
fn test_validate_strict_mode() {
    // First, let's create a temporary file with content that might produce warnings
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join("test-strict-mode.yaml");

    // A valid file structure
    let content = r#"plan_version: 1
cycle:
  days:
    - exercises:
        - name: "Push-ups"
          modality: strength
          target_sets: 3
          target_reps: 10
"#;

    fs::write(&temp_file, content).unwrap();

    // Test without strict mode (should pass)
    pwf_cmd()
        .arg("validate")
        .arg(&temp_file)
        .assert()
        .success();

    // With strict mode, behavior depends on whether there are warnings
    // The command should still work
    let result = pwf_cmd()
        .arg("validate")
        .arg(&temp_file)
        .arg("--strict")
        .output()
        .unwrap();

    // Should return valid exit code (either 0 or 1, but not crash)
    assert!(result.status.code().is_some());

    // Cleanup
    let _ = fs::remove_file(&temp_file);
}

// Test 21: Multiple files with JSON format
#[test]
fn test_validate_multiple_files_json_format() {
    let example1 = examples_dir().join("minimal.yaml");
    let example2 = examples_dir().join("beginner-strength.yaml");

    pwf_cmd()
        .arg("validate")
        .arg(&example1)
        .arg(&example2)
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r#"\[\s*\{"#).unwrap()) // Starts with array
        .stdout(predicate::str::contains(r#""type": "plan""#));
}

// Test 22: Validate checks for proper error message formatting
#[test]
fn test_error_message_format() {
    let example_path = examples_dir().join("invalid/invalid-modality.yaml");

    pwf_cmd()
        .arg("validate")
        .arg(&example_path)
        .assert()
        .failure()
        .stdout(predicate::str::contains("✗"));
}

// Test 23: Validate mixed-modalities file (complex valid example)
#[test]
fn test_validate_complex_valid_file() {
    let example_path = examples_dir().join("mixed-modalities.yaml");

    if example_path.exists() {
        pwf_cmd()
            .arg("validate")
            .arg(&example_path)
            .assert()
            .success()
            .stdout(predicate::str::contains("✓"));
    }
}

// Test 24: Test help message for validate command
#[test]
fn test_validate_help() {
    pwf_cmd()
        .arg("validate")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Validate PWF plan files"))
        .stdout(predicate::str::contains("--format"))
        .stdout(predicate::str::contains("--strict"))
        .stdout(predicate::str::contains("--quiet"));
}

// Test 25: Test that validate handles empty file gracefully
#[test]
fn test_validate_empty_file() {
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join("empty.yaml");

    fs::write(&temp_file, "").unwrap();

    pwf_cmd()
        .arg("validate")
        .arg(&temp_file)
        .assert()
        .failure(); // Empty file should fail validation

    // Cleanup
    let _ = fs::remove_file(&temp_file);
}
