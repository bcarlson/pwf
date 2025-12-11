//! Common types shared between plan and history modules

use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported exercise modalities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Modality {
    Strength,
    Countdown,
    Stopwatch,
    Interval,
}

impl fmt::Display for Modality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Modality::Strength => write!(f, "strength"),
            Modality::Countdown => write!(f, "countdown"),
            Modality::Stopwatch => write!(f, "stopwatch"),
            Modality::Interval => write!(f, "interval"),
        }
    }
}

impl std::str::FromStr for Modality {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "strength" => Ok(Modality::Strength),
            "countdown" => Ok(Modality::Countdown),
            "stopwatch" => Ok(Modality::Stopwatch),
            "interval" => Ok(Modality::Interval),
            _ => Err(format!("Unknown modality: {}", s)),
        }
    }
}

/// Standard equipment tags
pub const EQUIPMENT_TAGS: &[&str] = &[
    "barbell",
    "dumbbells",
    "kettlebell",
    "pullup_bar",
    "bench",
    "cables",
    "bands",
    "bodyweight",
    "machine",
];

/// Weight unit for history export
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum WeightUnit {
    #[default]
    Kg,
    Lb,
}

impl fmt::Display for WeightUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WeightUnit::Kg => write!(f, "kg"),
            WeightUnit::Lb => write!(f, "lb"),
        }
    }
}

/// Distance unit for history export
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DistanceUnit {
    #[default]
    Meters,
    Kilometers,
    Miles,
    Feet,
    Yards,
}

impl fmt::Display for DistanceUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DistanceUnit::Meters => write!(f, "m"),
            DistanceUnit::Kilometers => write!(f, "km"),
            DistanceUnit::Miles => write!(f, "mi"),
            DistanceUnit::Feet => write!(f, "ft"),
            DistanceUnit::Yards => write!(f, "yd"),
        }
    }
}
