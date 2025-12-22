//! Integration tests for CLI history and init commands
//!
//! Tests cover:
//! - History validation with various formats
//! - History validation with strict mode
//! - Statistics output verification
//! - Init command for plan and history templates
//! - File overwrite protection
//! - Error handling and exit codes

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;

/// Helper to get the binary command
#[allow(deprecated)]
fn pwf() -> Command {
    Command::cargo_bin("pwf").unwrap()
}

/// Helper to get test fixture path
fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples")
        .join(name)
}

/// Helper to create a temporary file for init tests
struct TempFile {
    path: PathBuf,
}

impl TempFile {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("pwf_test_{}", name));
        // Clean up if it exists from previous run
        let _ = fs::remove_file(&path);
        TempFile { path }
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

// ============================================================================
// History Command Tests
// ============================================================================

#[test]
fn test_history_valid_minimal() {
    pwf()
        .arg("history")
        .arg(fixture_path("history-minimal.yaml"))
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"))
        .stdout(predicate::str::contains("history-minimal.yaml"));
}

#[test]
fn test_history_valid_complete() {
    pwf()
        .arg("history")
        .arg(fixture_path("history-export.yaml"))
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"))
        .stdout(predicate::str::contains("history-export.yaml"));
}

#[test]
fn test_history_statistics_output() {
    pwf()
        .arg("history")
        .arg(fixture_path("history-export.yaml"))
        .assert()
        .success()
        .stdout(predicate::str::contains("workouts"))
        .stdout(predicate::str::contains("sets"))
        .stdout(predicate::str::contains("volume"));
}

#[test]
fn test_history_format_json() {
    let output = pwf()
        .arg("history")
        .arg("--format")
        .arg("json")
        .arg(fixture_path("history-minimal.yaml"))
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json_str = String::from_utf8(output).unwrap();
    let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    // Verify JSON structure
    assert!(json.is_array());
    let first = &json[0];
    assert_eq!(first["type"], "history");
    assert!(first["valid"].as_bool().unwrap());
    assert!(first.get("statistics").is_some());
    assert!(first.get("file").is_some());
    assert!(first.get("errors").is_some());
    assert!(first.get("warnings").is_some());
}

#[test]
fn test_history_format_compact() {
    pwf()
        .arg("history")
        .arg("--format")
        .arg("compact")
        .arg(fixture_path("history-minimal.yaml"))
        .assert()
        .success()
        .stdout(predicate::str::contains("✓ "))
        .stdout(predicate::str::contains("history-minimal.yaml"));
}

#[test]
fn test_history_format_pretty() {
    pwf()
        .arg("history")
        .arg("--format")
        .arg("pretty")
        .arg(fixture_path("history-export.yaml"))
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"))
        .stdout(predicate::str::contains("workouts"))
        .stdout(predicate::str::contains("Date range:"));
}

#[test]
fn test_history_invalid_rpe() {
    pwf()
        .arg("history")
        .arg(fixture_path(
            "invalid/invalid-history-rpe-out-of-range.yaml",
        ))
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"))
        .stdout(predicate::str::contains("⚠"))
        .stdout(predicate::str::contains("RPE"));
}

#[test]
fn test_history_invalid_rpe_strict_mode() {
    // In strict mode, files with warnings are marked as invalid
    pwf()
        .arg("history")
        .arg("--strict")
        .arg(fixture_path(
            "invalid/invalid-history-rpe-out-of-range.yaml",
        ))
        .assert()
        .failure()
        .stdout(predicate::str::contains("✗"));
}

#[test]
fn test_history_missing_version() {
    pwf()
        .arg("history")
        .arg(fixture_path("invalid/missing-version.yaml"))
        .assert()
        .failure()
        .stdout(predicate::str::contains("✗"));
}

#[test]
fn test_history_nonexistent_file() {
    pwf()
        .arg("history")
        .arg("nonexistent-file.yaml")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
}

#[test]
fn test_history_multiple_files() {
    pwf()
        .arg("history")
        .arg(fixture_path("history-minimal.yaml"))
        .arg(fixture_path("history-export.yaml"))
        .assert()
        .success()
        .stdout(predicate::str::contains("history-minimal.yaml"))
        .stdout(predicate::str::contains("history-export.yaml"));
}

#[test]
fn test_history_multiple_files_with_failures() {
    pwf()
        .arg("history")
        .arg(fixture_path("history-minimal.yaml"))
        .arg(fixture_path("invalid/missing-version.yaml"))
        .assert()
        .failure()
        .stdout(predicate::str::contains("✓").and(predicate::str::contains("✗")));
}

// ============================================================================
// Init Command Tests
// ============================================================================

#[test]
fn test_init_creates_plan_default() {
    let temp = TempFile::new("init_plan_default.yaml");

    pwf()
        .arg("init")
        .arg(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Created"))
        .stdout(predicate::str::contains("Next steps"));

    // Verify file was created
    assert!(temp.path().exists());

    // Verify content is valid YAML and contains plan structure
    let content = fs::read_to_string(temp.path()).unwrap();
    assert!(content.contains("plan_version: 1"));
    assert!(content.contains("meta:"));
    assert!(content.contains("cycle:"));
    assert!(content.contains("days:"));
    assert!(content.contains("exercises:"));
}

#[test]
fn test_init_creates_history_template() {
    let temp = TempFile::new("init_history.yaml");

    pwf()
        .arg("init")
        .arg("--history")
        .arg(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Created"));

    // Verify file was created
    assert!(temp.path().exists());

    // Verify content is valid YAML and contains history structure
    let content = fs::read_to_string(temp.path()).unwrap();
    assert!(content.contains("history_version: 1"));
    assert!(content.contains("exported_at:"));
    assert!(content.contains("export_source:"));
    assert!(content.contains("workouts:"));
    assert!(content.contains("personal_records:"));
    assert!(content.contains("body_measurements:"));
}

#[test]
fn test_init_validates_created_plan() {
    let temp = TempFile::new("init_validate_plan.yaml");

    // Create the file
    pwf().arg("init").arg(temp.path()).assert().success();

    // Validate it
    pwf()
        .arg("validate")
        .arg(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_init_validates_created_history() {
    let temp = TempFile::new("init_validate_history.yaml");

    // Create the file
    pwf()
        .arg("init")
        .arg("--history")
        .arg(temp.path())
        .assert()
        .success();

    // Validate it
    pwf()
        .arg("history")
        .arg(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("✓"));
}

#[test]
fn test_init_refuses_overwrite() {
    let temp = TempFile::new("init_no_overwrite.yaml");

    // Create the file first
    pwf().arg("init").arg(temp.path()).assert().success();

    // Try to create again - should fail
    pwf()
        .arg("init")
        .arg(temp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn test_init_refuses_overwrite_history() {
    let temp = TempFile::new("init_no_overwrite_history.yaml");

    // Create the file first
    pwf()
        .arg("init")
        .arg("--history")
        .arg(temp.path())
        .assert()
        .success();

    // Try to create again - should fail
    pwf()
        .arg("init")
        .arg("--history")
        .arg(temp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn test_init_invalid_directory() {
    pwf()
        .arg("init")
        .arg("/nonexistent/directory/file.yaml")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error"));
}

#[test]
fn test_init_default_filename() {
    // Using a unique subdirectory to avoid conflicts
    let temp_dir = std::env::temp_dir().join(format!("pwf_test_init_{}", std::process::id()));
    fs::create_dir_all(&temp_dir).unwrap();
    let default_path = temp_dir.join("plan.yaml");

    // Clean up if exists
    let _ = fs::remove_file(&default_path);

    pwf()
        .arg("init")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("plan.yaml"));

    // Verify file was created
    assert!(default_path.exists());

    // Clean up
    let _ = fs::remove_file(&default_path);
    let _ = fs::remove_dir(&temp_dir);
}

// ============================================================================
// History JSON Output Structure Tests
// ============================================================================

#[test]
fn test_history_json_statistics_fields() {
    let output = pwf()
        .arg("history")
        .arg("--format")
        .arg("json")
        .arg(fixture_path("history-export.yaml"))
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json_str = String::from_utf8(output).unwrap();
    let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    let first = &json[0];
    let stats = first["statistics"].as_object().unwrap();

    // Verify statistics fields exist
    assert!(stats.contains_key("total_workouts"));
    assert!(stats.contains_key("total_sets"));
    assert!(stats.contains_key("total_volume_kg"));
    assert!(stats.contains_key("date_range_start"));
    assert!(stats.contains_key("date_range_end"));

    // Verify statistics values are reasonable
    let total_workouts = stats["total_workouts"].as_u64().unwrap();
    assert!(total_workouts > 0);

    let total_sets = stats["total_sets"].as_u64().unwrap();
    assert!(total_sets > 0);

    let total_volume = stats["total_volume_kg"].as_f64().unwrap();
    assert!(total_volume > 0.0);
}

#[test]
fn test_history_json_with_warnings() {
    let output = pwf()
        .arg("history")
        .arg("--format")
        .arg("json")
        .arg(fixture_path(
            "invalid/invalid-history-rpe-out-of-range.yaml",
        ))
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json_str = String::from_utf8(output).unwrap();
    let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    let first = &json[0];
    assert!(first["valid"].as_bool().unwrap());

    let warnings = first["warnings"].as_array().unwrap();
    assert!(!warnings.is_empty());

    // Verify warning structure
    let warning = &warnings[0];
    assert!(warning.get("path").is_some());
    assert!(warning.get("message").is_some());
    assert!(warning.get("code").is_some());
}

#[test]
fn test_history_json_strict_mode_marks_invalid() {
    let output = pwf()
        .arg("history")
        .arg("--format")
        .arg("json")
        .arg("--strict")
        .arg(fixture_path(
            "invalid/invalid-history-rpe-out-of-range.yaml",
        ))
        .assert()
        .failure()
        .get_output()
        .stdout
        .clone();

    let json_str = String::from_utf8(output).unwrap();
    let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    let first = &json[0];
    // In strict mode, files with warnings should be marked as invalid
    assert!(!first["valid"].as_bool().unwrap());

    let warnings = first["warnings"].as_array().unwrap();
    assert!(!warnings.is_empty());
}

#[test]
fn test_history_compact_format_with_invalid_file() {
    pwf()
        .arg("history")
        .arg("--format")
        .arg("compact")
        .arg(fixture_path("invalid/missing-version.yaml"))
        .assert()
        .failure()
        .stdout(predicate::str::contains("✗"))
        .stdout(predicate::str::contains("missing-version.yaml"));
}

#[test]
fn test_history_pretty_format_with_invalid_file() {
    pwf()
        .arg("history")
        .arg("--format")
        .arg("pretty")
        .arg(fixture_path("invalid/missing-version.yaml"))
        .assert()
        .failure()
        .stdout(predicate::str::contains("✗"))
        .stdout(predicate::str::contains("missing-version.yaml"));
}
