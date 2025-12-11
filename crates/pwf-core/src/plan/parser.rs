//! YAML parsing for PWF plans

use super::types::WpsPlan;
use crate::error::ParseError;

/// Parse a YAML string into a WpsPlan
pub fn parse(yaml: &str) -> Result<WpsPlan, ParseError> {
    let plan: WpsPlan = serde_yaml::from_str(yaml)?;
    Ok(plan)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_plan() {
        let yaml = r#"
plan_version: 1
cycle:
  days:
    - exercises:
        - name: Push-ups
          modality: strength
"#;
        let plan = parse(yaml).unwrap();
        assert_eq!(plan.plan_version, 1);
        assert_eq!(plan.cycle.days.len(), 1);
    }

    #[test]
    fn parse_full_plan() {
        let yaml = r#"
plan_version: 1
meta:
  title: "Test Plan"
  author: "Test Author"
  equipment: [barbell, dumbbells]
  daysPerWeek: 3
cycle:
  notes: "Test notes"
  days:
    - focus: "Day 1"
      exercises:
        - name: "Squat"
          modality: strength
          target_sets: 3
          target_reps: 5
"#;
        let plan = parse(yaml).unwrap();
        assert_eq!(plan.meta.as_ref().unwrap().title, "Test Plan");
        assert_eq!(plan.meta.as_ref().unwrap().equipment.len(), 2);
    }

    #[test]
    fn parse_invalid_yaml() {
        let yaml = "not: valid: yaml: [";
        assert!(parse(yaml).is_err());
    }
}
