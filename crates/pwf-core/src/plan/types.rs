//! Plan type definitions

use crate::Modality;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root plan structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WpsPlan {
    pub plan_version: u32,
    #[serde(default)]
    pub meta: Option<PlanMeta>,
    #[serde(default)]
    pub glossary: HashMap<String, String>,
    pub cycle: PlanCycle,
}

/// Plan metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlanMeta {
    #[serde(default)]
    pub id: Option<String>,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub status: Option<PlanStatus>,
    #[serde(default)]
    pub activated_at: Option<String>,
    #[serde(default)]
    pub completed_at: Option<String>,
    #[serde(default)]
    pub equipment: Vec<String>,
    #[serde(default, rename = "daysPerWeek")]
    pub days_per_week: Option<u8>,
    #[serde(default, rename = "recommendedFirst")]
    pub recommended_first: bool,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Plan status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PlanStatus {
    #[default]
    Draft,
    Active,
    Completed,
    Archived,
}

/// Training cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanCycle {
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    pub days: Vec<PlanDay>,
}

/// Single training day
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDay {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub order: Option<u32>,
    #[serde(default)]
    pub focus: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub scheduled_date: Option<String>,
    #[serde(default, rename = "target_session_length_min")]
    pub target_session_length_min: Option<u32>,
    pub exercises: Vec<PlanExercise>,
}

/// Exercise grouping type for supersets and circuits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupType {
    Superset,
    Circuit,
}

/// Single exercise in a plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanExercise {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    pub modality: Modality,
    #[serde(default)]
    pub target_sets: Option<u32>,
    #[serde(default)]
    pub target_reps: Option<u32>,
    #[serde(default)]
    pub target_duration_sec: Option<u32>,
    #[serde(default)]
    pub target_distance_meters: Option<f64>,
    #[serde(default)]
    pub target_load: Option<String>,
    #[serde(default)]
    pub target_weight_percent: Option<f64>,
    #[serde(default)]
    pub percent_of: Option<String>,
    #[serde(default)]
    pub reference_exercise: Option<String>,
    #[serde(default)]
    pub cues: Option<String>,
    #[serde(default)]
    pub target_notes: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub group_type: Option<GroupType>,
    #[serde(default)]
    pub rest_between_sets_sec: Option<u32>,
    #[serde(default)]
    pub rest_after_sec: Option<u32>,
}

/// Statistics about a parsed plan
#[derive(Debug, Clone, Default, Serialize)]
pub struct PlanStatistics {
    pub total_days: usize,
    pub total_exercises: usize,
    pub strength_count: usize,
    pub countdown_count: usize,
    pub stopwatch_count: usize,
    pub interval_count: usize,
    pub equipment: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Modality;

    #[test]
    fn test_wps_plan_roundtrip() {
        let plan = WpsPlan {
            plan_version: 1,
            meta: Some(PlanMeta {
                title: "Test Plan".to_string(),
                ..Default::default()
            }),
            glossary: HashMap::new(),
            cycle: PlanCycle {
                start_date: None,
                notes: None,
                days: vec![],
            },
        };

        let yaml = serde_yaml::to_string(&plan).unwrap();
        let deserialized: WpsPlan = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(plan.plan_version, deserialized.plan_version);
        assert!(deserialized.meta.is_some());
        assert_eq!(
            plan.meta.as_ref().unwrap().title,
            deserialized.meta.as_ref().unwrap().title
        );
    }

    #[test]
    fn test_wps_plan_with_glossary() {
        let mut glossary = HashMap::new();
        glossary.insert("BB".to_string(), "Barbell".to_string());
        glossary.insert("DB".to_string(), "Dumbbell".to_string());

        let plan = WpsPlan {
            plan_version: 1,
            meta: None,
            glossary: glossary.clone(),
            cycle: PlanCycle {
                start_date: None,
                notes: None,
                days: vec![],
            },
        };

        let yaml = serde_yaml::to_string(&plan).unwrap();
        let deserialized: WpsPlan = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(deserialized.glossary.len(), 2);
        assert_eq!(
            deserialized.glossary.get("BB"),
            Some(&"Barbell".to_string())
        );
        assert_eq!(
            deserialized.glossary.get("DB"),
            Some(&"Dumbbell".to_string())
        );
    }

    #[test]
    fn test_plan_meta_all_fields() {
        let meta = PlanMeta {
            id: Some("plan-123".to_string()),
            title: "Advanced Strength Program".to_string(),
            description: Some("A comprehensive strength training program".to_string()),
            author: Some("John Doe".to_string()),
            status: Some(PlanStatus::Active),
            activated_at: Some("2024-01-15".to_string()),
            completed_at: Some("2024-04-15".to_string()),
            equipment: vec!["Barbell".to_string(), "Bench".to_string()],
            days_per_week: Some(4),
            recommended_first: true,
            tags: vec!["strength".to_string(), "hypertrophy".to_string()],
        };

        let yaml = serde_yaml::to_string(&meta).unwrap();
        let deserialized: PlanMeta = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(meta.id, deserialized.id);
        assert_eq!(meta.title, deserialized.title);
        assert_eq!(meta.description, deserialized.description);
        assert_eq!(meta.author, deserialized.author);
        assert_eq!(meta.status, deserialized.status);
        assert_eq!(meta.activated_at, deserialized.activated_at);
        assert_eq!(meta.completed_at, deserialized.completed_at);
        assert_eq!(meta.equipment, deserialized.equipment);
        assert_eq!(meta.days_per_week, deserialized.days_per_week);
        assert_eq!(meta.recommended_first, deserialized.recommended_first);
        assert_eq!(meta.tags, deserialized.tags);
    }

    #[test]
    fn test_plan_meta_optional_fields_omitted() {
        let meta = PlanMeta {
            title: "Minimal Plan".to_string(),
            ..Default::default()
        };

        let yaml = serde_yaml::to_string(&meta).unwrap();

        // Optional fields with None values will be explicitly shown as null in YAML
        // but when deserialized from minimal YAML, they should be None
        let deserialized: PlanMeta = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(deserialized.id, None);
        assert_eq!(deserialized.description, None);
        assert_eq!(deserialized.author, None);
        assert_eq!(deserialized.activated_at, None);
        assert_eq!(deserialized.completed_at, None);
        assert_eq!(deserialized.days_per_week, None);

        // Verify that minimal YAML without these fields also deserializes correctly
        let minimal_yaml = r#"
title: "Minimal Plan"
equipment: []
recommendedFirst: false
tags: []
"#;
        let minimal_deserialized: PlanMeta = serde_yaml::from_str(minimal_yaml).unwrap();
        assert_eq!(minimal_deserialized.id, None);
        assert_eq!(minimal_deserialized.description, None);
        assert_eq!(minimal_deserialized.author, None);
        assert_eq!(minimal_deserialized.status, None);
    }

    #[test]
    fn test_plan_meta_fields_absent_deserialize_none() {
        let yaml = r#"
title: "Basic Plan"
"#;
        let deserialized: PlanMeta = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(deserialized.title, "Basic Plan");
        assert_eq!(deserialized.id, None);
        assert_eq!(deserialized.description, None);
        assert_eq!(deserialized.author, None);
        assert_eq!(deserialized.status, None);
        assert_eq!(deserialized.activated_at, None);
        assert_eq!(deserialized.completed_at, None);
        assert_eq!(deserialized.equipment, Vec::<String>::new());
        assert_eq!(deserialized.days_per_week, None);
        assert!(!deserialized.recommended_first);
        assert_eq!(deserialized.tags, Vec::<String>::new());
    }

    #[test]
    fn test_plan_meta_default() {
        let meta = PlanMeta::default();

        assert_eq!(meta.id, None);
        assert_eq!(meta.title, "");
        assert_eq!(meta.description, None);
        assert_eq!(meta.author, None);
        assert_eq!(meta.status, None);
        assert_eq!(meta.activated_at, None);
        assert_eq!(meta.completed_at, None);
        assert_eq!(meta.equipment, Vec::<String>::new());
        assert_eq!(meta.days_per_week, None);
        assert!(!meta.recommended_first);
        assert_eq!(meta.tags, Vec::<String>::new());
    }

    #[test]
    fn test_plan_status_all_variants() {
        let statuses = vec![
            PlanStatus::Draft,
            PlanStatus::Active,
            PlanStatus::Completed,
            PlanStatus::Archived,
        ];

        for status in statuses {
            let yaml = serde_yaml::to_string(&status).unwrap();
            let deserialized: PlanStatus = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(status, deserialized);
        }
    }

    #[test]
    fn test_plan_status_snake_case_serialization() {
        assert_eq!(
            serde_yaml::to_string(&PlanStatus::Draft).unwrap().trim(),
            "draft"
        );
        assert_eq!(
            serde_yaml::to_string(&PlanStatus::Active).unwrap().trim(),
            "active"
        );
        assert_eq!(
            serde_yaml::to_string(&PlanStatus::Completed)
                .unwrap()
                .trim(),
            "completed"
        );
        assert_eq!(
            serde_yaml::to_string(&PlanStatus::Archived).unwrap().trim(),
            "archived"
        );
    }

    #[test]
    fn test_plan_status_default() {
        let status = PlanStatus::default();
        assert_eq!(status, PlanStatus::Draft);
    }

    #[test]
    fn test_plan_cycle_roundtrip() {
        let cycle = PlanCycle {
            start_date: Some("2024-01-01".to_string()),
            notes: Some("Week 1-4: Foundation phase".to_string()),
            days: vec![
                PlanDay {
                    id: Some("day-1".to_string()),
                    order: Some(1),
                    focus: Some("Upper Body".to_string()),
                    notes: None,
                    scheduled_date: None,
                    target_session_length_min: Some(60),
                    exercises: vec![],
                },
                PlanDay {
                    id: Some("day-2".to_string()),
                    order: Some(2),
                    focus: Some("Lower Body".to_string()),
                    notes: None,
                    scheduled_date: None,
                    target_session_length_min: Some(75),
                    exercises: vec![],
                },
            ],
        };

        let yaml = serde_yaml::to_string(&cycle).unwrap();
        let deserialized: PlanCycle = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(cycle.start_date, deserialized.start_date);
        assert_eq!(cycle.notes, deserialized.notes);
        assert_eq!(cycle.days.len(), deserialized.days.len());
        assert_eq!(
            cycle.days[0].focus.as_ref().unwrap(),
            deserialized.days[0].focus.as_ref().unwrap()
        );
        assert_eq!(
            cycle.days[1].focus.as_ref().unwrap(),
            deserialized.days[1].focus.as_ref().unwrap()
        );
    }

    #[test]
    fn test_plan_day_all_fields() {
        let day = PlanDay {
            id: Some("upper-1".to_string()),
            order: Some(1),
            focus: Some("Upper Body Power".to_string()),
            notes: Some("Focus on explosive movements".to_string()),
            scheduled_date: Some("2024-01-15".to_string()),
            target_session_length_min: Some(90),
            exercises: vec![],
        };

        let yaml = serde_yaml::to_string(&day).unwrap();
        let deserialized: PlanDay = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(day.id, deserialized.id);
        assert_eq!(day.order, deserialized.order);
        assert_eq!(day.focus, deserialized.focus);
        assert_eq!(day.notes, deserialized.notes);
        assert_eq!(day.scheduled_date, deserialized.scheduled_date);
        assert_eq!(
            day.target_session_length_min,
            deserialized.target_session_length_min
        );
    }

    #[test]
    fn test_plan_exercise_strength_modality() {
        let exercise = PlanExercise {
            id: Some("ex-1".to_string()),
            name: Some("Bench Press".to_string()),
            modality: Modality::Strength,
            target_sets: Some(4),
            target_reps: Some(8),
            target_duration_sec: None,
            target_distance_meters: None,
            target_load: Some("80%".to_string()),
            target_weight_percent: None,
            percent_of: None,
            reference_exercise: None,
            cues: Some("Keep shoulders back and down".to_string()),
            target_notes: Some("RPE 8".to_string()),
            link: Some("https://example.com/bench-press".to_string()),
            image: Some("bench-press.jpg".to_string()),
            group: None,
            group_type: None,
            rest_between_sets_sec: None,
            rest_after_sec: None,
        };

        let yaml = serde_yaml::to_string(&exercise).unwrap();
        let deserialized: PlanExercise = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(exercise.id, deserialized.id);
        assert_eq!(exercise.name, deserialized.name);
        assert_eq!(exercise.target_sets, deserialized.target_sets);
        assert_eq!(exercise.target_reps, deserialized.target_reps);
        assert_eq!(exercise.target_load, deserialized.target_load);
        assert_eq!(exercise.cues, deserialized.cues);
        assert_eq!(exercise.target_notes, deserialized.target_notes);
        assert_eq!(exercise.link, deserialized.link);
        assert_eq!(exercise.image, deserialized.image);
    }

    #[test]
    fn test_plan_exercise_all_modalities() {
        let modalities = vec![
            (Modality::Strength, "strength"),
            (Modality::Countdown, "countdown"),
            (Modality::Stopwatch, "stopwatch"),
            (Modality::Interval, "interval"),
        ];

        for (modality, expected_name) in modalities {
            let exercise = PlanExercise {
                id: None,
                name: Some(format!("{} exercise", expected_name)),
                modality,
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
            };

            let yaml = serde_yaml::to_string(&exercise).unwrap();
            let deserialized: PlanExercise = serde_yaml::from_str(&yaml).unwrap();

            assert_eq!(
                format!("{:?}", exercise.modality),
                format!("{:?}", deserialized.modality)
            );
        }
    }

    #[test]
    fn test_plan_exercise_countdown_modality() {
        let exercise = PlanExercise {
            id: Some("ex-2".to_string()),
            name: Some("Plank Hold".to_string()),
            modality: Modality::Countdown,
            target_sets: Some(3),
            target_reps: None,
            target_duration_sec: Some(60),
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
            rest_between_sets_sec: None,
            rest_after_sec: None,
        };

        let yaml = serde_yaml::to_string(&exercise).unwrap();
        let deserialized: PlanExercise = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(exercise.name, deserialized.name);
        assert_eq!(
            exercise.target_duration_sec,
            deserialized.target_duration_sec
        );
    }

    #[test]
    fn test_plan_exercise_stopwatch_modality() {
        let exercise = PlanExercise {
            id: Some("ex-3".to_string()),
            name: Some("Row".to_string()),
            modality: Modality::Stopwatch,
            target_sets: None,
            target_reps: None,
            target_duration_sec: None,
            target_distance_meters: Some(2000.0),
            target_load: None,
            target_weight_percent: None,
            percent_of: None,
            reference_exercise: None,
            cues: Some("Maintain steady pace".to_string()),
            target_notes: Some("Target 2:00/500m split".to_string()),
            link: None,
            image: None,
            group: None,
            group_type: None,
            rest_between_sets_sec: None,
            rest_after_sec: None,
        };

        let yaml = serde_yaml::to_string(&exercise).unwrap();
        let deserialized: PlanExercise = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(
            exercise.target_distance_meters,
            deserialized.target_distance_meters
        );
        assert_eq!(exercise.target_notes, deserialized.target_notes);
    }

    #[test]
    fn test_plan_exercise_interval_modality() {
        let exercise = PlanExercise {
            id: Some("ex-4".to_string()),
            name: Some("Sprint Intervals".to_string()),
            modality: Modality::Interval,
            target_sets: Some(8),
            target_reps: None,
            target_duration_sec: Some(30),
            target_distance_meters: None,
            target_load: None,
            target_weight_percent: None,
            percent_of: None,
            reference_exercise: None,
            cues: Some("Max effort".to_string()),
            target_notes: Some("30s work / 90s rest".to_string()),
            link: None,
            image: None,
            group: None,
            group_type: None,
            rest_between_sets_sec: None,
            rest_after_sec: None,
        };

        let yaml = serde_yaml::to_string(&exercise).unwrap();
        let deserialized: PlanExercise = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(exercise.target_sets, deserialized.target_sets);
        assert_eq!(
            exercise.target_duration_sec,
            deserialized.target_duration_sec
        );
    }

    #[test]
    fn test_edge_case_empty_strings() {
        let meta = PlanMeta {
            id: Some("".to_string()),
            title: "".to_string(),
            description: Some("".to_string()),
            ..Default::default()
        };

        let yaml = serde_yaml::to_string(&meta).unwrap();
        let deserialized: PlanMeta = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(deserialized.id, Some("".to_string()));
        assert_eq!(deserialized.title, "");
        assert_eq!(deserialized.description, Some("".to_string()));
    }

    #[test]
    fn test_edge_case_unicode_strings() {
        let meta = PlanMeta {
            title: "力量训练计划".to_string(),
            description: Some("Программа тренировок 💪".to_string()),
            author: Some("José García".to_string()),
            tags: vec!["日本語".to_string(), "Русский".to_string()],
            ..Default::default()
        };

        let yaml = serde_yaml::to_string(&meta).unwrap();
        let deserialized: PlanMeta = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(deserialized.title, "力量训练计划");
        assert_eq!(
            deserialized.description,
            Some("Программа тренировок 💪".to_string())
        );
        assert_eq!(deserialized.author, Some("José García".to_string()));
        assert_eq!(deserialized.tags.len(), 2);
    }

    #[test]
    fn test_edge_case_very_long_strings() {
        let long_string = "a".repeat(10000);
        let meta = PlanMeta {
            title: long_string.clone(),
            description: Some(long_string.clone()),
            ..Default::default()
        };

        let yaml = serde_yaml::to_string(&meta).unwrap();
        let deserialized: PlanMeta = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(deserialized.title.len(), 10000);
        assert_eq!(deserialized.description.as_ref().unwrap().len(), 10000);
    }

    #[test]
    fn test_edge_case_large_numbers() {
        let exercise = PlanExercise {
            id: None,
            name: Some("Test".to_string()),
            modality: Modality::Strength,
            target_sets: Some(999999),
            target_reps: Some(999999),
            target_duration_sec: Some(999999),
            target_distance_meters: Some(999999.999),
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
        };

        let yaml = serde_yaml::to_string(&exercise).unwrap();
        let deserialized: PlanExercise = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(deserialized.target_sets, Some(999999));
        assert_eq!(deserialized.target_reps, Some(999999));
        assert_eq!(deserialized.target_duration_sec, Some(999999));
        assert_eq!(deserialized.target_distance_meters, Some(999999.999));
    }

    #[test]
    fn test_complete_plan_roundtrip() {
        let mut glossary = HashMap::new();
        glossary.insert("BB".to_string(), "Barbell".to_string());

        let plan = WpsPlan {
            plan_version: 1,
            meta: Some(PlanMeta {
                id: Some("plan-001".to_string()),
                title: "Complete Test Plan".to_string(),
                description: Some("Full integration test".to_string()),
                author: Some("Test Author".to_string()),
                status: Some(PlanStatus::Active),
                activated_at: Some("2024-01-01".to_string()),
                completed_at: None,
                equipment: vec!["Barbell".to_string(), "Bench".to_string()],
                days_per_week: Some(3),
                recommended_first: false,
                tags: vec!["strength".to_string()],
            }),
            glossary,
            cycle: PlanCycle {
                start_date: Some("2024-01-01".to_string()),
                notes: Some("Cycle notes".to_string()),
                days: vec![PlanDay {
                    id: Some("day-1".to_string()),
                    order: Some(1),
                    focus: Some("Upper".to_string()),
                    notes: Some("Day notes".to_string()),
                    scheduled_date: Some("2024-01-01".to_string()),
                    target_session_length_min: Some(60),
                    exercises: vec![
                        PlanExercise {
                            id: Some("ex-1".to_string()),
                            name: Some("Bench Press".to_string()),
                            modality: Modality::Strength,
                            target_sets: Some(3),
                            target_reps: Some(8),
                            target_duration_sec: None,
                            target_distance_meters: None,
                            target_load: Some("100kg".to_string()),
                            target_weight_percent: None,
                            percent_of: None,
                            reference_exercise: None,
                            cues: Some("Keep tight".to_string()),
                            target_notes: Some("RPE 8".to_string()),
                            link: Some("https://example.com".to_string()),
                            image: Some("bench.jpg".to_string()),
                            group: None,
                            group_type: None,
                            rest_between_sets_sec: None,
                            rest_after_sec: None,
                        },
                        PlanExercise {
                            id: Some("ex-2".to_string()),
                            name: Some("Plank".to_string()),
                            modality: Modality::Countdown,
                            target_sets: Some(3),
                            target_reps: None,
                            target_duration_sec: Some(60),
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
                        },
                    ],
                }],
            },
        };

        let yaml = serde_yaml::to_string(&plan).unwrap();
        let deserialized: WpsPlan = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(plan.plan_version, deserialized.plan_version);
        assert_eq!(
            plan.meta.as_ref().unwrap().title,
            deserialized.meta.as_ref().unwrap().title
        );
        assert_eq!(plan.glossary.len(), deserialized.glossary.len());
        assert_eq!(plan.cycle.days.len(), deserialized.cycle.days.len());
        assert_eq!(
            plan.cycle.days[0].exercises.len(),
            deserialized.cycle.days[0].exercises.len()
        );
        assert_eq!(
            plan.cycle.days[0].exercises[0].name,
            deserialized.cycle.days[0].exercises[0].name
        );
        assert_eq!(
            plan.cycle.days[0].exercises[1].target_duration_sec,
            deserialized.cycle.days[0].exercises[1].target_duration_sec
        );
    }

    #[test]
    fn test_plan_meta_rename_fields() {
        let meta = PlanMeta {
            title: "Test".to_string(),
            days_per_week: Some(5),
            recommended_first: true,
            ..Default::default()
        };

        let yaml = serde_yaml::to_string(&meta).unwrap();

        // Check that daysPerWeek is properly renamed in YAML
        assert!(yaml.contains("daysPerWeek: 5"));
        assert!(yaml.contains("recommendedFirst: true"));

        // Test deserialization with renamed fields
        let yaml_input = r#"
title: "Test Plan"
daysPerWeek: 4
recommendedFirst: false
"#;
        let deserialized: PlanMeta = serde_yaml::from_str(yaml_input).unwrap();

        assert_eq!(deserialized.days_per_week, Some(4));
        assert!(!deserialized.recommended_first);
    }

    #[test]
    fn test_plan_day_target_session_length_rename() {
        let day = PlanDay {
            id: None,
            order: None,
            focus: None,
            notes: None,
            scheduled_date: None,
            target_session_length_min: Some(45),
            exercises: vec![],
        };

        let yaml = serde_yaml::to_string(&day).unwrap();

        // Check that target_session_length_min is properly serialized
        assert!(yaml.contains("target_session_length_min: 45"));

        let deserialized: PlanDay = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(deserialized.target_session_length_min, Some(45));
    }

    #[test]
    fn test_group_type_serialization() {
        assert_eq!(
            serde_yaml::to_string(&GroupType::Superset).unwrap().trim(),
            "superset"
        );
        assert_eq!(
            serde_yaml::to_string(&GroupType::Circuit).unwrap().trim(),
            "circuit"
        );
    }

    #[test]
    fn test_group_type_deserialization() {
        let superset: GroupType = serde_yaml::from_str("superset").unwrap();
        assert_eq!(superset, GroupType::Superset);

        let circuit: GroupType = serde_yaml::from_str("circuit").unwrap();
        assert_eq!(circuit, GroupType::Circuit);
    }

    #[test]
    fn test_plan_exercise_with_group() {
        let exercise = PlanExercise {
            id: Some("ex-1".to_string()),
            name: Some("Bench Press".to_string()),
            modality: Modality::Strength,
            target_sets: Some(3),
            target_reps: Some(8),
            target_duration_sec: None,
            target_distance_meters: None,
            target_load: Some("135 lbs".to_string()),
            target_weight_percent: None,
            percent_of: None,
            reference_exercise: None,
            cues: None,
            target_notes: None,
            link: None,
            image: None,
            group: Some("A".to_string()),
            group_type: Some(GroupType::Superset),
            rest_between_sets_sec: None,
            rest_after_sec: None,
        };

        let yaml = serde_yaml::to_string(&exercise).unwrap();
        let deserialized: PlanExercise = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(exercise.group, deserialized.group);
        assert_eq!(exercise.group_type, deserialized.group_type);
    }

    #[test]
    fn test_plan_exercise_without_group() {
        let exercise = PlanExercise {
            id: Some("ex-1".to_string()),
            name: Some("Squat".to_string()),
            modality: Modality::Strength,
            target_sets: Some(5),
            target_reps: Some(5),
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
        };

        let yaml = serde_yaml::to_string(&exercise).unwrap();
        let deserialized: PlanExercise = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(deserialized.group, None);
        assert_eq!(deserialized.group_type, None);
    }

    #[test]
    fn test_plan_exercise_group_roundtrip() {
        let yaml = r#"
name: "Bench Press"
modality: strength
target_sets: 3
target_reps: 8
group: "A"
group_type: superset
"#;
        let exercise: PlanExercise = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(exercise.group, Some("A".to_string()));
        assert_eq!(exercise.group_type, Some(GroupType::Superset));

        let serialized = serde_yaml::to_string(&exercise).unwrap();
        assert!(serialized.contains("group: A"));
        assert!(serialized.contains("group_type: superset"));
    }

    #[test]
    fn test_plan_exercise_circuit_group() {
        let yaml = r#"
name: "Burpees"
modality: strength
group: "circuit1"
group_type: circuit
"#;
        let exercise: PlanExercise = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(exercise.group, Some("circuit1".to_string()));
        assert_eq!(exercise.group_type, Some(GroupType::Circuit));
    }
}
