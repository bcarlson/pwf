//! YAML parsing for WPS history exports

use super::types::WpsHistory;
use crate::error::ParseError;

/// Parse a YAML string into a WpsHistory
pub fn parse(yaml: &str) -> Result<WpsHistory, ParseError> {
    let history: WpsHistory = serde_yaml::from_str(yaml)?;
    Ok(history)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_history() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: Squat
        sets:
          - reps: 5
            weight_kg: 100
"#;
        let history = parse(yaml).unwrap();
        assert_eq!(history.history_version, 1);
        assert_eq!(history.workouts.len(), 1);
    }

    #[test]
    fn parse_full_history() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
export_source:
  app_name: "OwnLift"
  app_version: "1.0.0"
units:
  weight: kg
  distance: meters
workouts:
  - date: "2025-01-15"
    title: "Push Day"
    duration_sec: 3600
    exercises:
      - name: "Bench Press"
        modality: strength
        sets:
          - reps: 5
            weight_kg: 100
            rpe: 8
          - reps: 5
            weight_kg: 100
            rpe: 8.5
personal_records:
  - exercise_name: "Bench Press"
    record_type: max_weight
    value: 100
    achieved_at: "2025-01-15"
body_measurements:
  - date: "2025-01-15"
    weight_kg: 85.5
"#;
        let history = parse(yaml).unwrap();
        assert_eq!(history.workouts.len(), 1);
        assert_eq!(history.personal_records.len(), 1);
        assert_eq!(history.body_measurements.len(), 1);
    }
}
