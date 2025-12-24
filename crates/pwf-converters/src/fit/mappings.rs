//! Field mapping logic from FIT to PWF

use pwf_core::history::StrokeType;
use pwf_core::Sport;

/// Map FIT sport type to PWF Sport enum
///
/// FIT SDK sport types: https://developer.garmin.com/fit/file-types/activity/
/// Reference: FIT Protocol documentation for complete sport/subsport mappings
pub fn map_fit_sport(fit_sport: u8, fit_subsport: Option<u8>) -> Sport {
    match fit_sport {
        0 => Sport::Running,             // Generic running
        1 => Sport::Cycling,             // Generic cycling
        2 => Sport::Transition,          // Transition (multi-sport events)
        3 => Sport::StrengthTraining,    // Fitness equipment / transition fitness
        4 => Sport::StrengthTraining,    // Fitness equipment (generic gym)
        5 => Sport::Swimming,            // Swimming (pool or open water)
        6 => Sport::Walking,             // Walking
        8 => Sport::Hiking,              // Hiking
        9 => Sport::Cardio,              // E-bike fitness / generic cardio
        11 => Sport::Walking,            // Walking (alternative code)
        13 => Sport::Hiking,             // Hiking (alternative code)
        15 => Sport::Walking,            // Walking (alternative code)
        17 => Sport::Rowing,             // Rowing
        19 => Sport::CrossCountrySkiing, // Cross-country skiing
        20 => Sport::DownhillSkiing,     // Alpine skiing / downhill
        21 => Sport::DownhillSkiing,     // Backcountry skiing
        22 => Sport::DownhillSkiing,     // Snowboarding
        25 => Sport::Cardio,             // Stand-up paddleboarding
        26 => Sport::Strength,           // Strength training
        27 => Sport::Yoga,               // Yoga
        29 => Sport::Pilates,            // Pilates (unofficial, some devices use 29)
        30 => Sport::Cardio,             // Indoor cardio / generic cardio
        31 => Sport::Elliptical,         // Elliptical
        32 => Sport::StairClimbing,      // Stair stepping / climbing
        37 => Sport::Calisthenics,       // HIIT / Calisthenics
        38 => Sport::StrengthTraining,   // Training / generic strength
        43 => Sport::Yoga,               // Breathwork / yoga variant
        45 => Sport::Cardio,             // Cardio training
        46 => Sport::StrengthTraining,   // Floor climbing / strength variant
        54 => Sport::Calisthenics,       // Functional strength training
        62 => Sport::CrossFit,           // CrossFit (unofficial)
        _ => {
            // Handle subsport for more granular mapping
            match fit_subsport {
                Some(2) => Sport::StrengthTraining, // Strength training subsport
                Some(3) => Sport::Cardio,           // Cardio subsport
                Some(8) => Sport::Elliptical,       // Elliptical subsport
                Some(11) => Sport::StairClimbing,   // Stair climbing subsport
                Some(15) => Sport::Rowing,          // Indoor rowing subsport
                Some(19) => Sport::Pilates,         // Pilates subsport
                Some(26) => Sport::Yoga,            // Yoga subsport
                Some(30) => Sport::CrossCountrySkiing, // XC skiing subsport
                Some(31) => Sport::DownhillSkiing,  // Downhill skiing subsport
                _ => Sport::Other,                  // Unknown sport
            }
        }
    }
}

/// Map FIT swim stroke type to PWF StrokeType enum
///
/// FIT SDK swim stroke types
pub fn map_swim_stroke(fit_stroke: u8) -> StrokeType {
    match fit_stroke {
        0 => StrokeType::Freestyle,
        1 => StrokeType::Backstroke,
        2 => StrokeType::Breaststroke,
        3 => StrokeType::Butterfly,
        4 => StrokeType::Drill,
        5 => StrokeType::Mixed,
        6 => StrokeType::IndividualMedley,
        _ => StrokeType::Freestyle, // Default to freestyle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_fit_sport_basic() {
        assert_eq!(map_fit_sport(0, None), Sport::Running);
        assert_eq!(map_fit_sport(1, None), Sport::Cycling);
        assert_eq!(map_fit_sport(5, None), Sport::Swimming);
        assert_eq!(map_fit_sport(2, None), Sport::Transition);
        assert_eq!(map_fit_sport(17, None), Sport::Rowing);
    }

    #[test]
    fn test_map_fit_sport_strength_training() {
        assert_eq!(map_fit_sport(3, None), Sport::StrengthTraining);
        assert_eq!(map_fit_sport(4, None), Sport::StrengthTraining);
        assert_eq!(map_fit_sport(26, None), Sport::Strength);
        assert_eq!(map_fit_sport(38, None), Sport::StrengthTraining);
        assert_eq!(map_fit_sport(46, None), Sport::StrengthTraining);
    }

    #[test]
    fn test_map_fit_sport_walking_hiking() {
        assert_eq!(map_fit_sport(6, None), Sport::Walking);
        assert_eq!(map_fit_sport(11, None), Sport::Walking);
        assert_eq!(map_fit_sport(15, None), Sport::Walking);
        assert_eq!(map_fit_sport(8, None), Sport::Hiking);
        assert_eq!(map_fit_sport(13, None), Sport::Hiking);
    }

    #[test]
    fn test_map_fit_sport_skiing() {
        assert_eq!(map_fit_sport(19, None), Sport::CrossCountrySkiing);
        assert_eq!(map_fit_sport(20, None), Sport::DownhillSkiing);
        assert_eq!(map_fit_sport(21, None), Sport::DownhillSkiing);
        assert_eq!(map_fit_sport(22, None), Sport::DownhillSkiing);
    }

    #[test]
    fn test_map_fit_sport_yoga_pilates() {
        assert_eq!(map_fit_sport(27, None), Sport::Yoga);
        assert_eq!(map_fit_sport(43, None), Sport::Yoga);
        assert_eq!(map_fit_sport(29, None), Sport::Pilates);
    }

    #[test]
    fn test_map_fit_sport_cardio_calisthenics() {
        assert_eq!(map_fit_sport(9, None), Sport::Cardio);
        assert_eq!(map_fit_sport(25, None), Sport::Cardio);
        assert_eq!(map_fit_sport(30, None), Sport::Cardio);
        assert_eq!(map_fit_sport(45, None), Sport::Cardio);
        assert_eq!(map_fit_sport(37, None), Sport::Calisthenics);
        assert_eq!(map_fit_sport(54, None), Sport::Calisthenics);
    }

    #[test]
    fn test_map_fit_sport_gym_equipment() {
        assert_eq!(map_fit_sport(31, None), Sport::Elliptical);
        assert_eq!(map_fit_sport(32, None), Sport::StairClimbing);
    }

    #[test]
    fn test_map_fit_sport_crossfit() {
        assert_eq!(map_fit_sport(62, None), Sport::CrossFit);
    }

    #[test]
    fn test_map_fit_sport_with_subsport() {
        // Unknown sport with strength subsport
        assert_eq!(map_fit_sport(99, Some(2)), Sport::StrengthTraining);
        // Unknown sport with cardio subsport
        assert_eq!(map_fit_sport(99, Some(3)), Sport::Cardio);
        // Unknown sport with elliptical subsport
        assert_eq!(map_fit_sport(99, Some(8)), Sport::Elliptical);
        // Unknown sport with stair climbing subsport
        assert_eq!(map_fit_sport(99, Some(11)), Sport::StairClimbing);
        // Unknown sport with rowing subsport
        assert_eq!(map_fit_sport(99, Some(15)), Sport::Rowing);
        // Unknown sport with pilates subsport
        assert_eq!(map_fit_sport(99, Some(19)), Sport::Pilates);
        // Unknown sport with yoga subsport
        assert_eq!(map_fit_sport(99, Some(26)), Sport::Yoga);
        // Unknown sport with XC skiing subsport
        assert_eq!(map_fit_sport(99, Some(30)), Sport::CrossCountrySkiing);
        // Unknown sport with downhill skiing subsport
        assert_eq!(map_fit_sport(99, Some(31)), Sport::DownhillSkiing);
    }

    #[test]
    fn test_map_fit_sport_unknown() {
        assert_eq!(map_fit_sport(99, None), Sport::Other);
        assert_eq!(map_fit_sport(255, None), Sport::Other);
        assert_eq!(map_fit_sport(100, Some(99)), Sport::Other);
    }

    #[test]
    fn test_map_swim_stroke() {
        assert_eq!(map_swim_stroke(0), StrokeType::Freestyle);
        assert_eq!(map_swim_stroke(1), StrokeType::Backstroke);
        assert_eq!(map_swim_stroke(2), StrokeType::Breaststroke);
        assert_eq!(map_swim_stroke(3), StrokeType::Butterfly);
        assert_eq!(map_swim_stroke(4), StrokeType::Drill);
        assert_eq!(map_swim_stroke(5), StrokeType::Mixed);
        assert_eq!(map_swim_stroke(6), StrokeType::IndividualMedley);
    }

    #[test]
    fn test_map_swim_stroke_unknown() {
        assert_eq!(map_swim_stroke(99), StrokeType::Freestyle);
    }
}
