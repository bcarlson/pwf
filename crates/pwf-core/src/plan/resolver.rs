//! Exercise resolution logic for merging library exercises with plan overrides

use super::types::{LibraryExercise, PlanExercise};
use crate::Modality;

/// Resolved exercise with all fields merged from library and overrides
#[derive(Debug, Clone)]
pub struct ResolvedExercise {
    pub id: Option<String>,
    pub name: String,
    pub modality: Modality,
    pub target_sets: Option<u32>,
    pub target_reps: Option<u32>,
    pub target_duration_sec: Option<u32>,
    pub target_distance_meters: Option<f64>,
    pub target_load: Option<String>,
    pub target_weight_percent: Option<f64>,
    pub percent_of: Option<String>,
    pub reference_exercise: Option<String>,
    pub cues: Option<String>,
    pub target_notes: Option<String>,
    pub link: Option<String>,
    pub image: Option<String>,
    pub group: Option<String>,
    pub group_type: Option<super::types::GroupType>,
    pub rest_between_sets_sec: Option<u32>,
    pub rest_after_sec: Option<u32>,
}

/// Resolve a plan exercise, merging library exercise if exercise_ref is present
pub fn resolve_exercise(
    exercise: &PlanExercise,
    library: &[LibraryExercise],
) -> Option<ResolvedExercise> {
    if let Some(ref exercise_ref) = exercise.exercise_ref {
        // Find library exercise
        let lib_exercise = library.iter().find(|lib| &lib.id == exercise_ref)?;

        // Merge library defaults with plan overrides
        Some(ResolvedExercise {
            id: exercise.id.clone(),
            name: exercise
                .name
                .clone()
                .unwrap_or_else(|| lib_exercise.name.clone()),
            modality: exercise.modality.unwrap_or(lib_exercise.modality),
            target_sets: exercise.target_sets.or(lib_exercise.default_sets),
            target_reps: exercise.target_reps.or(lib_exercise.default_reps),
            target_duration_sec: exercise
                .target_duration_sec
                .or(lib_exercise.default_duration_sec),
            target_distance_meters: exercise
                .target_distance_meters
                .or(lib_exercise.default_distance_meters),
            target_load: exercise.target_load.clone(),
            target_weight_percent: exercise.target_weight_percent,
            percent_of: exercise.percent_of.clone(),
            reference_exercise: exercise.reference_exercise.clone(),
            cues: exercise.cues.clone().or_else(|| lib_exercise.cues.clone()),
            target_notes: exercise.target_notes.clone(),
            link: exercise.link.clone().or_else(|| lib_exercise.link.clone()),
            image: exercise
                .image
                .clone()
                .or_else(|| lib_exercise.image.clone()),
            group: exercise.group.clone(),
            group_type: exercise.group_type,
            rest_between_sets_sec: exercise.rest_between_sets_sec,
            rest_after_sec: exercise.rest_after_sec,
        })
    } else {
        // No library reference, use plan exercise directly
        exercise.modality.map(|modality| ResolvedExercise {
            id: exercise.id.clone(),
            name: exercise
                .name
                .clone()
                .unwrap_or_else(|| "Unnamed Exercise".to_string()),
            modality,
            target_sets: exercise.target_sets,
            target_reps: exercise.target_reps,
            target_duration_sec: exercise.target_duration_sec,
            target_distance_meters: exercise.target_distance_meters,
            target_load: exercise.target_load.clone(),
            target_weight_percent: exercise.target_weight_percent,
            percent_of: exercise.percent_of.clone(),
            reference_exercise: exercise.reference_exercise.clone(),
            cues: exercise.cues.clone(),
            target_notes: exercise.target_notes.clone(),
            link: exercise.link.clone(),
            image: exercise.image.clone(),
            group: exercise.group.clone(),
            group_type: exercise.group_type,
            rest_between_sets_sec: exercise.rest_between_sets_sec,
            rest_after_sec: exercise.rest_after_sec,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::types::{Difficulty, LibraryExercise, PlanExercise};
    use crate::Modality;

    #[test]
    fn test_resolve_with_library_reference() {
        let library = vec![LibraryExercise {
            id: "squat".to_string(),
            name: "Barbell Back Squat".to_string(),
            description: Some("Compound leg exercise".to_string()),
            equipment: vec!["barbell".to_string()],
            muscle_groups: vec!["quads".to_string(), "glutes".to_string()],
            difficulty: Some(Difficulty::Intermediate),
            modality: Modality::Strength,
            default_sets: Some(3),
            default_reps: Some(8),
            default_duration_sec: None,
            default_distance_meters: None,
            cues: Some("Keep chest up".to_string()),
            link: Some("https://example.com/squat".to_string()),
            image: Some("https://example.com/squat.jpg".to_string()),
        }];

        let exercise = PlanExercise {
            id: None,
            name: None,
            exercise_ref: Some("squat".to_string()),
            modality: None,
            target_sets: Some(5), // Override
            target_reps: None,    // Use default
            target_duration_sec: None,
            target_distance_meters: None,
            target_load: Some("100kg".to_string()),
            target_weight_percent: None,
            percent_of: None,
            reference_exercise: None,
            cues: None,
            target_notes: Some("Add 2.5kg from last week".to_string()),
            link: None,
            image: None,
            group: None,
            group_type: None,
            rest_between_sets_sec: Some(180),
            rest_after_sec: None,
            zones: None,
            ramp: None,
            interval_phases: None,
        };

        let resolved = resolve_exercise(&exercise, &library).unwrap();

        assert_eq!(resolved.name, "Barbell Back Squat");
        assert_eq!(resolved.modality, Modality::Strength);
        assert_eq!(resolved.target_sets, Some(5)); // Overridden
        assert_eq!(resolved.target_reps, Some(8)); // From library
        assert_eq!(resolved.target_load, Some("100kg".to_string()));
        assert_eq!(resolved.cues, Some("Keep chest up".to_string())); // From library
        assert_eq!(
            resolved.target_notes,
            Some("Add 2.5kg from last week".to_string())
        );
        assert_eq!(resolved.rest_between_sets_sec, Some(180));
    }

    #[test]
    fn test_resolve_without_library_reference() {
        let library = vec![];

        let exercise = PlanExercise {
            id: None,
            name: Some("Push-ups".to_string()),
            exercise_ref: None,
            modality: Some(Modality::Strength),
            target_sets: Some(3),
            target_reps: Some(15),
            target_duration_sec: None,
            target_distance_meters: None,
            target_load: None,
            target_weight_percent: None,
            percent_of: None,
            reference_exercise: None,
            cues: Some("Keep core tight".to_string()),
            target_notes: None,
            link: None,
            image: None,
            group: None,
            group_type: None,
            rest_between_sets_sec: Some(60),
            rest_after_sec: None,
            zones: None,
            ramp: None,
            interval_phases: None,
        };

        let resolved = resolve_exercise(&exercise, &library).unwrap();

        assert_eq!(resolved.name, "Push-ups");
        assert_eq!(resolved.modality, Modality::Strength);
        assert_eq!(resolved.target_sets, Some(3));
        assert_eq!(resolved.target_reps, Some(15));
        assert_eq!(resolved.cues, Some("Keep core tight".to_string()));
    }

    #[test]
    fn test_resolve_library_not_found() {
        let library = vec![];

        let exercise = PlanExercise {
            id: None,
            name: None,
            exercise_ref: Some("nonexistent".to_string()),
            modality: None,
            target_sets: None,
            target_reps: None,
            target_duration_sec: None,
            target_distance_meters: None,
            target_load: None,
            target_weight_percent: None,
            percent_of: None,
            reference_exercise: None,
            cues: None,
            target_notes: None,
            link: None,
            image: None,
            group: None,
            group_type: None,
            rest_between_sets_sec: None,
            rest_after_sec: None,
            zones: None,
            ramp: None,
            interval_phases: None,
        };

        let resolved = resolve_exercise(&exercise, &library);

        assert!(resolved.is_none());
    }

    #[test]
    fn test_resolve_override_all_fields() {
        let library = vec![LibraryExercise {
            id: "plank".to_string(),
            name: "Plank Hold".to_string(),
            description: None,
            equipment: vec![],
            muscle_groups: vec!["core".to_string()],
            difficulty: Some(Difficulty::Beginner),
            modality: Modality::Countdown,
            default_sets: Some(3),
            default_reps: None,
            default_duration_sec: Some(30),
            default_distance_meters: None,
            cues: Some("Keep back flat".to_string()),
            link: None,
            image: None,
        }];

        let exercise = PlanExercise {
            id: Some("plank-1".to_string()),
            name: Some("Modified Plank".to_string()),
            exercise_ref: Some("plank".to_string()),
            modality: Some(Modality::Countdown),
            target_sets: Some(4),
            target_reps: None,
            target_duration_sec: Some(45),
            target_distance_meters: None,
            target_load: None,
            target_weight_percent: None,
            percent_of: None,
            reference_exercise: None,
            cues: Some("Use knees if needed".to_string()),
            target_notes: Some("Focus on form".to_string()),
            link: Some("https://example.com/modified".to_string()),
            image: None,
            group: None,
            group_type: None,
            rest_between_sets_sec: Some(90),
            rest_after_sec: None,
            zones: None,
            ramp: None,
            interval_phases: None,
        };

        let resolved = resolve_exercise(&exercise, &library).unwrap();

        assert_eq!(resolved.id, Some("plank-1".to_string()));
        assert_eq!(resolved.name, "Modified Plank");
        assert_eq!(resolved.target_sets, Some(4));
        assert_eq!(resolved.target_duration_sec, Some(45));
        assert_eq!(resolved.cues, Some("Use knees if needed".to_string()));
        assert_eq!(
            resolved.link,
            Some("https://example.com/modified".to_string())
        );
    }
}
