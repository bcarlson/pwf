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
pub mod error;
pub mod fit;

// Re-export main types and functions
pub use error::{ConversionError, ConversionResult, ConversionWarning};
pub use fit::fit_to_pwf;
