//! Error codes for history validation
//!
//! PWF history validation error codes follow the pattern PWF-H###
//! where H indicates history and ### is a sequential number.

// Root level validation (001-099)
pub const INVALID_VERSION: &str = "PWF-H001";
pub const MISSING_EXPORTED_AT: &str = "PWF-H002";

// Workout validation (100-199)
pub const MISSING_WORKOUT_DATE: &str = "PWF-H101";
pub const NO_EXERCISES: &str = "PWF-H102";
#[allow(dead_code)]
pub const INVALID_WORKOUT_DATE: &str = "PWF-H103"; // Reserved for future use

// Exercise validation (200-299)
pub const MISSING_EXERCISE_NAME: &str = "PWF-H201";
pub const NO_SETS: &str = "PWF-H202";

// Set validation (300-399)
pub const NO_METRICS: &str = "PWF-H301";
pub const RPE_OUT_OF_RANGE: &str = "PWF-H302";
pub const RIR_OUT_OF_RANGE: &str = "PWF-H303";
pub const RPE_RIR_BOTH_SET: &str = "PWF-H304";

// Personal record validation (400-499)
pub const MISSING_PR_EXERCISE: &str = "PWF-H401";
pub const MISSING_PR_DATE: &str = "PWF-H402";
pub const PR_MISSING_UNIT: &str = "PWF-H403";

// Body measurement validation (500-599)
pub const MISSING_BM_DATE: &str = "PWF-H501";
pub const NO_BM_VALUES: &str = "PWF-H502";

// Preferred units validation (600-699)
pub const PREFERRED_UNITS_MISMATCH: &str = "PWF-H601";
#[allow(dead_code)]
pub const NO_WEIGHT_DATA: &str = "PWF-H602"; // Reserved for future use
