//! Common utilities for format conversion

use chrono::{DateTime, Utc};

/// Convert a Unix timestamp to ISO 8601 format (PWF standard)
///
/// FIT files use seconds since UTC 00:00 Dec 31 1989
/// This converts to ISO 8601 string format required by PWF
pub fn fit_timestamp_to_iso8601(fit_timestamp: u32) -> String {
    // FIT epoch: 1989-12-31 00:00:00 UTC
    const FIT_EPOCH: i64 = 631065600;

    let unix_timestamp = FIT_EPOCH + fit_timestamp as i64;
    let dt = DateTime::<Utc>::from_timestamp(unix_timestamp, 0)
        .unwrap_or_else(|| DateTime::<Utc>::from_timestamp(0, 0).unwrap());

    dt.to_rfc3339()
}

/// Convert seconds to ISO 8601 duration format
///
/// Example: 3665 seconds -> "PT1H1M5S"
pub fn seconds_to_duration(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("PT{}H{}M{}S", hours, minutes, secs)
    } else if minutes > 0 {
        format!("PT{}M{}S", minutes, secs)
    } else {
        format!("PT{}S", secs)
    }
}

/// Convert meters to kilometers
pub fn meters_to_km(meters: f64) -> f64 {
    meters / 1000.0
}

/// Convert meters per second to kilometers per hour
pub fn mps_to_kph(mps: f64) -> f64 {
    mps * 3.6
}

/// Convert semicircles to degrees (FIT position format)
///
/// FIT stores GPS coordinates as semicircles (2^31 semicircles = 180 degrees)
pub fn semicircles_to_degrees(semicircles: i32) -> f64 {
    semicircles as f64 * (180.0 / 2_147_483_648.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fit_timestamp_to_iso8601() {
        // Test that we get valid ISO8601 format
        // FIT epoch is 1989-12-31 00:00:00 UTC
        // 0 seconds from FIT epoch should be 1989-12-31
        let iso8601 = fit_timestamp_to_iso8601(0);
        assert!(iso8601.starts_with("1989-12-31T00:00:00"));

        // Test a more recent timestamp
        // 1,000,000,000 seconds from FIT epoch
        let iso8601 = fit_timestamp_to_iso8601(1_000_000_000);
        // Should be valid ISO8601 format
        assert!(iso8601.contains("T"));
        assert!(iso8601.contains("Z") || iso8601.contains("+"));
    }

    #[test]
    fn test_seconds_to_duration() {
        assert_eq!(seconds_to_duration(3665), "PT1H1M5S");
        assert_eq!(seconds_to_duration(125), "PT2M5S");
        assert_eq!(seconds_to_duration(30), "PT30S");
        assert_eq!(seconds_to_duration(0), "PT0S");
    }

    #[test]
    fn test_meters_to_km() {
        assert_eq!(meters_to_km(5000.0), 5.0);
        assert_eq!(meters_to_km(1500.0), 1.5);
    }

    #[test]
    fn test_mps_to_kph() {
        assert_eq!(mps_to_kph(10.0), 36.0);
        assert_eq!(mps_to_kph(5.0), 18.0);
    }

    #[test]
    fn test_semicircles_to_degrees() {
        // 90 degrees = 2^30 semicircles
        let semicircles = 1_073_741_824; // 2^30
        let degrees = semicircles_to_degrees(semicircles);
        assert!((degrees - 90.0).abs() < 0.001);

        // -180 degrees = -2^31 semicircles
        let semicircles = -2_147_483_648_i32;
        let degrees = semicircles_to_degrees(semicircles);
        assert!((degrees + 180.0).abs() < 0.001);
    }
}
