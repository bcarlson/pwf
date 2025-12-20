# Multi-Week Periodization Implementation for PWF v2.0

## Overview

This document describes the implementation of multi-week periodization support for PWF v2.0. This feature enables coaches and athletes to create sophisticated training programs that vary systematically over multiple weeks.

## Design Goals

1. **Backward Compatibility**: Single-week plans (v1 and v2) must continue to work without modification
2. **Flexibility**: Support all major periodization models (linear, block, undulating, etc.)
3. **Override System**: Week-specific parameters override base exercise definitions
4. **Clarity**: The YAML structure should be intuitive and readable

## Data Structure

### Type Definitions

Three new types are added to `/home/nitro/Projects/pwf/crates/pwf-core/src/plan/types.rs`:

#### 1. PlanCycle (Modified)

```rust
/// Training cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanCycle {
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub weeks: Option<Vec<PlanWeek>>,  // NEW: Multi-week support
    pub days: Vec<PlanDay>,
}
```

#### 2. PlanWeek (New)

```rust
/// Week definition for multi-week periodization (PWF v2.0)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanWeek {
    #[serde(default)]
    pub week_number: Option<u32>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub overrides: Vec<ExerciseOverride>,
}
```

#### 3. ExerciseOverride (New)

```rust
/// Exercise override for specific week(s) (PWF v2.0)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseOverride {
    // Exercise identification (must match base exercise)
    #[serde(default)]
    pub exercise_id: Option<String>,
    #[serde(default)]
    pub exercise_name: Option<String>,

    // Day identification (where this exercise lives)
    #[serde(default)]
    pub day_id: Option<String>,
    #[serde(default)]
    pub day_order: Option<u32>,

    // Overridable training parameters
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
    pub target_notes: Option<String>,
    #[serde(default)]
    pub rest_between_sets_sec: Option<u32>,
}
```

## How It Works

### 1. Base Template Pattern

The `days` array in `PlanCycle` serves as the base template. It defines:
- The exercise selection (what movements are in the program)
- The weekly structure (which days have which exercises)
- Default parameters (fallback values if no override exists)

### 2. Week-Specific Overrides

The `weeks` array contains weekly variations. Each week can override specific exercise parameters by matching:
- **By exercise_id**: Most reliable - matches `exercise.id` in the base template
- **By exercise_name**: Fallback - matches `exercise.name`
- **By day_id or day_order**: Specifies which day the exercise is on

### 3. Override Resolution

When loading a plan for a specific week:

```
For each exercise in the base template:
  1. Look for an override in the current week matching:
     - exercise_id + (day_id OR day_order)
     - exercise_name + (day_id OR day_order)
  2. If found, merge override values over base values
  3. If not found, use base values
```

Only fields specified in the override are replaced. Unspecified fields retain base values.

## Validation Rules

Validators in `/home/nitro/Projects/pwf/crates/pwf-core/src/plan/validator.rs` should enforce:

### Required Validations (Errors)

1. **PWF-P050**: `weeks` array is only valid in `plan_version: 2`
2. **PWF-P051**: Week numbers must be unique if specified
3. **PWF-P052**: Week numbers must be sequential starting from 1 if specified
4. **PWF-P053**: Exercise override must specify either `exercise_id` or `exercise_name`
5. **PWF-P054**: Exercise override must specify either `day_id` or `day_order`
6. **PWF-P055**: Override `exercise_id` must match an existing exercise in the base template
7. **PWF-P056**: Override `day_id` or `day_order` must match an existing day in the base template
8. **PWF-P057**: Percentage-based loading rules still apply to overrides (require `percent_of` with `target_weight_percent`)
9. **PWF-P058**: Cannot override modality (exercises cannot change type between weeks)

### Recommended Warnings

1. **PWF-P060**: Override references `exercise_name` but could use `exercise_id` for reliability
2. **PWF-P061**: Week has no overrides (possibly unnecessary week entry)
3. **PWF-P062**: Exercise in base template is never overridden across all weeks (possibly forgot to program it)
4. **PWF-P063**: Week name not specified (harder to track in logs)
5. **PWF-P064**: Large jump in intensity between consecutive weeks (>10%)

## JSON Schema Changes

Update `/home/nitro/Projects/pwf/schema/pwf-v2.json`:

```json
{
  "$defs": {
    "Cycle": {
      "properties": {
        "weeks": {
          "type": "array",
          "items": {"$ref": "#/$defs/Week"},
          "description": "Multi-week periodization (PWF v2.0)"
        }
      }
    },
    "Week": {
      "type": "object",
      "properties": {
        "week_number": {
          "type": "integer",
          "minimum": 1,
          "description": "Week number (1-indexed)"
        },
        "name": {
          "type": "string",
          "description": "Week name or phase (e.g., 'Accumulation Week 1')"
        },
        "notes": {
          "type": "string",
          "description": "Coaching notes for this week"
        },
        "overrides": {
          "type": "array",
          "items": {"$ref": "#/$defs/ExerciseOverride"}
        }
      }
    },
    "ExerciseOverride": {
      "type": "object",
      "properties": {
        "exercise_id": {"type": "string"},
        "exercise_name": {"type": "string"},
        "day_id": {"type": "string"},
        "day_order": {"type": "integer", "minimum": 0},
        "target_sets": {"type": "integer", "minimum": 1},
        "target_reps": {"type": "integer", "minimum": 1},
        "target_duration_sec": {"type": "integer", "minimum": 1},
        "target_distance_meters": {"type": "number", "minimum": 0},
        "target_load": {"type": "string"},
        "target_weight_percent": {"type": "number", "minimum": 0, "maximum": 200},
        "percent_of": {"type": "string", "enum": ["1rm", "3rm", "5rm", "10rm"]},
        "target_notes": {"type": "string"},
        "rest_between_sets_sec": {"type": "integer", "minimum": 0}
      },
      "oneOf": [
        {"required": ["exercise_id"]},
        {"required": ["exercise_name"]}
      ],
      "anyOf": [
        {"required": ["day_id"]},
        {"required": ["day_order"]}
      ]
    }
  }
}
```

## Example Patterns

### 1. Linear Periodization

Progressive increase in intensity, decrease in volume over time:

```yaml
weeks:
  - week_number: 1
    name: "Volume Phase"
    overrides:
      - exercise_id: "squat"
        target_sets: 5
        target_reps: 8
        target_weight_percent: 70
        percent_of: "1rm"

  - week_number: 4
    name: "Strength Phase"
    overrides:
      - exercise_id: "squat"
        target_sets: 5
        target_reps: 5
        target_weight_percent: 80
        percent_of: "1rm"

  - week_number: 8
    name: "Peaking Phase"
    overrides:
      - exercise_id: "squat"
        target_sets: 6
        target_reps: 1
        target_weight_percent: 95
        percent_of: "1rm"
```

See: `/home/nitro/Projects/pwf/examples/periodization-linear.yaml`

### 2. Block Periodization

Distinct training blocks with different emphases:

```yaml
weeks:
  # Hypertrophy Block (Weeks 1-3)
  - week_number: 1
    name: "Hypertrophy Block 1"
    overrides:
      - exercise_id: "squat"
        target_sets: 4
        target_reps: 10
        target_weight_percent: 65
        percent_of: "1rm"

  # Strength Block (Weeks 4-6)
  - week_number: 4
    name: "Strength Block 1"
    overrides:
      - exercise_id: "squat"
        target_sets: 5
        target_reps: 5
        target_weight_percent: 78
        percent_of: "1rm"

  # Power Block (Weeks 7-9)
  - week_number: 7
    name: "Power Block 1"
    overrides:
      - exercise_id: "squat"
        target_sets: 6
        target_reps: 2
        target_weight_percent: 85
        percent_of: "1rm"
```

See: `/home/nitro/Projects/pwf/examples/periodization-block.yaml`

### 3. Daily Undulating Periodization (DUP)

Different intensities on different days within the same week:

```yaml
weeks:
  - week_number: 1
    name: "DUP Week 1"
    overrides:
      # Monday: Power
      - day_order: 0
        exercise_id: "squat"
        target_sets: 6
        target_reps: 2
        target_weight_percent: 85
        percent_of: "1rm"

      # Wednesday: Strength
      - day_order: 1
        exercise_id: "squat"
        target_sets: 4
        target_reps: 5
        target_weight_percent: 75
        percent_of: "1rm"

      # Friday: Hypertrophy
      - day_order: 2
        exercise_id: "squat"
        target_sets: 4
        target_reps: 10
        target_weight_percent: 65
        percent_of: "1rm"
```

See: `/home/nitro/Projects/pwf/examples/periodization-undulating.yaml`

## Testing Requirements

Tests in `/home/nitro/Projects/pwf/crates/pwf-core/tests/periodization_tests.rs`:

### Unit Tests

1. **Serialization/Deserialization**
   - `test_plan_week_roundtrip()`
   - `test_exercise_override_roundtrip()`
   - `test_multi_week_plan_roundtrip()`

2. **Validation Tests**
   - `test_weeks_only_in_v2()`
   - `test_unique_week_numbers()`
   - `test_sequential_week_numbers()`
   - `test_override_requires_exercise_identifier()`
   - `test_override_requires_day_identifier()`
   - `test_override_exercise_must_exist()`
   - `test_override_day_must_exist()`
   - `test_override_percentage_loading_rules()`
   - `test_cannot_override_modality()`

3. **Integration Tests**
   - `test_linear_periodization_example()`
   - `test_block_periodization_example()`
   - `test_dup_example()`
   - `test_backward_compatibility_single_week()`

## CLI Usage

Users can validate multi-week plans using the existing CLI:

```bash
# Validate a multi-week plan
pwf validate examples/periodization-linear.yaml

# Strict validation
pwf validate --strict examples/periodization-block.yaml

# JSON output
pwf validate --format json examples/periodization-undulating.yaml
```

## Migration Path

### For Existing v1 Plans

No changes required. Plans without a `weeks` array work as before.

### For New v2 Multi-Week Plans

1. Set `plan_version: 2`
2. Define base exercise template in `cycle.days`
3. Add `cycle.weeks` array with weekly overrides
4. Ensure exercises have `id` fields for reliable matching

## Best Practices

### 1. Use Exercise IDs

Always set `id` on exercises in the base template:

```yaml
exercises:
  - id: "squat"  # Good - reliable override matching
    name: "Back Squat"
    modality: strength
```

### 2. Document Week Phases

Use `name` and `notes` to explain programming intent:

```yaml
- week_number: 1
  name: "Accumulation Week 1"
  notes: "High volume, moderate intensity. Focus on work capacity."
```

### 3. Override Only What Changes

Don't repeat unchanged parameters:

```yaml
# Good
overrides:
  - exercise_id: "squat"
    target_weight_percent: 75  # Only override the changing value

# Bad (verbose)
overrides:
  - exercise_id: "squat"
    target_sets: 4  # Same as base
    target_reps: 8  # Same as base
    target_weight_percent: 75  # Only this changed
```

### 4. Include Deload Weeks

Plan for recovery:

```yaml
- week_number: 4
  name: "Deload Week"
  notes: "Reduce intensity and volume by 40% for recovery"
  overrides:
    - exercise_id: "squat"
      target_sets: 3
      target_reps: 6
      target_weight_percent: 60
```

### 5. Progressive Overload

Ensure systematic progression:

```yaml
# Week 1
- week_number: 1
  overrides:
    - exercise_id: "bench"
      target_weight_percent: 75

# Week 2 (progress)
- week_number: 2
  overrides:
    - exercise_id: "bench"
      target_weight_percent: 77

# Week 3 (progress)
- week_number: 3
  overrides:
    - exercise_id: "bench"
      target_weight_percent: 80
```

## Statistics and Analysis

When calculating plan statistics for multi-week plans:

1. **Total Volume**: Sum volume across all weeks
2. **Average Intensity**: Calculate per week, then average across weeks
3. **Volume Landmarks**: Track volume high/low points
4. **Intensity Landmarks**: Track intensity progression
5. **Deload Weeks**: Identify and flag recovery weeks

Example statistics output:

```json
{
  "total_weeks": 12,
  "total_days": 48,
  "avg_intensity": 82.5,
  "peak_intensity_week": 11,
  "peak_volume_week": 3,
  "deload_weeks": [4, 8, 12],
  "progression_type": "linear"
}
```

## Documentation

Create `/home/nitro/Projects/pwf/docs/periodization.md` with:

1. Overview of periodization models
2. How to structure multi-week plans
3. Common programming strategies
4. Example code snippets
5. Troubleshooting guide

## Future Enhancements

Potential v2.1 features:

1. **Auto-Progression**: Define progression rules once, auto-generate weeks
2. **Week Ranges**: Apply same override to multiple weeks (e.g., weeks 1-3)
3. **Conditional Logic**: If-then rules (e.g., "if all reps completed, increase weight")
4. **Templates**: Pre-built periodization templates for common models
5. **Training Load Metrics**: Calculate TSS, volume load, etc.

## Implementation Checklist

- [x] Design data structures (PlanWeek, ExerciseOverride)
- [ ] Update types.rs with new structures
- [ ] Implement validation logic in validator.rs
- [ ] Update JSON schema (pwf-v2.json)
- [x] Create example plans (linear, block, DUP)
- [ ] Write comprehensive tests
- [ ] Update documentation
- [ ] Run cargo test
- [ ] Run cargo fmt
- [ ] Run cargo clippy
- [ ] Integration test with CLI

## References

- **Linear Periodization**: Bompa & Haff (2009). Periodization: Theory and Methodology of Training
- **Block Periodization**: Issurin (2010). New Horizons for the Methodology and Physiology of Training Periodization
- **DUP**: Rhea et al. (2002). A comparison of linear and daily undulating periodized programs with equated volume and intensity for strength
- **PWF Specification**: `/home/nitro/Projects/pwf/docs/SPECIFICATION.md`
