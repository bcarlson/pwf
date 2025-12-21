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

// Telemetry validation (700-799) - PWF v2
pub const HEART_RATE_OUT_OF_RANGE: &str = "PWF-H701";
pub const POWER_NEGATIVE: &str = "PWF-H702";
pub const ELEVATION_NEGATIVE: &str = "PWF-H703";
pub const SPEED_NEGATIVE: &str = "PWF-H704";
pub const CADENCE_OUT_OF_RANGE: &str = "PWF-H705";
pub const HUMIDITY_OUT_OF_RANGE: &str = "PWF-H706";
#[allow(dead_code)]
pub const CALORIES_NEGATIVE: &str = "PWF-H707";
#[allow(dead_code)]
pub const STROKE_RATE_NEGATIVE: &str = "PWF-H708";
pub const TELEMETRY_UNIT_MISMATCH: &str = "PWF-H709";
pub const PACE_NEGATIVE: &str = "PWF-H710";

// PWF v2.1 validation (800-899)
// Swimming validation (800-819)
pub const SWOLF_MISMATCH: &str = "PWF-H801";
pub const POOL_LENGTH_INVALID: &str = "PWF-H802";

// Time-series validation (820-839)
pub const TIME_SERIES_LENGTH_MISMATCH: &str = "PWF-H821";

// Sport segment validation (840-859)
pub const SEGMENT_INDEX_GAP: &str = "PWF-H841";
pub const SEGMENT_INDEX_DUPLICATE: &str = "PWF-H842";

// Transition validation (860-869)
pub const TRANSITION_SPORT_MISMATCH: &str = "PWF-H861";

// Zone validation (870-879)
pub const ZONE_ARRAY_LENGTH_MISMATCH: &str = "PWF-H871";

// GPS validation (880-889)
pub const GPS_LATITUDE_OUT_OF_RANGE: &str = "PWF-H881";
pub const GPS_LONGITUDE_OUT_OF_RANGE: &str = "PWF-H882";
pub const GPS_HEADING_OUT_OF_RANGE: &str = "PWF-H883";

// Advanced metrics validation (890-899)
pub const TRAINING_EFFECT_OUT_OF_RANGE: &str = "PWF-H891";
pub const PERFORMANCE_CONDITION_OUT_OF_RANGE: &str = "PWF-H892";
pub const INTENSITY_FACTOR_MISMATCH: &str = "PWF-H893";
pub const VARIABILITY_INDEX_MISMATCH: &str = "PWF-H894";
