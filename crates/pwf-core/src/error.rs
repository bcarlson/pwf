//! Error types for PWF parsing and validation

use thiserror::Error;

/// Errors that can occur during YAML parsing
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("YAML syntax error: {0}")]
    YamlSyntax(#[from] serde_yaml::Error),

    #[error("Missing required field: {field}")]
    MissingField { field: String },

    #[error("Invalid value for {field}: {message}")]
    InvalidValue { field: String, message: String },
}

/// Severity level for validation issues
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
}

/// A single validation issue
#[derive(Debug, Clone, serde::Serialize)]
pub struct ValidationIssue {
    pub path: String,
    pub message: String,
    pub severity: Severity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl ValidationIssue {
    pub fn error(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
            severity: Severity::Error,
            code: None,
        }
    }

    pub fn warning(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
            severity: Severity::Warning,
            code: None,
        }
    }

    pub fn error_with_code(
        path: impl Into<String>,
        message: impl Into<String>,
        code: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
            severity: Severity::Error,
            code: Some(code.into()),
        }
    }

    pub fn warning_with_code(
        path: impl Into<String>,
        message: impl Into<String>,
        code: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
            severity: Severity::Warning,
            code: Some(code.into()),
        }
    }
}

/// Combined error type for PWF operations
#[derive(Debug, Error)]
pub enum WpsError {
    #[error(transparent)]
    Parse(#[from] ParseError),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== ValidationIssue Constructor Tests =====

    #[test]
    fn test_validation_issue_error() {
        let issue = ValidationIssue::error("exercises[0].name", "Name is required");

        assert_eq!(issue.path, "exercises[0].name");
        assert_eq!(issue.message, "Name is required");
        assert_eq!(issue.severity, Severity::Error);
        assert!(issue.code.is_none());
    }

    #[test]
    fn test_validation_issue_warning() {
        let issue = ValidationIssue::warning("workouts[1].sets", "Sets should be greater than 0");

        assert_eq!(issue.path, "workouts[1].sets");
        assert_eq!(issue.message, "Sets should be greater than 0");
        assert_eq!(issue.severity, Severity::Warning);
        assert!(issue.code.is_none());
    }

    #[test]
    fn test_validation_issue_error_with_code() {
        let issue =
            ValidationIssue::error_with_code("plan.start_date", "Invalid date format", "E001");

        assert_eq!(issue.path, "plan.start_date");
        assert_eq!(issue.message, "Invalid date format");
        assert_eq!(issue.severity, Severity::Error);
        assert_eq!(issue.code, Some("E001".to_string()));
    }

    #[test]
    fn test_validation_issue_warning_with_code() {
        let issue = ValidationIssue::warning_with_code(
            "exercises[2].rest",
            "Rest period is longer than recommended",
            "W042",
        );

        assert_eq!(issue.path, "exercises[2].rest");
        assert_eq!(issue.message, "Rest period is longer than recommended");
        assert_eq!(issue.severity, Severity::Warning);
        assert_eq!(issue.code, Some("W042".to_string()));
    }

    #[test]
    fn test_validation_issue_accepts_string_types() {
        // Test that Into<String> works with different types
        let owned = ValidationIssue::error(String::from("path"), String::from("message"));
        assert_eq!(owned.path, "path");

        let str_ref = ValidationIssue::error("path", "message");
        assert_eq!(str_ref.path, "path");
    }

    // ===== Severity Tests =====

    #[test]
    fn test_severity_equality() {
        assert_eq!(Severity::Error, Severity::Error);
        assert_eq!(Severity::Warning, Severity::Warning);
        assert_ne!(Severity::Error, Severity::Warning);
    }

    #[test]
    fn test_severity_clone() {
        let error = Severity::Error;
        let cloned = error;
        assert_eq!(error, cloned);
    }

    #[test]
    fn test_severity_serialization() {
        let error_json = serde_json::to_string(&Severity::Error).unwrap();
        assert_eq!(error_json, r#""error""#);

        let warning_json = serde_json::to_string(&Severity::Warning).unwrap();
        assert_eq!(warning_json, r#""warning""#);
    }

    // ===== ParseError Tests =====

    #[test]
    fn test_parse_error_yaml_syntax() {
        let yaml_error = serde_yaml::from_str::<serde_yaml::Value>("{ invalid yaml").unwrap_err();
        let parse_error = ParseError::from(yaml_error);

        let error_msg = format!("{}", parse_error);
        assert!(error_msg.starts_with("YAML syntax error:"));
    }

    #[test]
    fn test_parse_error_missing_field() {
        let error = ParseError::MissingField {
            field: "name".to_string(),
        };

        assert_eq!(format!("{}", error), "Missing required field: name");
    }

    #[test]
    fn test_parse_error_invalid_value() {
        let error = ParseError::InvalidValue {
            field: "reps".to_string(),
            message: "must be positive".to_string(),
        };

        assert_eq!(
            format!("{}", error),
            "Invalid value for reps: must be positive"
        );
    }

    #[test]
    fn test_parse_error_debug() {
        let error = ParseError::MissingField {
            field: "test".to_string(),
        };

        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("MissingField"));
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_parse_error_is_error_trait() {
        let error = ParseError::MissingField {
            field: "test".to_string(),
        };

        // Should implement std::error::Error
        let _error_ref: &dyn std::error::Error = &error;
    }

    // ===== WpsError Tests =====

    #[test]
    fn test_wps_error_from_parse_error() {
        let parse_error = ParseError::MissingField {
            field: "version".to_string(),
        };
        let wps_error: WpsError = parse_error.into();

        match wps_error {
            WpsError::Parse(ParseError::MissingField { field }) => {
                assert_eq!(field, "version");
            }
            _ => panic!("Expected Parse variant"),
        }
    }

    #[test]
    fn test_wps_error_validation() {
        let error = WpsError::Validation("Invalid workout plan".to_string());

        assert_eq!(
            format!("{}", error),
            "Validation failed: Invalid workout plan"
        );
    }

    #[test]
    fn test_wps_error_from_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let wps_error: WpsError = io_error.into();

        match wps_error {
            WpsError::Io(_) => {
                let msg = format!("{}", wps_error);
                assert!(msg.starts_with("IO error:"));
                assert!(msg.contains("file not found"));
            }
            _ => panic!("Expected Io variant"),
        }
    }

    #[test]
    fn test_wps_error_display() {
        let parse_error = WpsError::Parse(ParseError::MissingField {
            field: "name".to_string(),
        });
        assert_eq!(format!("{}", parse_error), "Missing required field: name");

        let validation_error = WpsError::Validation("test validation".to_string());
        assert_eq!(
            format!("{}", validation_error),
            "Validation failed: test validation"
        );
    }

    #[test]
    fn test_wps_error_debug() {
        let error = WpsError::Validation("test".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("Validation"));
    }

    // ===== JSON Serialization Tests =====

    #[test]
    fn test_validation_issue_json_with_code() {
        let issue = ValidationIssue::error_with_code(
            "exercises[0].sets",
            "Sets must be positive",
            "ERR_NEGATIVE_SETS",
        );

        let json = serde_json::to_value(&issue).unwrap();

        assert_eq!(json["path"], "exercises[0].sets");
        assert_eq!(json["message"], "Sets must be positive");
        assert_eq!(json["severity"], "error");
        assert_eq!(json["code"], "ERR_NEGATIVE_SETS");
    }

    #[test]
    fn test_validation_issue_json_without_code() {
        let issue = ValidationIssue::warning("workouts[0]", "Low intensity");

        let json = serde_json::to_value(&issue).unwrap();

        assert_eq!(json["path"], "workouts[0]");
        assert_eq!(json["message"], "Low intensity");
        assert_eq!(json["severity"], "warning");
        // Code should not be present in JSON when None
        assert!(json.get("code").is_none());
    }

    #[test]
    fn test_validation_issue_array_serialization() {
        let issues = vec![
            ValidationIssue::error("path1", "error message"),
            ValidationIssue::warning_with_code("path2", "warning message", "W001"),
            ValidationIssue::error_with_code("path3", "critical error", "E500"),
        ];

        let json = serde_json::to_string(&issues).unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.len(), 3);

        // First issue - error without code
        assert_eq!(parsed[0]["severity"], "error");
        assert!(parsed[0].get("code").is_none());

        // Second issue - warning with code
        assert_eq!(parsed[1]["severity"], "warning");
        assert_eq!(parsed[1]["code"], "W001");

        // Third issue - error with code
        assert_eq!(parsed[2]["severity"], "error");
        assert_eq!(parsed[2]["code"], "E500");
    }

    #[test]
    fn test_validation_issue_json_formatting() {
        let issue = ValidationIssue::error_with_code("test.path", "test message", "TEST001");

        let json = serde_json::to_string_pretty(&issue).unwrap();

        assert!(json.contains("\"path\""));
        assert!(json.contains("\"message\""));
        assert!(json.contains("\"severity\""));
        assert!(json.contains("\"code\""));
        assert!(json.contains("test.path"));
        assert!(json.contains("test message"));
        assert!(json.contains("error"));
        assert!(json.contains("TEST001"));
    }

    // ===== Additional Coverage Tests =====

    #[test]
    fn test_validation_issue_clone() {
        let original = ValidationIssue::error_with_code("path", "message", "CODE");
        let cloned = original.clone();

        assert_eq!(original.path, cloned.path);
        assert_eq!(original.message, cloned.message);
        assert_eq!(original.severity, cloned.severity);
        assert_eq!(original.code, cloned.code);
    }

    #[test]
    fn test_empty_strings_accepted() {
        let issue = ValidationIssue::error("", "");
        assert_eq!(issue.path, "");
        assert_eq!(issue.message, "");
    }
}
