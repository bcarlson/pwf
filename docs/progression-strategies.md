# Progression Strategies

Progressive overload is the foundational principle of strength training - gradually increasing the training stimulus over time. PWF v2.0 introduces `progression_rules` to formalize and automate progression strategies within workout plans.

## Overview

The `progression_rules` field is an optional block that can be added to any exercise in PWF v2.0. It defines:
- **When to progress** (success conditions)
- **How to progress** (weight/rep increments)
- **When to deload** (failure conditions)
- **How to deload** (deload percentages)

## Progression Types

PWF supports two primary progression strategies:

### Linear Progression

**Type:** `linear`

Linear progression is the simplest and most effective strategy for beginners. Add a fixed amount of weight each time you successfully complete all prescribed sets and reps.

**Required Fields:**
- `weight_increment_kg` OR `weight_increment_lbs`
- `success_condition`

**Example:**
```yaml
progression_rules:
  type: linear
  success_condition: all_sets_completed
  weight_increment_kg: 2.5
  deload_condition: failed_twice_consecutive
  deload_percent: 90
```

**How it works:**
1. Complete target sets × reps (e.g., 3×5)
2. If successful → add weight increment next session
3. If failed → repeat same weight
4. If failed multiple times → deload

**Best for:**
- Beginner lifters
- Main compound movements (squat, bench, deadlift)
- Programs where consistent weekly progression is expected

**Typical increments:**
- Lower body: 2.5-5 kg (5-10 lbs)
- Upper body: 1.25-2.5 kg (2.5-5 lbs)
- Overhead/isolation: 0.5-1.25 kg (1-2.5 lbs)

### Double Progression

**Type:** `double_progression`

Double progression increases reps within a target range before adding weight. This approach is more sustainable for intermediate lifters and hypertrophy-focused training.

**Required Fields:**
- `weight_increment_kg` OR `weight_increment_lbs`
- `reps_range_min`
- `reps_range_max`
- `success_condition`

**Example:**
```yaml
progression_rules:
  type: double_progression
  success_condition: all_sets_completed
  weight_increment_kg: 2.0
  reps_range_min: 8
  reps_range_max: 12
  deload_condition: failed_twice_consecutive
  deload_percent: 85
```

**How it works:**
1. Start at bottom of rep range (e.g., 3×8)
2. Add reps each session (3×9, 3×10, etc.)
3. When hitting top of range (3×12) → add weight and drop back to bottom (3×8)
4. Repeat the cycle

**Best for:**
- Intermediate/advanced lifters
- Hypertrophy (muscle building) programs
- Accessory/isolation exercises
- When linear progression has stalled

**Typical rep ranges:**
- Compound movements: 5-8, 6-10, 8-12
- Isolation exercises: 10-15, 12-15, 15-20

## Success Conditions

Success conditions determine when progression is triggered:

### `all_sets_completed`
Progress when **all prescribed sets** hit the target reps.
- **Most common** and **most conservative**
- Example: For 3×8, all 3 sets must reach 8 reps
- Best for: Compound movements, strength focus

### `last_set_completed`
Progress when the **final set** hits target reps.
- More aggressive progression
- Example: For 3×8, only the 3rd set needs 8 reps (earlier sets could be 8, 7, 8)
- Best for: Isolation exercises, volume-focused training

### `average_reps_reached`
Progress when **average reps across sets** reaches target.
- Most aggressive approach
- Example: For 3×8 target (24 total), could be 9+8+7 = 24 reps
- Best for: High-volume training, advanced lifters

## Deload Conditions

Deload conditions determine when to reduce training load:

### `failed_once_consecutive`
Deload after **one failed session**.
- Most conservative
- Best for: Deadlifts (highly fatiguing), peaking phases

### `failed_twice_consecutive`
Deload after **two consecutive failures**.
- **Most common** recommendation
- Balances progression with recovery
- Best for: Main compound lifts

### `failed_three_consecutive`
Deload after **three consecutive failures**.
- More aggressive, gives more chances
- Best for: Isolation exercises, hypertrophy training

### `rir_above_target`
Deload when Reps in Reserve consistently exceeds target.
- Advanced technique requiring RPE/RIR tracking
- Indicates fatigue accumulation
- Best for: RPE-based programs, experienced lifters

## Deload Parameters

### `deload_percent`
Percentage of current weight to reduce to (50-100).

**Typical values:**
- **90%**: Light deload, maintain most strength
- **85%**: Moderate deload, standard recommendation
- **80%**: Deeper deload, for accumulated fatigue
- **70%**: Significant deload, for extended fatigue or injury recovery

### `deload_weeks`
Number of weeks to maintain deload weight.

**Typical values:**
- **1 week**: Standard deload duration
- **2 weeks**: Extended recovery for accumulated fatigue

## Weight Constraints

### Maximum Weight Limits

Use `max_weight_kg` or `max_weight_lbs` to cap progression:

```yaml
progression_rules:
  type: linear
  success_condition: all_sets_completed
  weight_increment_kg: 2.5
  max_weight_kg: 200
  notes: "Safety limit for back squat"
```

**Use cases:**
- Safety constraints based on equipment
- Goal weights (e.g., "work up to 2× bodyweight squat")
- Competition preparation targets

## Complete Examples

### Beginner Linear Progression Program

```yaml
- name: "Barbell Back Squat"
  modality: strength
  target_sets: 3
  target_reps: 5
  target_load: "100kg"
  progression_rules:
    type: linear
    success_condition: all_sets_completed
    weight_increment_kg: 2.5
    deload_condition: failed_twice_consecutive
    deload_percent: 90
    max_weight_kg: 200
    notes: "Add 2.5kg when all 3×5 completed successfully"
```

### Intermediate Double Progression

```yaml
- name: "Dumbbell Bench Press"
  modality: strength
  target_sets: 3
  target_reps: 8
  target_load: "30kg DBs"
  progression_rules:
    type: double_progression
    success_condition: all_sets_completed
    weight_increment_kg: 2.0
    reps_range_min: 8
    reps_range_max: 12
    deload_condition: failed_twice_consecutive
    deload_percent: 85
    notes: "Work up to 3×12, then increase weight and return to 3×8"
```

### Isolation Exercise with Aggressive Progression

```yaml
- name: "Dumbbell Curl"
  modality: strength
  target_sets: 3
  target_reps: 10
  target_load: "14kg DBs"
  progression_rules:
    type: double_progression
    success_condition: last_set_completed
    weight_increment_kg: 1.0
    reps_range_min: 10
    reps_range_max: 12
    deload_condition: failed_three_consecutive
    deload_percent: 85
    notes: "Smaller jumps for isolation work"
```

## Best Practices

### When to Use Linear Progression

- **Beginner lifters** (first 6-12 months)
- **Main compound lifts** (squat, bench, deadlift, overhead press)
- **Strength-focused phases** (1-5 rep ranges)
- **When progress is consistent** (adding weight weekly)

### When to Use Double Progression

- **Intermediate/advanced lifters** (linear gains have slowed)
- **Hypertrophy training** (muscle building focus)
- **Accessory/isolation exercises**
- **When weekly weight increases become difficult**
- **Higher rep ranges** (8+ reps)

### Increment Guidelines

**Lower body compounds:** Larger increments tolerated
- Squat: 2.5-5 kg (5-10 lbs)
- Deadlift: 5-10 kg (10-20 lbs)

**Upper body compounds:** Smaller increments needed
- Bench Press: 2.5 kg (5 lbs)
- Overhead Press: 1.25 kg (2.5 lbs)

**Isolation exercises:** Smallest increments
- Curls, extensions, raises: 1-2 kg (2-5 lbs)
- Consider microplates (0.5 kg / 1 lb)

### Deload Strategy

**Reactive deloads** (based on failure):
- Most common in linear progression
- Triggered by `deload_condition`
- Typically 90% of working weight for 1 week

**Planned deloads** (preventative):
- Common in periodized programs
- Every 4-6 weeks regardless of performance
- Can be implemented outside of `progression_rules`

## Validation Rules

PWF validates progression rules to prevent common errors:

### Required Fields
- Linear: Must specify weight increment
- Double Progression: Must specify weight increment AND reps range

### Constraints
- Cannot specify both `_kg` and `_lbs` fields
- `reps_range_min` must be less than `reps_range_max`
- `deload_percent` must be 50-100
- Weight increments must be positive
- Maximum weights must be positive

### Warnings
- Very large weight increments (>50kg or >100lbs)
- Very large rep ranges (>100 reps)
- Long deload periods (>8 weeks)
- Missing `deload_percent` when `deload_condition` specified

## Integration with PWF Features

### With Exercise Library

Progression rules can be combined with exercise references:

```yaml
exercise_library:
  - id: bench-press
    name: "Barbell Bench Press"
    modality: strength
    default_sets: 3
    default_reps: 8

cycle:
  days:
    - exercises:
        - exercise_ref: bench-press
          target_load: "80kg"
          progression_rules:
            type: linear
            success_condition: all_sets_completed
            weight_increment_kg: 2.5
```

### With Percentage-Based Loading

Combine with `target_weight_percent` for auto-regulation:

```yaml
- name: "Pause Squat"
  modality: strength
  target_sets: 3
  target_reps: 5
  target_weight_percent: 80
  percent_of: "1rm"
  reference_exercise: "Back Squat"
  progression_rules:
    type: linear
    success_condition: all_sets_completed
    weight_increment_kg: 2.5
```

## Future Enhancements

Potential additions in future PWF versions:
- Wave loading patterns
- Block periodization templates
- RPE/RIR-based auto-regulation
- Velocity-based training rules
- Time-based progression for conditioning

## Version Compatibility

- **PWF v2.0+**: Full support for `progression_rules`
- **PWF v1.x**: Field ignored with warning

Always set `plan_version: 2` to use progression rules.

## See Also

- [Exercise Block Documentation](blocks/exercise.md)
- [PWF v2.0 Specification](SPECIFICATION.md)
- [Example Plans](../examples/)
  - [progression-linear.yaml](../examples/progression-linear.yaml)
  - [progression-double.yaml](../examples/progression-double.yaml)
