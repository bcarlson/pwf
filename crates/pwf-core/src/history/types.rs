//! History export type definitions

use crate::{DistanceUnit, Modality, WeightUnit};
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
