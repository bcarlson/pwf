//! Field mapping logic from TCX to PWF

use pwf_core::history::StrokeType;
use pwf_core::Sport;

/// Map TCX sport type to PWF Sport enum
///
/// TCX uses sport attribute on Activity element
pub fn map_tcx_sport(tcx_sport: &str) -> Sport {
    match tcx_sport.to_lowercase().as_str() {
        "running" => Sport::Running,
        "biking" | "cycling" => Sport::Cycling,
        "swimming" => Sport::Swimming,
        "rowing" => Sport::Rowing,
        "transition" => Sport::Transition,
        "other" => Sport::Other,
        // Map hiking/walking to "other" since PWF doesn't have these specific sports
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
/// TCX uses sport attribute on Activity element with values: Running, Biking, Other
pub fn map_pwf_sport_to_tcx(sport: &Sport) -> String {
    match sport {
        Sport::Running => "Running".to_string(),
        Sport::Cycling => "Biking".to_string(),
        Sport::Swimming => "Other".to_string(), // TCX v2 doesn't have Swimming as standard
        Sport::Rowing => "Other".to_string(),
        Sport::Transition => "Other".to_string(),
        Sport::Strength => "Other".to_string(),
        Sport::Other => "Other".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_tcx_sport() {
        assert_eq!(map_tcx_sport("Running"), Sport::Running);
        assert_eq!(map_tcx_sport("Biking"), Sport::Cycling);
        assert_eq!(map_tcx_sport("Cycling"), Sport::Cycling);
        assert_eq!(map_tcx_sport("Swimming"), Sport::Swimming);
        assert_eq!(map_tcx_sport("Rowing"), Sport::Rowing);
        assert_eq!(map_tcx_sport("Transition"), Sport::Transition);
        assert_eq!(map_tcx_sport("Other"), Sport::Other);
        assert_eq!(map_tcx_sport("Unknown"), Sport::Other);
        // Hiking and walking map to Other
        assert_eq!(map_tcx_sport("Hiking"), Sport::Other);
        assert_eq!(map_tcx_sport("Walking"), Sport::Other);
    }

    #[test]
    fn test_map_tcx_sport_case_insensitive() {
        assert_eq!(map_tcx_sport("RUNNING"), Sport::Running);
        assert_eq!(map_tcx_sport("running"), Sport::Running);
        assert_eq!(map_tcx_sport("RuNnInG"), Sport::Running);
    }
}
