//! Field mapping logic from FIT to PWF

use pwf_core::history::StrokeType;
use pwf_core::Sport;

/// Map FIT sport type to PWF Sport enum
///
/// FIT SDK sport types: https://developer.garmin.com/fit/file-types/activity/
pub fn map_fit_sport(fit_sport: u8, _fit_subsport: Option<u8>) -> Sport {
    match fit_sport {
        0 => Sport::Running,
        1 => Sport::Cycling,
        2 => Sport::Running, // Transition - treat as running
        5 => Sport::Swimming,
        // TODO: Add more sport mappings as needed
        _ => Sport::Other,
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
    fn test_map_fit_sport() {
        assert_eq!(map_fit_sport(0, None), Sport::Running);
        assert_eq!(map_fit_sport(1, None), Sport::Cycling);
        assert_eq!(map_fit_sport(5, None), Sport::Swimming);
        assert_eq!(map_fit_sport(99, None), Sport::Other);
    }

    #[test]
    fn test_map_swim_stroke() {
        assert_eq!(map_swim_stroke(0), StrokeType::Freestyle);
        assert_eq!(map_swim_stroke(1), StrokeType::Backstroke);
        assert_eq!(map_swim_stroke(2), StrokeType::Breaststroke);
        assert_eq!(map_swim_stroke(3), StrokeType::Butterfly);
    }
}
