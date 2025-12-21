//! History export type definitions

use crate::{DistanceUnit, Modality, Sport, WeightUnit};
use serde::{Deserialize, Serialize};

/// Root history export structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WpsHistory {
    pub history_version: u32,
    pub exported_at: String,
    #[serde(default)]
    pub export_source: Option<ExportSource>,
    #[serde(default)]
    pub units: Units,
    pub workouts: Vec<Workout>,
    #[serde(default)]
    pub personal_records: Vec<PersonalRecord>,
    #[serde(default)]
    pub body_measurements: Vec<BodyMeasurement>,
}

impl WpsHistory {
    /// Returns true if this is a v2 history export
    pub fn is_v2(&self) -> bool {
        self.history_version == 2
    }
}

/// Information about the exporting application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExportSource {
    #[serde(default)]
    pub app_name: Option<String>,
    #[serde(default)]
    pub app_version: Option<String>,
    #[serde(default)]
    pub platform: Option<String>,
    #[serde(default)]
    pub preferred_units: Option<Units>,
}

/// Default units for the export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Units {
    #[serde(default)]
    pub weight: WeightUnit,
    #[serde(default)]
    pub distance: DistanceUnit,
}

impl Default for Units {
    fn default() -> Self {
        Self {
            weight: WeightUnit::Kg,
            distance: DistanceUnit::Meters,
        }
    }
}

/// A completed workout session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workout {
    #[serde(default)]
    pub id: Option<String>,
    pub date: String,
    #[serde(default)]
    pub started_at: Option<String>,
    #[serde(default)]
    pub ended_at: Option<String>,
    #[serde(default)]
    pub duration_sec: Option<u32>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub plan_id: Option<String>,
    #[serde(default)]
    pub plan_day_id: Option<String>,
    pub exercises: Vec<CompletedExercise>,
    #[serde(default)]
    pub telemetry: Option<WorkoutTelemetry>,
    #[serde(default)]
    pub devices: Vec<DeviceInfo>,

    // PWF v2.1: Sport tracking and multi-sport support
    /// Primary sport for this workout
    #[serde(default)]
    pub sport: Option<Sport>,

    /// Sport segments for multi-sport workouts (e.g., triathlon)
    /// If present, this is a multi-sport session with distinct segments
    #[serde(default)]
    pub sport_segments: Option<Vec<SportSegment>>,
}

/// Telemetry metrics for an entire workout session (PWF v2)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkoutTelemetry {
    // Heart Rate
    #[serde(default)]
    pub heart_rate_avg: Option<u32>,
    #[serde(default)]
    pub heart_rate_max: Option<u32>,
    #[serde(default)]
    pub heart_rate_min: Option<u32>,

    // Power (watts)
    #[serde(default)]
    pub power_avg: Option<u32>,
    #[serde(default)]
    pub power_max: Option<u32>,

    // Total distance (meters or feet)
    #[serde(default)]
    pub total_distance_m: Option<f64>,
    #[serde(default)]
    pub total_distance_km: Option<f64>,
    #[serde(default)]
    pub total_distance_mi: Option<f64>,

    // Elevation
    #[serde(default)]
    pub total_elevation_gain_m: Option<f64>,
    #[serde(default)]
    pub total_elevation_gain_ft: Option<f64>,
    #[serde(default)]
    pub total_elevation_loss_m: Option<f64>,
    #[serde(default)]
    pub total_elevation_loss_ft: Option<f64>,

    // Speed
    #[serde(default)]
    pub speed_avg_kph: Option<f64>,
    #[serde(default)]
    pub speed_avg_mph: Option<f64>,
    #[serde(default)]
    pub speed_max_kph: Option<f64>,
    #[serde(default)]
    pub speed_max_mph: Option<f64>,

    // Pace
    #[serde(default)]
    pub pace_avg_sec_per_km: Option<u32>,
    #[serde(default)]
    pub pace_avg_sec_per_mi: Option<u32>,

    // Cadence
    #[serde(default)]
    pub cadence_avg: Option<u32>,

    // Environmental
    #[serde(default)]
    pub temperature_c: Option<f64>,
    #[serde(default)]
    pub temperature_f: Option<f64>,
    #[serde(default)]
    pub humidity_percent: Option<f64>,

    // Calories burned
    #[serde(default)]
    pub total_calories: Option<u32>,

    // GPS/Route data
    #[serde(default)]
    pub gps_route_id: Option<String>,

    // PWF v2.1: GPS route with full position data
    #[serde(default)]
    pub gps_route: Option<GpsRoute>,

    // PWF v2.1: Advanced physiological metrics
    #[serde(default)]
    pub advanced_metrics: Option<AdvancedMetrics>,

    // PWF v2.1: Power-based cycling metrics
    #[serde(default)]
    pub power_metrics: Option<PowerMetrics>,

    // PWF v2.1: Time in HR/power zones
    #[serde(default)]
    pub time_in_zones: Option<TimeInZones>,
}

// ============================================================================
// PWF v2.1: Advanced Metrics
// ============================================================================

/// Advanced physiological and performance metrics
/// Primarily from Garmin/Firstbeat algorithms
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdvancedMetrics {
    /// Training Effect score (0.0 - 5.0)
    /// Measures aerobic training stimulus
    #[serde(default)]
    pub training_effect: Option<f64>,

    /// Anaerobic Training Effect (0.0 - 5.0)
    /// Measures high-intensity training stimulus
    #[serde(default)]
    pub anaerobic_training_effect: Option<f64>,

    /// Recommended recovery time (hours)
    #[serde(default)]
    pub recovery_time_hours: Option<u32>,

    /// VO2 Max estimate (ml/kg/min)
    #[serde(default)]
    pub vo2_max_estimate: Option<f64>,

    /// Lactate threshold data
    #[serde(default)]
    pub lactate_threshold: Option<LactateThreshold>,

    /// Performance Condition (-20 to +20)
    /// Real-time performance assessment during workout
    #[serde(default)]
    pub performance_condition: Option<i8>,

    /// Training Load (0-1000+)
    /// Cumulative training stress
    #[serde(default)]
    pub training_load: Option<u32>,

    /// Training Status assessment
    #[serde(default)]
    pub training_status: Option<TrainingStatus>,
}

/// Lactate threshold tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LactateThreshold {
    /// Heart rate at lactate threshold (bpm)
    #[serde(default)]
    pub heart_rate_bpm: Option<u32>,

    /// Speed/pace at lactate threshold (m/s)
    #[serde(default)]
    pub speed_mps: Option<f64>,

    /// Power at lactate threshold (watts, for cycling)
    #[serde(default)]
    pub power_watts: Option<u32>,

    /// When threshold was detected/calculated
    #[serde(default)]
    pub detected_at: Option<String>,
}

/// Training status classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrainingStatus {
    Detraining,
    Recovery,
    Maintaining,
    Productive,
    Peaking,
    Overreaching,
    Unknown,
}

/// Power-based cycling metrics (from power meter data)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PowerMetrics {
    /// Normalized Power (NP) - weighted average accounting for variability
    #[serde(default)]
    pub normalized_power: Option<u32>,

    /// Training Stress Score (TSS) - quantifies training load
    #[serde(default)]
    pub training_stress_score: Option<f64>,

    /// Intensity Factor (IF) - ratio of NP to FTP
    #[serde(default)]
    pub intensity_factor: Option<f64>,

    /// Variability Index (VI) - ratio of NP to average power
    #[serde(default)]
    pub variability_index: Option<f64>,

    /// Functional Threshold Power used for calculations (watts)
    #[serde(default)]
    pub ftp_watts: Option<u32>,

    /// Work in kilojoules (total energy expenditure)
    #[serde(default)]
    pub total_work_kj: Option<f64>,

    /// Left/right power balance (percentage left)
    #[serde(default)]
    pub left_right_balance: Option<f64>,

    /// Average left pedal smoothness (percentage)
    #[serde(default)]
    pub left_pedal_smoothness: Option<f64>,

    /// Average right pedal smoothness (percentage)
    #[serde(default)]
    pub right_pedal_smoothness: Option<f64>,

    /// Average left torque effectiveness (percentage)
    #[serde(default)]
    pub left_torque_effectiveness: Option<f64>,

    /// Average right torque effectiveness (percentage)
    #[serde(default)]
    pub right_torque_effectiveness: Option<f64>,
}

/// Time spent in heart rate and power zones
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeInZones {
    /// Time in each HR zone (seconds per zone)
    /// Zones typically: 1=easy, 2=moderate, 3=tempo, 4=threshold, 5=max
    #[serde(default)]
    pub hr_zones_sec: Option<Vec<u32>>,

    /// Time in each power zone (seconds per zone)
    /// Zones typically based on FTP percentages
    #[serde(default)]
    pub power_zones_sec: Option<Vec<u32>>,

    /// HR zone configuration (zone boundaries in bpm)
    #[serde(default)]
    pub hr_zone_boundaries: Option<Vec<u32>>,

    /// Power zone configuration (zone boundaries in watts)
    #[serde(default)]
    pub power_zone_boundaries: Option<Vec<u32>>,

    /// Pace zones (seconds per zone, for running)
    #[serde(default)]
    pub pace_zones_sec: Option<Vec<u32>>,

    /// Pace zone boundaries (seconds per km)
    #[serde(default)]
    pub pace_zone_boundaries: Option<Vec<u32>>,
}

// ============================================================================
// PWF v2.1: Multi-Sport Sessions
// ============================================================================

/// A segment within a multi-sport workout (e.g., swim/bike/run in triathlon)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SportSegment {
    /// Segment identifier
    pub segment_id: String,

    /// Sport for this segment
    pub sport: Sport,

    /// Segment number in sequence (0-indexed)
    pub segment_index: u32,

    /// When segment started (ISO 8601)
    #[serde(default)]
    pub started_at: Option<String>,

    /// Segment duration (seconds)
    #[serde(default)]
    pub duration_sec: Option<u32>,

    /// Distance covered in this segment (meters)
    #[serde(default)]
    pub distance_m: Option<f64>,

    /// Exercises/sets completed during this segment
    #[serde(default)]
    pub exercise_ids: Vec<String>,

    /// Telemetry specific to this segment
    #[serde(default)]
    pub telemetry: Option<WorkoutTelemetry>,

    /// Transition data after this segment (if applicable)
    #[serde(default)]
    pub transition: Option<TransitionData>,

    /// Notes specific to this segment
    #[serde(default)]
    pub notes: Option<String>,
}

/// Transition between sports in a multi-sport event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionData {
    /// Transition identifier (e.g., "T1", "T2")
    pub transition_id: String,

    /// From which sport
    pub from_sport: Sport,

    /// To which sport
    pub to_sport: Sport,

    /// Transition duration (seconds)
    #[serde(default)]
    pub duration_sec: Option<u32>,

    /// When transition started (ISO 8601)
    #[serde(default)]
    pub started_at: Option<String>,

    /// Average heart rate during transition
    #[serde(default)]
    pub heart_rate_avg: Option<u32>,

    /// Notes about transition (e.g., "Changed shoes", "Equipment issues")
    #[serde(default)]
    pub notes: Option<String>,
}

// ============================================================================
// PWF v2.1: GPS and Position Data
// ============================================================================

/// A single GPS position/waypoint with timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsPosition {
    /// Latitude in decimal degrees (WGS84)
    pub latitude_deg: f64,

    /// Longitude in decimal degrees (WGS84)
    pub longitude_deg: f64,

    /// Timestamp when position was recorded (ISO 8601)
    pub timestamp: String,

    /// Elevation/altitude above sea level (meters)
    #[serde(default)]
    pub elevation_m: Option<f64>,

    /// Horizontal accuracy/uncertainty (meters)
    #[serde(default)]
    pub accuracy_m: Option<f64>,

    /// Speed at this point (meters per second)
    #[serde(default)]
    pub speed_mps: Option<f64>,

    /// Heading/bearing (degrees from north, 0-360)
    #[serde(default)]
    pub heading_deg: Option<f64>,

    /// Heart rate at this position (bpm)
    #[serde(default)]
    pub heart_rate_bpm: Option<u32>,

    /// Power at this position (watts)
    #[serde(default)]
    pub power_watts: Option<u32>,

    /// Cadence at this position (RPM or SPM)
    #[serde(default)]
    pub cadence: Option<u32>,

    /// Temperature at this position (Celsius)
    #[serde(default)]
    pub temperature_c: Option<f64>,
}

/// A GPS route/track containing multiple positions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsRoute {
    /// Unique identifier for this route
    pub route_id: String,

    /// Human-readable route name
    #[serde(default)]
    pub name: Option<String>,

    /// GPS positions in chronological order
    pub positions: Vec<GpsPosition>,

    /// Total distance calculated from GPS (meters)
    #[serde(default)]
    pub total_distance_m: Option<f64>,

    /// Total elevation gain (meters)
    #[serde(default)]
    pub total_ascent_m: Option<f64>,

    /// Total elevation loss (meters)
    #[serde(default)]
    pub total_descent_m: Option<f64>,

    /// Minimum elevation on route (meters)
    #[serde(default)]
    pub min_elevation_m: Option<f64>,

    /// Maximum elevation on route (meters)
    #[serde(default)]
    pub max_elevation_m: Option<f64>,

    /// Bounding box - southwest corner latitude
    #[serde(default)]
    pub bbox_sw_lat: Option<f64>,

    /// Bounding box - southwest corner longitude
    #[serde(default)]
    pub bbox_sw_lng: Option<f64>,

    /// Bounding box - northeast corner latitude
    #[serde(default)]
    pub bbox_ne_lat: Option<f64>,

    /// Bounding box - northeast corner longitude
    #[serde(default)]
    pub bbox_ne_lng: Option<f64>,

    /// Recording mode (e.g., "auto", "smart", "1s", "gps_only")
    #[serde(default)]
    pub recording_mode: Option<String>,

    /// GPS fix quality indicator
    #[serde(default)]
    pub gps_fix: Option<GpsFix>,
}

/// GPS fix quality indicator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GpsFix {
    /// No GPS fix
    None,
    /// 2D fix (lat/lon only)
    #[serde(rename = "fix_2d")]
    Fix2D,
    /// 3D fix (lat/lon/elevation)
    #[serde(rename = "fix_3d")]
    Fix3D,
    /// Differential GPS
    Dgps,
    /// Unknown fix quality
    Unknown,
}

// ============================================================================
// PWF v2.1: Time-Series Telemetry Data
// ============================================================================

/// Columnar time-series data for second-by-second telemetry
/// Uses columnar storage for better compression and flexibility
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeSeriesData {
    /// Timestamps for each record (ISO 8601)
    /// All other arrays must match this length
    pub timestamps: Vec<String>,

    /// Elapsed time in seconds since start (parallel array)
    /// Useful for quick lookups without parsing timestamps
    #[serde(default)]
    pub elapsed_sec: Option<Vec<u32>>,

    /// Heart rate readings (bpm)
    #[serde(default)]
    pub heart_rate: Option<Vec<u32>>,

    /// Power readings (watts)
    #[serde(default)]
    pub power: Option<Vec<u32>>,

    /// Cadence readings (RPM for cycling, SPM for running/swimming)
    #[serde(default)]
    pub cadence: Option<Vec<u32>>,

    /// Speed readings (meters per second)
    #[serde(default)]
    pub speed_mps: Option<Vec<f64>>,

    /// Distance readings (cumulative meters)
    #[serde(default)]
    pub distance_m: Option<Vec<f64>>,

    /// Elevation/altitude readings (meters)
    #[serde(default)]
    pub elevation_m: Option<Vec<f64>>,

    /// Temperature readings (Celsius)
    #[serde(default)]
    pub temperature_c: Option<Vec<f64>>,

    /// Latitude readings (decimal degrees)
    #[serde(default)]
    pub latitude: Option<Vec<f64>>,

    /// Longitude readings (decimal degrees)
    #[serde(default)]
    pub longitude: Option<Vec<f64>>,

    /// Grade/slope readings (percentage, -100 to +100)
    #[serde(default)]
    pub grade_percent: Option<Vec<f64>>,

    /// Respiration rate (breaths per minute)
    #[serde(default)]
    pub respiration_rate: Option<Vec<u32>>,

    /// Core body temperature (Celsius)
    #[serde(default)]
    pub core_temperature_c: Option<Vec<f64>>,

    /// Muscle oxygen saturation (percentage)
    #[serde(default)]
    pub muscle_oxygen_percent: Option<Vec<f64>>,

    /// Left/right power balance (percentage left)
    #[serde(default)]
    pub power_balance: Option<Vec<f64>>,

    /// Cycling-specific: Left pedal smoothness (percentage)
    #[serde(default)]
    pub left_pedal_smoothness: Option<Vec<f64>>,

    /// Cycling-specific: Right pedal smoothness (percentage)
    #[serde(default)]
    pub right_pedal_smoothness: Option<Vec<f64>>,

    /// Cycling-specific: Left torque effectiveness (percentage)
    #[serde(default)]
    pub left_torque_effectiveness: Option<Vec<f64>>,

    /// Cycling-specific: Right torque effectiveness (percentage)
    #[serde(default)]
    pub right_torque_effectiveness: Option<Vec<f64>>,

    /// Running-specific: Stride length (meters)
    #[serde(default)]
    pub stride_length_m: Option<Vec<f64>>,

    /// Running-specific: Vertical oscillation (centimeters)
    #[serde(default)]
    pub vertical_oscillation_cm: Option<Vec<f64>>,

    /// Running-specific: Ground contact time (milliseconds)
    #[serde(default)]
    pub ground_contact_time_ms: Option<Vec<u32>>,

    /// Running-specific: Ground contact balance (percentage left)
    #[serde(default)]
    pub ground_contact_balance: Option<Vec<f64>>,

    /// Swimming-specific: Stroke rate (strokes per minute)
    #[serde(default)]
    pub stroke_rate: Option<Vec<u32>>,

    /// Swimming-specific: Stroke count (cumulative)
    #[serde(default)]
    pub stroke_count: Option<Vec<u32>>,

    /// Swimming-specific: SWOLF score
    #[serde(default)]
    pub swolf: Option<Vec<u32>>,

    /// Swimming-specific: Stroke type at each point
    #[serde(default)]
    pub stroke_type: Option<Vec<StrokeType>>,
}

impl TimeSeriesData {
    /// Validate that all data arrays match the length of timestamps
    pub fn validate_lengths(&self) -> Result<(), String> {
        let expected_len = self.timestamps.len();

        macro_rules! check_length {
            ($field:expr, $name:expr) => {
                if let Some(ref data) = $field {
                    if data.len() != expected_len {
                        return Err(format!(
                            "{} length ({}) doesn't match timestamps length ({})",
                            $name,
                            data.len(),
                            expected_len
                        ));
                    }
                }
            };
        }

        check_length!(self.elapsed_sec, "elapsed_sec");
        check_length!(self.heart_rate, "heart_rate");
        check_length!(self.power, "power");
        check_length!(self.cadence, "cadence");
        check_length!(self.speed_mps, "speed_mps");
        check_length!(self.distance_m, "distance_m");
        check_length!(self.elevation_m, "elevation_m");
        check_length!(self.temperature_c, "temperature_c");
        check_length!(self.latitude, "latitude");
        check_length!(self.longitude, "longitude");
        check_length!(self.grade_percent, "grade_percent");
        check_length!(self.respiration_rate, "respiration_rate");
        check_length!(self.core_temperature_c, "core_temperature_c");
        check_length!(self.muscle_oxygen_percent, "muscle_oxygen_percent");
        check_length!(self.power_balance, "power_balance");
        check_length!(self.left_pedal_smoothness, "left_pedal_smoothness");
        check_length!(self.right_pedal_smoothness, "right_pedal_smoothness");
        check_length!(self.left_torque_effectiveness, "left_torque_effectiveness");
        check_length!(
            self.right_torque_effectiveness,
            "right_torque_effectiveness"
        );
        check_length!(self.stride_length_m, "stride_length_m");
        check_length!(self.vertical_oscillation_cm, "vertical_oscillation_cm");
        check_length!(self.ground_contact_time_ms, "ground_contact_time_ms");
        check_length!(self.ground_contact_balance, "ground_contact_balance");
        check_length!(self.stroke_rate, "stroke_rate");
        check_length!(self.stroke_count, "stroke_count");
        check_length!(self.swolf, "swolf");
        check_length!(self.stroke_type, "stroke_type");

        Ok(())
    }

    /// Get the number of data points
    pub fn len(&self) -> usize {
        self.timestamps.len()
    }

    /// Check if the time series is empty
    pub fn is_empty(&self) -> bool {
        self.timestamps.is_empty()
    }

    /// Get the duration of the time series (first to last timestamp)
    /// Returns None if there are fewer than 2 timestamps
    pub fn duration_sec(&self) -> Option<u32> {
        if let Some(elapsed) = &self.elapsed_sec {
            if let Some(&last) = elapsed.last() {
                return Some(last);
            }
        }
        None
    }
}

/// A completed exercise with recorded sets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedExercise {
    #[serde(default)]
    pub id: Option<String>,
    pub name: String,
    #[serde(default)]
    pub modality: Option<Modality>,
    #[serde(default)]
    pub notes: Option<String>,
    pub sets: Vec<CompletedSet>,

    // PWF v2.1: Pool configuration (for swimming exercises)
    #[serde(default)]
    pub pool_config: Option<PoolConfig>,

    // PWF v2.1: Sport classification (for multi-sport workouts)
    #[serde(default)]
    pub sport: Option<Sport>,
}

/// A single completed set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedSet {
    #[serde(default)]
    pub set_number: Option<u32>,
    #[serde(default)]
    pub set_type: Option<SetType>,
    #[serde(default)]
    pub reps: Option<u32>,
    #[serde(default)]
    pub weight_kg: Option<f64>,
    #[serde(default)]
    pub weight_lb: Option<f64>,
    #[serde(default)]
    pub duration_sec: Option<u32>,
    #[serde(default)]
    pub distance_meters: Option<f64>,
    #[serde(default)]
    pub rpe: Option<f64>,
    #[serde(default)]
    pub rir: Option<u32>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub is_pr: Option<bool>,
    #[serde(default)]
    pub completed_at: Option<String>,
    #[serde(default)]
    pub telemetry: Option<SetTelemetry>,

    // PWF v2.1: Swimming-specific data
    #[serde(default)]
    pub swimming: Option<SwimmingSetData>,
}

/// Telemetry metrics for a completed set (PWF v2)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetTelemetry {
    // Heart Rate
    #[serde(default)]
    pub heart_rate_avg: Option<u32>,
    #[serde(default)]
    pub heart_rate_max: Option<u32>,
    #[serde(default)]
    pub heart_rate_min: Option<u32>,

    // Power (watts)
    #[serde(default)]
    pub power_avg: Option<u32>,
    #[serde(default)]
    pub power_max: Option<u32>,
    #[serde(default)]
    pub power_min: Option<u32>,

    // Elevation (meters or feet depending on preferred_units)
    #[serde(default)]
    pub elevation_gain_m: Option<f64>,
    #[serde(default)]
    pub elevation_gain_ft: Option<f64>,
    #[serde(default)]
    pub elevation_loss_m: Option<f64>,
    #[serde(default)]
    pub elevation_loss_ft: Option<f64>,

    // Speed (m/s, km/h, or mph depending on context)
    #[serde(default)]
    pub speed_avg_mps: Option<f64>,
    #[serde(default)]
    pub speed_avg_kph: Option<f64>,
    #[serde(default)]
    pub speed_avg_mph: Option<f64>,
    #[serde(default)]
    pub speed_max_mps: Option<f64>,
    #[serde(default)]
    pub speed_max_kph: Option<f64>,
    #[serde(default)]
    pub speed_max_mph: Option<f64>,

    // Pace (seconds per km or mile)
    #[serde(default)]
    pub pace_avg_sec_per_km: Option<u32>,
    #[serde(default)]
    pub pace_avg_sec_per_mi: Option<u32>,

    // Cadence
    #[serde(default)]
    pub cadence_avg: Option<u32>,
    #[serde(default)]
    pub cadence_max: Option<u32>,

    // Environmental
    #[serde(default)]
    pub temperature_c: Option<f64>,
    #[serde(default)]
    pub temperature_f: Option<f64>,
    #[serde(default)]
    pub humidity_percent: Option<f64>,

    // Calories burned (if calculated by tracking device)
    #[serde(default)]
    pub calories: Option<u32>,

    // Stroke rate (for swimming/rowing)
    #[serde(default)]
    pub stroke_rate: Option<u32>,

    // GPS/Route data
    #[serde(default)]
    pub gps_route_id: Option<String>,

    // PWF v2.1: Second-by-second time-series data
    #[serde(default)]
    pub time_series: Option<TimeSeriesData>,
}

/// Type of set (working, warmup, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SetType {
    #[default]
    Working,
    Warmup,
    Dropset,
    Failure,
    Amrap,
}

// ============================================================================
// Swimming-specific types (PWF v2.1)
// ============================================================================

/// Swimming stroke type for pool and open water swimming
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StrokeType {
    /// Freestyle/front crawl
    Freestyle,
    /// Backstroke
    Backstroke,
    /// Breaststroke
    Breaststroke,
    /// Butterfly
    Butterfly,
    /// Drill (technique work, not racing stroke)
    Drill,
    /// Mixed strokes within the same length
    Mixed,
    /// Individual Medley (all four strokes in sequence)
    #[serde(rename = "im")]
    IndividualMedley,
}

/// Unit for pool length measurement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum PoolLengthUnit {
    #[default]
    Meters,
    Yards,
}

/// Pool configuration for swimming workouts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    /// Length of the pool in the specified units
    pub pool_length: f64,

    /// Unit for pool length (meters or yards)
    #[serde(default)]
    pub pool_length_unit: PoolLengthUnit,
}

impl PoolConfig {
    /// Get pool length in meters (standard conversion)
    pub fn length_in_meters(&self) -> f64 {
        match self.pool_length_unit {
            PoolLengthUnit::Meters => self.pool_length,
            PoolLengthUnit::Yards => self.pool_length * 0.9144, // 1 yard = 0.9144 meters
        }
    }

    /// Common 25m pool configuration
    pub fn pool_25m() -> Self {
        Self {
            pool_length: 25.0,
            pool_length_unit: PoolLengthUnit::Meters,
        }
    }

    /// Common 50m pool configuration
    pub fn pool_50m() -> Self {
        Self {
            pool_length: 50.0,
            pool_length_unit: PoolLengthUnit::Meters,
        }
    }

    /// Common 25yd pool configuration
    pub fn pool_25yd() -> Self {
        Self {
            pool_length: 25.0,
            pool_length_unit: PoolLengthUnit::Yards,
        }
    }
}

/// A single length (one pool length) within a swimming set/lap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwimmingLength {
    /// Length number within the set (1-indexed)
    pub length_number: u32,

    /// Stroke type used for this length
    pub stroke_type: StrokeType,

    /// Duration of this length in seconds
    pub duration_sec: u32,

    /// Number of strokes taken during this length
    #[serde(default)]
    pub stroke_count: Option<u32>,

    /// SWOLF score (duration + stroke_count) - efficiency metric
    /// Lower is better. Calculated as: duration_sec + stroke_count
    #[serde(default)]
    pub swolf: Option<u32>,

    /// Timestamp when this length started (ISO 8601)
    #[serde(default)]
    pub started_at: Option<String>,

    /// Whether this was an active length (vs. rest at wall)
    /// FIT files distinguish between active and rest lengths
    #[serde(default)]
    pub active: Option<bool>,
}

impl SwimmingLength {
    /// Calculate SWOLF score from duration and stroke count
    /// Returns None if stroke_count is not available
    pub fn calculate_swolf(&self) -> Option<u32> {
        self.stroke_count.map(|count| self.duration_sec + count)
    }

    /// Validate that SWOLF matches calculated value (if both present)
    pub fn validate_swolf(&self) -> bool {
        match (self.swolf, self.calculate_swolf()) {
            (Some(recorded), Some(calculated)) => recorded == calculated,
            _ => true, // If either is missing, no validation error
        }
    }
}

/// Swimming-specific data for a completed set
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SwimmingSetData {
    /// Individual lengths within this set/lap
    #[serde(default)]
    pub lengths: Vec<SwimmingLength>,

    /// Primary stroke type for the set (if all lengths same stroke)
    #[serde(default)]
    pub stroke_type: Option<StrokeType>,

    /// Total number of lengths in this set
    #[serde(default)]
    pub total_lengths: Option<u32>,

    /// Number of active lengths (excludes rest at wall)
    #[serde(default)]
    pub active_lengths: Option<u32>,

    /// Average SWOLF across all lengths in this set
    #[serde(default)]
    pub swolf_avg: Option<u32>,

    /// Whether this set was drill work (technique focus)
    #[serde(default)]
    pub drill_mode: Option<bool>,
}

impl SwimmingSetData {
    /// Calculate average SWOLF from lengths
    pub fn calculate_avg_swolf(&self) -> Option<u32> {
        if self.lengths.is_empty() {
            return None;
        }

        let swolf_values: Vec<u32> = self
            .lengths
            .iter()
            .filter_map(|l| l.swolf.or_else(|| l.calculate_swolf()))
            .collect();

        if swolf_values.is_empty() {
            return None;
        }

        let sum: u32 = swolf_values.iter().sum();
        Some(sum / swolf_values.len() as u32)
    }

    /// Count active lengths from length data
    pub fn count_active_lengths(&self) -> u32 {
        self.lengths
            .iter()
            .filter(|l| l.active.unwrap_or(true)) // Default to active if not specified
            .count() as u32
    }
}

/// A personal record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalRecord {
    pub exercise_name: String,
    pub record_type: RecordType,
    pub value: f64,
    #[serde(default)]
    pub unit: Option<String>,
    pub achieved_at: String,
    #[serde(default)]
    pub workout_id: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}

/// Type of personal record
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordType {
    #[serde(rename = "1rm")]
    OneRepMax,
    #[serde(rename = "max_weight_3rm")]
    MaxWeight3rm,
    #[serde(rename = "max_weight_5rm")]
    MaxWeight5rm,
    #[serde(rename = "max_weight_8rm")]
    MaxWeight8rm,
    #[serde(rename = "max_weight_10rm")]
    MaxWeight10rm,
    MaxWeight,
    MaxReps,
    MaxVolume,
    MaxDuration,
    MaxDistance,
    FastestTime,
}

/// A body measurement entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyMeasurement {
    pub date: String,
    #[serde(default)]
    pub recorded_at: Option<String>,
    #[serde(default)]
    pub weight_kg: Option<f64>,
    #[serde(default)]
    pub weight_lb: Option<f64>,
    #[serde(default)]
    pub body_fat_percent: Option<f64>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub measurements: Option<BodyDimensions>,
}

/// Body dimension measurements
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BodyDimensions {
    #[serde(default)]
    pub neck_cm: Option<f64>,
    #[serde(default)]
    pub shoulders_cm: Option<f64>,
    #[serde(default)]
    pub chest_cm: Option<f64>,
    #[serde(default)]
    pub waist_cm: Option<f64>,
    #[serde(default)]
    pub hips_cm: Option<f64>,
    #[serde(default)]
    pub bicep_left_cm: Option<f64>,
    #[serde(default)]
    pub bicep_right_cm: Option<f64>,
    #[serde(default)]
    pub forearm_left_cm: Option<f64>,
    #[serde(default)]
    pub forearm_right_cm: Option<f64>,
    #[serde(default)]
    pub thigh_left_cm: Option<f64>,
    #[serde(default)]
    pub thigh_right_cm: Option<f64>,
    #[serde(default)]
    pub calf_left_cm: Option<f64>,
    #[serde(default)]
    pub calf_right_cm: Option<f64>,
}

/// Statistics about a history export
#[derive(Debug, Clone, Default, Serialize)]
pub struct HistoryStatistics {
    pub total_workouts: usize,
    pub total_exercises: usize,
    pub total_sets: usize,
    pub total_volume_kg: f64,
    pub date_range_start: Option<String>,
    pub date_range_end: Option<String>,
    pub personal_records_count: usize,
    pub body_measurements_count: usize,
}

/// Information about a device used during the workout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Device index for multi-device workouts (e.g., 0=watch, 1=HRM, 2=power meter)
    #[serde(default)]
    pub device_index: Option<u8>,
    /// Type of device
    pub device_type: DeviceType,
    /// Device manufacturer
    pub manufacturer: Manufacturer,
    /// Specific product/model name
    #[serde(default)]
    pub product: Option<String>,
    /// Unique device serial number
    #[serde(default)]
    pub serial_number: Option<String>,
    /// Software/firmware version
    #[serde(default)]
    pub software_version: Option<String>,
    /// Hardware version
    #[serde(default)]
    pub hardware_version: Option<String>,
    /// Battery information
    #[serde(default)]
    pub battery: Option<BatteryInfo>,
    /// Cumulative operating time in hours
    #[serde(default)]
    pub cumulative_operating_time_hours: Option<f64>,
    /// Connection information for sensors
    #[serde(default)]
    pub connection: Option<ConnectionInfo>,
    /// Calibration information for sensors
    #[serde(default)]
    pub calibration: Option<CalibrationInfo>,
}

/// Type of device
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceType {
    /// GPS sports watch
    Watch,
    /// Bike computer (head unit)
    BikeComputer,
    /// Heart rate monitor
    HeartRateMonitor,
    /// Power meter (cycling)
    PowerMeter,
    /// Speed sensor
    SpeedSensor,
    /// Cadence sensor
    CadenceSensor,
    /// Speed and cadence combo sensor
    SpeedCadenceSensor,
    /// Foot pod (running dynamics)
    FootPod,
    /// Smart trainer (indoor cycling)
    SmartTrainer,
    /// Action camera
    Camera,
    /// Smartphone app
    Phone,
    /// Unknown or other device type
    Other,
}

/// Device manufacturer - supports both known manufacturers and custom strings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Manufacturer {
    /// Known manufacturer from standard list
    Known(KnownManufacturer),
    /// Custom manufacturer name for non-standard devices
    Custom(String),
}

/// Well-known device manufacturers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownManufacturer {
    Garmin,
    Wahoo,
    Polar,
    Suunto,
    Coros,
    Hammerhead,
    Stages,
    Sram,
    Shimano,
    Quarq,
    PowerTap,
    Stryd,
    Whoop,
    Apple,
    Samsung,
    Fitbit,
    /// Other known manufacturer
    Other,
}

/// Battery information for a device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryInfo {
    /// Battery level at start of workout (percentage)
    #[serde(default)]
    pub start_percent: Option<u8>,
    /// Battery level at end of workout (percentage)
    #[serde(default)]
    pub end_percent: Option<u8>,
    /// Battery voltage
    #[serde(default)]
    pub voltage: Option<f64>,
    /// Battery status indicator
    #[serde(default)]
    pub status: Option<BatteryStatus>,
}

/// Battery status indicator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatteryStatus {
    /// Battery level is good
    Good,
    /// Battery is low and needs charging
    Low,
    /// Battery is critically low
    Critical,
    /// Battery is charging
    Charging,
    /// Unknown battery status
    Unknown,
}

/// Connection information for wireless sensors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    /// Type of connection
    pub connection_type: ConnectionType,
    /// ANT+ device number (for ANT+ sensors)
    #[serde(default)]
    pub ant_device_number: Option<u32>,
    /// Bluetooth MAC address or identifier
    #[serde(default)]
    pub bluetooth_id: Option<String>,
}

/// Type of device connection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionType {
    /// Local device (watch/bike computer itself)
    Local,
    /// ANT+ wireless protocol
    AntPlus,
    /// Bluetooth Low Energy
    BluetoothLe,
    /// Standard Bluetooth
    Bluetooth,
    /// WiFi connection
    Wifi,
    /// USB wired connection
    Usb,
    /// Unknown connection type
    Unknown,
}

/// Calibration information for sensors (e.g., power meters)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationInfo {
    /// Calibration factor or zero offset
    #[serde(default)]
    pub calibration_factor: Option<f64>,
    /// Timestamp of last calibration
    #[serde(default)]
    pub last_calibrated: Option<String>,
    /// Auto-zero setting (for power meters)
    #[serde(default)]
    pub auto_zero_enabled: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DistanceUnit, Modality, WeightUnit};

    #[test]
    fn test_wps_history_round_trip() {
        let history = WpsHistory {
            history_version: 1,
            exported_at: "2025-01-15T12:00:00Z".to_string(),
            export_source: Some(ExportSource {
                app_name: Some("TestApp".to_string()),
                app_version: Some("1.0.0".to_string()),
                platform: Some("iOS".to_string()),
                preferred_units: Some(Units {
                    weight: WeightUnit::Lb,
                    distance: DistanceUnit::Miles,
                }),
            }),
            units: Units {
                weight: WeightUnit::Kg,
                distance: DistanceUnit::Meters,
            },
            workouts: vec![],
            personal_records: vec![],
            body_measurements: vec![],
        };

        let json = serde_json::to_string(&history).unwrap();
        let deserialized: WpsHistory = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.history_version, 1);
        assert_eq!(deserialized.exported_at, "2025-01-15T12:00:00Z");
        assert!(deserialized.export_source.is_some());
        assert_eq!(
            deserialized.export_source.as_ref().unwrap().app_name,
            Some("TestApp".to_string())
        );
    }

    #[test]
    fn test_wps_history_minimal() {
        let history = WpsHistory {
            history_version: 1,
            exported_at: "2025-01-15T12:00:00Z".to_string(),
            export_source: None,
            units: Units::default(),
            workouts: vec![],
            personal_records: vec![],
            body_measurements: vec![],
        };

        let json = serde_json::to_string(&history).unwrap();
        let deserialized: WpsHistory = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.history_version, 1);
        assert!(deserialized.export_source.is_none());
        assert_eq!(deserialized.units.weight, WeightUnit::Kg);
        assert_eq!(deserialized.units.distance, DistanceUnit::Meters);
    }

    #[test]
    fn test_export_source_with_preferred_units() {
        let source = ExportSource {
            app_name: Some("MyApp".to_string()),
            app_version: Some("2.0.0".to_string()),
            platform: Some("Android".to_string()),
            preferred_units: Some(Units {
                weight: WeightUnit::Lb,
                distance: DistanceUnit::Miles,
            }),
        };

        let json = serde_json::to_string(&source).unwrap();
        let deserialized: ExportSource = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.app_name, Some("MyApp".to_string()));
        assert_eq!(deserialized.app_version, Some("2.0.0".to_string()));
        assert_eq!(deserialized.platform, Some("Android".to_string()));
        assert!(deserialized.preferred_units.is_some());
        assert_eq!(deserialized.preferred_units.unwrap().weight, WeightUnit::Lb);
    }

    #[test]
    fn test_export_source_minimal() {
        let source = ExportSource {
            app_name: None,
            app_version: None,
            platform: None,
            preferred_units: None,
        };

        let json = serde_json::to_string(&source).unwrap();
        let deserialized: ExportSource = serde_json::from_str(&json).unwrap();

        assert!(deserialized.app_name.is_none());
        assert!(deserialized.app_version.is_none());
        assert!(deserialized.platform.is_none());
        assert!(deserialized.preferred_units.is_none());
    }

    #[test]
    fn test_units_default() {
        let units = Units::default();
        assert_eq!(units.weight, WeightUnit::Kg);
        assert_eq!(units.distance, DistanceUnit::Meters);

        let json = serde_json::to_string(&units).unwrap();
        let deserialized: Units = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.weight, WeightUnit::Kg);
        assert_eq!(deserialized.distance, DistanceUnit::Meters);
    }

    #[test]
    fn test_units_lb_miles() {
        let units = Units {
            weight: WeightUnit::Lb,
            distance: DistanceUnit::Miles,
        };

        let json = serde_json::to_string(&units).unwrap();
        let deserialized: Units = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.weight, WeightUnit::Lb);
        assert_eq!(deserialized.distance, DistanceUnit::Miles);
    }

    #[test]
    fn test_workout_full() {
        let workout = Workout {
            id: Some("workout-123".to_string()),
            date: "2025-01-15".to_string(),
            started_at: Some("2025-01-15T10:00:00Z".to_string()),
            ended_at: Some("2025-01-15T11:30:00Z".to_string()),
            duration_sec: Some(5400),
            title: Some("Leg Day".to_string()),
            notes: Some("Great session!".to_string()),
            plan_id: Some("plan-456".to_string()),
            plan_day_id: Some("day-789".to_string()),
            exercises: vec![],
            telemetry: None,
            devices: vec![],
            sport: None,
            sport_segments: None,
        };

        let json = serde_json::to_string(&workout).unwrap();
        let deserialized: Workout = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, Some("workout-123".to_string()));
        assert_eq!(deserialized.date, "2025-01-15");
        assert_eq!(deserialized.duration_sec, Some(5400));
        assert_eq!(deserialized.title, Some("Leg Day".to_string()));
    }

    #[test]
    fn test_workout_minimal() {
        let workout = Workout {
            id: None,
            date: "2025-01-15".to_string(),
            started_at: None,
            ended_at: None,
            duration_sec: None,
            title: None,
            notes: None,
            plan_id: None,
            plan_day_id: None,
            exercises: vec![],
            telemetry: None,
            devices: vec![],
            sport: None,
            sport_segments: None,
        };

        let json = serde_json::to_string(&workout).unwrap();
        let deserialized: Workout = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.date, "2025-01-15");
        assert!(deserialized.id.is_none());
        assert!(deserialized.title.is_none());
    }

    #[test]
    fn test_completed_exercise_with_sets() {
        let exercise = CompletedExercise {
            id: Some("ex-123".to_string()),
            name: "Bench Press".to_string(),
            modality: Some(Modality::Strength),
            notes: Some("Felt strong".to_string()),
            sets: vec![
                CompletedSet {
                    set_number: Some(1),
                    set_type: Some(SetType::Warmup),
                    reps: Some(10),
                    weight_kg: Some(60.0),
                    weight_lb: None,
                    duration_sec: None,
                    distance_meters: None,
                    rpe: None,
                    rir: None,
                    notes: None,
                    is_pr: Some(false),
                    completed_at: None,
                    telemetry: None,
                    swimming: None,
                },
                CompletedSet {
                    set_number: Some(2),
                    set_type: Some(SetType::Working),
                    reps: Some(8),
                    weight_kg: Some(100.0),
                    weight_lb: None,
                    duration_sec: None,
                    distance_meters: None,
                    rpe: Some(8.0),
                    rir: Some(2),
                    notes: None,
                    is_pr: Some(true),
                    completed_at: Some("2025-01-15T10:30:00Z".to_string()),
                    telemetry: None,
                    swimming: None,
                },
            ],
            pool_config: None,
            sport: None,
        };

        let json = serde_json::to_string(&exercise).unwrap();
        let deserialized: CompletedExercise = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "Bench Press");
        assert_eq!(deserialized.modality, Some(Modality::Strength));
        assert_eq!(deserialized.sets.len(), 2);
        assert_eq!(deserialized.sets[0].set_type, Some(SetType::Warmup));
        assert_eq!(deserialized.sets[1].rpe, Some(8.0));
        assert_eq!(deserialized.sets[1].rir, Some(2));
    }

    #[test]
    fn test_completed_set_all_fields() {
        let set = CompletedSet {
            set_number: Some(1),
            set_type: Some(SetType::Working),
            reps: Some(10),
            weight_kg: Some(100.0),
            weight_lb: Some(220.46),
            duration_sec: Some(30),
            distance_meters: Some(500.0),
            rpe: Some(8.5),
            rir: Some(2),
            notes: Some("Felt good".to_string()),
            is_pr: Some(true),
            completed_at: Some("2025-01-15T10:30:00Z".to_string()),
            telemetry: None,
            swimming: None,
        };

        let json = serde_json::to_string(&set).unwrap();
        let deserialized: CompletedSet = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.set_number, Some(1));
        assert_eq!(deserialized.set_type, Some(SetType::Working));
        assert_eq!(deserialized.reps, Some(10));
        assert_eq!(deserialized.weight_kg, Some(100.0));
        assert_eq!(deserialized.weight_lb, Some(220.46));
        assert_eq!(deserialized.duration_sec, Some(30));
        assert_eq!(deserialized.distance_meters, Some(500.0));
        assert_eq!(deserialized.rpe, Some(8.5));
        assert_eq!(deserialized.rir, Some(2));
        assert_eq!(deserialized.is_pr, Some(true));
    }

    #[test]
    fn test_completed_set_optional_fields_skipped() {
        let set = CompletedSet {
            set_number: None,
            set_type: None,
            reps: Some(10),
            weight_kg: Some(100.0),
            weight_lb: None,
            duration_sec: None,
            distance_meters: None,
            rpe: None,
            rir: None,
            notes: None,
            is_pr: None,
            completed_at: None,
            telemetry: None,
            swimming: None,
        };

        let json = serde_json::to_string(&set).unwrap();

        // Verify that None fields are serialized as null (serde default behavior)
        assert!(json.contains("\"set_number\":null"));
        assert!(json.contains("\"set_type\":null"));
        assert!(json.contains("\"weight_lb\":null"));
        assert!(json.contains("\"rpe\":null"));
        assert!(json.contains("\"rir\":null"));

        // Verify that present fields are properly serialized
        assert!(json.contains("\"reps\":10"));
        assert!(json.contains("\"weight_kg\":100.0"));

        let deserialized: CompletedSet = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.reps, Some(10));
        assert_eq!(deserialized.weight_kg, Some(100.0));
        assert!(deserialized.weight_lb.is_none());
        assert!(deserialized.rir.is_none());
        assert!(deserialized.rpe.is_none());
    }

    #[test]
    fn test_set_type_all_variants() {
        let variants = vec![
            (SetType::Working, "working"),
            (SetType::Warmup, "warmup"),
            (SetType::Dropset, "dropset"),
            (SetType::Failure, "failure"),
            (SetType::Amrap, "amrap"),
        ];

        for (variant, expected_json) in variants {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, format!("\"{}\"", expected_json));

            let deserialized: SetType = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, variant);
        }
    }

    #[test]
    fn test_set_type_default() {
        let default = SetType::default();
        assert_eq!(default, SetType::Working);
    }

    #[test]
    fn test_personal_record_1rm() {
        let pr = PersonalRecord {
            exercise_name: "Squat".to_string(),
            record_type: RecordType::OneRepMax,
            value: 200.0,
            unit: Some("kg".to_string()),
            achieved_at: "2025-01-15T10:30:00Z".to_string(),
            workout_id: Some("workout-123".to_string()),
            notes: Some("New PR!".to_string()),
        };

        let json = serde_json::to_string(&pr).unwrap();
        assert!(json.contains("\"1rm\""));

        let deserialized: PersonalRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.exercise_name, "Squat");
        assert_eq!(deserialized.record_type, RecordType::OneRepMax);
        assert_eq!(deserialized.value, 200.0);
    }

    #[test]
    fn test_record_type_all_variants() {
        let variants = vec![
            (RecordType::OneRepMax, "1rm"),
            (RecordType::MaxWeight3rm, "max_weight_3rm"),
            (RecordType::MaxWeight5rm, "max_weight_5rm"),
            (RecordType::MaxWeight8rm, "max_weight_8rm"),
            (RecordType::MaxWeight10rm, "max_weight_10rm"),
            (RecordType::MaxWeight, "max_weight"),
            (RecordType::MaxReps, "max_reps"),
            (RecordType::MaxVolume, "max_volume"),
            (RecordType::MaxDuration, "max_duration"),
            (RecordType::MaxDistance, "max_distance"),
            (RecordType::FastestTime, "fastest_time"),
        ];

        for (variant, expected_json) in variants {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, format!("\"{}\"", expected_json));

            let deserialized: RecordType = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, variant);
        }
    }

    #[test]
    fn test_personal_record_all_record_types() {
        let record_types = vec![
            RecordType::OneRepMax,
            RecordType::MaxWeight3rm,
            RecordType::MaxWeight5rm,
            RecordType::MaxWeight8rm,
            RecordType::MaxWeight10rm,
            RecordType::MaxWeight,
            RecordType::MaxReps,
            RecordType::MaxVolume,
            RecordType::MaxDuration,
            RecordType::MaxDistance,
            RecordType::FastestTime,
        ];

        for record_type in record_types {
            let pr = PersonalRecord {
                exercise_name: "Test Exercise".to_string(),
                record_type,
                value: 100.0,
                unit: Some("kg".to_string()),
                achieved_at: "2025-01-15T10:30:00Z".to_string(),
                workout_id: None,
                notes: None,
            };

            let json = serde_json::to_string(&pr).unwrap();
            let deserialized: PersonalRecord = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized.record_type, record_type);
        }
    }

    #[test]
    fn test_body_measurement_full() {
        let measurement = BodyMeasurement {
            date: "2025-01-15".to_string(),
            recorded_at: Some("2025-01-15T08:00:00Z".to_string()),
            weight_kg: Some(80.5),
            weight_lb: Some(177.47),
            body_fat_percent: Some(15.2),
            notes: Some("Morning weigh-in".to_string()),
            measurements: Some(BodyDimensions {
                neck_cm: Some(38.0),
                shoulders_cm: Some(120.0),
                chest_cm: Some(100.0),
                waist_cm: Some(85.0),
                hips_cm: Some(95.0),
                bicep_left_cm: Some(35.0),
                bicep_right_cm: Some(35.5),
                forearm_left_cm: Some(28.0),
                forearm_right_cm: Some(28.5),
                thigh_left_cm: Some(58.0),
                thigh_right_cm: Some(58.5),
                calf_left_cm: Some(38.0),
                calf_right_cm: Some(38.5),
            }),
        };

        let json = serde_json::to_string(&measurement).unwrap();
        let deserialized: BodyMeasurement = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.date, "2025-01-15");
        assert_eq!(deserialized.weight_kg, Some(80.5));
        assert_eq!(deserialized.body_fat_percent, Some(15.2));
        assert!(deserialized.measurements.is_some());

        let dims = deserialized.measurements.unwrap();
        assert_eq!(dims.chest_cm, Some(100.0));
        assert_eq!(dims.waist_cm, Some(85.0));
        assert_eq!(dims.bicep_left_cm, Some(35.0));
    }

    #[test]
    fn test_body_measurement_minimal() {
        let measurement = BodyMeasurement {
            date: "2025-01-15".to_string(),
            recorded_at: None,
            weight_kg: Some(80.0),
            weight_lb: None,
            body_fat_percent: None,
            notes: None,
            measurements: None,
        };

        let json = serde_json::to_string(&measurement).unwrap();
        let deserialized: BodyMeasurement = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.date, "2025-01-15");
        assert_eq!(deserialized.weight_kg, Some(80.0));
        assert!(deserialized.measurements.is_none());
    }

    #[test]
    fn test_body_dimensions_default() {
        let dimensions = BodyDimensions::default();

        assert!(dimensions.neck_cm.is_none());
        assert!(dimensions.chest_cm.is_none());
        assert!(dimensions.waist_cm.is_none());

        let json = serde_json::to_string(&dimensions).unwrap();
        let deserialized: BodyDimensions = serde_json::from_str(&json).unwrap();

        assert!(deserialized.neck_cm.is_none());
    }

    #[test]
    fn test_full_export_round_trip() {
        let history = WpsHistory {
            history_version: 1,
            exported_at: "2025-01-15T12:00:00Z".to_string(),
            export_source: Some(ExportSource {
                app_name: Some("TestApp".to_string()),
                app_version: Some("1.0.0".to_string()),
                platform: Some("iOS".to_string()),
                preferred_units: Some(Units {
                    weight: WeightUnit::Lb,
                    distance: DistanceUnit::Miles,
                }),
            }),
            units: Units::default(),
            workouts: vec![Workout {
                id: Some("w1".to_string()),
                date: "2025-01-15".to_string(),
                started_at: Some("2025-01-15T10:00:00Z".to_string()),
                ended_at: Some("2025-01-15T11:00:00Z".to_string()),
                duration_sec: Some(3600),
                title: Some("Push Day".to_string()),
                notes: None,
                plan_id: None,
                plan_day_id: None,
                exercises: vec![CompletedExercise {
                    id: Some("e1".to_string()),
                    name: "Bench Press".to_string(),
                    modality: Some(Modality::Strength),
                    notes: None,
                    sets: vec![CompletedSet {
                        set_number: Some(1),
                        set_type: Some(SetType::Working),
                        reps: Some(8),
                        weight_kg: Some(100.0),
                        weight_lb: None,
                        duration_sec: None,
                        distance_meters: None,
                        rpe: Some(8.0),
                        rir: Some(2),
                        notes: None,
                        is_pr: Some(true),
                        completed_at: None,
                        telemetry: None,
                        swimming: None,
                    }],
                    pool_config: None,
                    sport: None,
                }],
                telemetry: None,
                devices: vec![],
                sport: None,
                sport_segments: None,
            }],
            personal_records: vec![PersonalRecord {
                exercise_name: "Bench Press".to_string(),
                record_type: RecordType::OneRepMax,
                value: 140.0,
                unit: Some("kg".to_string()),
                achieved_at: "2025-01-15T10:30:00Z".to_string(),
                workout_id: Some("w1".to_string()),
                notes: None,
            }],
            body_measurements: vec![BodyMeasurement {
                date: "2025-01-15".to_string(),
                recorded_at: Some("2025-01-15T08:00:00Z".to_string()),
                weight_kg: Some(80.0),
                weight_lb: None,
                body_fat_percent: Some(15.0),
                notes: None,
                measurements: Some(BodyDimensions {
                    chest_cm: Some(100.0),
                    waist_cm: Some(85.0),
                    ..Default::default()
                }),
            }],
        };

        let json = serde_json::to_string_pretty(&history).unwrap();
        let deserialized: WpsHistory = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.history_version, 1);
        assert_eq!(deserialized.workouts.len(), 1);
        assert_eq!(deserialized.workouts[0].exercises.len(), 1);
        assert_eq!(deserialized.workouts[0].exercises[0].sets.len(), 1);
        assert_eq!(deserialized.personal_records.len(), 1);
        assert_eq!(deserialized.body_measurements.len(), 1);
        assert_eq!(
            deserialized.personal_records[0].record_type,
            RecordType::OneRepMax
        );
        assert_eq!(deserialized.workouts[0].exercises[0].sets[0].rir, Some(2));
    }

    // ===== PWF v2.1 Swimming Tests =====

    #[test]
    fn test_swimming_length_calculate_swolf() {
        let length = SwimmingLength {
            length_number: 1,
            stroke_type: StrokeType::Freestyle,
            duration_sec: 30,
            stroke_count: Some(15),
            swolf: None,
            started_at: None,
            active: Some(true),
        };

        assert_eq!(length.calculate_swolf(), Some(45)); // 30 + 15
    }

    #[test]
    fn test_swimming_length_calculate_swolf_missing_count() {
        let length = SwimmingLength {
            length_number: 1,
            stroke_type: StrokeType::Freestyle,
            duration_sec: 30,
            stroke_count: None,
            swolf: None,
            started_at: None,
            active: Some(true),
        };

        assert_eq!(length.calculate_swolf(), None);
    }

    #[test]
    fn test_swimming_length_validate_swolf_correct() {
        let length = SwimmingLength {
            length_number: 1,
            stroke_type: StrokeType::Freestyle,
            duration_sec: 30,
            stroke_count: Some(15),
            swolf: Some(45),
            started_at: None,
            active: Some(true),
        };

        assert!(length.validate_swolf());
    }

    #[test]
    fn test_swimming_length_validate_swolf_incorrect() {
        let length = SwimmingLength {
            length_number: 1,
            stroke_type: StrokeType::Freestyle,
            duration_sec: 30,
            stroke_count: Some(15),
            swolf: Some(50), // Wrong! Should be 45
            started_at: None,
            active: Some(true),
        };

        assert!(!length.validate_swolf());
    }

    #[test]
    fn test_swimming_length_validate_swolf_missing() {
        let length = SwimmingLength {
            length_number: 1,
            stroke_type: StrokeType::Freestyle,
            duration_sec: 30,
            stroke_count: Some(15),
            swolf: None,
            started_at: None,
            active: Some(true),
        };

        assert!(length.validate_swolf()); // No validation error if SWOLF missing
    }

    #[test]
    fn test_pool_config_length_in_meters_yards() {
        let pool = PoolConfig {
            pool_length: 25.0,
            pool_length_unit: PoolLengthUnit::Yards,
        };

        assert!((pool.length_in_meters() - 22.86).abs() < 0.01); // 25 yards = 22.86m
    }

    #[test]
    fn test_pool_config_length_in_meters_meters() {
        let pool = PoolConfig {
            pool_length: 50.0,
            pool_length_unit: PoolLengthUnit::Meters,
        };

        assert_eq!(pool.length_in_meters(), 50.0);
    }

    #[test]
    fn test_pool_config_presets() {
        assert_eq!(PoolConfig::pool_25m().pool_length, 25.0);
        assert_eq!(PoolConfig::pool_25m().pool_length_unit, PoolLengthUnit::Meters);

        assert_eq!(PoolConfig::pool_50m().pool_length, 50.0);
        assert_eq!(PoolConfig::pool_50m().pool_length_unit, PoolLengthUnit::Meters);

        assert_eq!(PoolConfig::pool_25yd().pool_length, 25.0);
        assert_eq!(PoolConfig::pool_25yd().pool_length_unit, PoolLengthUnit::Yards);
    }

    #[test]
    fn test_swimming_set_data_calculate_avg_swolf() {
        let set_data = SwimmingSetData {
            lengths: vec![
                SwimmingLength {
                    length_number: 1,
                    stroke_type: StrokeType::Freestyle,
                    duration_sec: 30,
                    stroke_count: Some(15),
                    swolf: Some(45),
                    started_at: None,
                    active: Some(true),
                },
                SwimmingLength {
                    length_number: 2,
                    stroke_type: StrokeType::Freestyle,
                    duration_sec: 32,
                    stroke_count: Some(16),
                    swolf: Some(48),
                    started_at: None,
                    active: Some(true),
                },
            ],
            stroke_type: Some(StrokeType::Freestyle),
            total_lengths: Some(2),
            active_lengths: Some(2),
            swolf_avg: None,
            drill_mode: Some(false),
        };

        assert_eq!(set_data.calculate_avg_swolf(), Some(46)); // (45 + 48) / 2 = 46
    }

    #[test]
    fn test_swimming_set_data_calculate_avg_swolf_empty() {
        let set_data = SwimmingSetData {
            lengths: vec![],
            stroke_type: None,
            total_lengths: None,
            active_lengths: None,
            swolf_avg: None,
            drill_mode: None,
        };

        assert_eq!(set_data.calculate_avg_swolf(), None);
    }

    #[test]
    fn test_swimming_set_data_count_active_lengths() {
        let set_data = SwimmingSetData {
            lengths: vec![
                SwimmingLength {
                    length_number: 1,
                    stroke_type: StrokeType::Freestyle,
                    duration_sec: 30,
                    stroke_count: Some(15),
                    swolf: Some(45),
                    started_at: None,
                    active: Some(true),
                },
                SwimmingLength {
                    length_number: 2,
                    stroke_type: StrokeType::Freestyle,
                    duration_sec: 40,
                    stroke_count: Some(20),
                    swolf: Some(60),
                    started_at: None,
                    active: Some(false), // Drill/rest length
                },
                SwimmingLength {
                    length_number: 3,
                    stroke_type: StrokeType::Freestyle,
                    duration_sec: 32,
                    stroke_count: Some(16),
                    swolf: Some(48),
                    started_at: None,
                    active: Some(true),
                },
            ],
            stroke_type: Some(StrokeType::Freestyle),
            total_lengths: Some(3),
            active_lengths: None,
            swolf_avg: None,
            drill_mode: Some(false),
        };

        assert_eq!(set_data.count_active_lengths(), 2);
    }

    #[test]
    fn test_stroke_type_serde() {
        assert_eq!(
            serde_json::to_string(&StrokeType::Freestyle).unwrap(),
            "\"freestyle\""
        );
        assert_eq!(
            serde_json::to_string(&StrokeType::IndividualMedley).unwrap(),
            "\"im\""
        );
        assert_eq!(
            serde_json::from_str::<StrokeType>("\"butterfly\"").unwrap(),
            StrokeType::Butterfly
        );
    }

    // ===== PWF v2.1 Time-Series Tests =====

    #[test]
    fn test_time_series_validate_lengths_valid() {
        let ts = TimeSeriesData {
            timestamps: vec![
                "2025-01-01T10:00:00Z".to_string(),
                "2025-01-01T10:00:01Z".to_string(),
                "2025-01-01T10:00:02Z".to_string(),
            ],
            elapsed_sec: Some(vec![0, 1, 2]),
            heart_rate: Some(vec![120, 125, 130]),
            power: Some(vec![200, 210, 220]),
            ..Default::default()
        };

        assert!(ts.validate_lengths().is_ok());
    }

    #[test]
    fn test_time_series_validate_lengths_mismatch() {
        let ts = TimeSeriesData {
            timestamps: vec![
                "2025-01-01T10:00:00Z".to_string(),
                "2025-01-01T10:00:01Z".to_string(),
                "2025-01-01T10:00:02Z".to_string(),
            ],
            elapsed_sec: Some(vec![0, 1]), // Wrong length!
            heart_rate: Some(vec![120, 125, 130]),
            ..Default::default()
        };

        assert!(ts.validate_lengths().is_err());
        let err = ts.validate_lengths().unwrap_err();
        assert!(err.contains("elapsed_sec"));
        assert!(err.contains("doesn't match timestamps length"));
    }

    #[test]
    fn test_time_series_len_and_is_empty() {
        let ts = TimeSeriesData {
            timestamps: vec![
                "2025-01-01T10:00:00Z".to_string(),
                "2025-01-01T10:00:01Z".to_string(),
            ],
            ..Default::default()
        };

        assert_eq!(ts.len(), 2);
        assert!(!ts.is_empty());

        let empty_ts = TimeSeriesData::default();
        assert_eq!(empty_ts.len(), 0);
        assert!(empty_ts.is_empty());
    }

    #[test]
    fn test_time_series_duration_sec() {
        let ts = TimeSeriesData {
            timestamps: vec![
                "2025-01-01T10:00:00Z".to_string(),
                "2025-01-01T10:01:00Z".to_string(),
                "2025-01-01T10:02:00Z".to_string(),
            ],
            elapsed_sec: Some(vec![0, 60, 120]),
            ..Default::default()
        };

        assert_eq!(ts.duration_sec(), Some(120));
    }

    #[test]
    fn test_time_series_duration_sec_no_elapsed() {
        let ts = TimeSeriesData {
            timestamps: vec![
                "2025-01-01T10:00:00Z".to_string(),
                "2025-01-01T10:01:00Z".to_string(),
            ],
            elapsed_sec: None,
            ..Default::default()
        };

        assert_eq!(ts.duration_sec(), None);
    }

    // ===== PWF v2.1 Advanced Metrics Tests =====

    #[test]
    fn test_training_status_serde() {
        assert_eq!(
            serde_json::to_string(&TrainingStatus::Productive).unwrap(),
            "\"productive\""
        );
        assert_eq!(
            serde_json::from_str::<TrainingStatus>("\"peaking\"").unwrap(),
            TrainingStatus::Peaking
        );
    }

    #[test]
    fn test_gps_fix_serde() {
        assert_eq!(
            serde_json::to_string(&GpsFix::Fix3D).unwrap(),
            "\"fix_3d\""
        );
        assert_eq!(
            serde_json::from_str::<GpsFix>("\"dgps\"").unwrap(),
            GpsFix::Dgps
        );
    }

    // ===== PWF v2.1 Integration Tests =====

    #[test]
    fn test_workout_with_swimming_data() {
        let workout = Workout {
            id: Some("swim-1".to_string()),
            date: "2025-01-15".to_string(),
            started_at: Some("2025-01-15T10:00:00Z".to_string()),
            ended_at: Some("2025-01-15T11:00:00Z".to_string()),
            duration_sec: Some(3600),
            title: Some("Pool Swim".to_string()),
            notes: None,
            plan_id: None,
            plan_day_id: None,
            exercises: vec![CompletedExercise {
                id: Some("ex-1".to_string()),
                name: "Freestyle".to_string(),
                modality: Some(Modality::Swimming),
                notes: None,
                sets: vec![CompletedSet {
                    set_number: Some(1),
                    set_type: None,
                    reps: None,
                    weight_kg: None,
                    weight_lb: None,
                    duration_sec: Some(120),
                    distance_meters: Some(100.0),
                    rpe: None,
                    rir: None,
                    notes: None,
                    is_pr: None,
                    completed_at: None,
                    telemetry: None,
                    swimming: Some(SwimmingSetData {
                        lengths: vec![SwimmingLength {
                            length_number: 1,
                            stroke_type: StrokeType::Freestyle,
                            duration_sec: 30,
                            stroke_count: Some(15),
                            swolf: Some(45),
                            started_at: None,
                            active: Some(true),
                        }],
                        stroke_type: Some(StrokeType::Freestyle),
                        total_lengths: Some(4),
                        active_lengths: Some(4),
                        swolf_avg: Some(45),
                        drill_mode: Some(false),
                    }),
                }],
                pool_config: Some(PoolConfig::pool_25m()),
                sport: Some(Sport::Swimming),
            }],
            telemetry: None,
            devices: vec![],
            sport: Some(Sport::Swimming),
            sport_segments: None,
        };

        let json = serde_json::to_string(&workout).unwrap();
        let deserialized: Workout = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.sport, Some(Sport::Swimming));
        assert!(deserialized.exercises[0].pool_config.is_some());
        assert!(deserialized.exercises[0].sets[0].swimming.is_some());
        assert_eq!(
            deserialized.exercises[0].sets[0]
                .swimming
                .as_ref()
                .unwrap()
                .lengths
                .len(),
            1
        );
    }

    #[test]
    fn test_workout_with_time_series_data() {
        let workout = Workout {
            id: Some("ride-1".to_string()),
            date: "2025-01-15".to_string(),
            started_at: Some("2025-01-15T10:00:00Z".to_string()),
            ended_at: None,
            duration_sec: Some(3600),
            title: Some("Bike Ride".to_string()),
            notes: None,
            plan_id: None,
            plan_day_id: None,
            exercises: vec![CompletedExercise {
                id: Some("ex-1".to_string()),
                name: "Interval".to_string(),
                modality: Some(Modality::Interval),
                notes: None,
                sets: vec![CompletedSet {
                    set_number: Some(1),
                    set_type: Some(SetType::Working),
                    reps: None,
                    weight_kg: None,
                    weight_lb: None,
                    duration_sec: Some(600),
                    distance_meters: Some(5000.0),
                    rpe: Some(8.0),
                    rir: None,
                    notes: None,
                    is_pr: None,
                    completed_at: None,
                    telemetry: Some(SetTelemetry {
                        heart_rate_avg: Some(165),
                        power_avg: Some(250),
                        time_series: Some(TimeSeriesData {
                            timestamps: vec![
                                "2025-01-15T10:00:00Z".to_string(),
                                "2025-01-15T10:00:01Z".to_string(),
                            ],
                            elapsed_sec: Some(vec![0, 1]),
                            heart_rate: Some(vec![160, 165]),
                            power: Some(vec![245, 255]),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    swimming: None,
                }],
                pool_config: None,
                sport: Some(Sport::Cycling),
            }],
            telemetry: None,
            devices: vec![],
            sport: Some(Sport::Cycling),
            sport_segments: None,
        };

        let json = serde_json::to_string(&workout).unwrap();
        let deserialized: Workout = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.sport, Some(Sport::Cycling));
        let ts = deserialized.exercises[0].sets[0]
            .telemetry
            .as_ref()
            .unwrap()
            .time_series
            .as_ref()
            .unwrap();
        assert_eq!(ts.len(), 2);
        assert!(ts.validate_lengths().is_ok());
    }
}
