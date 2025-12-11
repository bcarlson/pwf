//! Plan type definitions

use crate::Modality;
use serde::{Deserialize, Serialize};

/// Root plan structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WpsPlan {
    pub plan_version: u32,
    #[serde(default)]
    pub meta: Option<PlanMeta>,
    pub cycle: PlanCycle,
}

/// Plan metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlanMeta {
    #[serde(default)]
    pub id: Option<String>,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub equipment: Vec<String>,
    #[serde(default, rename = "daysPerWeek")]
    pub days_per_week: Option<u8>,
    #[serde(default, rename = "recommendedFirst")]
    pub recommended_first: bool,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Training cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanCycle {
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    pub days: Vec<PlanDay>,
}

/// Single training day
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDay {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub order: Option<u32>,
    #[serde(default)]
    pub focus: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub scheduled_date: Option<String>,
    #[serde(default, rename = "target_session_length_min")]
    pub target_session_length_min: Option<u32>,
    pub exercises: Vec<PlanExercise>,
}

/// Single exercise in a plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanExercise {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    pub modality: Modality,
    #[serde(default)]
    pub target_sets: Option<u32>,
    #[serde(default)]
    pub target_reps: Option<u32>,
    #[serde(default)]
    pub target_duration_sec: Option<u32>,
    #[serde(default)]
    pub target_distance_meters: Option<f64>,
    #[serde(default)]
    pub target_load: Option<String>,
    #[serde(default)]
    pub cues: Option<String>,
    #[serde(default)]
    pub target_notes: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
    #[serde(default)]
    pub image: Option<String>,
}

/// Statistics about a parsed plan
#[derive(Debug, Clone, Default, Serialize)]
pub struct PlanStatistics {
    pub total_days: usize,
    pub total_exercises: usize,
    pub strength_count: usize,
    pub countdown_count: usize,
    pub stopwatch_count: usize,
    pub interval_count: usize,
    pub equipment: Vec<String>,
}
