//! Validation rules for PWF history exports

use super::error_codes;
use super::parser::parse;
use super::types::{HistoryStatistics, RecordType, WpsHistory};
use crate::error::ValidationIssue;
use crate::types::WeightUnit;

/// Result of history validation
#[derive(Debug, Clone, serde::Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<WpsHistory>,
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<HistoryStatistics>,
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

/// Validate a YAML string as a PWF history export
pub fn validate(yaml: &str) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Parse YAML
    let history = match parse(yaml) {
        Ok(h) => h,
        Err(e) => {
            errors.push(ValidationIssue::error("", e.to_string()));
            return ValidationResult {
                valid: false,
                history: None,
                errors,
                warnings,
                statistics: None,
            };
        }
    };

    // Validate history_version (support v1 and v2)
    if history.history_version != 1 && history.history_version != 2 {
        errors.push(ValidationIssue::error_with_code(
            "history_version",
            format!(
                "Unsupported history_version: {}. Supported versions: 1, 2",
                history.history_version
            ),
            error_codes::INVALID_VERSION,
        ));
    }

    // Validate exported_at
    if history.exported_at.is_empty() {
        errors.push(ValidationIssue::error_with_code(
            "exported_at",
            "exported_at timestamp is required",
            error_codes::MISSING_EXPORTED_AT,
        ));
    }

    // Validate workouts
    for (workout_idx, workout) in history.workouts.iter().enumerate() {
        let workout_path = format!("workouts[{}]", workout_idx);

        // Validate date
        if workout.date.is_empty() {
            errors.push(ValidationIssue::error_with_code(
                format!("{}.date", workout_path),
                "Workout date is required",
                error_codes::MISSING_WORKOUT_DATE,
            ));
        }

        // Validate exercises
        if workout.exercises.is_empty() {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.exercises", workout_path),
                "Workout has no exercises",
                error_codes::NO_EXERCISES,
            ));
        }

        for (ex_idx, exercise) in workout.exercises.iter().enumerate() {
            let ex_path = format!("{}.exercises[{}]", workout_path, ex_idx);

            // Validate exercise name
            if exercise.name.is_empty() {
                errors.push(ValidationIssue::error_with_code(
                    format!("{}.name", ex_path),
                    "Exercise name is required",
                    error_codes::MISSING_EXERCISE_NAME,
                ));
            }

            // Validate sets
            if exercise.sets.is_empty() {
                warnings.push(ValidationIssue::warning_with_code(
                    format!("{}.sets", ex_path),
                    "Exercise has no recorded sets",
                    error_codes::NO_SETS,
                ));
            }

            for (set_idx, set) in exercise.sets.iter().enumerate() {
                let set_path = format!("{}.sets[{}]", ex_path, set_idx);

                // Check for at least one metric
                let has_metric = set.reps.is_some()
                    || set.weight_kg.is_some()
                    || set.weight_lb.is_some()
                    || set.duration_sec.is_some()
                    || set.distance_meters.is_some();

                if !has_metric {
                    warnings.push(ValidationIssue::warning_with_code(
                        &set_path,
                        "Set has no recorded metrics (reps, weight, duration, or distance)",
                        error_codes::NO_METRICS,
                    ));
                }

                // Validate RPE range
                if let Some(rpe) = set.rpe {
                    if !(0.0..=10.0).contains(&rpe) {
                        warnings.push(ValidationIssue::warning_with_code(
                            format!("{}.rpe", set_path),
                            format!("RPE should be between 0 and 10, got {}", rpe),
                            error_codes::RPE_OUT_OF_RANGE,
                        ));
                    }
                }

                // Validate RIR range
                if let Some(rir) = set.rir {
                    if rir > 10 {
                        warnings.push(ValidationIssue::warning_with_code(
                            format!("{}.rir", set_path),
                            format!("RIR typically ranges 0-10, got {}", rir),
                            error_codes::RIR_OUT_OF_RANGE,
                        ));
                    }
                }

                // Warn if both RPE and RIR are set
                if set.rpe.is_some() && set.rir.is_some() {
                    warnings.push(ValidationIssue::warning_with_code(
                        &set_path,
                        "Both RPE and RIR are set. Typically only one should be used.",
                        error_codes::RPE_RIR_BOTH_SET,
                    ));
                }

                // Validate telemetry (v2 feature)
                if let Some(telemetry) = &set.telemetry {
                    validate_set_telemetry(&set_path, telemetry, &mut warnings);
                }

                // PWF v2.1: Validate swimming data
                if let Some(swimming) = &set.swimming {
                    validate_swimming_set(&set_path, swimming, &mut errors, &mut warnings);
                }
            }

            // PWF v2.1: Validate pool configuration
            if let Some(pool_config) = &exercise.pool_config {
                validate_pool_config(&ex_path, pool_config, &mut errors);
            }
        }

        // Validate workout-level telemetry (v2 feature)
        if let Some(telemetry) = &workout.telemetry {
            validate_workout_telemetry(&workout_path, telemetry, &mut warnings);
        }

        // PWF v2.1: Validate sport segments (multi-sport workouts)
        if let Some(segments) = &workout.sport_segments {
            validate_sport_segments(&workout_path, segments, &mut errors, &mut warnings);
        }
    }

    // Validate personal records
    for (pr_idx, pr) in history.personal_records.iter().enumerate() {
        let pr_path = format!("personal_records[{}]", pr_idx);

        if pr.exercise_name.is_empty() {
            errors.push(ValidationIssue::error_with_code(
                format!("{}.exercise_name", pr_path),
                "Personal record must have exercise_name",
                error_codes::MISSING_PR_EXERCISE,
            ));
        }

        if pr.achieved_at.is_empty() {
            errors.push(ValidationIssue::error_with_code(
                format!("{}.achieved_at", pr_path),
                "Personal record must have achieved_at date",
                error_codes::MISSING_PR_DATE,
            ));
        }

        // Validate rep-specific PRs have units
        match pr.record_type {
            RecordType::OneRepMax
            | RecordType::MaxWeight3rm
            | RecordType::MaxWeight5rm
            | RecordType::MaxWeight8rm
            | RecordType::MaxWeight10rm
            | RecordType::MaxWeight => {
                if pr.unit.is_none() {
                    warnings.push(ValidationIssue::warning_with_code(
                        format!("{}.unit", pr_path),
                        "Weight-based personal records should specify a unit (kg or lb)",
                        error_codes::PR_MISSING_UNIT,
                    ));
                }
            }
            RecordType::MaxDistance | RecordType::FastestTime => {
                if pr.unit.is_none() {
                    warnings.push(ValidationIssue::warning_with_code(
                        format!("{}.unit", pr_path),
                        "Distance/time personal records should specify appropriate units",
                        error_codes::PR_MISSING_UNIT,
                    ));
                }
            }
            _ => {}
        }
    }

    // Validate body measurements
    for (bm_idx, bm) in history.body_measurements.iter().enumerate() {
        let bm_path = format!("body_measurements[{}]", bm_idx);

        if bm.date.is_empty() {
            errors.push(ValidationIssue::error_with_code(
                format!("{}.date", bm_path),
                "Body measurement must have date",
                error_codes::MISSING_BM_DATE,
            ));
        }

        // Check for at least one measurement
        let has_measurement = bm.weight_kg.is_some()
            || bm.weight_lb.is_some()
            || bm.body_fat_percent.is_some()
            || bm.measurements.is_some();

        if !has_measurement {
            warnings.push(ValidationIssue::warning_with_code(
                &bm_path,
                "Body measurement entry has no recorded values",
                error_codes::NO_BM_VALUES,
            ));
        }
    }

    // Validate preferred_units consistency
    if let Some(export_source) = &history.export_source {
        if let Some(preferred) = &export_source.preferred_units {
            // Check if preferred units match actual usage in workouts
            let mut uses_kg = false;
            let mut uses_lb = false;

            for workout in &history.workouts {
                for exercise in &workout.exercises {
                    for set in &exercise.sets {
                        if set.weight_kg.is_some() {
                            uses_kg = true;
                        }
                        if set.weight_lb.is_some() {
                            uses_lb = true;
                        }
                    }
                }
            }

            // Warn if preferred unit doesn't match actual data
            if uses_kg && !uses_lb {
                // Only kg data present
                if preferred.weight == WeightUnit::Lb {
                    warnings.push(ValidationIssue::warning_with_code(
                        "export_source.preferred_units.weight",
                        "Preferred weight unit is 'lb', but only kg values are present in workouts",
                        error_codes::PREFERRED_UNITS_MISMATCH,
                    ));
                }
            } else if uses_lb && !uses_kg {
                // Only lb data present
                if preferred.weight == WeightUnit::Kg {
                    warnings.push(ValidationIssue::warning_with_code(
                        "export_source.preferred_units.weight",
                        "Preferred weight unit is 'kg', but only lb values are present in workouts",
                        error_codes::PREFERRED_UNITS_MISMATCH,
                    ));
                }
            }
        }
    }

    // Calculate statistics
    let statistics = if errors.is_empty() {
        Some(calculate_statistics(&history))
    } else {
        None
    };

    let valid = errors.is_empty();

    ValidationResult {
        valid,
        history: if valid { Some(history) } else { None },
        errors,
        warnings,
        statistics,
    }
}

fn calculate_statistics(history: &WpsHistory) -> HistoryStatistics {
    let mut stats = HistoryStatistics {
        total_workouts: history.workouts.len(),
        personal_records_count: history.personal_records.len(),
        body_measurements_count: history.body_measurements.len(),
        ..Default::default()
    };

    let mut dates: Vec<&str> = Vec::new();

    for workout in &history.workouts {
        dates.push(&workout.date);
        stats.total_exercises += workout.exercises.len();

        for exercise in &workout.exercises {
            stats.total_sets += exercise.sets.len();

            for set in &exercise.sets {
                if let (Some(reps), Some(weight)) = (set.reps, set.weight_kg) {
                    stats.total_volume_kg += reps as f64 * weight;
                }
            }
        }
    }

    // Get date range
    dates.sort();
    if let Some(first) = dates.first() {
        stats.date_range_start = Some(first.to_string());
    }
    if let Some(last) = dates.last() {
        stats.date_range_end = Some(last.to_string());
    }

    stats
}

fn validate_set_telemetry(
    path: &str,
    telemetry: &super::types::SetTelemetry,
    warnings: &mut Vec<ValidationIssue>,
) {
    // Validate heart rate
    if let Some(hr) = telemetry.heart_rate_avg {
        if hr > 250 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.heart_rate_avg", path),
                format!("Heart rate {} seems unusually high", hr),
                error_codes::HEART_RATE_OUT_OF_RANGE,
            ));
        }
    }
    if let Some(hr) = telemetry.heart_rate_max {
        if hr > 250 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.heart_rate_max", path),
                format!("Max heart rate {} seems unusually high", hr),
                error_codes::HEART_RATE_OUT_OF_RANGE,
            ));
        }
    }
    if let Some(hr) = telemetry.heart_rate_min {
        if hr < 30 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.heart_rate_min", path),
                format!("Min heart rate {} seems unusually low", hr),
                error_codes::HEART_RATE_OUT_OF_RANGE,
            ));
        }
    }

    // Validate power (watts)
    if let Some(power) = telemetry.power_avg {
        if power > 2000 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.power_avg", path),
                format!("Average power {} watts seems unusually high", power),
                error_codes::POWER_NEGATIVE,
            ));
        }
    }

    // Validate elevation
    if let Some(elev) = telemetry.elevation_gain_m {
        if elev < 0.0 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.elevation_gain_m", path),
                "Elevation gain cannot be negative",
                error_codes::ELEVATION_NEGATIVE,
            ));
        }
    }
    if let Some(elev) = telemetry.elevation_gain_ft {
        if elev < 0.0 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.elevation_gain_ft", path),
                "Elevation gain cannot be negative",
                error_codes::ELEVATION_NEGATIVE,
            ));
        }
    }

    // Validate speed
    if let Some(speed) = telemetry.speed_avg_mps {
        if speed < 0.0 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.speed_avg_mps", path),
                "Speed cannot be negative",
                error_codes::SPEED_NEGATIVE,
            ));
        }
    }

    // Validate cadence
    if let Some(cadence) = telemetry.cadence_avg {
        if cadence > 300 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.cadence_avg", path),
                format!("Cadence {} seems unusually high", cadence),
                error_codes::CADENCE_OUT_OF_RANGE,
            ));
        }
    }

    // Validate humidity
    if let Some(humidity) = telemetry.humidity_percent {
        if !(0.0..=100.0).contains(&humidity) {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.humidity_percent", path),
                format!("Humidity must be between 0 and 100, got {}", humidity),
                error_codes::HUMIDITY_OUT_OF_RANGE,
            ));
        }
    }

    // Validate pace
    if let Some(pace) = telemetry.pace_avg_sec_per_km {
        if pace == 0 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.pace_avg_sec_per_km", path),
                "Pace cannot be zero",
                error_codes::PACE_NEGATIVE,
            ));
        }
    }

    // Warn if conflicting unit systems are used
    let has_metric_elevation =
        telemetry.elevation_gain_m.is_some() || telemetry.elevation_loss_m.is_some();
    let has_imperial_elevation =
        telemetry.elevation_gain_ft.is_some() || telemetry.elevation_loss_ft.is_some();

    if has_metric_elevation && has_imperial_elevation {
        warnings.push(ValidationIssue::warning_with_code(
            format!("{}.telemetry", path),
            "Both metric and imperial elevation units provided. Consider using only one unit system.",
            error_codes::TELEMETRY_UNIT_MISMATCH,
        ));
    }

    let has_metric_speed = telemetry.speed_avg_kph.is_some() || telemetry.speed_max_kph.is_some();
    let has_imperial_speed = telemetry.speed_avg_mph.is_some() || telemetry.speed_max_mph.is_some();

    if has_metric_speed && has_imperial_speed {
        warnings.push(ValidationIssue::warning_with_code(
            format!("{}.telemetry", path),
            "Both metric (km/h) and imperial (mph) speed units provided. Consider using only one unit system.",
            error_codes::TELEMETRY_UNIT_MISMATCH,
        ));
    }

    // PWF v2.1: Validate time-series data if present
    if let Some(time_series) = &telemetry.time_series {
        let mut errors = Vec::new();
        validate_time_series(
            &format!("{}.telemetry.time_series", path),
            time_series,
            &mut errors,
        );
        // Convert errors to warnings for telemetry (non-critical)
        for error in errors {
            warnings.push(ValidationIssue::warning_with_code(
                error.path,
                error.message,
                error.code.as_deref().unwrap_or(""),
            ));
        }
    }
}

fn validate_workout_telemetry(
    path: &str,
    telemetry: &super::types::WorkoutTelemetry,
    warnings: &mut Vec<ValidationIssue>,
) {
    // Validate heart rate
    if let Some(hr) = telemetry.heart_rate_avg {
        if hr > 250 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.heart_rate_avg", path),
                format!("Average heart rate {} seems unusually high", hr),
                error_codes::HEART_RATE_OUT_OF_RANGE,
            ));
        }
    }

    // Validate power
    if let Some(power) = telemetry.power_avg {
        if power > 2000 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.power_avg", path),
                format!("Average power {} watts seems unusually high", power),
                error_codes::POWER_NEGATIVE,
            ));
        }
    }

    // Validate total distance
    if let Some(dist) = telemetry.total_distance_m {
        if dist < 0.0 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.total_distance_m", path),
                "Total distance cannot be negative",
                error_codes::SPEED_NEGATIVE,
            ));
        }
    }

    // Validate elevation
    if let Some(elev) = telemetry.total_elevation_gain_m {
        if elev < 0.0 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.total_elevation_gain_m", path),
                "Total elevation gain cannot be negative",
                error_codes::ELEVATION_NEGATIVE,
            ));
        }
    }

    // Validate humidity
    if let Some(humidity) = telemetry.humidity_percent {
        if !(0.0..=100.0).contains(&humidity) {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.telemetry.humidity_percent", path),
                format!("Humidity must be between 0 and 100, got {}", humidity),
                error_codes::HUMIDITY_OUT_OF_RANGE,
            ));
        }
    }

    // Warn about conflicting units
    let has_metric_distance =
        telemetry.total_distance_m.is_some() || telemetry.total_distance_km.is_some();
    let has_imperial_distance = telemetry.total_distance_mi.is_some();

    if has_metric_distance && has_imperial_distance {
        warnings.push(ValidationIssue::warning_with_code(
            format!("{}.telemetry", path),
            "Both metric and imperial distance units provided. Consider using only one unit system.",
            error_codes::TELEMETRY_UNIT_MISMATCH,
        ));
    }

    // PWF v2.1: Validate GPS route
    if let Some(gps_route) = &telemetry.gps_route {
        validate_gps_route(
            &format!("{}.telemetry.gps_route", path),
            gps_route,
            warnings,
        );
    }

    // PWF v2.1: Validate advanced metrics
    if let Some(advanced) = &telemetry.advanced_metrics {
        validate_advanced_metrics(
            &format!("{}.telemetry.advanced_metrics", path),
            advanced,
            warnings,
        );
    }

    // PWF v2.1: Validate power metrics
    if let Some(power_metrics) = &telemetry.power_metrics {
        validate_power_metrics(
            &format!("{}.telemetry.power_metrics", path),
            power_metrics,
            telemetry.power_avg,
            warnings,
        );
    }

    // PWF v2.1: Validate time in zones
    if let Some(zones) = &telemetry.time_in_zones {
        validate_time_in_zones(
            &format!("{}.telemetry.time_in_zones", path),
            zones,
            warnings,
        );
    }
}

// ============================================================================
// PWF v2.1 Validation Functions
// ============================================================================

/// Validate swimming set data
fn validate_swimming_set(
    path: &str,
    swimming: &super::types::SwimmingSetData,
    errors: &mut Vec<ValidationIssue>,
    warnings: &mut Vec<ValidationIssue>,
) {
    // Validate SWOLF for each length
    for (idx, length) in swimming.lengths.iter().enumerate() {
        if !length.validate_swolf() {
            errors.push(ValidationIssue::error_with_code(
                format!("{}.swimming.lengths[{}].swolf", path, idx),
                format!(
                    "SWOLF mismatch: recorded {} but should be {} (duration {} + stroke_count {})",
                    length.swolf.unwrap_or(0),
                    length.calculate_swolf().unwrap_or(0),
                    length.duration_sec,
                    length.stroke_count.unwrap_or(0)
                ),
                error_codes::SWOLF_MISMATCH,
            ));
        }
    }

    // Warn if calculated average SWOLF doesn't match recorded
    if let Some(recorded_avg) = swimming.swolf_avg {
        if let Some(calculated_avg) = swimming.calculate_avg_swolf() {
            if recorded_avg != calculated_avg {
                warnings.push(ValidationIssue::warning_with_code(
                    format!("{}.swimming.swolf_avg", path),
                    format!(
                        "Average SWOLF mismatch: recorded {} but calculated {} from lengths",
                        recorded_avg, calculated_avg
                    ),
                    error_codes::SWOLF_MISMATCH,
                ));
            }
        }
    }

    // Warn if total_lengths doesn't match actual length count
    if let Some(total) = swimming.total_lengths {
        if total != swimming.lengths.len() as u32 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.swimming.total_lengths", path),
                format!(
                    "total_lengths ({}) doesn't match actual length count ({})",
                    total,
                    swimming.lengths.len()
                ),
                error_codes::SWOLF_MISMATCH,
            ));
        }
    }

    // Warn if active_lengths doesn't match calculated count
    if let Some(active) = swimming.active_lengths {
        let calculated = swimming.count_active_lengths();
        if active != calculated {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.swimming.active_lengths", path),
                format!(
                    "active_lengths ({}) doesn't match calculated count ({})",
                    active, calculated
                ),
                error_codes::SWOLF_MISMATCH,
            ));
        }
    }
}

/// Validate pool configuration
fn validate_pool_config(
    path: &str,
    pool_config: &super::types::PoolConfig,
    errors: &mut Vec<ValidationIssue>,
) {
    if pool_config.pool_length <= 0.0 {
        errors.push(ValidationIssue::error_with_code(
            format!("{}.pool_config.pool_length", path),
            format!(
                "Pool length must be greater than 0, got {}",
                pool_config.pool_length
            ),
            error_codes::POOL_LENGTH_INVALID,
        ));
    }
}

/// Validate time-series telemetry data
fn validate_time_series(
    path: &str,
    time_series: &super::types::TimeSeriesData,
    errors: &mut Vec<ValidationIssue>,
) {
    if let Err(msg) = time_series.validate_lengths() {
        errors.push(ValidationIssue::error_with_code(
            path.to_string(),
            msg,
            error_codes::TIME_SERIES_LENGTH_MISMATCH,
        ));
    }

    // Validate GPS coordinates if present
    if let Some(latitudes) = &time_series.latitude {
        for (idx, &lat) in latitudes.iter().enumerate() {
            if !(-90.0..=90.0).contains(&lat) {
                errors.push(ValidationIssue::error_with_code(
                    format!("{}.latitude[{}]", path, idx),
                    format!("Latitude must be between -90 and 90 degrees, got {}", lat),
                    error_codes::GPS_LATITUDE_OUT_OF_RANGE,
                ));
            }
        }
    }

    if let Some(longitudes) = &time_series.longitude {
        for (idx, &lng) in longitudes.iter().enumerate() {
            if !(-180.0..=180.0).contains(&lng) {
                errors.push(ValidationIssue::error_with_code(
                    format!("{}.longitude[{}]", path, idx),
                    format!(
                        "Longitude must be between -180 and 180 degrees, got {}",
                        lng
                    ),
                    error_codes::GPS_LONGITUDE_OUT_OF_RANGE,
                ));
            }
        }
    }
}

/// Validate sport segments (multi-sport workouts)
fn validate_sport_segments(
    path: &str,
    segments: &[super::types::SportSegment],
    errors: &mut Vec<ValidationIssue>,
    warnings: &mut Vec<ValidationIssue>,
) {
    // Check for duplicate indices first
    let mut seen_indices = std::collections::HashSet::new();
    for segment in segments {
        if !seen_indices.insert(segment.segment_index) {
            errors.push(ValidationIssue::error_with_code(
                format!("{}.sport_segments", path),
                format!("Duplicate segment_index: {}", segment.segment_index),
                error_codes::SEGMENT_INDEX_DUPLICATE,
            ));
        }
    }

    // Then check for sequential indices (0, 1, 2, ...)
    let mut indices: Vec<u32> = segments.iter().map(|s| s.segment_index).collect();
    indices.sort_unstable();
    indices.dedup(); // Remove duplicates for this check

    for (expected, &actual) in indices.iter().enumerate() {
        if actual != expected as u32 {
            errors.push(ValidationIssue::error_with_code(
                format!("{}.sport_segments", path),
                format!(
                    "Segment indices must be sequential starting from 0. Expected {}, found {}",
                    expected, actual
                ),
                error_codes::SEGMENT_INDEX_GAP,
            ));
            break;
        }
    }

    // Validate transitions match adjacent segments
    for (idx, segment) in segments.iter().enumerate() {
        if let Some(transition) = &segment.transition {
            // Check from_sport matches current segment
            if transition.from_sport != segment.sport {
                errors.push(ValidationIssue::error_with_code(
                    format!("{}.sport_segments[{}].transition", path, idx),
                    format!(
                        "Transition from_sport ({:?}) doesn't match segment sport ({:?})",
                        transition.from_sport, segment.sport
                    ),
                    error_codes::TRANSITION_SPORT_MISMATCH,
                ));
            }

            // Check to_sport matches next segment (if exists)
            if let Some(next_segment) = segments.get(idx + 1) {
                if transition.to_sport != next_segment.sport {
                    errors.push(ValidationIssue::error_with_code(
                        format!("{}.sport_segments[{}].transition", path, idx),
                        format!(
                            "Transition to_sport ({:?}) doesn't match next segment sport ({:?})",
                            transition.to_sport, next_segment.sport
                        ),
                        error_codes::TRANSITION_SPORT_MISMATCH,
                    ));
                }
            }
        }

        // Validate segment telemetry if present
        if let Some(telemetry) = &segment.telemetry {
            validate_workout_telemetry(
                &format!("{}.sport_segments[{}]", path, idx),
                telemetry,
                warnings,
            );
        }
    }
}

/// Validate GPS route
fn validate_gps_route(
    path: &str,
    gps_route: &super::types::GpsRoute,
    warnings: &mut Vec<ValidationIssue>,
) {
    for (idx, position) in gps_route.positions.iter().enumerate() {
        let pos_path = format!("{}.positions[{}]", path, idx);

        // Validate latitude
        if !(-90.0..=90.0).contains(&position.latitude_deg) {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.latitude_deg", pos_path),
                format!(
                    "Latitude must be between -90 and 90 degrees, got {}",
                    position.latitude_deg
                ),
                error_codes::GPS_LATITUDE_OUT_OF_RANGE,
            ));
        }

        // Validate longitude
        if !(-180.0..=180.0).contains(&position.longitude_deg) {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.longitude_deg", pos_path),
                format!(
                    "Longitude must be between -180 and 180 degrees, got {}",
                    position.longitude_deg
                ),
                error_codes::GPS_LONGITUDE_OUT_OF_RANGE,
            ));
        }

        // Validate heading
        if let Some(heading) = position.heading_deg {
            if !(0.0..=360.0).contains(&heading) {
                warnings.push(ValidationIssue::warning_with_code(
                    format!("{}.heading_deg", pos_path),
                    format!("Heading must be between 0 and 360 degrees, got {}", heading),
                    error_codes::GPS_HEADING_OUT_OF_RANGE,
                ));
            }
        }
    }
}

/// Validate advanced physiological metrics
fn validate_advanced_metrics(
    path: &str,
    advanced: &super::types::AdvancedMetrics,
    warnings: &mut Vec<ValidationIssue>,
) {
    // Validate training effect (0.0 - 5.0)
    if let Some(te) = advanced.training_effect {
        if !(0.0..=5.0).contains(&te) {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.training_effect", path),
                format!("Training Effect should be between 0.0 and 5.0, got {}", te),
                error_codes::TRAINING_EFFECT_OUT_OF_RANGE,
            ));
        }
    }

    // Validate anaerobic training effect (0.0 - 5.0)
    if let Some(ate) = advanced.anaerobic_training_effect {
        if !(0.0..=5.0).contains(&ate) {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.anaerobic_training_effect", path),
                format!(
                    "Anaerobic Training Effect should be between 0.0 and 5.0, got {}",
                    ate
                ),
                error_codes::TRAINING_EFFECT_OUT_OF_RANGE,
            ));
        }
    }

    // Validate performance condition (-20 to +20)
    if let Some(pc) = advanced.performance_condition {
        if !(-20..=20).contains(&pc) {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.performance_condition", path),
                format!(
                    "Performance Condition should be between -20 and +20, got {}",
                    pc
                ),
                error_codes::PERFORMANCE_CONDITION_OUT_OF_RANGE,
            ));
        }
    }
}

/// Validate power-based cycling metrics
fn validate_power_metrics(
    path: &str,
    power_metrics: &super::types::PowerMetrics,
    avg_power: Option<u32>,
    warnings: &mut Vec<ValidationIssue>,
) {
    // Validate Intensity Factor = NP / FTP
    if let (Some(if_val), Some(np), Some(ftp)) = (
        power_metrics.intensity_factor,
        power_metrics.normalized_power,
        power_metrics.ftp_watts,
    ) {
        if ftp > 0 {
            let calculated_if = np as f64 / ftp as f64;
            let diff = (if_val - calculated_if).abs();
            if diff > 0.01 {
                // Allow small floating point errors
                warnings.push(ValidationIssue::warning_with_code(
                    format!("{}.intensity_factor", path),
                    format!(
                        "Intensity Factor mismatch: recorded {} but should be {:.3} (NP {} / FTP {})",
                        if_val, calculated_if, np, ftp
                    ),
                    error_codes::INTENSITY_FACTOR_MISMATCH,
                ));
            }
        }
    }

    // Validate Variability Index = NP / avg_power
    if let (Some(vi), Some(np), Some(avg)) = (
        power_metrics.variability_index,
        power_metrics.normalized_power,
        avg_power,
    ) {
        if avg > 0 {
            let calculated_vi = np as f64 / avg as f64;
            let diff = (vi - calculated_vi).abs();
            if diff > 0.01 {
                // Allow small floating point errors
                warnings.push(ValidationIssue::warning_with_code(
                    format!("{}.variability_index", path),
                    format!(
                        "Variability Index mismatch: recorded {} but should be {:.3} (NP {} / avg_power {})",
                        vi, calculated_vi, np, avg
                    ),
                    error_codes::VARIABILITY_INDEX_MISMATCH,
                ));
            }
        }
    }
}

/// Validate time in zones
fn validate_time_in_zones(
    path: &str,
    zones: &super::types::TimeInZones,
    warnings: &mut Vec<ValidationIssue>,
) {
    // Validate HR zones array matches boundaries
    if let (Some(hr_zones), Some(hr_boundaries)) = (&zones.hr_zones_sec, &zones.hr_zone_boundaries)
    {
        // Boundaries define N zones, so we expect N+1 boundary values (including min/max)
        // But typically stored as N-1 boundaries (between zones)
        // We'll just check that if both are present, they're reasonably related
        if hr_zones.len() > hr_boundaries.len() + 2 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.hr_zones_sec", path),
                format!(
                    "HR zones array length ({}) doesn't match boundaries length ({}) + 1",
                    hr_zones.len(),
                    hr_boundaries.len()
                ),
                error_codes::ZONE_ARRAY_LENGTH_MISMATCH,
            ));
        }
    }

    // Validate power zones array matches boundaries
    if let (Some(power_zones), Some(power_boundaries)) =
        (&zones.power_zones_sec, &zones.power_zone_boundaries)
    {
        if power_zones.len() > power_boundaries.len() + 2 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.power_zones_sec", path),
                format!(
                    "Power zones array length ({}) doesn't match boundaries length ({}) + 1",
                    power_zones.len(),
                    power_boundaries.len()
                ),
                error_codes::ZONE_ARRAY_LENGTH_MISMATCH,
            ));
        }
    }

    // Validate pace zones array matches boundaries
    if let (Some(pace_zones), Some(pace_boundaries)) =
        (&zones.pace_zones_sec, &zones.pace_zone_boundaries)
    {
        if pace_zones.len() > pace_boundaries.len() + 2 {
            warnings.push(ValidationIssue::warning_with_code(
                format!("{}.pace_zones_sec", path),
                format!(
                    "Pace zones array length ({}) doesn't match boundaries length ({}) + 1",
                    pace_zones.len(),
                    pace_boundaries.len()
                ),
                error_codes::ZONE_ARRAY_LENGTH_MISMATCH,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_minimal_history() {
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
        let result = validate(yaml);
        assert!(result.is_valid());
    }

    #[test]
    fn validate_missing_date() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - exercises:
      - name: Squat
        sets:
          - reps: 5
"#;
        let result = validate(yaml);
        assert!(!result.is_valid());
        assert!(result.errors.iter().any(|e| e.message.contains("date")));
    }

    #[test]
    fn validate_statistics() {
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
          - reps: 5
            weight_kg: 100
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        let stats = result.statistics.unwrap();
        assert_eq!(stats.total_workouts, 1);
        assert_eq!(stats.total_sets, 2);
        assert_eq!(stats.total_volume_kg, 1000.0);
    }

    #[test]
    fn validate_rir_out_of_range() {
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
            rir: 15
"#;
        let result = validate(yaml);
        assert!(result.is_valid()); // Still valid, just a warning
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::RIR_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_rir_valid_range() {
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
            rir: 2
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::RIR_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_rpe_rir_both_set() {
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
            rpe: 8.0
            rir: 2
"#;
        let result = validate(yaml);
        assert!(result.is_valid()); // Still valid, just a warning
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::RPE_RIR_BOTH_SET.to_string())));
    }

    #[test]
    fn validate_pr_missing_unit_one_rep_max() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "Bench Press"
    record_type: 1rm
    value: 225.0
    achieved_at: "2025-01-10"
"#;
        let result = validate(yaml);
        assert!(result.is_valid()); // Still valid, just a warning
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    #[test]
    fn validate_pr_with_unit_three_rm() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "Squat"
    record_type: max_weight_3rm
    value: 315
    unit: lb
    achieved_at: "2025-01-12"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    #[test]
    fn validate_preferred_units_mismatch_kg_data_lb_preferred() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
export_source:
  app_name: "Test App"
  preferred_units:
    weight: lb
workouts:
  - date: "2025-01-15"
    exercises:
      - name: Squat
        sets:
          - reps: 5
            weight_kg: 100
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PREFERRED_UNITS_MISMATCH.to_string())));
    }

    #[test]
    fn validate_preferred_units_mismatch_lb_data_kg_preferred() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
export_source:
  app_name: "Test App"
  preferred_units:
    weight: kg
workouts:
  - date: "2025-01-15"
    exercises:
      - name: Bench Press
        sets:
          - reps: 5
            weight_lb: 225
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PREFERRED_UNITS_MISMATCH.to_string())));
    }

    #[test]
    fn validate_preferred_units_match() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
export_source:
  app_name: "Test App"
  preferred_units:
    weight: kg
workouts:
  - date: "2025-01-15"
    exercises:
      - name: Squat
        sets:
          - reps: 5
            weight_kg: 100
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PREFERRED_UNITS_MISMATCH.to_string())));
    }

    // RecordType Tests - Testing all variants
    #[test]
    fn validate_pr_max_weight_missing_unit() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "Deadlift"
    record_type: max_weight
    value: 500.0
    achieved_at: "2025-01-10"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    #[test]
    fn validate_pr_max_reps_no_unit_warning() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "Pull-ups"
    record_type: max_reps
    value: 25.0
    achieved_at: "2025-01-10"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        // max_reps should NOT warn about missing unit
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    #[test]
    fn validate_pr_max_volume_missing_unit() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "Squat"
    record_type: max_volume
    value: 5000.0
    achieved_at: "2025-01-10"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        // max_volume should NOT warn about missing unit (currently not implemented)
        // This test documents current behavior
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    #[test]
    fn validate_pr_max_duration_missing_unit() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "Plank"
    record_type: max_duration
    value: 300.0
    achieved_at: "2025-01-10"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        // max_duration should NOT warn about missing unit (currently not implemented)
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    #[test]
    fn validate_pr_max_distance_missing_unit() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "Run"
    record_type: max_distance
    value: 10000.0
    achieved_at: "2025-01-10"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    #[test]
    fn validate_pr_fastest_time_missing_unit() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "5K Run"
    record_type: fastest_time
    value: 1200.0
    achieved_at: "2025-01-10"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    #[test]
    fn validate_pr_max_weight_8rm_missing_unit() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "Bench Press"
    record_type: max_weight_8rm
    value: 185.0
    achieved_at: "2025-01-10"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    #[test]
    fn validate_pr_max_weight_10rm_with_unit() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
personal_records:
  - exercise_name: "Overhead Press"
    record_type: max_weight_10rm
    value: 135.0
    unit: lb
    achieved_at: "2025-01-10"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PR_MISSING_UNIT.to_string())));
    }

    // Body Measurement Tests
    #[test]
    fn validate_body_measurement_missing_date() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
body_measurements:
  - date: ""
    weight_kg: 75.5
"#;
        let result = validate(yaml);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| e.code == Some(error_codes::MISSING_BM_DATE.to_string())));
    }

    #[test]
    fn validate_body_measurement_no_values() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
body_measurements:
  - date: "2025-01-15"
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::NO_BM_VALUES.to_string())));
    }

    #[test]
    fn validate_body_measurement_valid() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
body_measurements:
  - date: "2025-01-15"
    weight_kg: 75.5
    body_fat_percent: 15.2
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::NO_BM_VALUES.to_string())));
    }

    #[test]
    fn validate_body_measurement_with_dimensions() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
body_measurements:
  - date: "2025-01-15"
    measurements:
      chest_cm: 100.0
      waist_cm: 85.0
      bicep_left_cm: 38.5
      bicep_right_cm: 38.0
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::NO_BM_VALUES.to_string())));
    }

    // Empty Array Tests
    #[test]
    fn validate_empty_exercises() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises: []
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::NO_EXERCISES.to_string())));
    }

    #[test]
    fn validate_empty_sets() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Bench Press"
        sets: []
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::NO_SETS.to_string())));
    }

    #[test]
    fn validate_set_no_metrics() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Bench Press"
        sets:
          - set_number: 1
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::NO_METRICS.to_string())));
    }

    // RPE Edge Cases
    #[test]
    fn validate_rpe_exactly_zero() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Warmup"
        sets:
          - reps: 10
            rpe: 0.0
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::RPE_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_rpe_exactly_ten() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Max Effort"
        sets:
          - reps: 1
            weight_kg: 200
            rpe: 10.0
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::RPE_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_rpe_negative() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Squat"
        sets:
          - reps: 5
            weight_kg: 100
            rpe: -1.0
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::RPE_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_rpe_above_ten() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Squat"
        sets:
          - reps: 5
            weight_kg: 100
            rpe: 10.5
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::RPE_OUT_OF_RANGE.to_string())));
    }

    // Export Source Tests
    #[test]
    fn validate_export_source_missing_app_name() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
export_source:
  platform: "iOS"
  app_version: "1.0.0"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Squat"
        sets:
          - reps: 5
            weight_kg: 100
"#;
        let result = validate(yaml);
        // app_name is optional, should still be valid
        assert!(result.is_valid());
    }

    #[test]
    fn validate_export_source_different_platforms() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
export_source:
  app_name: "Fitness Tracker"
  platform: "Android"
  app_version: "2.1.0"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Deadlift"
        sets:
          - reps: 5
            weight_kg: 150
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
    }

    // Statistics Edge Cases
    #[test]
    fn validate_statistics_empty_workouts() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts: []
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        let stats = result.statistics.unwrap();
        assert_eq!(stats.total_workouts, 0);
        assert_eq!(stats.total_exercises, 0);
        assert_eq!(stats.total_sets, 0);
        assert_eq!(stats.total_volume_kg, 0.0);
        assert!(stats.date_range_start.is_none());
        assert!(stats.date_range_end.is_none());
    }

    #[test]
    fn validate_statistics_workout_no_sets() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Stretching"
        sets: []
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        let stats = result.statistics.unwrap();
        assert_eq!(stats.total_workouts, 1);
        assert_eq!(stats.total_exercises, 1);
        assert_eq!(stats.total_sets, 0);
        assert_eq!(stats.total_volume_kg, 0.0);
    }

    #[test]
    fn validate_statistics_single_workout_date_range() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-10"
    exercises:
      - name: "Squat"
        sets:
          - reps: 5
            weight_kg: 100
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        let stats = result.statistics.unwrap();
        assert_eq!(stats.date_range_start, Some("2025-01-10".to_string()));
        assert_eq!(stats.date_range_end, Some("2025-01-10".to_string()));
    }

    #[test]
    fn validate_statistics_multiple_workouts_date_range() {
        let yaml = r#"
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-05"
    exercises:
      - name: "Bench Press"
        sets:
          - reps: 5
            weight_kg: 80
  - date: "2025-01-10"
    exercises:
      - name: "Squat"
        sets:
          - reps: 5
            weight_kg: 100
  - date: "2025-01-15"
    exercises:
      - name: "Deadlift"
        sets:
          - reps: 3
            weight_kg: 150
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        let stats = result.statistics.unwrap();
        assert_eq!(stats.total_workouts, 3);
        assert_eq!(stats.date_range_start, Some("2025-01-05".to_string()));
        assert_eq!(stats.date_range_end, Some("2025-01-15".to_string()));
    }

    // ===== History v2 Tests =====

    #[test]
    fn validate_history_v2() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            distance_meters: 5000
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
    }

    #[test]
    fn validate_history_unsupported_version() {
        let yaml = r#"
history_version: 3
exported_at: "2025-12-20T10:30:00Z"
workouts: []
"#;
        let result = validate(yaml);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| e.code == Some(error_codes::INVALID_VERSION.to_string())));
    }

    // ===== Telemetry Tests =====

    #[test]
    fn validate_set_telemetry_heart_rate() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Cycling"
        sets:
          - duration_sec: 600
            telemetry:
              heart_rate_avg: 145
              heart_rate_max: 165
              heart_rate_min: 120
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_set_telemetry_heart_rate_high() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Cycling"
        sets:
          - duration_sec: 600
            telemetry:
              heart_rate_avg: 255
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::HEART_RATE_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_set_telemetry_heart_rate_low() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Cycling"
        sets:
          - duration_sec: 600
            telemetry:
              heart_rate_min: 25
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::HEART_RATE_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_set_telemetry_power() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Cycling"
        sets:
          - duration_sec: 600
            telemetry:
              power_avg: 250
              power_max: 420
              power_min: 180
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_set_telemetry_power_very_high() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Cycling"
        sets:
          - duration_sec: 600
            telemetry:
              power_avg: 2500
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::POWER_NEGATIVE.to_string())));
    }

    #[test]
    fn validate_set_telemetry_elevation() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              elevation_gain_m: 150.5
              elevation_loss_m: 125.2
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_set_telemetry_elevation_negative() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              elevation_gain_m: -50.0
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::ELEVATION_NEGATIVE.to_string())));
    }

    #[test]
    fn validate_set_telemetry_speed() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              speed_avg_kph: 12.5
              speed_max_kph: 15.8
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_set_telemetry_speed_negative() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              speed_avg_mps: -2.5
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::SPEED_NEGATIVE.to_string())));
    }

    #[test]
    fn validate_set_telemetry_cadence() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              cadence_avg: 172
              cadence_max: 185
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_set_telemetry_cadence_very_high() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              cadence_avg: 350
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::CADENCE_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_set_telemetry_humidity() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              humidity_percent: 65.5
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_set_telemetry_humidity_out_of_range() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              humidity_percent: 150.0
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::HUMIDITY_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_set_telemetry_pace_zero() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              pace_avg_sec_per_km: 0
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PACE_NEGATIVE.to_string())));
    }

    #[test]
    fn validate_set_telemetry_unit_mismatch_elevation() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              elevation_gain_m: 150.0
              elevation_gain_ft: 492.0
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::TELEMETRY_UNIT_MISMATCH.to_string())));
    }

    #[test]
    fn validate_set_telemetry_unit_mismatch_speed() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
            telemetry:
              speed_avg_kph: 12.5
              speed_avg_mph: 7.8
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::TELEMETRY_UNIT_MISMATCH.to_string())));
    }

    #[test]
    fn validate_workout_telemetry() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    telemetry:
      heart_rate_avg: 155
      heart_rate_max: 182
      power_avg: 245
      total_distance_km: 45.2
      total_elevation_gain_m: 250
      speed_avg_kph: 30.1
      cadence_avg: 88
      total_calories: 680
      temperature_c: 22.5
      humidity_percent: 65
    exercises:
      - name: "Cycling"
        sets:
          - duration_sec: 5400
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_workout_telemetry_unit_mismatch() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    telemetry:
      total_distance_km: 10.0
      total_distance_mi: 6.2
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 1800
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::TELEMETRY_UNIT_MISMATCH.to_string())));
    }

    #[test]
    fn validate_swimming_with_stroke_rate() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    exercises:
      - name: "Swimming"
        modality: swimming
        sets:
          - duration_sec: 105
            distance_meters: 100
            telemetry:
              heart_rate_avg: 145
              pace_avg_sec_per_km: 1050
              stroke_rate: 34
              calories: 19
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_complete_telemetry_example() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-20T10:30:00Z"
workouts:
  - date: "2025-12-20"
    title: "Morning Run"
    telemetry:
      heart_rate_avg: 158
      heart_rate_max: 185
      total_distance_km: 10.94
      total_elevation_gain_m: 250
      speed_avg_kph: 11.9
      pace_avg_sec_per_km: 303
      cadence_avg: 172
      temperature_c: 14.4
      humidity_percent: 72
      total_calories: 785
    exercises:
      - name: "Trail Run"
        modality: running
        sets:
          - duration_sec: 1980
            distance_meters: 5500
            telemetry:
              heart_rate_avg: 165
              heart_rate_max: 185
              elevation_gain_m: 198
              speed_avg_kph: 10.1
              pace_avg_sec_per_km: 355
              cadence_avg: 168
              temperature_c: 13.3
              humidity_percent: 75
              calories: 425
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    // ===== PWF v2.1 Swimming Tests =====

    #[test]
    fn validate_swimming_swolf_correct() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    exercises:
      - name: "Freestyle"
        pool_config:
          pool_length: 25.0
          pool_length_unit: meters
        sets:
          - duration_sec: 120
            distance_meters: 100
            swimming:
              lengths:
                - length_number: 1
                  stroke_type: freestyle
                  duration_sec: 30
                  stroke_count: 15
                  swolf: 45
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_swimming_swolf_mismatch() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    exercises:
      - name: "Freestyle"
        sets:
          - duration_sec: 120
            swimming:
              lengths:
                - length_number: 1
                  stroke_type: freestyle
                  duration_sec: 30
                  stroke_count: 15
                  swolf: 50
"#;
        let result = validate(yaml);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| e.code == Some(error_codes::SWOLF_MISMATCH.to_string())));
    }

    #[test]
    fn validate_pool_config_invalid_length() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    exercises:
      - name: "Swimming"
        pool_config:
          pool_length: 0
          pool_length_unit: meters
        sets:
          - duration_sec: 60
"#;
        let result = validate(yaml);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| e.code == Some(error_codes::POOL_LENGTH_INVALID.to_string())));
    }

    #[test]
    fn validate_pool_config_negative_length() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    exercises:
      - name: "Swimming"
        pool_config:
          pool_length: -25.0
          pool_length_unit: meters
        sets:
          - duration_sec: 60
"#;
        let result = validate(yaml);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| e.code == Some(error_codes::POOL_LENGTH_INVALID.to_string())));
    }

    // ===== PWF v2.1 Time-Series Tests =====

    #[test]
    fn validate_time_series_length_match() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    exercises:
      - name: "Cycling"
        sets:
          - duration_sec: 600
            telemetry:
              time_series:
                timestamps: ["2025-12-21T10:00:00Z", "2025-12-21T10:00:01Z", "2025-12-21T10:00:02Z"]
                elapsed_sec: [0, 1, 2]
                heart_rate: [120, 125, 130]
                power: [200, 210, 220]
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_time_series_length_mismatch() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    exercises:
      - name: "Cycling"
        sets:
          - duration_sec: 600
            telemetry:
              time_series:
                timestamps: ["2025-12-21T10:00:00Z", "2025-12-21T10:00:01Z", "2025-12-21T10:00:02Z"]
                elapsed_sec: [0, 1]
                heart_rate: [120, 125, 130]
"#;
        let result = validate(yaml);
        assert!(result.is_valid()); // Still valid (warnings only for telemetry)
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::TIME_SERIES_LENGTH_MISMATCH.to_string())));
    }

    #[test]
    fn validate_time_series_gps_valid() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 600
            telemetry:
              time_series:
                timestamps: ["2025-12-21T10:00:00Z", "2025-12-21T10:00:01Z"]
                latitude: [40.7128, 40.7129]
                longitude: [-74.0060, -74.0061]
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_time_series_latitude_out_of_range() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 600
            telemetry:
              time_series:
                timestamps: ["2025-12-21T10:00:00Z"]
                latitude: [91.0]
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::GPS_LATITUDE_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_time_series_longitude_out_of_range() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    exercises:
      - name: "Running"
        sets:
          - duration_sec: 600
            telemetry:
              time_series:
                timestamps: ["2025-12-21T10:00:00Z"]
                longitude: [181.0]
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::GPS_LONGITUDE_OUT_OF_RANGE.to_string())));
    }

    // ===== PWF v2.1 Sport Segments Tests =====

    #[test]
    fn validate_sport_segments_sequential() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    sport: other
    sport_segments:
      - segment_id: "swim"
        sport: swimming
        segment_index: 0
        duration_sec: 1800
      - segment_id: "bike"
        sport: cycling
        segment_index: 1
        duration_sec: 3600
      - segment_id: "run"
        sport: running
        segment_index: 2
        duration_sec: 2400
    exercises:
      - name: "Triathlon"
        sets:
          - duration_sec: 7800
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_sport_segments_gap() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    sport_segments:
      - segment_id: "swim"
        sport: swimming
        segment_index: 0
        duration_sec: 1800
      - segment_id: "run"
        sport: running
        segment_index: 2
        duration_sec: 2400
    exercises:
      - name: "Test"
        sets:
          - duration_sec: 60
"#;
        let result = validate(yaml);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| e.code == Some(error_codes::SEGMENT_INDEX_GAP.to_string())));
    }

    #[test]
    fn validate_sport_segments_duplicate() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    sport_segments:
      - segment_id: "swim1"
        sport: swimming
        segment_index: 0
        duration_sec: 1800
      - segment_id: "swim2"
        sport: swimming
        segment_index: 0
        duration_sec: 1800
    exercises:
      - name: "Test"
        sets:
          - duration_sec: 60
"#;
        let result = validate(yaml);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| e.code == Some(error_codes::SEGMENT_INDEX_DUPLICATE.to_string())));
    }

    // ===== PWF v2.1 Transition Tests =====

    #[test]
    fn validate_transition_sport_match() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    sport_segments:
      - segment_id: "swim"
        sport: swimming
        segment_index: 0
        transition:
          transition_id: "T1"
          from_sport: swimming
          to_sport: cycling
          duration_sec: 120
      - segment_id: "bike"
        sport: cycling
        segment_index: 1
    exercises:
      - name: "Test"
        sets:
          - duration_sec: 60
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_transition_from_sport_mismatch() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    sport_segments:
      - segment_id: "swim"
        sport: swimming
        segment_index: 0
        transition:
          transition_id: "T1"
          from_sport: running
          to_sport: cycling
      - segment_id: "bike"
        sport: cycling
        segment_index: 1
    exercises:
      - name: "Test"
        sets:
          - duration_sec: 60
"#;
        let result = validate(yaml);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| e.code == Some(error_codes::TRANSITION_SPORT_MISMATCH.to_string())));
    }

    #[test]
    fn validate_transition_to_sport_mismatch() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    sport_segments:
      - segment_id: "swim"
        sport: swimming
        segment_index: 0
        transition:
          transition_id: "T1"
          from_sport: swimming
          to_sport: running
      - segment_id: "bike"
        sport: cycling
        segment_index: 1
    exercises:
      - name: "Test"
        sets:
          - duration_sec: 60
"#;
        let result = validate(yaml);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| e.code == Some(error_codes::TRANSITION_SPORT_MISMATCH.to_string())));
    }

    // ===== PWF v2.1 GPS Route Tests =====

    #[test]
    fn validate_gps_route_valid() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    telemetry:
      gps_route:
        route_id: "route-123"
        positions:
          - latitude_deg: 40.7128
            longitude_deg: -74.0060
            timestamp: "2025-12-21T10:00:00Z"
            heading_deg: 90.0
          - latitude_deg: 40.7129
            longitude_deg: -74.0061
            timestamp: "2025-12-21T10:00:01Z"
            heading_deg: 180.5
    exercises:
      - name: "Run"
        sets:
          - duration_sec: 60
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_gps_route_invalid_coordinates() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    telemetry:
      gps_route:
        route_id: "route-123"
        positions:
          - latitude_deg: 91.0
            longitude_deg: 181.0
            timestamp: "2025-12-21T10:00:00Z"
            heading_deg: 400.0
    exercises:
      - name: "Run"
        sets:
          - duration_sec: 60
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::GPS_LATITUDE_OUT_OF_RANGE.to_string())));
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::GPS_LONGITUDE_OUT_OF_RANGE.to_string())));
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::GPS_HEADING_OUT_OF_RANGE.to_string())));
    }

    // ===== PWF v2.1 Advanced Metrics Tests =====

    #[test]
    fn validate_advanced_metrics_valid() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    telemetry:
      advanced_metrics:
        training_effect: 3.5
        anaerobic_training_effect: 2.1
        performance_condition: 5
        vo2_max_estimate: 52.3
    exercises:
      - name: "Run"
        sets:
          - duration_sec: 3600
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_advanced_metrics_training_effect_out_of_range() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    telemetry:
      advanced_metrics:
        training_effect: 6.0
        anaerobic_training_effect: -1.0
    exercises:
      - name: "Run"
        sets:
          - duration_sec: 3600
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::TRAINING_EFFECT_OUT_OF_RANGE.to_string())));
    }

    #[test]
    fn validate_advanced_metrics_performance_condition_out_of_range() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    telemetry:
      advanced_metrics:
        performance_condition: 25
    exercises:
      - name: "Run"
        sets:
          - duration_sec: 3600
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::PERFORMANCE_CONDITION_OUT_OF_RANGE.to_string())));
    }

    // ===== PWF v2.1 Power Metrics Tests =====

    #[test]
    fn validate_power_metrics_intensity_factor_correct() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    telemetry:
      power_avg: 200
      power_metrics:
        normalized_power: 250
        ftp_watts: 300
        intensity_factor: 0.833
    exercises:
      - name: "Ride"
        sets:
          - duration_sec: 3600
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_power_metrics_intensity_factor_mismatch() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    telemetry:
      power_metrics:
        normalized_power: 250
        ftp_watts: 300
        intensity_factor: 0.9
    exercises:
      - name: "Ride"
        sets:
          - duration_sec: 3600
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::INTENSITY_FACTOR_MISMATCH.to_string())));
    }

    #[test]
    fn validate_power_metrics_variability_index_correct() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    telemetry:
      power_avg: 200
      power_metrics:
        normalized_power: 220
        variability_index: 1.1
    exercises:
      - name: "Ride"
        sets:
          - duration_sec: 3600
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_power_metrics_variability_index_mismatch() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    telemetry:
      power_avg: 200
      power_metrics:
        normalized_power: 220
        variability_index: 1.5
    exercises:
      - name: "Ride"
        sets:
          - duration_sec: 3600
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::VARIABILITY_INDEX_MISMATCH.to_string())));
    }

    // ===== PWF v2.1 Zone Validation Tests =====

    #[test]
    fn validate_time_in_zones_valid() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    telemetry:
      time_in_zones:
        hr_zones_sec: [600, 900, 1200, 300, 0]
        hr_zone_boundaries: [120, 140, 160, 180]
        power_zones_sec: [300, 600, 900, 600, 300, 100]
        power_zone_boundaries: [150, 200, 250, 300, 350]
    exercises:
      - name: "Ride"
        sets:
          - duration_sec: 3000
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn validate_time_in_zones_length_mismatch() {
        let yaml = r#"
history_version: 2
exported_at: "2025-12-21T10:00:00Z"
workouts:
  - date: "2025-12-21"
    telemetry:
      time_in_zones:
        hr_zones_sec: [600, 900, 1200, 300, 0, 100, 200, 300]
        hr_zone_boundaries: [120, 140, 160, 180]
    exercises:
      - name: "Ride"
        sets:
          - duration_sec: 3000
"#;
        let result = validate(yaml);
        assert!(result.is_valid());
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == Some(error_codes::ZONE_ARRAY_LENGTH_MISMATCH.to_string())));
    }
}
