/**
 * Swimming stroke type for pool and open water swimming
 */
export type StrokeType = "freestyle" | "backstroke" | "breaststroke" | "butterfly" | "drill" | "mixed" | "im";

/**
 * Schema for validating PWF workout history exports
 */
export interface PWFHistoryExportV1 {
  /**
   * Specification version (must be 1)
   */
  history_version: 1;
  /**
   * ISO 8601 datetime when export was created
   */
  exported_at: string;
  export_source?: ExportSource;
  units?: Units1;
  /**
   * Completed workout sessions
   */
  workouts: Workout[];
  /**
   * Personal records achieved
   */
  personal_records?: PersonalRecord[];
  /**
   * Body measurements recorded
   */
  body_measurements?: BodyMeasurement[];
}
export interface ExportSource {
  /**
   * Application name
   */
  app_name?: string;
  /**
   * Application version
   */
  app_version?: string;
  /**
   * Platform
   */
  platform?: "ios" | "android" | "web" | "desktop";
  preferred_units?: Units;
}
/**
 * User's preferred units
 */
export interface Units {
  weight?: "kg" | "lb";
  distance?: "meters" | "kilometers" | "miles" | "feet" | "yards";
}
export interface Units1 {
  weight?: "kg" | "lb";
  distance?: "meters" | "kilometers" | "miles" | "feet" | "yards";
}
export interface Workout {
  /**
   * Unique workout identifier
   */
  id?: string;
  /**
   * Workout date (YYYY-MM-DD)
   */
  date: string;
  /**
   * Start timestamp
   */
  started_at?: string;
  /**
   * End timestamp
   */
  ended_at?: string;
  /**
   * Total duration in seconds
   */
  duration_sec?: number;
  /**
   * Workout title
   */
  title?: string;
  /**
   * Workout notes
   */
  notes?: string;
  /**
   * Reference to PWF plan
   */
  plan_id?: string;
  /**
   * Reference to plan day
   */
  plan_day_id?: string;
  /**
   * Exercises performed
   */
  exercises: CompletedExercise[];
  telemetry?: WorkoutTelemetry;
  /**
   * Devices used during workout (PWF v2)
   */
  devices?: DeviceInfo[];
  /**
   * Primary sport for this workout (PWF v2.1)
   */
  sport?:
    | "swimming"
    | "cycling"
    | "running"
    | "rowing"
    | "transition"
    | "strength"
    | "strength-training"
    | "hiking"
    | "walking"
    | "yoga"
    | "pilates"
    | "functional-fitness"
    | "calisthenics"
    | "cardio"
    | "cross-country-skiing"
    | "downhill-skiing"
    | "snowboarding"
    | "stand-up-paddling"
    | "kayaking"
    | "elliptical"
    | "stair-climbing"
    | "other";
  /**
   * Sport segments for multi-sport workouts like triathlon (PWF v2.1)
   */
  sport_segments?: SportSegment[];
}
export interface CompletedExercise {
  /**
   * Unique exercise identifier
   */
  id?: string;
  /**
   * Exercise name
   */
  name: string;
  /**
   * Exercise modality
   */
  modality?: "strength" | "countdown" | "stopwatch" | "interval" | "swimming";
  /**
   * Exercise-level notes
   */
  notes?: string;
  /**
   * Completed sets
   */
  sets: CompletedSet[];
  pool_config?: PoolConfig;
  /**
   * Sport classification for this exercise (PWF v2.1)
   */
  sport?:
    | "swimming"
    | "cycling"
    | "running"
    | "rowing"
    | "transition"
    | "strength"
    | "strength-training"
    | "hiking"
    | "walking"
    | "yoga"
    | "pilates"
    | "functional-fitness"
    | "calisthenics"
    | "cardio"
    | "cross-country-skiing"
    | "downhill-skiing"
    | "snowboarding"
    | "stand-up-paddling"
    | "kayaking"
    | "elliptical"
    | "stair-climbing"
    | "other";
}
export interface CompletedSet {
  /**
   * Set order (1-indexed)
   */
  set_number?: number;
  /**
   * Type of set
   */
  set_type?: "working" | "warmup" | "dropset" | "failure" | "amrap";
  /**
   * Repetitions completed
   */
  reps?: number;
  /**
   * Weight in kilograms
   */
  weight_kg?: number;
  /**
   * Weight in pounds
   */
  weight_lb?: number;
  /**
   * Duration in seconds
   */
  duration_sec?: number;
  /**
   * Distance in meters
   */
  distance_meters?: number;
  /**
   * Rate of Perceived Exertion (1-10 scale)
   */
  rpe?: number;
  /**
   * Reps in Reserve (alternative to RPE)
   */
  rir?: number;
  /**
   * Set-level notes
   */
  notes?: string;
  /**
   * Whether this set was a personal record
   */
  is_pr?: boolean;
  /**
   * When set was completed
   */
  completed_at?: string;
  telemetry?: SetTelemetry;
  swimming?: SwimmingSetData;
}
/**
 * Telemetry metrics for this set (PWF v2)
 */
export interface SetTelemetry {
  /**
   * Average heart rate (bpm)
   */
  heart_rate_avg?: number;
  /**
   * Maximum heart rate (bpm)
   */
  heart_rate_max?: number;
  /**
   * Minimum heart rate (bpm)
   */
  heart_rate_min?: number;
  /**
   * Average power (watts)
   */
  power_avg?: number;
  /**
   * Maximum power (watts)
   */
  power_max?: number;
  /**
   * Minimum power (watts)
   */
  power_min?: number;
  /**
   * Elevation gain in meters
   */
  elevation_gain_m?: number;
  /**
   * Elevation gain in feet
   */
  elevation_gain_ft?: number;
  /**
   * Elevation loss in meters
   */
  elevation_loss_m?: number;
  /**
   * Elevation loss in feet
   */
  elevation_loss_ft?: number;
  /**
   * Average speed in m/s
   */
  speed_avg_mps?: number;
  /**
   * Average speed in km/h
   */
  speed_avg_kph?: number;
  /**
   * Average speed in mph
   */
  speed_avg_mph?: number;
  /**
   * Maximum speed in m/s
   */
  speed_max_mps?: number;
  /**
   * Maximum speed in km/h
   */
  speed_max_kph?: number;
  /**
   * Maximum speed in mph
   */
  speed_max_mph?: number;
  /**
   * Average pace in seconds per km
   */
  pace_avg_sec_per_km?: number;
  /**
   * Average pace in seconds per mile
   */
  pace_avg_sec_per_mi?: number;
  /**
   * Average cadence (RPM or SPM)
   */
  cadence_avg?: number;
  /**
   * Maximum cadence (RPM or SPM)
   */
  cadence_max?: number;
  /**
   * Temperature in Celsius
   */
  temperature_c?: number;
  /**
   * Temperature in Fahrenheit
   */
  temperature_f?: number;
  /**
   * Humidity percentage
   */
  humidity_percent?: number;
  /**
   * Calories burned in this set
   */
  calories?: number;
  /**
   * Stroke rate for swimming/rowing (strokes per minute)
   */
  stroke_rate?: number;
  /**
   * GPS route identifier
   */
  gps_route_id?: string;
  time_series?: TimeSeriesData;
}
/**
 * Second-by-second time-series data (PWF v2.1)
 */
export interface TimeSeriesData {
  /**
   * Timestamps for each record (ISO 8601). All other arrays must match this length.
   */
  timestamps: string[];
  /**
   * Elapsed time in seconds since start
   */
  elapsed_sec?: number[];
  /**
   * Heart rate readings (bpm)
   */
  heart_rate?: number[];
  /**
   * Power readings (watts)
   */
  power?: number[];
  /**
   * Cadence readings (RPM for cycling, SPM for running/swimming)
   */
  cadence?: number[];
  /**
   * Speed readings (meters per second)
   */
  speed_mps?: number[];
  /**
   * Distance readings (cumulative meters)
   */
  distance_m?: number[];
  /**
   * Elevation/altitude readings (meters)
   */
  elevation_m?: number[];
  /**
   * Temperature readings (Celsius)
   */
  temperature_c?: number[];
  /**
   * Latitude readings (decimal degrees)
   */
  latitude?: number[];
  /**
   * Longitude readings (decimal degrees)
   */
  longitude?: number[];
  /**
   * Grade/slope readings (percentage)
   */
  grade_percent?: number[];
  /**
   * Respiration rate (breaths per minute)
   */
  respiration_rate?: number[];
  /**
   * Core body temperature (Celsius)
   */
  core_temperature_c?: number[];
  /**
   * Muscle oxygen saturation (percentage)
   */
  muscle_oxygen_percent?: number[];
  /**
   * Left/right power balance (percentage left)
   */
  power_balance?: number[];
  /**
   * Left pedal smoothness (percentage)
   */
  left_pedal_smoothness?: number[];
  /**
   * Right pedal smoothness (percentage)
   */
  right_pedal_smoothness?: number[];
  /**
   * Left torque effectiveness (percentage)
   */
  left_torque_effectiveness?: number[];
  /**
   * Right torque effectiveness (percentage)
   */
  right_torque_effectiveness?: number[];
  /**
   * Running stride length (meters)
   */
  stride_length_m?: number[];
  /**
   * Running vertical oscillation (centimeters)
   */
  vertical_oscillation_cm?: number[];
  /**
   * Running ground contact time (milliseconds)
   */
  ground_contact_time_ms?: number[];
  /**
   * Running ground contact balance (percentage left)
   */
  ground_contact_balance?: number[];
  /**
   * Swimming stroke rate (strokes per minute)
   */
  stroke_rate?: number[];
  /**
   * Swimming stroke count (cumulative)
   */
  stroke_count?: number[];
  /**
   * Swimming SWOLF score
   */
  swolf?: number[];
  /**
   * Swimming stroke type at each point
   */
  stroke_type?: StrokeType[];
}
/**
 * Swimming-specific data for this set (PWF v2.1)
 */
export interface SwimmingSetData {
  /**
   * Individual lengths within this set/lap
   */
  lengths?: SwimmingLength[];
  /**
   * Swimming stroke type for pool and open water swimming
   */
  stroke_type?: "freestyle" | "backstroke" | "breaststroke" | "butterfly" | "drill" | "mixed" | "im";
  /**
   * Total number of lengths in this set
   */
  total_lengths?: number;
  /**
   * Number of active lengths (excludes rest at wall)
   */
  active_lengths?: number;
  /**
   * Average SWOLF across all lengths in this set
   */
  swolf_avg?: number;
  /**
   * Whether this set was drill work (technique focus)
   */
  drill_mode?: boolean;
}
/**
 * A single length (one pool length) within a swimming set/lap (PWF v2.1)
 */
export interface SwimmingLength {
  /**
   * Length number within the set (1-indexed)
   */
  length_number: number;
  /**
   * Swimming stroke type for pool and open water swimming
   */
  stroke_type: "freestyle" | "backstroke" | "breaststroke" | "butterfly" | "drill" | "mixed" | "im";
  /**
   * Duration of this length in seconds
   */
  duration_sec: number;
  /**
   * Number of strokes taken during this length
   */
  stroke_count?: number;
  /**
   * SWOLF score (duration + stroke_count) - lower is better
   */
  swolf?: number;
  /**
   * Timestamp when this length started (ISO 8601)
   */
  started_at?: string;
  /**
   * Whether this was an active length (vs. rest at wall)
   */
  active?: boolean;
}
/**
 * Pool configuration for swimming exercises (PWF v2.1)
 */
export interface PoolConfig {
  /**
   * Length of the pool in the specified units
   */
  pool_length: number;
  /**
   * Unit for pool length (meters or yards)
   */
  pool_length_unit?: "meters" | "yards";
}
/**
 * Telemetry metrics for entire workout session (PWF v2)
 */
export interface WorkoutTelemetry {
  /**
   * Average heart rate (bpm)
   */
  heart_rate_avg?: number;
  /**
   * Maximum heart rate (bpm)
   */
  heart_rate_max?: number;
  /**
   * Minimum heart rate (bpm)
   */
  heart_rate_min?: number;
  /**
   * Average power (watts)
   */
  power_avg?: number;
  /**
   * Maximum power (watts)
   */
  power_max?: number;
  /**
   * Total distance in meters
   */
  total_distance_m?: number;
  /**
   * Total distance in kilometers
   */
  total_distance_km?: number;
  /**
   * Total distance in miles
   */
  total_distance_mi?: number;
  /**
   * Total elevation gain in meters
   */
  total_elevation_gain_m?: number;
  /**
   * Total elevation gain in feet
   */
  total_elevation_gain_ft?: number;
  /**
   * Total elevation loss in meters
   */
  total_elevation_loss_m?: number;
  /**
   * Total elevation loss in feet
   */
  total_elevation_loss_ft?: number;
  /**
   * Average speed in km/h
   */
  speed_avg_kph?: number;
  /**
   * Average speed in mph
   */
  speed_avg_mph?: number;
  /**
   * Maximum speed in km/h
   */
  speed_max_kph?: number;
  /**
   * Maximum speed in mph
   */
  speed_max_mph?: number;
  /**
   * Average pace in seconds per km
   */
  pace_avg_sec_per_km?: number;
  /**
   * Average pace in seconds per mile
   */
  pace_avg_sec_per_mi?: number;
  /**
   * Average cadence (RPM or SPM)
   */
  cadence_avg?: number;
  /**
   * Temperature in Celsius
   */
  temperature_c?: number;
  /**
   * Temperature in Fahrenheit
   */
  temperature_f?: number;
  /**
   * Humidity percentage
   */
  humidity_percent?: number;
  /**
   * Total calories burned
   */
  total_calories?: number;
  /**
   * GPS route identifier
   */
  gps_route_id?: string;
  gps_route?: GpsRoute;
  advanced_metrics?: AdvancedMetrics;
  power_metrics?: PowerMetrics;
  time_in_zones?: TimeInZones;
}
/**
 * Full GPS route data (PWF v2.1)
 */
export interface GpsRoute {
  /**
   * Unique identifier for this route
   */
  route_id: string;
  /**
   * Human-readable route name
   */
  name?: string;
  /**
   * GPS positions in chronological order
   */
  positions: GpsPosition[];
  /**
   * Total distance calculated from GPS (meters)
   */
  total_distance_m?: number;
  /**
   * Total elevation gain (meters)
   */
  total_ascent_m?: number;
  /**
   * Total elevation loss (meters)
   */
  total_descent_m?: number;
  /**
   * Minimum elevation on route (meters)
   */
  min_elevation_m?: number;
  /**
   * Maximum elevation on route (meters)
   */
  max_elevation_m?: number;
  /**
   * Bounding box - southwest corner latitude
   */
  bbox_sw_lat?: number;
  /**
   * Bounding box - southwest corner longitude
   */
  bbox_sw_lng?: number;
  /**
   * Bounding box - northeast corner latitude
   */
  bbox_ne_lat?: number;
  /**
   * Bounding box - northeast corner longitude
   */
  bbox_ne_lng?: number;
  /**
   * Recording mode (e.g., auto, smart, 1s, gps_only)
   */
  recording_mode?: string;
  /**
   * GPS fix quality indicator
   */
  gps_fix?: "none" | "fix_2d" | "fix_3d" | "dgps" | "unknown";
}
/**
 * A single GPS position/waypoint with timestamp (PWF v2.1)
 */
export interface GpsPosition {
  /**
   * Latitude in decimal degrees (WGS84)
   */
  latitude_deg: number;
  /**
   * Longitude in decimal degrees (WGS84)
   */
  longitude_deg: number;
  /**
   * Timestamp when position was recorded (ISO 8601)
   */
  timestamp: string;
  /**
   * Elevation/altitude above sea level (meters)
   */
  elevation_m?: number;
  /**
   * Horizontal accuracy/uncertainty (meters)
   */
  accuracy_m?: number;
  /**
   * Speed at this point (meters per second)
   */
  speed_mps?: number;
  /**
   * Heading/bearing (degrees from north, 0-360)
   */
  heading_deg?: number;
  /**
   * Heart rate at this position (bpm)
   */
  heart_rate_bpm?: number;
  /**
   * Power at this position (watts)
   */
  power_watts?: number;
  /**
   * Cadence at this position (RPM or SPM)
   */
  cadence?: number;
  /**
   * Temperature at this position (Celsius)
   */
  temperature_c?: number;
}
/**
 * Advanced physiological metrics (PWF v2.1)
 */
export interface AdvancedMetrics {
  /**
   * Aerobic Training Effect score (0.0-5.0)
   */
  training_effect?: number;
  /**
   * Anaerobic Training Effect score (0.0-5.0)
   */
  anaerobic_training_effect?: number;
  /**
   * Recommended recovery time in hours
   */
  recovery_time_hours?: number;
  /**
   * VO2 Max estimate in ml/kg/min
   */
  vo2_max_estimate?: number;
  lactate_threshold?: LactateThreshold;
  /**
   * Real-time performance assessment (-20 to +20)
   */
  performance_condition?: number;
  /**
   * Cumulative training stress (0-1000+)
   */
  training_load?: number;
  /**
   * Training status assessment
   */
  training_status?: "detraining" | "recovery" | "maintaining" | "productive" | "peaking" | "overreaching" | "unknown";
}
/**
 * Lactate threshold data
 */
export interface LactateThreshold {
  /**
   * Heart rate at lactate threshold (bpm)
   */
  heart_rate_bpm?: number;
  /**
   * Speed at lactate threshold (m/s)
   */
  speed_mps?: number;
  /**
   * Power at lactate threshold (watts, for cycling)
   */
  power_watts?: number;
  /**
   * When threshold was detected/calculated
   */
  detected_at?: string;
}
/**
 * Power-based cycling metrics (PWF v2.1)
 */
export interface PowerMetrics {
  /**
   * Normalized Power (NP) - weighted average accounting for variability
   */
  normalized_power?: number;
  /**
   * Training Stress Score (TSS) - quantifies training load
   */
  training_stress_score?: number;
  /**
   * Intensity Factor (IF) - ratio of NP to FTP
   */
  intensity_factor?: number;
  /**
   * Variability Index (VI) - ratio of NP to average power
   */
  variability_index?: number;
  /**
   * Functional Threshold Power used for calculations (watts)
   */
  ftp_watts?: number;
  /**
   * Total work in kilojoules
   */
  total_work_kj?: number;
  /**
   * Left/right power balance (percentage left)
   */
  left_right_balance?: number;
  /**
   * Average left pedal smoothness (percentage)
   */
  left_pedal_smoothness?: number;
  /**
   * Average right pedal smoothness (percentage)
   */
  right_pedal_smoothness?: number;
  /**
   * Average left torque effectiveness (percentage)
   */
  left_torque_effectiveness?: number;
  /**
   * Average right torque effectiveness (percentage)
   */
  right_torque_effectiveness?: number;
}
/**
 * Time in HR/power zones (PWF v2.1)
 */
export interface TimeInZones {
  /**
   * Time in each HR zone (seconds per zone)
   */
  hr_zones_sec?: number[];
  /**
   * Time in each power zone (seconds per zone)
   */
  power_zones_sec?: number[];
  /**
   * HR zone boundaries in bpm
   */
  hr_zone_boundaries?: number[];
  /**
   * Power zone boundaries in watts
   */
  power_zone_boundaries?: number[];
  /**
   * Time in each pace zone (seconds per zone)
   */
  pace_zones_sec?: number[];
  /**
   * Pace zone boundaries (seconds per km)
   */
  pace_zone_boundaries?: number[];
}
/**
 * Information about a device used during the workout (PWF v2)
 */
export interface DeviceInfo {
  /**
   * Device index for multi-device workouts (e.g., 0=watch, 1=HRM, 2=power meter)
   */
  device_index?: number;
  /**
   * Type of device
   */
  device_type:
    | "watch"
    | "bike_computer"
    | "heart_rate_monitor"
    | "power_meter"
    | "speed_sensor"
    | "cadence_sensor"
    | "speed_cadence_sensor"
    | "foot_pod"
    | "smart_trainer"
    | "camera"
    | "phone"
    | "other";
  /**
   * Device manufacturer (can be a known manufacturer or custom string)
   */
  manufacturer: string;
  /**
   * Specific product/model name
   */
  product?: string;
  /**
   * Unique device serial number
   */
  serial_number?: string;
  /**
   * Software/firmware version
   */
  software_version?: string;
  /**
   * Hardware version
   */
  hardware_version?: string;
  battery?: BatteryInfo;
  /**
   * Cumulative operating time in hours
   */
  cumulative_operating_time_hours?: number;
  connection?: ConnectionInfo;
  calibration?: CalibrationInfo;
}
/**
 * Battery information
 */
export interface BatteryInfo {
  /**
   * Battery level at start of workout (percentage)
   */
  start_percent?: number;
  /**
   * Battery level at end of workout (percentage)
   */
  end_percent?: number;
  /**
   * Battery voltage
   */
  voltage?: number;
  /**
   * Battery status indicator
   */
  status?: "good" | "low" | "critical" | "charging" | "unknown";
}
/**
 * Connection information for sensors
 */
export interface ConnectionInfo {
  /**
   * Type of connection
   */
  connection_type: "local" | "ant_plus" | "bluetooth_le" | "bluetooth" | "wifi" | "usb" | "unknown";
  /**
   * ANT+ device number (for ANT+ sensors)
   */
  ant_device_number?: number;
  /**
   * Bluetooth MAC address or identifier
   */
  bluetooth_id?: string;
}
/**
 * Calibration information for sensors
 */
export interface CalibrationInfo {
  /**
   * Calibration factor or zero offset
   */
  calibration_factor?: number;
  /**
   * Timestamp of last calibration
   */
  last_calibrated?: string;
  /**
   * Auto-zero setting (for power meters)
   */
  auto_zero_enabled?: boolean;
}
/**
 * A segment within a multi-sport workout (PWF v2.1)
 */
export interface SportSegment {
  /**
   * Segment identifier
   */
  segment_id: string;
  /**
   * Sport for this segment
   */
  sport:
    | "swimming"
    | "cycling"
    | "running"
    | "rowing"
    | "transition"
    | "strength"
    | "strength-training"
    | "hiking"
    | "walking"
    | "yoga"
    | "pilates"
    | "functional-fitness"
    | "calisthenics"
    | "cardio"
    | "cross-country-skiing"
    | "downhill-skiing"
    | "snowboarding"
    | "stand-up-paddling"
    | "kayaking"
    | "elliptical"
    | "stair-climbing"
    | "other";
  /**
   * Segment number in sequence (0-indexed)
   */
  segment_index: number;
  /**
   * When segment started (ISO 8601)
   */
  started_at?: string;
  /**
   * Segment duration in seconds
   */
  duration_sec?: number;
  /**
   * Distance covered in this segment (meters)
   */
  distance_m?: number;
  /**
   * Exercises/sets completed during this segment
   */
  exercise_ids?: string[];
  telemetry?: WorkoutTelemetry1;
  transition?: TransitionData;
  /**
   * Notes specific to this segment
   */
  notes?: string;
}
/**
 * Telemetry specific to this segment
 */
export interface WorkoutTelemetry1 {
  /**
   * Average heart rate (bpm)
   */
  heart_rate_avg?: number;
  /**
   * Maximum heart rate (bpm)
   */
  heart_rate_max?: number;
  /**
   * Minimum heart rate (bpm)
   */
  heart_rate_min?: number;
  /**
   * Average power (watts)
   */
  power_avg?: number;
  /**
   * Maximum power (watts)
   */
  power_max?: number;
  /**
   * Total distance in meters
   */
  total_distance_m?: number;
  /**
   * Total distance in kilometers
   */
  total_distance_km?: number;
  /**
   * Total distance in miles
   */
  total_distance_mi?: number;
  /**
   * Total elevation gain in meters
   */
  total_elevation_gain_m?: number;
  /**
   * Total elevation gain in feet
   */
  total_elevation_gain_ft?: number;
  /**
   * Total elevation loss in meters
   */
  total_elevation_loss_m?: number;
  /**
   * Total elevation loss in feet
   */
  total_elevation_loss_ft?: number;
  /**
   * Average speed in km/h
   */
  speed_avg_kph?: number;
  /**
   * Average speed in mph
   */
  speed_avg_mph?: number;
  /**
   * Maximum speed in km/h
   */
  speed_max_kph?: number;
  /**
   * Maximum speed in mph
   */
  speed_max_mph?: number;
  /**
   * Average pace in seconds per km
   */
  pace_avg_sec_per_km?: number;
  /**
   * Average pace in seconds per mile
   */
  pace_avg_sec_per_mi?: number;
  /**
   * Average cadence (RPM or SPM)
   */
  cadence_avg?: number;
  /**
   * Temperature in Celsius
   */
  temperature_c?: number;
  /**
   * Temperature in Fahrenheit
   */
  temperature_f?: number;
  /**
   * Humidity percentage
   */
  humidity_percent?: number;
  /**
   * Total calories burned
   */
  total_calories?: number;
  /**
   * GPS route identifier
   */
  gps_route_id?: string;
  gps_route?: GpsRoute;
  advanced_metrics?: AdvancedMetrics;
  power_metrics?: PowerMetrics;
  time_in_zones?: TimeInZones;
}
/**
 * Transition data after this segment
 */
export interface TransitionData {
  /**
   * Transition identifier (e.g., T1, T2)
   */
  transition_id: string;
  /**
   * From which sport
   */
  from_sport:
    | "swimming"
    | "cycling"
    | "running"
    | "rowing"
    | "transition"
    | "strength"
    | "strength-training"
    | "hiking"
    | "walking"
    | "yoga"
    | "pilates"
    | "functional-fitness"
    | "calisthenics"
    | "cardio"
    | "cross-country-skiing"
    | "downhill-skiing"
    | "snowboarding"
    | "stand-up-paddling"
    | "kayaking"
    | "elliptical"
    | "stair-climbing"
    | "other";
  /**
   * To which sport
   */
  to_sport:
    | "swimming"
    | "cycling"
    | "running"
    | "rowing"
    | "transition"
    | "strength"
    | "strength-training"
    | "hiking"
    | "walking"
    | "yoga"
    | "pilates"
    | "functional-fitness"
    | "calisthenics"
    | "cardio"
    | "cross-country-skiing"
    | "downhill-skiing"
    | "snowboarding"
    | "stand-up-paddling"
    | "kayaking"
    | "elliptical"
    | "stair-climbing"
    | "other";
  /**
   * Transition duration in seconds
   */
  duration_sec?: number;
  /**
   * When transition started (ISO 8601)
   */
  started_at?: string;
  /**
   * Average heart rate during transition
   */
  heart_rate_avg?: number;
  /**
   * Notes about transition (e.g., equipment changes)
   */
  notes?: string;
}
export interface PersonalRecord {
  /**
   * Exercise name
   */
  exercise_name: string;
  /**
   * Type of record
   */
  record_type:
    | "1rm"
    | "max_weight_3rm"
    | "max_weight_5rm"
    | "max_weight_8rm"
    | "max_weight_10rm"
    | "max_weight"
    | "max_reps"
    | "max_volume"
    | "max_duration"
    | "max_distance"
    | "fastest_time";
  /**
   * Record value
   */
  value: number;
  /**
   * Unit for the value
   */
  unit?: string;
  /**
   * Date achieved
   */
  achieved_at: string;
  /**
   * Reference to workout
   */
  workout_id?: string;
  /**
   * Additional notes
   */
  notes?: string;
}
export interface BodyMeasurement {
  /**
   * Measurement date
   */
  date: string;
  /**
   * Exact timestamp
   */
  recorded_at?: string;
  /**
   * Body weight in kg
   */
  weight_kg?: number;
  /**
   * Body weight in lb
   */
  weight_lb?: number;
  /**
   * Body fat percentage
   */
  body_fat_percent?: number;
  /**
   * Notes
   */
  notes?: string;
  measurements?: BodyDimensions;
}
export interface BodyDimensions {
  neck_cm?: number;
  shoulders_cm?: number;
  chest_cm?: number;
  waist_cm?: number;
  hips_cm?: number;
  bicep_left_cm?: number;
  bicep_right_cm?: number;
  forearm_left_cm?: number;
  forearm_right_cm?: number;
  thigh_left_cm?: number;
  thigh_right_cm?: number;
  calf_left_cm?: number;
  calf_right_cm?: number;
}
