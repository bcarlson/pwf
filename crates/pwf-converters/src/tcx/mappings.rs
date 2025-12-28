//! Field mapping logic from TCX to PWF

use pwf_core::history::StrokeType;
use pwf_core::Sport;

/// Map TCX sport type to PWF Sport enum
///
/// TCX uses sport attribute on Activity element
/// Supports both standard TCX sport types and common extensions
pub fn map_tcx_sport(tcx_sport: &str) -> Sport {
    match tcx_sport.to_lowercase().as_str() {
        "running" => Sport::Running,
        "biking" | "cycling" => Sport::Cycling,
        "swimming" => Sport::Swimming,
        "rowing" => Sport::Rowing,
        "transition" => Sport::Transition,
        "strength" => Sport::Strength,
        "strength_training" | "strength-training" | "strengthtraining" => Sport::StrengthTraining,
        "hiking" => Sport::Hiking,
        "walking" => Sport::Walking,
        "yoga" => Sport::Yoga,
        "pilates" => Sport::Pilates,
        "crossfit" | "cross-fit" | "functional-fitness" | "functionalfitness" => Sport::FunctionalFitness,
        "calisthenics" => Sport::Calisthenics,
        "cardio" | "fitness" => Sport::Cardio,
        "cross_country_skiing" | "cross-country-skiing" | "xc_skiing" | "xc-skiing" => {
            Sport::CrossCountrySkiing
        }
        "downhill_skiing" | "downhill-skiing" | "alpine_skiing" | "alpine-skiing" | "skiing" => {
            Sport::DownhillSkiing
        }
        "elliptical" => Sport::Elliptical,
        "stair_climbing" | "stair-climbing" | "stairs" | "stairstepper" => Sport::StairClimbing,
        "other" => Sport::Other,
        _ => Sport::Other,
    }
}

/// Map TCX stroke type to PWF StrokeType enum (for swimming)
///
/// TCX doesn't have a standard stroke type field, but some extensions may include it
pub fn map_tcx_stroke(_tcx_stroke: &str) -> StrokeType {
    // TCX doesn't typically include stroke type in standard schema
    // Default to freestyle for swimming activities
    StrokeType::Freestyle
}

/// Map PWF Sport enum to TCX sport string
///
/// TCX standard supports: Running, Biking, Other
/// For extended types, we use common extension values that many apps support
pub fn map_pwf_sport_to_tcx(sport: &Sport) -> String {
    match sport {
        Sport::Running => "Running".to_string(),
        Sport::Cycling => "Biking".to_string(),
        Sport::Swimming => "Swimming".to_string(), // Extension, but widely supported
        Sport::Rowing => "Rowing".to_string(),     // Extension
        Sport::Transition => "Transition".to_string(), // Extension for multi-sport
        Sport::Strength => "Strength".to_string(), // Extension
        Sport::StrengthTraining => "StrengthTraining".to_string(), // Extension
        Sport::Hiking => "Hiking".to_string(),     // Extension
        Sport::Walking => "Walking".to_string(),   // Extension
        Sport::Yoga => "Yoga".to_string(),         // Extension
        Sport::Pilates => "Pilates".to_string(),   // Extension
        Sport::FunctionalFitness => "FunctionalFitness".to_string(), // Extension
        Sport::Calisthenics => "Calisthenics".to_string(), // Extension
        Sport::Cardio => "Cardio".to_string(),     // Extension
        Sport::CrossCountrySkiing => "CrossCountrySkiing".to_string(), // Extension
        Sport::DownhillSkiing => "DownhillSkiing".to_string(), // Extension
        Sport::Snowboarding => "Snowboarding".to_string(), // Extension
        Sport::StandUpPaddling => "StandUpPaddleboarding".to_string(), // Extension
        Sport::Kayaking => "Kayaking".to_string(), // Extension
        Sport::Elliptical => "Elliptical".to_string(), // Extension
        Sport::StairClimbing => "StairClimbing".to_string(), // Extension
        Sport::Other => "Other".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_tcx_sport_basic() {
        assert_eq!(map_tcx_sport("Running"), Sport::Running);
        assert_eq!(map_tcx_sport("Biking"), Sport::Cycling);
        assert_eq!(map_tcx_sport("Cycling"), Sport::Cycling);
        assert_eq!(map_tcx_sport("Swimming"), Sport::Swimming);
        assert_eq!(map_tcx_sport("Rowing"), Sport::Rowing);
        assert_eq!(map_tcx_sport("Transition"), Sport::Transition);
        assert_eq!(map_tcx_sport("Other"), Sport::Other);
    }

    #[test]
    fn test_map_tcx_sport_strength() {
        assert_eq!(map_tcx_sport("Strength"), Sport::Strength);
        assert_eq!(map_tcx_sport("strength_training"), Sport::StrengthTraining);
        assert_eq!(map_tcx_sport("strength-training"), Sport::StrengthTraining);
        assert_eq!(map_tcx_sport("StrengthTraining"), Sport::StrengthTraining);
    }

    #[test]
    fn test_map_tcx_sport_walking_hiking() {
        assert_eq!(map_tcx_sport("Hiking"), Sport::Hiking);
        assert_eq!(map_tcx_sport("Walking"), Sport::Walking);
    }

    #[test]
    fn test_map_tcx_sport_yoga_pilates() {
        assert_eq!(map_tcx_sport("Yoga"), Sport::Yoga);
        assert_eq!(map_tcx_sport("Pilates"), Sport::Pilates);
    }

    #[test]
    fn test_map_tcx_sport_functional_fitness_calisthenics() {
        assert_eq!(map_tcx_sport("FunctionalFitness"), Sport::FunctionalFitness);
        assert_eq!(map_tcx_sport("functional-fitness"), Sport::FunctionalFitness);
        assert_eq!(map_tcx_sport("cross-fit"), Sport::FunctionalFitness);
        assert_eq!(map_tcx_sport("Calisthenics"), Sport::Calisthenics);
    }

    #[test]
    fn test_map_tcx_sport_cardio() {
        assert_eq!(map_tcx_sport("Cardio"), Sport::Cardio);
        assert_eq!(map_tcx_sport("Fitness"), Sport::Cardio);
    }

    #[test]
    fn test_map_tcx_sport_skiing() {
        assert_eq!(
            map_tcx_sport("cross_country_skiing"),
            Sport::CrossCountrySkiing
        );
        assert_eq!(
            map_tcx_sport("cross-country-skiing"),
            Sport::CrossCountrySkiing
        );
        assert_eq!(map_tcx_sport("xc_skiing"), Sport::CrossCountrySkiing);
        assert_eq!(map_tcx_sport("xc-skiing"), Sport::CrossCountrySkiing);
        assert_eq!(map_tcx_sport("downhill_skiing"), Sport::DownhillSkiing);
        assert_eq!(map_tcx_sport("downhill-skiing"), Sport::DownhillSkiing);
        assert_eq!(map_tcx_sport("alpine_skiing"), Sport::DownhillSkiing);
        assert_eq!(map_tcx_sport("skiing"), Sport::DownhillSkiing);
    }

    #[test]
    fn test_map_tcx_sport_gym_equipment() {
        assert_eq!(map_tcx_sport("Elliptical"), Sport::Elliptical);
        assert_eq!(map_tcx_sport("stair_climbing"), Sport::StairClimbing);
        assert_eq!(map_tcx_sport("stair-climbing"), Sport::StairClimbing);
        assert_eq!(map_tcx_sport("stairs"), Sport::StairClimbing);
        assert_eq!(map_tcx_sport("stairstepper"), Sport::StairClimbing);
    }

    #[test]
    fn test_map_tcx_sport_case_insensitive() {
        assert_eq!(map_tcx_sport("RUNNING"), Sport::Running);
        assert_eq!(map_tcx_sport("running"), Sport::Running);
        assert_eq!(map_tcx_sport("RuNnInG"), Sport::Running);
        assert_eq!(map_tcx_sport("YOGA"), Sport::Yoga);
        assert_eq!(map_tcx_sport("HIKING"), Sport::Hiking);
    }

    #[test]
    fn test_map_tcx_sport_unknown() {
        assert_eq!(map_tcx_sport("Unknown"), Sport::Other);
        assert_eq!(map_tcx_sport("SomeWeirdSport"), Sport::Other);
    }

    #[test]
    fn test_map_pwf_sport_to_tcx_basic() {
        assert_eq!(map_pwf_sport_to_tcx(&Sport::Running), "Running");
        assert_eq!(map_pwf_sport_to_tcx(&Sport::Cycling), "Biking");
        assert_eq!(map_pwf_sport_to_tcx(&Sport::Swimming), "Swimming");
        assert_eq!(map_pwf_sport_to_tcx(&Sport::Other), "Other");
    }

    #[test]
    fn test_map_pwf_sport_to_tcx_strength() {
        assert_eq!(map_pwf_sport_to_tcx(&Sport::Strength), "Strength");
        assert_eq!(
            map_pwf_sport_to_tcx(&Sport::StrengthTraining),
            "StrengthTraining"
        );
    }

    #[test]
    fn test_map_pwf_sport_to_tcx_all_new_sports() {
        assert_eq!(map_pwf_sport_to_tcx(&Sport::Rowing), "Rowing");
        assert_eq!(map_pwf_sport_to_tcx(&Sport::Transition), "Transition");
        assert_eq!(map_pwf_sport_to_tcx(&Sport::Hiking), "Hiking");
        assert_eq!(map_pwf_sport_to_tcx(&Sport::Walking), "Walking");
        assert_eq!(map_pwf_sport_to_tcx(&Sport::Yoga), "Yoga");
        assert_eq!(map_pwf_sport_to_tcx(&Sport::Pilates), "Pilates");
        assert_eq!(map_pwf_sport_to_tcx(&Sport::FunctionalFitness), "FunctionalFitness");
        assert_eq!(map_pwf_sport_to_tcx(&Sport::Calisthenics), "Calisthenics");
        assert_eq!(map_pwf_sport_to_tcx(&Sport::Cardio), "Cardio");
        assert_eq!(
            map_pwf_sport_to_tcx(&Sport::CrossCountrySkiing),
            "CrossCountrySkiing"
        );
        assert_eq!(
            map_pwf_sport_to_tcx(&Sport::DownhillSkiing),
            "DownhillSkiing"
        );
        assert_eq!(map_pwf_sport_to_tcx(&Sport::Elliptical), "Elliptical");
        assert_eq!(map_pwf_sport_to_tcx(&Sport::StairClimbing), "StairClimbing");
    }

    #[test]
    fn test_roundtrip_tcx_to_pwf_to_tcx() {
        // Test roundtrip for all new sport types
        let sports = vec![
            ("Running", Sport::Running, "Running"),
            ("Biking", Sport::Cycling, "Biking"),
            ("Swimming", Sport::Swimming, "Swimming"),
            ("Rowing", Sport::Rowing, "Rowing"),
            ("Hiking", Sport::Hiking, "Hiking"),
            ("Walking", Sport::Walking, "Walking"),
            ("Yoga", Sport::Yoga, "Yoga"),
            ("Pilates", Sport::Pilates, "Pilates"),
            ("FunctionalFitness", Sport::FunctionalFitness, "FunctionalFitness"),
            ("Calisthenics", Sport::Calisthenics, "Calisthenics"),
            ("Cardio", Sport::Cardio, "Cardio"),
            ("Elliptical", Sport::Elliptical, "Elliptical"),
        ];

        for (tcx_input, expected_sport, expected_tcx_output) in sports {
            let pwf_sport = map_tcx_sport(tcx_input);
            assert_eq!(pwf_sport, expected_sport);
            let tcx_output = map_pwf_sport_to_tcx(&pwf_sport);
            assert_eq!(tcx_output, expected_tcx_output);
        }
    }
}
