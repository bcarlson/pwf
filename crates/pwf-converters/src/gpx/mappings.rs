//! GPX activity type mappings to PWF sports

/// Map GPX activity type to PWF sport
///
/// GPX doesn't have a standard activity type field, so this is primarily
/// for future extension if we parse GPX extension data.
pub fn map_gpx_type_to_sport(gpx_type: Option<&str>) -> String {
    match gpx_type {
        Some("run") | Some("running") => "Running".to_string(),
        Some("bike") | Some("biking") | Some("cycling") => "Cycling".to_string(),
        Some("hike") | Some("hiking") => "Hiking".to_string(),
        Some("walk") | Some("walking") => "Walking".to_string(),
        Some("swim") | Some("swimming") => "Swimming".to_string(),
        Some("ski") | Some("skiing") => "Skiing".to_string(),
        Some("paddle") | Some("paddling") | Some("kayaking") => "Paddling".to_string(),
        Some("row") | Some("rowing") => "Rowing".to_string(),
        _ => "Other".to_string(),
    }
}

/// Infer sport type from GPX metadata
///
/// Some GPX files include metadata about the activity type.
/// This function tries to extract that information.
pub fn infer_sport_from_metadata(gpx: &gpx::Gpx) -> String {
    // Check metadata keywords or description for sport hints
    if let Some(ref metadata) = gpx.metadata {
        if let Some(ref keywords) = metadata.keywords {
            let keywords_lower = keywords.to_lowercase();
            if keywords_lower.contains("run") {
                return "Running".to_string();
            } else if keywords_lower.contains("bike") || keywords_lower.contains("cycl") {
                return "Cycling".to_string();
            } else if keywords_lower.contains("hike") {
                return "Hiking".to_string();
            } else if keywords_lower.contains("walk") {
                return "Walking".to_string();
            } else if keywords_lower.contains("swim") {
                return "Swimming".to_string();
            }
        }

        if let Some(ref description) = metadata.description {
            let desc_lower = description.to_lowercase();
            if desc_lower.contains("run") {
                return "Running".to_string();
            } else if desc_lower.contains("bike") || desc_lower.contains("cycl") {
                return "Cycling".to_string();
            } else if desc_lower.contains("hike") {
                return "Hiking".to_string();
            } else if desc_lower.contains("walk") {
                return "Walking".to_string();
            }
        }
    }

    // Default to "Other" if we can't determine the sport
    "Other".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_gpx_type_to_sport() {
        assert_eq!(map_gpx_type_to_sport(Some("run")), "Running");
        assert_eq!(map_gpx_type_to_sport(Some("running")), "Running");
        assert_eq!(map_gpx_type_to_sport(Some("bike")), "Cycling");
        assert_eq!(map_gpx_type_to_sport(Some("cycling")), "Cycling");
        assert_eq!(map_gpx_type_to_sport(Some("hike")), "Hiking");
        assert_eq!(map_gpx_type_to_sport(Some("walk")), "Walking");
        assert_eq!(map_gpx_type_to_sport(Some("swim")), "Swimming");
        assert_eq!(map_gpx_type_to_sport(Some("unknown")), "Other");
        assert_eq!(map_gpx_type_to_sport(None), "Other");
    }

    #[test]
    fn test_infer_sport_from_metadata_keywords() {
        let mut gpx = gpx::Gpx::default();
        let metadata = gpx::Metadata {
            keywords: Some("running morning workout".to_string()),
            ..Default::default()
        };
        gpx.metadata = Some(metadata);

        assert_eq!(infer_sport_from_metadata(&gpx), "Running");
    }

    #[test]
    fn test_infer_sport_from_metadata_description() {
        let mut gpx = gpx::Gpx::default();
        let metadata = gpx::Metadata {
            description: Some("Cycling in the park".to_string()),
            ..Default::default()
        };
        gpx.metadata = Some(metadata);

        assert_eq!(infer_sport_from_metadata(&gpx), "Cycling");
    }

    #[test]
    fn test_infer_sport_default() {
        let gpx = gpx::Gpx::default();
        assert_eq!(infer_sport_from_metadata(&gpx), "Other");
    }
}
