//! GPX activity type mappings to PWF sports

use pwf_core::Sport;

/// Map GPX activity type to PWF sport
///
/// GPX doesn't have a standard activity type field, so this is primarily
/// for future extension if we parse GPX extension data.
pub fn map_gpx_type_to_sport(gpx_type: Option<&str>) -> Sport {
    match gpx_type {
        Some("run") | Some("running") => Sport::Running,
        Some("bike") | Some("biking") | Some("cycling") => Sport::Cycling,
        Some("hike") | Some("hiking") => Sport::Hiking,
        Some("walk") | Some("walking") => Sport::Walking,
        Some("swim") | Some("swimming") => Sport::Swimming,
        Some("ski") | Some("skiing") | Some("xc-ski") => Sport::CrossCountrySkiing,
        Some("downhill") | Some("alpine") => Sport::DownhillSkiing,
        Some("snowboard") | Some("snowboarding") => Sport::Snowboarding,
        Some("paddle") | Some("paddling") | Some("sup") => Sport::StandUpPaddling,
        Some("kayak") | Some("kayaking") | Some("canoe") => Sport::Kayaking,
        Some("row") | Some("rowing") => Sport::Rowing,
        _ => Sport::Other,
    }
}

/// Infer sport type from GPX metadata
///
/// Some GPX files include metadata about the activity type.
/// This function tries to extract that information.
pub fn infer_sport_from_metadata(gpx: &gpx::Gpx) -> Sport {
    // Check metadata keywords or description for sport hints
    if let Some(ref metadata) = gpx.metadata {
        if let Some(ref keywords) = metadata.keywords {
            let keywords_lower = keywords.to_lowercase();
            if keywords_lower.contains("run") {
                return Sport::Running;
            } else if keywords_lower.contains("bike") || keywords_lower.contains("cycl") {
                return Sport::Cycling;
            } else if keywords_lower.contains("hike") {
                return Sport::Hiking;
            } else if keywords_lower.contains("walk") {
                return Sport::Walking;
            } else if keywords_lower.contains("swim") {
                return Sport::Swimming;
            }
        }

        if let Some(ref description) = metadata.description {
            let desc_lower = description.to_lowercase();
            if desc_lower.contains("run") {
                return Sport::Running;
            } else if desc_lower.contains("bike") || desc_lower.contains("cycl") {
                return Sport::Cycling;
            } else if desc_lower.contains("hike") {
                return Sport::Hiking;
            } else if desc_lower.contains("walk") {
                return Sport::Walking;
            }
        }
    }

    // Default to "Other" if we can't determine the sport
    Sport::Other
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_gpx_type_to_sport() {
        assert_eq!(map_gpx_type_to_sport(Some("run")), Sport::Running);
        assert_eq!(map_gpx_type_to_sport(Some("running")), Sport::Running);
        assert_eq!(map_gpx_type_to_sport(Some("bike")), Sport::Cycling);
        assert_eq!(map_gpx_type_to_sport(Some("cycling")), Sport::Cycling);
        assert_eq!(map_gpx_type_to_sport(Some("hike")), Sport::Hiking);
        assert_eq!(map_gpx_type_to_sport(Some("walk")), Sport::Walking);
        assert_eq!(map_gpx_type_to_sport(Some("swim")), Sport::Swimming);
        assert_eq!(map_gpx_type_to_sport(Some("kayaking")), Sport::Kayaking);
        assert_eq!(
            map_gpx_type_to_sport(Some("snowboarding")),
            Sport::Snowboarding
        );
        assert_eq!(map_gpx_type_to_sport(Some("sup")), Sport::StandUpPaddling);
        assert_eq!(map_gpx_type_to_sport(Some("unknown")), Sport::Other);
        assert_eq!(map_gpx_type_to_sport(None), Sport::Other);
    }

    #[test]
    fn test_infer_sport_from_metadata_keywords() {
        let mut gpx = gpx::Gpx::default();
        let metadata = gpx::Metadata {
            keywords: Some("running morning workout".to_string()),
            ..Default::default()
        };
        gpx.metadata = Some(metadata);

        assert_eq!(infer_sport_from_metadata(&gpx), Sport::Running);
    }

    #[test]
    fn test_infer_sport_from_metadata_description() {
        let mut gpx = gpx::Gpx::default();
        let metadata = gpx::Metadata {
            description: Some("Cycling in the park".to_string()),
            ..Default::default()
        };
        gpx.metadata = Some(metadata);

        assert_eq!(infer_sport_from_metadata(&gpx), Sport::Cycling);
    }

    #[test]
    fn test_infer_sport_default() {
        let gpx = gpx::Gpx::default();
        assert_eq!(infer_sport_from_metadata(&gpx), Sport::Other);
    }
}
