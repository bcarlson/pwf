//! PWF format converters
//!
//! This library provides conversion utilities between PWF (Portable Workout Format)
//! and other fitness file formats like FIT, TCX, and GPX.
//!
//! # Example
//!
//! ```no_run
//! use pwf_converters::fit_to_pwf;
//! use std::fs::File;
//!
//! let fit_file = File::open("workout.fit").unwrap();
//! let result = fit_to_pwf(fit_file, false).unwrap();
//!
//! println!("Converted PWF:\n{}", result.pwf_yaml);
//! for warning in result.warnings {
//!     println!("Warning: {}", warning);
//! }
//! ```

pub mod common;
pub mod csv;
pub mod error;
pub mod fit;
pub mod gpx;
pub mod tcx;

// Re-export main types and functions
pub use csv::{export_telemetry_to_csv, CsvExportOptions};
pub use error::{
    ConversionError, ConversionResult, ConversionWarning, CsvExportResult, GpxExportResult,
    TcxExportResult,
};
pub use fit::fit_to_pwf;
pub use gpx::{gpx_to_pwf, pwf_to_gpx};
pub use tcx::{pwf_to_tcx, tcx_to_pwf};
