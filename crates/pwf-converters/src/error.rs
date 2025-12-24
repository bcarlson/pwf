//! Error types for PWF format conversions

use std::fmt;
use thiserror::Error;

/// Errors that can occur during format conversion
#[derive(Debug, Error)]
pub enum ConversionError {
    /// Failed to read or parse FIT file
    #[error("Failed to read FIT file: {0}")]
    FitReadError(#[from] fitparser::Error),

    /// Invalid or inconsistent data in FIT file
    #[error("Invalid FIT data: {0}")]
    InvalidFitData(String),

    /// Failed to read or parse TCX file
    #[error("Failed to read TCX file: {0}")]
    TcxReadError(String),

    /// Invalid or inconsistent data in TCX file
    #[error("Invalid TCX data: {0}")]
    InvalidTcxData(String),

    /// Failed to write or serialize TCX file
    #[error("Failed to write TCX file: {0}")]
    TcxWriteError(String),

    /// Failed to read or parse GPX file
    #[error("Failed to read GPX file: {0}")]
    GpxReadError(String),

    /// Invalid or inconsistent data in GPX file
    #[error("Invalid GPX data: {0}")]
    InvalidGpxData(String),

    /// Failed to write or serialize GPX file
    #[error("Failed to write GPX file: {0}")]
    GpxWriteError(String),

    /// PWF validation failed after conversion
    #[error("PWF validation failed: {0}")]
    PwfValidationError(String),

    /// I/O error during file operations
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// YAML serialization error
    #[error("YAML serialization error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    /// Unsupported file format
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    /// Missing required data in source file
    #[error("Missing required field: {0}")]
    MissingRequiredField(String),

    /// Failed to write or generate CSV file
    #[error("Failed to write CSV file: {0}")]
    CsvWriteError(String),
}

/// Result type for conversions with warnings
#[derive(Debug, serde::Serialize)]
pub struct ConversionResult {
    /// The converted PWF YAML content
    pub pwf_yaml: String,

    /// List of warnings about data loss or quality issues
    pub warnings: Vec<ConversionWarning>,
}

impl ConversionResult {
    /// Create a new conversion result
    pub fn new(pwf_yaml: String) -> Self {
        Self {
            pwf_yaml,
            warnings: Vec::new(),
        }
    }

    /// Add a warning to the result
    pub fn add_warning(&mut self, warning: ConversionWarning) {
        self.warnings.push(warning);
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

/// Result type for TCX export with warnings
#[derive(Debug, serde::Serialize)]
pub struct TcxExportResult {
    /// The generated TCX XML content
    pub tcx_xml: String,

    /// List of warnings about data loss or quality issues
    pub warnings: Vec<ConversionWarning>,
}

impl TcxExportResult {
    /// Create a new TCX export result
    pub fn new(tcx_xml: String) -> Self {
        Self {
            tcx_xml,
            warnings: Vec::new(),
        }
    }

    /// Add a warning to the result
    pub fn add_warning(&mut self, warning: ConversionWarning) {
        self.warnings.push(warning);
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

/// Result type for GPX export with warnings
#[derive(Debug, serde::Serialize)]
pub struct GpxExportResult {
    /// The generated GPX XML content
    pub gpx_xml: String,

    /// List of warnings about data loss or quality issues
    pub warnings: Vec<ConversionWarning>,
}

impl GpxExportResult {
    /// Create a new GPX export result
    pub fn new(gpx_xml: String) -> Self {
        Self {
            gpx_xml,
            warnings: Vec::new(),
        }
    }

    /// Add a warning to the result
    pub fn add_warning(&mut self, warning: ConversionWarning) {
        self.warnings.push(warning);
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

/// Result type for CSV export with statistics
#[derive(Debug, serde::Serialize)]
pub struct CsvExportResult {
    /// The generated CSV content
    pub csv_data: String,

    /// List of warnings about data or export issues
    pub warnings: Vec<ConversionWarning>,

    /// Number of data points exported
    pub data_points: usize,

    /// Number of workouts processed
    pub workouts_processed: usize,
}

impl CsvExportResult {
    /// Create a new CSV export result
    pub fn new(csv_data: String) -> Self {
        Self {
            csv_data,
            warnings: Vec::new(),
            data_points: 0,
            workouts_processed: 0,
        }
    }

    /// Add a warning to the result
    pub fn add_warning(&mut self, warning: ConversionWarning) {
        self.warnings.push(warning);
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

/// Warnings about conversion quality or data loss
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum ConversionWarning {
    /// A field from the source format couldn't be mapped
    MissingField {
        /// Name of the source field
        source_field: String,
        /// Reason it couldn't be mapped
        reason: String,
    },

    /// A value was clamped or adjusted to fit PWF constraints
    ValueClamped {
        /// Field name
        field: String,
        /// Original value
        original: String,
        /// Clamped value
        clamped: String,
    },

    /// A feature from the source format isn't supported in PWF
    UnsupportedFeature {
        /// Description of the feature
        feature: String,
    },

    /// Time-series data was skipped (e.g., --summary-only flag)
    TimeSeriesSkipped {
        /// Reason for skipping
        reason: String,
    },

    /// Data quality issue detected
    DataQualityIssue {
        /// Description of the issue
        issue: String,
    },
}

impl fmt::Display for ConversionWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionWarning::MissingField {
                source_field,
                reason,
            } => {
                write!(f, "Missing field '{}': {}", source_field, reason)
            }
            ConversionWarning::ValueClamped {
                field,
                original,
                clamped,
            } => {
                write!(
                    f,
                    "Value clamped in '{}': {} -> {}",
                    field, original, clamped
                )
            }
            ConversionWarning::UnsupportedFeature { feature } => {
                write!(f, "Unsupported feature: {}", feature)
            }
            ConversionWarning::TimeSeriesSkipped { reason } => {
                write!(f, "Time-series data skipped: {}", reason)
            }
            ConversionWarning::DataQualityIssue { issue } => {
                write!(f, "Data quality issue: {}", issue)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_result_new() {
        let result = ConversionResult::new("yaml: content".to_string());
        assert_eq!(result.pwf_yaml, "yaml: content");
        assert!(!result.has_warnings());
    }

    #[test]
    fn test_conversion_result_add_warning() {
        let mut result = ConversionResult::new("yaml: content".to_string());
        result.add_warning(ConversionWarning::UnsupportedFeature {
            feature: "test feature".to_string(),
        });
        assert!(result.has_warnings());
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_warning_display() {
        let warning = ConversionWarning::MissingField {
            source_field: "test_field".to_string(),
            reason: "not supported".to_string(),
        };
        assert_eq!(
            warning.to_string(),
            "Missing field 'test_field': not supported"
        );
    }
}
