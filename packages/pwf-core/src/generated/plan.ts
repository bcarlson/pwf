/**
 * Portable Workout Format v1 schema for validating plan documents
 */
export interface PWFPlanV1 {
  /**
   * Specification version (must be 1)
   */
  plan_version: 1;
  meta?: Meta;
  /**
   * Term definitions for exercises and concepts used in this plan
   */
  glossary?: {
    [k: string]: string;
  };
  cycle: Cycle;
}
/**
 * Plan metadata for display and organization
 */
export interface Meta {
  /**
   * Unique plan identifier
   */
  id?: string;
  /**
   * Plan display name
   */
  title: string;
  /**
   * Brief plan description
   */
  description?: string;
  /**
   * Coach or creator name
   */
  author?: string;
  /**
   * Plan status
   */
  status?: "draft" | "active" | "completed" | "archived";
  /**
   * ISO 8601 timestamp when plan was activated
   */
  activated_at?: string;
  /**
   * ISO 8601 timestamp when plan was completed
   */
  completed_at?: string;
  /**
   * Required equipment tags
   */
  equipment?: string[];
  /**
   * Intended training frequency
   */
  daysPerWeek?: number;
  /**
   * Suggest as starter plan
   */
  recommendedFirst?: boolean;
  /**
   * Searchable tags
   */
  tags?: string[];
  athlete_profile?: AthleteProfile;
}
/**
 * Athlete metrics for endurance training
 */
export interface AthleteProfile {
  /**
   * Functional Threshold Power in watts
   */
  ftp_watts?: number;
  /**
   * Threshold heart rate in BPM
   */
  threshold_hr_bpm?: number;
  /**
   * Maximum heart rate in BPM
   */
  max_hr_bpm?: number;
  /**
   * Threshold pace in seconds per kilometer
   */
  threshold_pace_sec_per_km?: number;
  /**
   * Athlete weight in kilograms
   */
  weight_kg?: number;
}
/**
 * Training cycle containing workout days
 */
export interface Cycle {
  /**
   * ISO 8601 date (YYYY-MM-DD)
   */
  start_date?: string;
  /**
   * Cycle-level coaching notes
   */
  notes?: string;
  /**
   * Training days in this cycle
   *
   * @minItems 1
   */
  days: [Day, ...Day[]];
}
/**
 * Single training day
 */
export interface Day {
  /**
   * Unique day identifier
   */
  id?: string;
  /**
   * Day sequence (0-indexed)
   */
  order?: number;
  /**
   * Training focus or theme
   */
  focus?: string;
  /**
   * Day-level coaching notes
   */
  notes?: string;
  /**
   * Planned workout date
   */
  scheduled_date?: string;
  /**
   * Expected duration in minutes
   */
  target_session_length_min?: number;
  /**
   * Exercises in this day
   *
   * @minItems 1
   */
  exercises: [Exercise, ...Exercise[]];
}
/**
 * Single exercise definition
 */
export interface Exercise {
  /**
   * Unique exercise identifier
   */
  id?: string;
  /**
   * Exercise name
   */
  name?: string;
  /**
   * Exercise type
   */
  modality: "strength" | "countdown" | "stopwatch" | "interval" | "cycling" | "running" | "rowing" | "swimming";
  /**
   * Target number of sets
   */
  target_sets?: number;
  /**
   * Target reps per set
   */
  target_reps?: number;
  /**
   * Target duration in seconds
   */
  target_duration_sec?: number;
  /**
   * Target distance in meters
   */
  target_distance_meters?: number;
  /**
   * Loading guidance (weight, RPE, %1RM)
   */
  target_load?: string;
  /**
   * Target weight as percentage of reference max (requires percent_of)
   */
  target_weight_percent?: number;
  /**
   * Reference max for percentage calculation (requires target_weight_percent)
   */
  percent_of?: "1rm" | "3rm" | "5rm" | "10rm";
  /**
   * Reference another exercise's max for percentage calculation
   */
  reference_exercise?: string;
  /**
   * Form cues (alias for target_notes)
   */
  cues?: string;
  /**
   * Coaching notes for this exercise
   */
  target_notes?: string;
  /**
   * Tutorial URL (HTTPS only)
   */
  link?: string;
  /**
   * Demo image URL (HTTPS only)
   */
  image?: string;
  /**
   * Group identifier for supersets/circuits (alphanumeric, hyphens, underscores only)
   */
  group?: string;
  /**
   * Type of exercise grouping
   */
  group_type?: "superset" | "circuit";
  /**
   * Rest period in seconds between sets
   */
  rest_between_sets_sec?: number;
  /**
   * Rest period in seconds after completing all sets
   */
  rest_after_sec?: number;
  /**
   * Training zones for endurance workouts
   *
   * @minItems 1
   */
  zones?: [TrainingZone, ...TrainingZone[]];
  ramp?: RampConfig;
  /**
   * Structured interval phases for complex endurance workouts
   *
   * @minItems 1
   */
  interval_phases?: [IntervalPhase, ...IntervalPhase[]];
}
/**
 * Training zone specification
 */
export interface TrainingZone {
  /**
   * Training zone number (1-7)
   */
  zone: number;
  /**
   * Duration in this zone (seconds)
   */
  duration_sec?: number;
  /**
   * Target power in watts
   */
  target_power_watts?: number;
  /**
   * Target heart rate in BPM
   */
  target_hr_bpm?: number;
  /**
   * Target pace in seconds per kilometer
   */
  target_pace_sec_per_km?: number;
}
/**
 * Ramp configuration for gradual intensity changes
 */
export interface RampConfig {
  /**
   * Starting power in watts
   */
  start_power_watts: number;
  /**
   * Ending power in watts
   */
  end_power_watts: number;
  /**
   * Ramp duration in seconds
   */
  duration_sec: number;
  /**
   * Duration of each step in seconds
   */
  step_duration_sec?: number;
}
/**
 * Phase in a structured interval workout
 */
export interface IntervalPhase {
  /**
   * Phase name (e.g., 'warmup', 'work', 'recovery')
   */
  name: string;
  /**
   * Phase duration in seconds
   */
  duration_sec: number;
  /**
   * Target power in watts
   */
  target_power_watts?: number;
  /**
   * Target heart rate in BPM
   */
  target_hr_bpm?: number;
  /**
   * Target pace in seconds per kilometer
   */
  target_pace_sec_per_km?: number;
  /**
   * Target cadence in RPM
   */
  cadence_rpm?: number;
}
