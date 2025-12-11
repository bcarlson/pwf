//! # WPS Core
//!
//! Core library for parsing and validating Workout Plan Specification (WPS) documents.
//!
//! This crate provides:
//! - Plan parsing and validation (`plan` module)
//! - History export parsing and validation (`history` module)
//! - Common types used across both formats
//!
//! ## Quick Start
//!
//! ### Validating a Plan
//!
//! ```rust
//! use wps_core::plan;
//!
//! let yaml = r#"
//! plan_version: 1
//! cycle:
//!   days:
//!     - exercises:
//!         - name: Push-ups
//!           modality: strength
//! "#;
//!
//! let result = plan::validate(yaml);
//! assert!(result.is_valid());
//! ```
//!
//! ### Validating Workout History
//!
//! ```rust
//! use wps_core::history;
//!
//! let yaml = r#"
//! history_version: 1
//! exported_at: "2025-01-15T10:30:00Z"
//! workouts:
//!   - date: "2025-01-15"
//!     exercises:
//!       - name: Squat
//!         sets:
//!           - reps: 5
//!             weight_kg: 100
//! "#;
//!
//! let result = history::validate(yaml);
//! assert!(result.is_valid());
//! ```

pub mod error;
pub mod history;
pub mod plan;
mod types;

pub use types::*;
