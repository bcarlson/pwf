//! Test helper functions for creating synthetic FIT data structures
//!
//! These helpers allow us to test the FIT to PWF conversion pipeline
//! without requiring real FIT files.

use fitparser::{FitDataField, FitDataRecord, Value};

/// Generate a minimal valid FIT file header
pub fn create_fit_header() -> Vec<u8> {
    let mut header = Vec::new();
    header.push(14); // Header size
    header.push(0x20); // Protocol version 2.0
    header.extend_from_slice(&[0x00, 0x08]); // Profile version 8.0
    header.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Data size (placeholder)
    header.extend_from_slice(b".FIT"); // Data type
    header.extend_from_slice(&[0x00, 0x00]); // CRC (placeholder)
    header
}

/// Create a FIT data field with a given name and value
pub fn create_field(name: &str, value: Value) -> FitDataField {
    FitDataField::new(name.to_string(), 0, value, String::new())
}

/// Create a synthetic FIT session record
pub fn create_session_record(
    start_time: u32,
    sport: u8,
    duration_sec: Option<u32>,
    distance_m: Option<f64>,
) -> FitDataRecord {
    let mut record = FitDataRecord::new(fitparser::profile::MesgNum::Session);

    record.push(create_field("start_time", Value::UInt32(start_time)));
    record.push(create_field("timestamp", Value::UInt32(start_time)));
    record.push(create_field("sport", Value::UInt8(sport)));

    if let Some(duration) = duration_sec {
        record.push(create_field("total_elapsed_time", Value::UInt32(duration)));
        record.push(create_field("total_timer_time", Value::UInt32(duration)));
    }

    if let Some(distance) = distance_m {
        record.push(create_field("total_distance", Value::Float64(distance)));
    }

    record
}

/// Create a synthetic FIT lap record
pub fn create_lap_record(
    start_time: u32,
    duration_sec: Option<u32>,
    distance_m: Option<f64>,
) -> FitDataRecord {
    let mut record = FitDataRecord::new(fitparser::profile::MesgNum::Lap);

    record.push(create_field("start_time", Value::UInt32(start_time)));
    record.push(create_field("timestamp", Value::UInt32(start_time)));

    if let Some(duration) = duration_sec {
        record.push(create_field("total_elapsed_time", Value::UInt32(duration)));
    }

    if let Some(distance) = distance_m {
        record.push(create_field("total_distance", Value::Float64(distance)));
    }

    record
}

/// Create a synthetic FIT record (GPS/sensor data point)
pub fn create_gps_record(
    timestamp: u32,
    lat_semicircles: i32,
    lng_semicircles: i32,
    altitude_m: Option<f64>,
    heart_rate: Option<u8>,
) -> FitDataRecord {
    let mut record = FitDataRecord::new(fitparser::profile::MesgNum::Record);

    record.push(create_field("timestamp", Value::UInt32(timestamp)));
    record.push(create_field("position_lat", Value::SInt32(lat_semicircles)));
    record.push(create_field(
        "position_long",
        Value::SInt32(lng_semicircles),
    ));

    if let Some(alt) = altitude_m {
        record.push(create_field("altitude", Value::Float64(alt)));
    }

    if let Some(hr) = heart_rate {
        record.push(create_field("heart_rate", Value::UInt8(hr)));
    }

    record
}

/// Create a synthetic FIT swimming length record
pub fn create_length_record(
    timestamp: u32,
    duration_sec: f64,
    stroke_type: u8,
    stroke_count: Option<u16>,
    pool_length_m: f64,
) -> FitDataRecord {
    let mut record = FitDataRecord::new(fitparser::profile::MesgNum::Length);

    record.push(create_field("timestamp", Value::UInt32(timestamp)));
    record.push(create_field("start_time", Value::UInt32(timestamp)));
    record.push(create_field(
        "total_elapsed_time",
        Value::Float64(duration_sec),
    ));
    record.push(create_field("swim_stroke", Value::UInt8(stroke_type)));
    record.push(create_field("pool_length", Value::Float64(pool_length_m)));
    record.push(create_field("length_type", Value::UInt8(1))); // 1 = active

    if let Some(strokes) = stroke_count {
        record.push(create_field("total_strokes", Value::UInt16(strokes)));
    }

    record
}

/// Create a synthetic FIT device info record
pub fn create_device_info_record(
    device_index: u8,
    device_type: u8,
    manufacturer: u16,
    product: Option<u16>,
    serial: Option<u32>,
) -> FitDataRecord {
    let mut record = FitDataRecord::new(fitparser::profile::MesgNum::DeviceInfo);

    record.push(create_field("device_index", Value::UInt8(device_index)));
    record.push(create_field("device_type", Value::UInt8(device_type)));
    record.push(create_field("manufacturer", Value::UInt16(manufacturer)));

    if let Some(prod) = product {
        record.push(create_field("product", Value::UInt16(prod)));
    }

    if let Some(sn) = serial {
        record.push(create_field("serial_number", Value::UInt32(sn)));
    }

    record
}

/// Create a session with telemetry data (heart rate, power, etc.)
#[allow(dead_code)]
pub fn create_session_with_telemetry(
    start_time: u32,
    sport: u8,
    duration_sec: u32,
    avg_hr: Option<u8>,
    max_hr: Option<u8>,
    avg_power: Option<u16>,
    calories: Option<u16>,
) -> FitDataRecord {
    let mut record = FitDataRecord::new(fitparser::profile::MesgNum::Session);

    record.push(create_field("start_time", Value::UInt32(start_time)));
    record.push(create_field("timestamp", Value::UInt32(start_time)));
    record.push(create_field("sport", Value::UInt8(sport)));
    record.push(create_field(
        "total_elapsed_time",
        Value::UInt32(duration_sec),
    ));

    if let Some(hr) = avg_hr {
        record.push(create_field("avg_heart_rate", Value::UInt8(hr)));
    }

    if let Some(hr_max) = max_hr {
        record.push(create_field("max_heart_rate", Value::UInt8(hr_max)));
    }

    if let Some(power) = avg_power {
        record.push(create_field("avg_power", Value::UInt16(power)));
    }

    if let Some(cals) = calories {
        record.push(create_field("total_calories", Value::UInt16(cals)));
    }

    record
}

/// Create a session with advanced power metrics
#[allow(dead_code)]
pub fn create_session_with_power_metrics(
    start_time: u32,
    normalized_power: Option<u16>,
    tss: Option<f64>,
    intensity_factor: Option<f64>,
    ftp: Option<u16>,
) -> FitDataRecord {
    let mut record = FitDataRecord::new(fitparser::profile::MesgNum::Session);

    record.push(create_field("start_time", Value::UInt32(start_time)));
    record.push(create_field("timestamp", Value::UInt32(start_time)));
    record.push(create_field("sport", Value::UInt8(1))); // Cycling
    record.push(create_field("total_elapsed_time", Value::UInt32(3600)));

    if let Some(np) = normalized_power {
        record.push(create_field("normalized_power", Value::UInt16(np)));
    }

    if let Some(tss_val) = tss {
        record.push(create_field(
            "training_stress_score",
            Value::Float64(tss_val),
        ));
    }

    if let Some(if_val) = intensity_factor {
        record.push(create_field("intensity_factor", Value::Float64(if_val)));
    }

    if let Some(ftp_val) = ftp {
        record.push(create_field("threshold_power", Value::UInt16(ftp_val)));
    }

    record
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_creation() {
        let header = create_fit_header();
        assert_eq!(header.len(), 14);
        assert_eq!(header[0], 14);
        assert_eq!(&header[8..12], b".FIT");
    }

    #[test]
    fn test_field_creation() {
        let field = create_field("test_field", Value::UInt32(100));
        assert_eq!(field.name(), "test_field");
    }

    #[test]
    fn test_session_record_creation() {
        let session = create_session_record(1000, 0, Some(3600), Some(5000.0));
        let fields = session.fields();

        // Check that required fields exist
        assert!(fields.iter().any(|f| f.name() == "start_time"));
        assert!(fields.iter().any(|f| f.name() == "sport"));
        assert!(fields.iter().any(|f| f.name() == "total_elapsed_time"));
    }

    #[test]
    fn test_lap_record_creation() {
        let lap = create_lap_record(1000, Some(600), Some(1000.0));
        let fields = lap.fields();

        assert!(fields.iter().any(|f| f.name() == "start_time"));
        assert!(fields.iter().any(|f| f.name() == "total_elapsed_time"));
    }

    #[test]
    fn test_gps_record_creation() {
        // San Francisco coordinates: ~37.77° N, ~122.42° W
        let lat_sc = (37.77 * 2147483648.0 / 180.0) as i32;
        let lng_sc = (-122.42 * 2147483648.0 / 180.0) as i32;

        let record = create_gps_record(1000, lat_sc, lng_sc, Some(10.0), Some(150));
        let fields = record.fields();

        assert!(fields.iter().any(|f| f.name() == "position_lat"));
        assert!(fields.iter().any(|f| f.name() == "position_long"));
        assert!(fields.iter().any(|f| f.name() == "altitude"));
    }

    #[test]
    fn test_length_record_creation() {
        let length = create_length_record(1000, 30.0, 0, Some(20), 50.0);
        let fields = length.fields();

        assert!(fields.iter().any(|f| f.name() == "swim_stroke"));
        assert!(fields.iter().any(|f| f.name() == "total_strokes"));
        assert!(fields.iter().any(|f| f.name() == "pool_length"));
    }

    #[test]
    fn test_device_info_creation() {
        let device = create_device_info_record(0, 1, 1, Some(2697), Some(123456));
        let fields = device.fields();

        assert!(fields.iter().any(|f| f.name() == "device_type"));
        assert!(fields.iter().any(|f| f.name() == "manufacturer"));
        assert!(fields.iter().any(|f| f.name() == "product"));
    }
}
