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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    // ===== Modality Tests =====

    #[test]
    fn test_modality_display_strength() {
        assert_eq!(Modality::Strength.to_string(), "strength");
    }

    #[test]
    fn test_modality_display_countdown() {
        assert_eq!(Modality::Countdown.to_string(), "countdown");
    }

    #[test]
    fn test_modality_display_stopwatch() {
        assert_eq!(Modality::Stopwatch.to_string(), "stopwatch");
    }

    #[test]
    fn test_modality_display_interval() {
        assert_eq!(Modality::Interval.to_string(), "interval");
    }

    #[test]
    fn test_modality_from_str_strength() {
        assert_eq!(Modality::from_str("strength").unwrap(), Modality::Strength);
    }

    #[test]
    fn test_modality_from_str_countdown() {
        assert_eq!(
            Modality::from_str("countdown").unwrap(),
            Modality::Countdown
        );
    }

    #[test]
    fn test_modality_from_str_stopwatch() {
        assert_eq!(
            Modality::from_str("stopwatch").unwrap(),
            Modality::Stopwatch
        );
    }

    #[test]
    fn test_modality_from_str_interval() {
        assert_eq!(Modality::from_str("interval").unwrap(), Modality::Interval);
    }

    #[test]
    fn test_modality_from_str_case_insensitive_uppercase() {
        assert_eq!(Modality::from_str("STRENGTH").unwrap(), Modality::Strength);
        assert_eq!(
            Modality::from_str("COUNTDOWN").unwrap(),
            Modality::Countdown
        );
        assert_eq!(
            Modality::from_str("STOPWATCH").unwrap(),
            Modality::Stopwatch
        );
        assert_eq!(Modality::from_str("INTERVAL").unwrap(), Modality::Interval);
    }

    #[test]
    fn test_modality_from_str_case_insensitive_mixed() {
        assert_eq!(Modality::from_str("StReNgTh").unwrap(), Modality::Strength);
        assert_eq!(
            Modality::from_str("CouNtDowN").unwrap(),
            Modality::Countdown
        );
        assert_eq!(
            Modality::from_str("StopWatch").unwrap(),
            Modality::Stopwatch
        );
        assert_eq!(Modality::from_str("InTeRvAl").unwrap(), Modality::Interval);
    }

    #[test]
    fn test_modality_from_str_invalid() {
        let result = Modality::from_str("invalid");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unknown modality: invalid");
    }

    #[test]
    fn test_modality_from_str_empty() {
        let result = Modality::from_str("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unknown modality: ");
    }

    #[test]
    fn test_modality_from_str_similar_but_wrong() {
        assert!(Modality::from_str("strenght").is_err()); // typo
        assert!(Modality::from_str("countdow").is_err()); // incomplete
        assert!(Modality::from_str("stop watch").is_err()); // space
        assert!(Modality::from_str("intervals").is_err()); // plural
    }

    #[test]
    fn test_modality_serde_serialize_strength() {
        let modality = Modality::Strength;
        let json = serde_json::to_string(&modality).unwrap();
        assert_eq!(json, "\"strength\"");
    }

    #[test]
    fn test_modality_serde_serialize_countdown() {
        let modality = Modality::Countdown;
        let json = serde_json::to_string(&modality).unwrap();
        assert_eq!(json, "\"countdown\"");
    }

    #[test]
    fn test_modality_serde_serialize_stopwatch() {
        let modality = Modality::Stopwatch;
        let json = serde_json::to_string(&modality).unwrap();
        assert_eq!(json, "\"stopwatch\"");
    }

    #[test]
    fn test_modality_serde_serialize_interval() {
        let modality = Modality::Interval;
        let json = serde_json::to_string(&modality).unwrap();
        assert_eq!(json, "\"interval\"");
    }

    #[test]
    fn test_modality_serde_deserialize_strength() {
        let json = "\"strength\"";
        let modality: Modality = serde_json::from_str(json).unwrap();
        assert_eq!(modality, Modality::Strength);
    }

    #[test]
    fn test_modality_serde_deserialize_countdown() {
        let json = "\"countdown\"";
        let modality: Modality = serde_json::from_str(json).unwrap();
        assert_eq!(modality, Modality::Countdown);
    }

    #[test]
    fn test_modality_serde_deserialize_stopwatch() {
        let json = "\"stopwatch\"";
        let modality: Modality = serde_json::from_str(json).unwrap();
        assert_eq!(modality, Modality::Stopwatch);
    }

    #[test]
    fn test_modality_serde_deserialize_interval() {
        let json = "\"interval\"";
        let modality: Modality = serde_json::from_str(json).unwrap();
        assert_eq!(modality, Modality::Interval);
    }

    #[test]
    fn test_modality_serde_deserialize_invalid() {
        let json = "\"invalid\"";
        let result: Result<Modality, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_modality_serde_roundtrip_all_variants() {
        let variants = vec![
            Modality::Strength,
            Modality::Countdown,
            Modality::Stopwatch,
            Modality::Interval,
        ];

        for variant in variants {
            let json = serde_json::to_string(&variant).unwrap();
            let deserialized: Modality = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, deserialized);
        }
    }

    #[test]
    fn test_modality_clone() {
        let modality = Modality::Strength;
        let cloned = modality;
        assert_eq!(modality, cloned);
    }

    #[test]
    fn test_modality_copy() {
        let modality = Modality::Strength;
        let copied = modality;
        assert_eq!(modality, copied);
    }

    #[test]
    fn test_modality_debug() {
        assert_eq!(format!("{:?}", Modality::Strength), "Strength");
        assert_eq!(format!("{:?}", Modality::Countdown), "Countdown");
        assert_eq!(format!("{:?}", Modality::Stopwatch), "Stopwatch");
        assert_eq!(format!("{:?}", Modality::Interval), "Interval");
    }

    #[test]
    fn test_modality_equality() {
        assert_eq!(Modality::Strength, Modality::Strength);
        assert_ne!(Modality::Strength, Modality::Countdown);
        assert_ne!(Modality::Countdown, Modality::Stopwatch);
        assert_ne!(Modality::Stopwatch, Modality::Interval);
    }

    // ===== WeightUnit Tests =====

    #[test]
    fn test_weight_unit_display_kg() {
        assert_eq!(WeightUnit::Kg.to_string(), "kg");
    }

    #[test]
    fn test_weight_unit_display_lb() {
        assert_eq!(WeightUnit::Lb.to_string(), "lb");
    }

    #[test]
    fn test_weight_unit_default() {
        assert_eq!(WeightUnit::default(), WeightUnit::Kg);
    }

    #[test]
    fn test_weight_unit_serde_serialize_kg() {
        let unit = WeightUnit::Kg;
        let json = serde_json::to_string(&unit).unwrap();
        assert_eq!(json, "\"kg\"");
    }

    #[test]
    fn test_weight_unit_serde_serialize_lb() {
        let unit = WeightUnit::Lb;
        let json = serde_json::to_string(&unit).unwrap();
        assert_eq!(json, "\"lb\"");
    }

    #[test]
    fn test_weight_unit_serde_deserialize_kg() {
        let json = "\"kg\"";
        let unit: WeightUnit = serde_json::from_str(json).unwrap();
        assert_eq!(unit, WeightUnit::Kg);
    }

    #[test]
    fn test_weight_unit_serde_deserialize_lb() {
        let json = "\"lb\"";
        let unit: WeightUnit = serde_json::from_str(json).unwrap();
        assert_eq!(unit, WeightUnit::Lb);
    }

    #[test]
    fn test_weight_unit_serde_roundtrip() {
        let variants = vec![WeightUnit::Kg, WeightUnit::Lb];

        for variant in variants {
            let json = serde_json::to_string(&variant).unwrap();
            let deserialized: WeightUnit = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, deserialized);
        }
    }

    #[test]
    fn test_weight_unit_debug() {
        assert_eq!(format!("{:?}", WeightUnit::Kg), "Kg");
        assert_eq!(format!("{:?}", WeightUnit::Lb), "Lb");
    }

    #[test]
    fn test_weight_unit_equality() {
        assert_eq!(WeightUnit::Kg, WeightUnit::Kg);
        assert_eq!(WeightUnit::Lb, WeightUnit::Lb);
        assert_ne!(WeightUnit::Kg, WeightUnit::Lb);
    }

    #[test]
    fn test_weight_unit_clone() {
        let unit = WeightUnit::Kg;
        let cloned = unit;
        assert_eq!(unit, cloned);
    }

    #[test]
    fn test_weight_unit_copy() {
        let unit = WeightUnit::Kg;
        let copied = unit;
        assert_eq!(unit, copied);
    }

    // ===== DistanceUnit Tests =====

    #[test]
    fn test_distance_unit_display_meters() {
        assert_eq!(DistanceUnit::Meters.to_string(), "m");
    }

    #[test]
    fn test_distance_unit_display_kilometers() {
        assert_eq!(DistanceUnit::Kilometers.to_string(), "km");
    }

    #[test]
    fn test_distance_unit_display_miles() {
        assert_eq!(DistanceUnit::Miles.to_string(), "mi");
    }

    #[test]
    fn test_distance_unit_display_feet() {
        assert_eq!(DistanceUnit::Feet.to_string(), "ft");
    }

    #[test]
    fn test_distance_unit_display_yards() {
        assert_eq!(DistanceUnit::Yards.to_string(), "yd");
    }

    #[test]
    fn test_distance_unit_default() {
        assert_eq!(DistanceUnit::default(), DistanceUnit::Meters);
    }

    #[test]
    fn test_distance_unit_serde_serialize_meters() {
        let unit = DistanceUnit::Meters;
        let json = serde_json::to_string(&unit).unwrap();
        assert_eq!(json, "\"meters\"");
    }

    #[test]
    fn test_distance_unit_serde_serialize_kilometers() {
        let unit = DistanceUnit::Kilometers;
        let json = serde_json::to_string(&unit).unwrap();
        assert_eq!(json, "\"kilometers\"");
    }

    #[test]
    fn test_distance_unit_serde_serialize_miles() {
        let unit = DistanceUnit::Miles;
        let json = serde_json::to_string(&unit).unwrap();
        assert_eq!(json, "\"miles\"");
    }

    #[test]
    fn test_distance_unit_serde_serialize_feet() {
        let unit = DistanceUnit::Feet;
        let json = serde_json::to_string(&unit).unwrap();
        assert_eq!(json, "\"feet\"");
    }

    #[test]
    fn test_distance_unit_serde_serialize_yards() {
        let unit = DistanceUnit::Yards;
        let json = serde_json::to_string(&unit).unwrap();
        assert_eq!(json, "\"yards\"");
    }

    #[test]
    fn test_distance_unit_serde_deserialize_meters() {
        let json = "\"meters\"";
        let unit: DistanceUnit = serde_json::from_str(json).unwrap();
        assert_eq!(unit, DistanceUnit::Meters);
    }

    #[test]
    fn test_distance_unit_serde_deserialize_kilometers() {
        let json = "\"kilometers\"";
        let unit: DistanceUnit = serde_json::from_str(json).unwrap();
        assert_eq!(unit, DistanceUnit::Kilometers);
    }

    #[test]
    fn test_distance_unit_serde_deserialize_miles() {
        let json = "\"miles\"";
        let unit: DistanceUnit = serde_json::from_str(json).unwrap();
        assert_eq!(unit, DistanceUnit::Miles);
    }

    #[test]
    fn test_distance_unit_serde_deserialize_feet() {
        let json = "\"feet\"";
        let unit: DistanceUnit = serde_json::from_str(json).unwrap();
        assert_eq!(unit, DistanceUnit::Feet);
    }

    #[test]
    fn test_distance_unit_serde_deserialize_yards() {
        let json = "\"yards\"";
        let unit: DistanceUnit = serde_json::from_str(json).unwrap();
        assert_eq!(unit, DistanceUnit::Yards);
    }

    #[test]
    fn test_distance_unit_serde_roundtrip() {
        let variants = vec![
            DistanceUnit::Meters,
            DistanceUnit::Kilometers,
            DistanceUnit::Miles,
            DistanceUnit::Feet,
            DistanceUnit::Yards,
        ];

        for variant in variants {
            let json = serde_json::to_string(&variant).unwrap();
            let deserialized: DistanceUnit = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, deserialized);
        }
    }

    #[test]
    fn test_distance_unit_debug() {
        assert_eq!(format!("{:?}", DistanceUnit::Meters), "Meters");
        assert_eq!(format!("{:?}", DistanceUnit::Kilometers), "Kilometers");
        assert_eq!(format!("{:?}", DistanceUnit::Miles), "Miles");
        assert_eq!(format!("{:?}", DistanceUnit::Feet), "Feet");
        assert_eq!(format!("{:?}", DistanceUnit::Yards), "Yards");
    }

    #[test]
    fn test_distance_unit_equality() {
        assert_eq!(DistanceUnit::Meters, DistanceUnit::Meters);
        assert_ne!(DistanceUnit::Meters, DistanceUnit::Kilometers);
        assert_ne!(DistanceUnit::Kilometers, DistanceUnit::Miles);
        assert_ne!(DistanceUnit::Miles, DistanceUnit::Feet);
        assert_ne!(DistanceUnit::Feet, DistanceUnit::Yards);
    }

    #[test]
    fn test_distance_unit_clone() {
        let unit = DistanceUnit::Meters;
        let cloned = unit;
        assert_eq!(unit, cloned);
    }

    #[test]
    fn test_distance_unit_copy() {
        let unit = DistanceUnit::Meters;
        let copied = unit;
        assert_eq!(unit, copied);
    }

    // ===== Equipment Tags Tests =====

    #[test]
    fn test_equipment_tags_contains_expected() {
        assert!(EQUIPMENT_TAGS.contains(&"barbell"));
        assert!(EQUIPMENT_TAGS.contains(&"dumbbells"));
        assert!(EQUIPMENT_TAGS.contains(&"kettlebell"));
        assert!(EQUIPMENT_TAGS.contains(&"pullup_bar"));
        assert!(EQUIPMENT_TAGS.contains(&"bench"));
        assert!(EQUIPMENT_TAGS.contains(&"cables"));
        assert!(EQUIPMENT_TAGS.contains(&"bands"));
        assert!(EQUIPMENT_TAGS.contains(&"bodyweight"));
        assert!(EQUIPMENT_TAGS.contains(&"machine"));
    }

    #[test]
    fn test_equipment_tags_count() {
        assert_eq!(EQUIPMENT_TAGS.len(), 9);
    }

    #[test]
    fn test_equipment_tags_no_duplicates() {
        let mut unique = EQUIPMENT_TAGS.to_vec();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), EQUIPMENT_TAGS.len());
    }

    #[test]
    fn test_equipment_tags_all_lowercase() {
        for tag in EQUIPMENT_TAGS {
            assert_eq!(tag, &tag.to_lowercase());
        }
    }

    #[test]
    fn test_equipment_tags_no_spaces() {
        for tag in EQUIPMENT_TAGS {
            assert!(!tag.contains(' '));
        }
    }
}
