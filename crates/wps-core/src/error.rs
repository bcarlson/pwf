//! Error types for WPS parsing and validation

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
}

impl ValidationIssue {
    pub fn error(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
            severity: Severity::Error,
        }
    }

    pub fn warning(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
            severity: Severity::Warning,
        }
    }
}

/// Combined error type for WPS operations
#[derive(Debug, Error)]
pub enum WpsError {
    #[error(transparent)]
    Parse(#[from] ParseError),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
