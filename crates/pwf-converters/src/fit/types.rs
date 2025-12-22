//! Type wrappers and helpers for FIT data structures

use fitparser::FitDataRecord;

/// Wrapper for FIT session data
#[derive(Debug)]
pub struct FitSession {
    pub record: FitDataRecord,
}

/// Wrapper for FIT lap data
#[derive(Debug)]
pub struct FitLap {
    pub record: FitDataRecord,
}

/// Wrapper for FIT record (GPS/sensor data point)
#[derive(Debug)]
pub struct FitRecord {
    pub record: FitDataRecord,
}

/// Wrapper for FIT length (swimming pool length)
#[derive(Debug)]
pub struct FitLength {
    pub record: FitDataRecord,
}
