//! History export type definitions

use crate::{DistanceUnit, Modality, WeightUnit};
use serde::{Deserialize, Serialize};

/// Root history export structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WpsHistory {
    pub history_version: u32,
    pub exported_at: String,
    #[serde(default)]
    pub export_source: Option<ExportSource>,
    #[serde(default)]
    pub units: Units,
    pub workouts: Vec<Workout>,
    #[serde(default)]
    pub personal_records: Vec<PersonalRecord>,
    #[serde(default)]
    pub body_measurements: Vec<BodyMeasurement>,
}

/// Information about the exporting application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExportSource {
    #[serde(default)]
    pub app_name: Option<String>,
    #[serde(default)]
    pub app_version: Option<String>,
    #[serde(default)]
    pub platform: Option<String>,
    #[serde(default)]
    pub preferred_units: Option<Units>,
}

/// Default units for the export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Units {
    #[serde(default)]
    pub weight: WeightUnit,
    #[serde(default)]
    pub distance: DistanceUnit,
}

impl Default for Units {
    fn default() -> Self {
        Self {
            weight: WeightUnit::Kg,
            distance: DistanceUnit::Meters,
        }
    }
}

/// A completed workout session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workout {
    #[serde(default)]
    pub id: Option<String>,
    pub date: String,
    #[serde(default)]
    pub started_at: Option<String>,
    #[serde(default)]
    pub ended_at: Option<String>,
    #[serde(default)]
    pub duration_sec: Option<u32>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub plan_id: Option<String>,
    #[serde(default)]
    pub plan_day_id: Option<String>,
    pub exercises: Vec<CompletedExercise>,
}

/// A completed exercise with recorded sets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedExercise {
    #[serde(default)]
    pub id: Option<String>,
    pub name: String,
    #[serde(default)]
    pub modality: Option<Modality>,
    #[serde(default)]
    pub notes: Option<String>,
    pub sets: Vec<CompletedSet>,
}

/// A single completed set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedSet {
    #[serde(default)]
    pub set_number: Option<u32>,
    #[serde(default)]
    pub set_type: Option<SetType>,
    #[serde(default)]
    pub reps: Option<u32>,
    #[serde(default)]
    pub weight_kg: Option<f64>,
    #[serde(default)]
    pub weight_lb: Option<f64>,
    #[serde(default)]
    pub duration_sec: Option<u32>,
    #[serde(default)]
    pub distance_meters: Option<f64>,
    #[serde(default)]
    pub rpe: Option<f64>,
    #[serde(default)]
    pub rir: Option<u32>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub is_pr: Option<bool>,
    #[serde(default)]
    pub completed_at: Option<String>,
}

/// Type of set (working, warmup, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SetType {
    #[default]
    Working,
    Warmup,
    Dropset,
    Failure,
    Amrap,
}

/// A personal record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalRecord {
    pub exercise_name: String,
    pub record_type: RecordType,
    pub value: f64,
    #[serde(default)]
    pub unit: Option<String>,
    pub achieved_at: String,
    #[serde(default)]
    pub workout_id: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}

/// Type of personal record
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordType {
    #[serde(rename = "1rm")]
    OneRepMax,
    #[serde(rename = "max_weight_3rm")]
    MaxWeight3rm,
    #[serde(rename = "max_weight_5rm")]
    MaxWeight5rm,
    #[serde(rename = "max_weight_8rm")]
    MaxWeight8rm,
    #[serde(rename = "max_weight_10rm")]
    MaxWeight10rm,
    MaxWeight,
    MaxReps,
    MaxVolume,
    MaxDuration,
    MaxDistance,
    FastestTime,
}

/// A body measurement entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyMeasurement {
    pub date: String,
    #[serde(default)]
    pub recorded_at: Option<String>,
    #[serde(default)]
    pub weight_kg: Option<f64>,
    #[serde(default)]
    pub weight_lb: Option<f64>,
    #[serde(default)]
    pub body_fat_percent: Option<f64>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub measurements: Option<BodyDimensions>,
}

/// Body dimension measurements
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BodyDimensions {
    #[serde(default)]
    pub neck_cm: Option<f64>,
    #[serde(default)]
    pub shoulders_cm: Option<f64>,
    #[serde(default)]
    pub chest_cm: Option<f64>,
    #[serde(default)]
    pub waist_cm: Option<f64>,
    #[serde(default)]
    pub hips_cm: Option<f64>,
    #[serde(default)]
    pub bicep_left_cm: Option<f64>,
    #[serde(default)]
    pub bicep_right_cm: Option<f64>,
    #[serde(default)]
    pub forearm_left_cm: Option<f64>,
    #[serde(default)]
    pub forearm_right_cm: Option<f64>,
    #[serde(default)]
    pub thigh_left_cm: Option<f64>,
    #[serde(default)]
    pub thigh_right_cm: Option<f64>,
    #[serde(default)]
    pub calf_left_cm: Option<f64>,
    #[serde(default)]
    pub calf_right_cm: Option<f64>,
}

/// Statistics about a history export
#[derive(Debug, Clone, Default, Serialize)]
pub struct HistoryStatistics {
    pub total_workouts: usize,
    pub total_exercises: usize,
    pub total_sets: usize,
    pub total_volume_kg: f64,
    pub date_range_start: Option<String>,
    pub date_range_end: Option<String>,
    pub personal_records_count: usize,
    pub body_measurements_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DistanceUnit, Modality, WeightUnit};

    #[test]
    fn test_wps_history_round_trip() {
        let history = WpsHistory {
            history_version: 1,
            exported_at: "2025-01-15T12:00:00Z".to_string(),
            export_source: Some(ExportSource {
                app_name: Some("TestApp".to_string()),
                app_version: Some("1.0.0".to_string()),
                platform: Some("iOS".to_string()),
                preferred_units: Some(Units {
                    weight: WeightUnit::Lb,
                    distance: DistanceUnit::Miles,
                }),
            }),
            units: Units {
                weight: WeightUnit::Kg,
                distance: DistanceUnit::Meters,
            },
            workouts: vec![],
            personal_records: vec![],
            body_measurements: vec![],
        };

        let json = serde_json::to_string(&history).unwrap();
        let deserialized: WpsHistory = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.history_version, 1);
        assert_eq!(deserialized.exported_at, "2025-01-15T12:00:00Z");
        assert!(deserialized.export_source.is_some());
        assert_eq!(
            deserialized.export_source.as_ref().unwrap().app_name,
            Some("TestApp".to_string())
        );
    }

    #[test]
    fn test_wps_history_minimal() {
        let history = WpsHistory {
            history_version: 1,
            exported_at: "2025-01-15T12:00:00Z".to_string(),
            export_source: None,
            units: Units::default(),
            workouts: vec![],
            personal_records: vec![],
            body_measurements: vec![],
        };

        let json = serde_json::to_string(&history).unwrap();
        let deserialized: WpsHistory = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.history_version, 1);
        assert!(deserialized.export_source.is_none());
        assert_eq!(deserialized.units.weight, WeightUnit::Kg);
        assert_eq!(deserialized.units.distance, DistanceUnit::Meters);
    }

    #[test]
    fn test_export_source_with_preferred_units() {
        let source = ExportSource {
            app_name: Some("MyApp".to_string()),
            app_version: Some("2.0.0".to_string()),
            platform: Some("Android".to_string()),
            preferred_units: Some(Units {
                weight: WeightUnit::Lb,
                distance: DistanceUnit::Miles,
            }),
        };

        let json = serde_json::to_string(&source).unwrap();
        let deserialized: ExportSource = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.app_name, Some("MyApp".to_string()));
        assert_eq!(deserialized.app_version, Some("2.0.0".to_string()));
        assert_eq!(deserialized.platform, Some("Android".to_string()));
        assert!(deserialized.preferred_units.is_some());
        assert_eq!(deserialized.preferred_units.unwrap().weight, WeightUnit::Lb);
    }

    #[test]
    fn test_export_source_minimal() {
        let source = ExportSource {
            app_name: None,
            app_version: None,
            platform: None,
            preferred_units: None,
        };

        let json = serde_json::to_string(&source).unwrap();
        let deserialized: ExportSource = serde_json::from_str(&json).unwrap();

        assert!(deserialized.app_name.is_none());
        assert!(deserialized.app_version.is_none());
        assert!(deserialized.platform.is_none());
        assert!(deserialized.preferred_units.is_none());
    }

    #[test]
    fn test_units_default() {
        let units = Units::default();
        assert_eq!(units.weight, WeightUnit::Kg);
        assert_eq!(units.distance, DistanceUnit::Meters);

        let json = serde_json::to_string(&units).unwrap();
        let deserialized: Units = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.weight, WeightUnit::Kg);
        assert_eq!(deserialized.distance, DistanceUnit::Meters);
    }

    #[test]
    fn test_units_lb_miles() {
        let units = Units {
            weight: WeightUnit::Lb,
            distance: DistanceUnit::Miles,
        };

        let json = serde_json::to_string(&units).unwrap();
        let deserialized: Units = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.weight, WeightUnit::Lb);
        assert_eq!(deserialized.distance, DistanceUnit::Miles);
    }

    #[test]
    fn test_workout_full() {
        let workout = Workout {
            id: Some("workout-123".to_string()),
            date: "2025-01-15".to_string(),
            started_at: Some("2025-01-15T10:00:00Z".to_string()),
            ended_at: Some("2025-01-15T11:30:00Z".to_string()),
            duration_sec: Some(5400),
            title: Some("Leg Day".to_string()),
            notes: Some("Great session!".to_string()),
            plan_id: Some("plan-456".to_string()),
            plan_day_id: Some("day-789".to_string()),
            exercises: vec![],
        };

        let json = serde_json::to_string(&workout).unwrap();
        let deserialized: Workout = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, Some("workout-123".to_string()));
        assert_eq!(deserialized.date, "2025-01-15");
        assert_eq!(deserialized.duration_sec, Some(5400));
        assert_eq!(deserialized.title, Some("Leg Day".to_string()));
    }

    #[test]
    fn test_workout_minimal() {
        let workout = Workout {
            id: None,
            date: "2025-01-15".to_string(),
            started_at: None,
            ended_at: None,
            duration_sec: None,
            title: None,
            notes: None,
            plan_id: None,
            plan_day_id: None,
            exercises: vec![],
        };

        let json = serde_json::to_string(&workout).unwrap();
        let deserialized: Workout = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.date, "2025-01-15");
        assert!(deserialized.id.is_none());
        assert!(deserialized.title.is_none());
    }

    #[test]
    fn test_completed_exercise_with_sets() {
        let exercise = CompletedExercise {
            id: Some("ex-123".to_string()),
            name: "Bench Press".to_string(),
            modality: Some(Modality::Strength),
            notes: Some("Felt strong".to_string()),
            sets: vec![
                CompletedSet {
                    set_number: Some(1),
                    set_type: Some(SetType::Warmup),
                    reps: Some(10),
                    weight_kg: Some(60.0),
                    weight_lb: None,
                    duration_sec: None,
                    distance_meters: None,
                    rpe: None,
                    rir: None,
                    notes: None,
                    is_pr: Some(false),
                    completed_at: None,
                },
                CompletedSet {
                    set_number: Some(2),
                    set_type: Some(SetType::Working),
                    reps: Some(8),
                    weight_kg: Some(100.0),
                    weight_lb: None,
                    duration_sec: None,
                    distance_meters: None,
                    rpe: Some(8.0),
                    rir: Some(2),
                    notes: None,
                    is_pr: Some(true),
                    completed_at: Some("2025-01-15T10:30:00Z".to_string()),
                },
            ],
        };

        let json = serde_json::to_string(&exercise).unwrap();
        let deserialized: CompletedExercise = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "Bench Press");
        assert_eq!(deserialized.modality, Some(Modality::Strength));
        assert_eq!(deserialized.sets.len(), 2);
        assert_eq!(deserialized.sets[0].set_type, Some(SetType::Warmup));
        assert_eq!(deserialized.sets[1].rpe, Some(8.0));
        assert_eq!(deserialized.sets[1].rir, Some(2));
    }

    #[test]
    fn test_completed_set_all_fields() {
        let set = CompletedSet {
            set_number: Some(1),
            set_type: Some(SetType::Working),
            reps: Some(10),
            weight_kg: Some(100.0),
            weight_lb: Some(220.46),
            duration_sec: Some(30),
            distance_meters: Some(500.0),
            rpe: Some(8.5),
            rir: Some(2),
            notes: Some("Felt good".to_string()),
            is_pr: Some(true),
            completed_at: Some("2025-01-15T10:30:00Z".to_string()),
        };

        let json = serde_json::to_string(&set).unwrap();
        let deserialized: CompletedSet = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.set_number, Some(1));
        assert_eq!(deserialized.set_type, Some(SetType::Working));
        assert_eq!(deserialized.reps, Some(10));
        assert_eq!(deserialized.weight_kg, Some(100.0));
        assert_eq!(deserialized.weight_lb, Some(220.46));
        assert_eq!(deserialized.duration_sec, Some(30));
        assert_eq!(deserialized.distance_meters, Some(500.0));
        assert_eq!(deserialized.rpe, Some(8.5));
        assert_eq!(deserialized.rir, Some(2));
        assert_eq!(deserialized.is_pr, Some(true));
    }

    #[test]
    fn test_completed_set_optional_fields_skipped() {
        let set = CompletedSet {
            set_number: None,
            set_type: None,
            reps: Some(10),
            weight_kg: Some(100.0),
            weight_lb: None,
            duration_sec: None,
            distance_meters: None,
            rpe: None,
            rir: None,
            notes: None,
            is_pr: None,
            completed_at: None,
        };

        let json = serde_json::to_string(&set).unwrap();

        // Verify that None fields are serialized as null (serde default behavior)
        assert!(json.contains("\"set_number\":null"));
        assert!(json.contains("\"set_type\":null"));
        assert!(json.contains("\"weight_lb\":null"));
        assert!(json.contains("\"rpe\":null"));
        assert!(json.contains("\"rir\":null"));

        // Verify that present fields are properly serialized
        assert!(json.contains("\"reps\":10"));
        assert!(json.contains("\"weight_kg\":100.0"));

        let deserialized: CompletedSet = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.reps, Some(10));
        assert_eq!(deserialized.weight_kg, Some(100.0));
        assert!(deserialized.weight_lb.is_none());
        assert!(deserialized.rir.is_none());
        assert!(deserialized.rpe.is_none());
    }

    #[test]
    fn test_set_type_all_variants() {
        let variants = vec![
            (SetType::Working, "working"),
            (SetType::Warmup, "warmup"),
            (SetType::Dropset, "dropset"),
            (SetType::Failure, "failure"),
            (SetType::Amrap, "amrap"),
        ];

        for (variant, expected_json) in variants {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, format!("\"{}\"", expected_json));

            let deserialized: SetType = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, variant);
        }
    }

    #[test]
    fn test_set_type_default() {
        let default = SetType::default();
        assert_eq!(default, SetType::Working);
    }

    #[test]
    fn test_personal_record_1rm() {
        let pr = PersonalRecord {
            exercise_name: "Squat".to_string(),
            record_type: RecordType::OneRepMax,
            value: 200.0,
            unit: Some("kg".to_string()),
            achieved_at: "2025-01-15T10:30:00Z".to_string(),
            workout_id: Some("workout-123".to_string()),
            notes: Some("New PR!".to_string()),
        };

        let json = serde_json::to_string(&pr).unwrap();
        assert!(json.contains("\"1rm\""));

        let deserialized: PersonalRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.exercise_name, "Squat");
        assert_eq!(deserialized.record_type, RecordType::OneRepMax);
        assert_eq!(deserialized.value, 200.0);
    }

    #[test]
    fn test_record_type_all_variants() {
        let variants = vec![
            (RecordType::OneRepMax, "1rm"),
            (RecordType::MaxWeight3rm, "max_weight_3rm"),
            (RecordType::MaxWeight5rm, "max_weight_5rm"),
            (RecordType::MaxWeight8rm, "max_weight_8rm"),
            (RecordType::MaxWeight10rm, "max_weight_10rm"),
            (RecordType::MaxWeight, "max_weight"),
            (RecordType::MaxReps, "max_reps"),
            (RecordType::MaxVolume, "max_volume"),
            (RecordType::MaxDuration, "max_duration"),
            (RecordType::MaxDistance, "max_distance"),
            (RecordType::FastestTime, "fastest_time"),
        ];

        for (variant, expected_json) in variants {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, format!("\"{}\"", expected_json));

            let deserialized: RecordType = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, variant);
        }
    }

    #[test]
    fn test_personal_record_all_record_types() {
        let record_types = vec![
            RecordType::OneRepMax,
            RecordType::MaxWeight3rm,
            RecordType::MaxWeight5rm,
            RecordType::MaxWeight8rm,
            RecordType::MaxWeight10rm,
            RecordType::MaxWeight,
            RecordType::MaxReps,
            RecordType::MaxVolume,
            RecordType::MaxDuration,
            RecordType::MaxDistance,
            RecordType::FastestTime,
        ];

        for record_type in record_types {
            let pr = PersonalRecord {
                exercise_name: "Test Exercise".to_string(),
                record_type,
                value: 100.0,
                unit: Some("kg".to_string()),
                achieved_at: "2025-01-15T10:30:00Z".to_string(),
                workout_id: None,
                notes: None,
            };

            let json = serde_json::to_string(&pr).unwrap();
            let deserialized: PersonalRecord = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized.record_type, record_type);
        }
    }

    #[test]
    fn test_body_measurement_full() {
        let measurement = BodyMeasurement {
            date: "2025-01-15".to_string(),
            recorded_at: Some("2025-01-15T08:00:00Z".to_string()),
            weight_kg: Some(80.5),
            weight_lb: Some(177.47),
            body_fat_percent: Some(15.2),
            notes: Some("Morning weigh-in".to_string()),
            measurements: Some(BodyDimensions {
                neck_cm: Some(38.0),
                shoulders_cm: Some(120.0),
                chest_cm: Some(100.0),
                waist_cm: Some(85.0),
                hips_cm: Some(95.0),
                bicep_left_cm: Some(35.0),
                bicep_right_cm: Some(35.5),
                forearm_left_cm: Some(28.0),
                forearm_right_cm: Some(28.5),
                thigh_left_cm: Some(58.0),
                thigh_right_cm: Some(58.5),
                calf_left_cm: Some(38.0),
                calf_right_cm: Some(38.5),
            }),
        };

        let json = serde_json::to_string(&measurement).unwrap();
        let deserialized: BodyMeasurement = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.date, "2025-01-15");
        assert_eq!(deserialized.weight_kg, Some(80.5));
        assert_eq!(deserialized.body_fat_percent, Some(15.2));
        assert!(deserialized.measurements.is_some());

        let dims = deserialized.measurements.unwrap();
        assert_eq!(dims.chest_cm, Some(100.0));
        assert_eq!(dims.waist_cm, Some(85.0));
        assert_eq!(dims.bicep_left_cm, Some(35.0));
    }

    #[test]
    fn test_body_measurement_minimal() {
        let measurement = BodyMeasurement {
            date: "2025-01-15".to_string(),
            recorded_at: None,
            weight_kg: Some(80.0),
            weight_lb: None,
            body_fat_percent: None,
            notes: None,
            measurements: None,
        };

        let json = serde_json::to_string(&measurement).unwrap();
        let deserialized: BodyMeasurement = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.date, "2025-01-15");
        assert_eq!(deserialized.weight_kg, Some(80.0));
        assert!(deserialized.measurements.is_none());
    }

    #[test]
    fn test_body_dimensions_default() {
        let dimensions = BodyDimensions::default();

        assert!(dimensions.neck_cm.is_none());
        assert!(dimensions.chest_cm.is_none());
        assert!(dimensions.waist_cm.is_none());

        let json = serde_json::to_string(&dimensions).unwrap();
        let deserialized: BodyDimensions = serde_json::from_str(&json).unwrap();

        assert!(deserialized.neck_cm.is_none());
    }

    #[test]
    fn test_full_export_round_trip() {
        let history = WpsHistory {
            history_version: 1,
            exported_at: "2025-01-15T12:00:00Z".to_string(),
            export_source: Some(ExportSource {
                app_name: Some("TestApp".to_string()),
                app_version: Some("1.0.0".to_string()),
                platform: Some("iOS".to_string()),
                preferred_units: Some(Units {
                    weight: WeightUnit::Lb,
                    distance: DistanceUnit::Miles,
                }),
            }),
            units: Units::default(),
            workouts: vec![Workout {
                id: Some("w1".to_string()),
                date: "2025-01-15".to_string(),
                started_at: Some("2025-01-15T10:00:00Z".to_string()),
                ended_at: Some("2025-01-15T11:00:00Z".to_string()),
                duration_sec: Some(3600),
                title: Some("Push Day".to_string()),
                notes: None,
                plan_id: None,
                plan_day_id: None,
                exercises: vec![CompletedExercise {
                    id: Some("e1".to_string()),
                    name: "Bench Press".to_string(),
                    modality: Some(Modality::Strength),
                    notes: None,
                    sets: vec![CompletedSet {
                        set_number: Some(1),
                        set_type: Some(SetType::Working),
                        reps: Some(8),
                        weight_kg: Some(100.0),
                        weight_lb: None,
                        duration_sec: None,
                        distance_meters: None,
                        rpe: Some(8.0),
                        rir: Some(2),
                        notes: None,
                        is_pr: Some(true),
                        completed_at: None,
                    }],
                }],
            }],
            personal_records: vec![PersonalRecord {
                exercise_name: "Bench Press".to_string(),
                record_type: RecordType::OneRepMax,
                value: 140.0,
                unit: Some("kg".to_string()),
                achieved_at: "2025-01-15T10:30:00Z".to_string(),
                workout_id: Some("w1".to_string()),
                notes: None,
            }],
            body_measurements: vec![BodyMeasurement {
                date: "2025-01-15".to_string(),
                recorded_at: Some("2025-01-15T08:00:00Z".to_string()),
                weight_kg: Some(80.0),
                weight_lb: None,
                body_fat_percent: Some(15.0),
                notes: None,
                measurements: Some(BodyDimensions {
                    chest_cm: Some(100.0),
                    waist_cm: Some(85.0),
                    ..Default::default()
                }),
            }],
        };

        let json = serde_json::to_string_pretty(&history).unwrap();
        let deserialized: WpsHistory = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.history_version, 1);
        assert_eq!(deserialized.workouts.len(), 1);
        assert_eq!(deserialized.workouts[0].exercises.len(), 1);
        assert_eq!(deserialized.workouts[0].exercises[0].sets.len(), 1);
        assert_eq!(deserialized.personal_records.len(), 1);
        assert_eq!(deserialized.body_measurements.len(), 1);
        assert_eq!(
            deserialized.personal_records[0].record_type,
            RecordType::OneRepMax
        );
        assert_eq!(deserialized.workouts[0].exercises[0].sets[0].rir, Some(2));
    }
}
