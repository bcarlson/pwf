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
    Cycling,
    Running,
    Rowing,
    Swimming,
}

impl fmt::Display for Modality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Modality::Strength => write!(f, "strength"),
            Modality::Countdown => write!(f, "countdown"),
            Modality::Stopwatch => write!(f, "stopwatch"),
            Modality::Interval => write!(f, "interval"),
            Modality::Cycling => write!(f, "cycling"),
            Modality::Running => write!(f, "running"),
            Modality::Rowing => write!(f, "rowing"),
            Modality::Swimming => write!(f, "swimming"),
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
            "cycling" => Ok(Modality::Cycling),
            "running" => Ok(Modality::Running),
            "rowing" => Ok(Modality::Rowing),
            "swimming" => Ok(Modality::Swimming),
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

/// Sport type for multi-sport activities (PWF v2)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Sport {
    Swimming,
    Cycling,
    Running,
    Rowing,
    Transition,
    Strength,
    StrengthTraining,
    Hiking,
    Walking,
    Yoga,
    Pilates,
    FunctionalFitness,
    Calisthenics,
    Cardio,
    CrossCountrySkiing,
    DownhillSkiing,
    Snowboarding,
    StandUpPaddling,
    Kayaking,
    Elliptical,
    StairClimbing,
    Other,
}

impl fmt::Display for Sport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sport::Swimming => write!(f, "swimming"),
            Sport::Cycling => write!(f, "cycling"),
            Sport::Running => write!(f, "running"),
            Sport::Rowing => write!(f, "rowing"),
            Sport::Transition => write!(f, "transition"),
            Sport::Strength => write!(f, "strength"),
            Sport::StrengthTraining => write!(f, "strength-training"),
            Sport::Hiking => write!(f, "hiking"),
            Sport::Walking => write!(f, "walking"),
            Sport::Yoga => write!(f, "yoga"),
            Sport::Pilates => write!(f, "pilates"),
            Sport::FunctionalFitness => write!(f, "functional-fitness"),
            Sport::Calisthenics => write!(f, "calisthenics"),
            Sport::Cardio => write!(f, "cardio"),
            Sport::CrossCountrySkiing => write!(f, "cross-country-skiing"),
            Sport::DownhillSkiing => write!(f, "downhill-skiing"),
            Sport::Snowboarding => write!(f, "snowboarding"),
            Sport::StandUpPaddling => write!(f, "stand-up-paddling"),
            Sport::Kayaking => write!(f, "kayaking"),
            Sport::Elliptical => write!(f, "elliptical"),
            Sport::StairClimbing => write!(f, "stair-climbing"),
            Sport::Other => write!(f, "other"),
        }
    }
}

impl std::str::FromStr for Sport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "swimming" => Ok(Sport::Swimming),
            "cycling" => Ok(Sport::Cycling),
            "running" => Ok(Sport::Running),
            "rowing" => Ok(Sport::Rowing),
            "transition" => Ok(Sport::Transition),
            "strength" => Ok(Sport::Strength),
            "strength-training" => Ok(Sport::StrengthTraining),
            "hiking" => Ok(Sport::Hiking),
            "walking" => Ok(Sport::Walking),
            "yoga" => Ok(Sport::Yoga),
            "pilates" => Ok(Sport::Pilates),
            "cross-fit" | "functional-fitness" => Ok(Sport::FunctionalFitness),
            "calisthenics" => Ok(Sport::Calisthenics),
            "cardio" => Ok(Sport::Cardio),
            "cross-country-skiing" => Ok(Sport::CrossCountrySkiing),
            "downhill-skiing" => Ok(Sport::DownhillSkiing),
            "snowboarding" => Ok(Sport::Snowboarding),
            "stand-up-paddling" | "sup" => Ok(Sport::StandUpPaddling),
            "kayaking" | "kayak" => Ok(Sport::Kayaking),
            "elliptical" => Ok(Sport::Elliptical),
            "stair-climbing" => Ok(Sport::StairClimbing),
            "other" => Ok(Sport::Other),
            _ => Err(format!("Unknown sport: {}", s)),
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
    fn test_modality_display_cycling() {
        assert_eq!(Modality::Cycling.to_string(), "cycling");
    }

    #[test]
    fn test_modality_display_running() {
        assert_eq!(Modality::Running.to_string(), "running");
    }

    #[test]
    fn test_modality_display_rowing() {
        assert_eq!(Modality::Rowing.to_string(), "rowing");
    }

    #[test]
    fn test_modality_display_swimming() {
        assert_eq!(Modality::Swimming.to_string(), "swimming");
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
    fn test_modality_from_str_cycling() {
        assert_eq!(Modality::from_str("cycling").unwrap(), Modality::Cycling);
    }

    #[test]
    fn test_modality_from_str_running() {
        assert_eq!(Modality::from_str("running").unwrap(), Modality::Running);
    }

    #[test]
    fn test_modality_from_str_rowing() {
        assert_eq!(Modality::from_str("rowing").unwrap(), Modality::Rowing);
    }

    #[test]
    fn test_modality_from_str_swimming() {
        assert_eq!(Modality::from_str("swimming").unwrap(), Modality::Swimming);
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
        assert_eq!(Modality::from_str("CYCLING").unwrap(), Modality::Cycling);
        assert_eq!(Modality::from_str("RUNNING").unwrap(), Modality::Running);
        assert_eq!(Modality::from_str("ROWING").unwrap(), Modality::Rowing);
        assert_eq!(Modality::from_str("SWIMMING").unwrap(), Modality::Swimming);
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
        assert_eq!(Modality::from_str("CyCLiNg").unwrap(), Modality::Cycling);
        assert_eq!(Modality::from_str("RuNnInG").unwrap(), Modality::Running);
        assert_eq!(Modality::from_str("RoWiNg").unwrap(), Modality::Rowing);
        assert_eq!(Modality::from_str("SwImMiNg").unwrap(), Modality::Swimming);
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

    // ===== Sport Tests =====

    #[test]
    fn test_sport_display() {
        assert_eq!(Sport::Swimming.to_string(), "swimming");
        assert_eq!(Sport::Cycling.to_string(), "cycling");
        assert_eq!(Sport::Running.to_string(), "running");
        assert_eq!(Sport::Rowing.to_string(), "rowing");
        assert_eq!(Sport::Transition.to_string(), "transition");
        assert_eq!(Sport::Strength.to_string(), "strength");
        assert_eq!(Sport::StrengthTraining.to_string(), "strength-training");
        assert_eq!(Sport::Hiking.to_string(), "hiking");
        assert_eq!(Sport::Walking.to_string(), "walking");
        assert_eq!(Sport::Yoga.to_string(), "yoga");
        assert_eq!(Sport::Pilates.to_string(), "pilates");
        assert_eq!(Sport::FunctionalFitness.to_string(), "functional-fitness");
        assert_eq!(Sport::Calisthenics.to_string(), "calisthenics");
        assert_eq!(Sport::Cardio.to_string(), "cardio");
        assert_eq!(
            Sport::CrossCountrySkiing.to_string(),
            "cross-country-skiing"
        );
        assert_eq!(Sport::DownhillSkiing.to_string(), "downhill-skiing");
        assert_eq!(Sport::Elliptical.to_string(), "elliptical");
        assert_eq!(Sport::StairClimbing.to_string(), "stair-climbing");
        assert_eq!(Sport::Other.to_string(), "other");
    }

    #[test]
    fn test_sport_from_str() {
        assert_eq!(Sport::from_str("swimming").unwrap(), Sport::Swimming);
        assert_eq!(Sport::from_str("cycling").unwrap(), Sport::Cycling);
        assert_eq!(Sport::from_str("running").unwrap(), Sport::Running);
        assert_eq!(Sport::from_str("rowing").unwrap(), Sport::Rowing);
        assert_eq!(Sport::from_str("transition").unwrap(), Sport::Transition);
        assert_eq!(Sport::from_str("strength").unwrap(), Sport::Strength);
        assert_eq!(
            Sport::from_str("strength-training").unwrap(),
            Sport::StrengthTraining
        );
        assert_eq!(Sport::from_str("hiking").unwrap(), Sport::Hiking);
        assert_eq!(Sport::from_str("walking").unwrap(), Sport::Walking);
        assert_eq!(Sport::from_str("yoga").unwrap(), Sport::Yoga);
        assert_eq!(Sport::from_str("pilates").unwrap(), Sport::Pilates);
        assert_eq!(
            Sport::from_str("functional-fitness").unwrap(),
            Sport::FunctionalFitness
        );
        assert_eq!(
            Sport::from_str("cross-fit").unwrap(),
            Sport::FunctionalFitness
        );
        assert_eq!(
            Sport::from_str("calisthenics").unwrap(),
            Sport::Calisthenics
        );
        assert_eq!(Sport::from_str("cardio").unwrap(), Sport::Cardio);
        assert_eq!(
            Sport::from_str("cross-country-skiing").unwrap(),
            Sport::CrossCountrySkiing
        );
        assert_eq!(
            Sport::from_str("downhill-skiing").unwrap(),
            Sport::DownhillSkiing
        );
        assert_eq!(Sport::from_str("elliptical").unwrap(), Sport::Elliptical);
        assert_eq!(
            Sport::from_str("stair-climbing").unwrap(),
            Sport::StairClimbing
        );
        assert_eq!(Sport::from_str("other").unwrap(), Sport::Other);
    }

    #[test]
    fn test_sport_from_str_case_insensitive() {
        assert_eq!(Sport::from_str("SWIMMING").unwrap(), Sport::Swimming);
        assert_eq!(Sport::from_str("CyCLiNg").unwrap(), Sport::Cycling);
        assert_eq!(Sport::from_str("Running").unwrap(), Sport::Running);
    }

    #[test]
    fn test_sport_from_str_invalid() {
        let result = Sport::from_str("invalid");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unknown sport: invalid");
    }

    #[test]
    fn test_sport_serde_serialize() {
        assert_eq!(
            serde_json::to_string(&Sport::Swimming).unwrap(),
            "\"swimming\""
        );
        assert_eq!(
            serde_json::to_string(&Sport::Cycling).unwrap(),
            "\"cycling\""
        );
        assert_eq!(
            serde_json::to_string(&Sport::Running).unwrap(),
            "\"running\""
        );
        assert_eq!(serde_json::to_string(&Sport::Rowing).unwrap(), "\"rowing\"");
        assert_eq!(
            serde_json::to_string(&Sport::Transition).unwrap(),
            "\"transition\""
        );
        assert_eq!(
            serde_json::to_string(&Sport::Strength).unwrap(),
            "\"strength\""
        );
        assert_eq!(
            serde_json::to_string(&Sport::StrengthTraining).unwrap(),
            "\"strength-training\""
        );
        assert_eq!(serde_json::to_string(&Sport::Hiking).unwrap(), "\"hiking\"");
        assert_eq!(
            serde_json::to_string(&Sport::Walking).unwrap(),
            "\"walking\""
        );
        assert_eq!(serde_json::to_string(&Sport::Yoga).unwrap(), "\"yoga\"");
        assert_eq!(
            serde_json::to_string(&Sport::Pilates).unwrap(),
            "\"pilates\""
        );
        assert_eq!(
            serde_json::to_string(&Sport::FunctionalFitness).unwrap(),
            "\"functional-fitness\""
        );
        assert_eq!(
            serde_json::to_string(&Sport::Calisthenics).unwrap(),
            "\"calisthenics\""
        );
        assert_eq!(serde_json::to_string(&Sport::Cardio).unwrap(), "\"cardio\"");
        assert_eq!(
            serde_json::to_string(&Sport::CrossCountrySkiing).unwrap(),
            "\"cross-country-skiing\""
        );
        assert_eq!(
            serde_json::to_string(&Sport::DownhillSkiing).unwrap(),
            "\"downhill-skiing\""
        );
        assert_eq!(
            serde_json::to_string(&Sport::Elliptical).unwrap(),
            "\"elliptical\""
        );
        assert_eq!(
            serde_json::to_string(&Sport::StairClimbing).unwrap(),
            "\"stair-climbing\""
        );
        assert_eq!(serde_json::to_string(&Sport::Other).unwrap(), "\"other\"");
    }

    #[test]
    fn test_sport_serde_deserialize() {
        assert_eq!(
            serde_json::from_str::<Sport>("\"swimming\"").unwrap(),
            Sport::Swimming
        );
        assert_eq!(
            serde_json::from_str::<Sport>("\"cycling\"").unwrap(),
            Sport::Cycling
        );
        assert_eq!(
            serde_json::from_str::<Sport>("\"running\"").unwrap(),
            Sport::Running
        );
    }

    #[test]
    fn test_sport_serde_roundtrip() {
        let variants = vec![
            Sport::Swimming,
            Sport::Cycling,
            Sport::Running,
            Sport::Rowing,
            Sport::Transition,
            Sport::Strength,
            Sport::StrengthTraining,
            Sport::Hiking,
            Sport::Walking,
            Sport::Yoga,
            Sport::Pilates,
            Sport::FunctionalFitness,
            Sport::Calisthenics,
            Sport::Cardio,
            Sport::CrossCountrySkiing,
            Sport::DownhillSkiing,
            Sport::Elliptical,
            Sport::StairClimbing,
            Sport::Other,
        ];

        for variant in variants {
            let json = serde_json::to_string(&variant).unwrap();
            let deserialized: Sport = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, deserialized);
        }
    }

    #[test]
    fn test_sport_debug() {
        assert_eq!(format!("{:?}", Sport::Swimming), "Swimming");
        assert_eq!(format!("{:?}", Sport::Cycling), "Cycling");
        assert_eq!(format!("{:?}", Sport::Running), "Running");
    }

    #[test]
    fn test_sport_equality() {
        assert_eq!(Sport::Swimming, Sport::Swimming);
        assert_ne!(Sport::Swimming, Sport::Cycling);
        assert_ne!(Sport::Cycling, Sport::Running);
    }

    #[test]
    fn test_sport_clone() {
        let sport = Sport::Swimming;
        let cloned = sport;
        assert_eq!(sport, cloned);
    }
}
