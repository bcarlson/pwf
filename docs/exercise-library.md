# Exercise Library (PWF v2.0)

## Overview

PWF v2.0 introduces the **exercise library** feature, allowing you to define reusable exercise templates that can be referenced throughout your workout plans. This eliminates repetition and ensures consistency across your training programs.

## Key Features

- **Reusable Definitions**: Define exercises once, reference them multiple times
- **Default Values**: Set default sets, reps, duration, and distance for each exercise
- **Override Support**: Override any library default on a per-exercise basis
- **Rich Metadata**: Include equipment, muscle groups, difficulty, cues, and media links
- **Type Safety**: Validation ensures all library references are valid

## Basic Usage

### Defining Library Exercises

```yaml
plan_version: 2
meta:
  title: "My Workout Plan"

exercise_library:
  - id: squat
    name: "Barbell Back Squat"
    modality: strength
    default_sets: 3
    default_reps: 8
    equipment:
      - barbell
      - squat_rack
    muscle_groups:
      - quads
      - glutes
    difficulty: intermediate
    cues: "Keep chest up, break at hips and knees simultaneously"

  - id: plank
    name: "Front Plank"
    modality: countdown
    default_sets: 3
    default_duration_sec: 30
    difficulty: beginner
    cues: "Straight line from head to heels"

cycle:
  days:
    - exercises:
        # Use library defaults
        - exercise_ref: squat

        # Override specific values
        - exercise_ref: squat
          target_sets: 5
          target_reps: 5
          target_load: "140kg"

        - exercise_ref: plank
          target_duration_sec: 45
```

## Library Exercise Fields

### Required Fields

- `id` (string, 1-100 chars, alphanumeric/-/_): Unique identifier for the exercise
- `name` (string, 1-100 chars): Display name
- `modality` (enum): Exercise type (strength, countdown, stopwatch, interval, cycling, running, rowing, swimming)

### Optional Metadata

- `description` (string, max 500 chars): Detailed exercise description
- `equipment` (array): Required equipment tags
- `muscle_groups` (array): Targeted muscle groups
- `difficulty` (enum): beginner, intermediate, or advanced

### Default Target Values

- `default_sets` (integer): Default number of sets
- `default_reps` (integer): Default reps per set
- `default_duration_sec` (integer): Default duration in seconds
- `default_distance_meters` (number): Default distance in meters

### Media & Coaching

- `cues` (string): Form cues and coaching points
- `link` (string, HTTPS URL): Tutorial or demo video
- `image` (string, HTTPS URL): Exercise demonstration image

## Referencing Library Exercises

### Using Defaults

When you reference a library exercise without overrides, all defaults are applied:

```yaml
exercises:
  - exercise_ref: squat  # Uses all defaults from library
```

### Overriding Values

Override any field by specifying it in the exercise:

```yaml
exercises:
  - exercise_ref: squat
    target_sets: 5        # Overrides default_sets
    target_load: "160kg"  # Adds loading info
    target_notes: "Focus on depth"
```

### Overriding Name and Cues

```yaml
exercises:
  - exercise_ref: squat
    name: "Front Squat"   # Different name
    cues: "Elbows high"   # Different coaching points
```

## Validation Rules

### Library Validation

- Maximum 500 exercises in library (PWF-P033)
- All exercise IDs must be unique (PWF-P034)
- IDs must be 1-100 characters, alphanumeric/-/_ only (PWF-P035)
- Names must be 1-100 characters (PWF-P036)
- Descriptions limited to 500 characters (PWF-P037)

### Reference Validation

- `exercise_ref` must reference a valid library ID (PWF-P032)
- In v2, each exercise must have either `modality` OR `exercise_ref` (PWF-P030)
- Warning if both are specified (library takes precedence) (PWF-P031)

## Resolution Order

When an exercise uses `exercise_ref`, fields are merged with this priority:

1. **Plan Exercise Fields**: Highest priority (your overrides)
2. **Library Exercise Fields**: Used if not overridden
3. **No Value**: Field remains unset

Example:

```yaml
exercise_library:
  - id: squat
    name: "Squat"
    default_sets: 3
    default_reps: 8
    cues: "Chest up"

# In your workout:
exercises:
  - exercise_ref: squat
    target_sets: 5        # Uses 5 (overridden)
    target_load: "100kg"  # Uses "100kg" (new field)
    # target_reps: 8      # Uses 8 (from library)
    # cues: "Chest up"    # Uses "Chest up" (from library)
```

## Backward Compatibility

### PWF v1 Plans

- `plan_version: 1` continues to work as before
- `exercise_library` is ignored with a warning
- `exercise_ref` is ignored with a warning
- `modality` remains required

### Migration to v2

To migrate from v1 to v2:

1. Change `plan_version` to `2`
2. Optionally create an `exercise_library` section
3. Replace repeated exercises with `exercise_ref`
4. Remove `modality` from exercises using `exercise_ref` (optional, defaults from library)

## Use Cases

### Programming Multiple Weeks

```yaml
exercise_library:
  - id: bench
    name: "Bench Press"
    modality: strength
    default_sets: 3
    default_reps: 10

cycle:
  days:
    - focus: "Week 1"
      exercises:
        - exercise_ref: bench
          target_load: "60kg"

    - focus: "Week 2"
      exercises:
        - exercise_ref: bench
          target_load: "65kg"

    - focus: "Week 3"
      exercises:
        - exercise_ref: bench
          target_load: "70kg"
```

### Consistent Exercise Database

Build a personal exercise database once, reuse across multiple plans:

```yaml
exercise_library:
  - id: squat-high-bar
    name: "High Bar Back Squat"
    # ... full definition

  - id: squat-low-bar
    name: "Low Bar Back Squat"
    # ... full definition

  - id: front-squat
    name: "Front Squat"
    # ... full definition
```

### Template Plans

Create base templates with exercises, then customize for each athlete:

```yaml
# base-strength-template.yaml
exercise_library:
  # ... extensive library

cycle:
  days:
    - focus: "Lower"
      exercises:
        - exercise_ref: squat
        - exercise_ref: deadlift
    - focus: "Upper"
      exercises:
        - exercise_ref: bench
        - exercise_ref: row
```

## Error Codes

| Code | Description |
|------|-------------|
| PWF-P030 | Exercise missing both `modality` and `exercise_ref` (v2 only) |
| PWF-P031 | Both `modality` and `exercise_ref` specified (warning) |
| PWF-P032 | `exercise_ref` references non-existent library exercise |
| PWF-P033 | Exercise library exceeds 500 entries |
| PWF-P034 | Duplicate exercise library ID |
| PWF-P035 | Invalid library exercise ID format |
| PWF-P036 | Library exercise name length invalid |
| PWF-P037 | Library exercise description too long |

## Examples

See the `examples/` directory for complete examples:

- `exercise-library-v2.yaml`: Comprehensive example with library usage
- `library-reference-minimal.yaml`: Minimal example showing basic usage
