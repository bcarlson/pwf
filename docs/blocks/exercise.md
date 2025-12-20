# Exercise Block

The `exercise` block defines a single exercise within a training day.

## Example Usage

```yaml
exercises:
  - id: "squat-main"
    name: "Barbell Back Squat"
    modality: strength
    target_sets: 5
    target_reps: 5
    target_load: "225 lbs"
    target_notes: "Depth: hip crease below knee. Brace core."
    link: "https://example.com/squat-tutorial"
```

## Argument Reference

### `id`

- **Type:** `string`
- **Required:** No
- **Default:** Auto-generated UUID

A unique identifier for this exercise within the plan.

```yaml
- id: "bench-press-main"
  name: "Bench Press"
```

---

### `name`

- **Type:** `string`
- **Required:** No
- **Default:** `"Exercise {n}"`

The display name of the exercise.

```yaml
- name: "Romanian Deadlift"
  modality: strength
```

> **Warning:** While technically optional, omitting `name` triggers a validation warning.

---

### `modality`

- **Type:** `string` (enum)
- **Required:** Yes
- **Valid Values:** `strength`, `countdown`, `stopwatch`, `interval`

The type of exercise, which determines how it's tracked and displayed.

```yaml
- name: "Squat"
  modality: strength    # Sets × reps

- name: "Plank"
  modality: countdown   # Fixed timer

- name: "Stretching"
  modality: stopwatch   # Open-ended

- name: "Sprints"
  modality: interval    # Repeating work
```

See [Modalities](../modalities.md) for detailed documentation of each type.

---

### `target_sets`

- **Type:** `integer`
- **Required:** Conditional (recommended for `strength` and `interval`)
- **Min Value:** 1

The target number of sets to perform.

```yaml
- name: "Bench Press"
  modality: strength
  target_sets: 4
  target_reps: 8
```

---

### `target_reps`

- **Type:** `integer`
- **Required:** Conditional (recommended for `strength`)
- **Min Value:** 1

The target number of repetitions per set.

```yaml
- name: "Pull-ups"
  modality: strength
  target_sets: 3
  target_reps: 10
```

---

### `target_duration_sec`

- **Type:** `integer`
- **Required:** Conditional (recommended for `countdown`, `stopwatch`, `interval`)
- **Min Value:** 1
- **Unit:** Seconds

The target duration in seconds.

```yaml
- name: "Plank Hold"
  modality: countdown
  target_duration_sec: 60    # 1 minute
```

---

### `target_distance_meters`

- **Type:** `number`
- **Required:** No
- **Min Value:** 0
- **Unit:** Meters

The target distance for distance-based exercises.

```yaml
- name: "400m Repeats"
  modality: interval
  target_sets: 8
  target_distance_meters: 400
  target_duration_sec: 90
```

---

### `target_load`

- **Type:** `string`
- **Required:** No
- **Mutually Exclusive With:** `target_weight_percent`

Loading guidance for the exercise as a freeform string. Can be absolute weight, RPE, or percentage notes.

```yaml
# Absolute weight
- name: "Squat"
  target_load: "225 lbs"

# RPE-based
- name: "Deadlift"
  target_load: "RPE 8"

# Freeform percentage note
- name: "Bench Press"
  target_load: "70% 1RM (estimated)"
```

> **Note:** For structured percentage-based loading, use `target_weight_percent` and `percent_of` instead.

---

### `target_weight_percent`

- **Type:** `number`
- **Required:** No (requires `percent_of` if used)
- **Range:** 0-200
- **Mutually Exclusive With:** `target_load`

Target weight as a percentage of a reference max (1RM, 3RM, 5RM, or 10RM).

```yaml
# Basic percentage-based loading
- name: "Squat"
  modality: strength
  target_sets: 5
  target_reps: 5
  target_weight_percent: 85
  percent_of: "1rm"

# Progressive overload example
- name: "Bench Press"
  target_weight_percent: 75  # Week 2 intensity
  percent_of: "1rm"
```

> **Validation:** Must be between 0 and 200. Requires `percent_of` to be set.

---

### `percent_of`

- **Type:** `string` (enum)
- **Required:** No (requires `target_weight_percent` if used)
- **Valid Values:** `1rm`, `3rm`, `5rm`, `10rm`

Specifies which reference max to use for percentage calculations.

```yaml
# Using 1RM (most common)
- name: "Squat"
  target_weight_percent: 85
  percent_of: "1rm"

# Using 3RM for volume work
- name: "Overhead Press"
  target_weight_percent: 90
  percent_of: "3rm"

# Using 5RM
- name: "Romanian Deadlift"
  target_weight_percent: 85
  percent_of: "5rm"

# Using 10RM for high-rep training
- name: "Barbell Row"
  target_weight_percent: 70
  percent_of: "10rm"
```

> **Validation:** Must be one of the valid enum values. Requires `target_weight_percent` to be set.

---

### `reference_exercise`

- **Type:** `string`
- **Required:** No
- **Requires:** `target_weight_percent` to be set

References another exercise's max for percentage calculations. Useful for accessory exercises based on main lifts.

```yaml
# Main lift
- name: "Barbell Back Squat"
  modality: strength
  target_sets: 5
  target_reps: 5

# Accessory using main lift's max
- name: "Front Squat"
  modality: strength
  target_sets: 3
  target_reps: 8
  target_weight_percent: 70
  percent_of: "1rm"
  reference_exercise: "Barbell Back Squat"
```

> **Validation:** If the referenced exercise name doesn't exist in the plan, a warning is issued. Matching is case-sensitive.

---

### `cues`

- **Type:** `string`
- **Required:** No
- **Alias:** Maps to `target_notes`

Form cues or coaching points. This is an alias for `target_notes` for backward compatibility.

```yaml
- name: "Squat"
  cues: "Chest up, knees out"
```

---

### `target_notes`

- **Type:** `string`
- **Required:** No

Coaching notes, form cues, or special instructions for the exercise.

```yaml
- name: "Romanian Deadlift"
  target_notes: "Soft knee bend. Feel hamstring stretch at bottom."
```

---

### `link`

- **Type:** `string` (URL)
- **Required:** No
- **Constraint:** Must be HTTPS

A URL to a tutorial video or reference for the exercise.

```yaml
- name: "Turkish Get-up"
  link: "https://youtube.com/watch?v=example"
```

> **Security:** HTTP URLs are rejected. Only HTTPS links are allowed.

---

### `image`

- **Type:** `string` (URL)
- **Required:** No
- **Constraint:** Must be HTTPS

A URL to a demonstration image for the exercise.

```yaml
- name: "Face Pull"
  image: "https://example.com/images/face-pull.png"
```

---

### `group`

- **Type:** `string`
- **Required:** No (must be paired with `group_type`)
- **Min Length:** 1
- **Max Length:** 50
- **Valid Characters:** Alphanumeric, hyphens (-), underscores (_)

A unique identifier for grouping exercises into supersets or circuits. Exercises with the same group identifier and group_type are performed together.

```yaml
# Superset example
- name: "Bench Press"
  modality: strength
  group: "A"
  group_type: superset

- name: "Bent Over Row"
  modality: strength
  group: "A"
  group_type: superset
```

> **Note:** `group` and `group_type` must be used together. Specifying one without the other will cause a validation error.

---

### `group_type`

- **Type:** `string` (enum)
- **Required:** No (must be paired with `group`)
- **Valid Values:** `superset`, `circuit`

Specifies the type of exercise grouping.

**`superset`**: Typically 2-3 exercises performed back-to-back with minimal rest between exercises. Common for antagonistic muscle pairings (e.g., chest + back).

**`circuit`**: A series of exercises (usually 3-6+) performed consecutively with minimal rest between exercises. Common for metabolic conditioning and fat loss training.

```yaml
# Circuit example
- name: "Squats"
  modality: strength
  group: "circuit1"
  group_type: circuit

- name: "Push-ups"
  modality: strength
  group: "circuit1"
  group_type: circuit

- name: "Burpees"
  modality: strength
  group: "circuit1"
  group_type: circuit
```

---

### `rest_between_sets_sec`

- **Type:** `integer`
- **Required:** No
- **Min Value:** 0
- **Unit:** Seconds

Rest period in seconds between sets of the same exercise. For grouped exercises, typically set to 0 to move immediately to the next exercise in the group.

```yaml
- name: "Bench Press"
  modality: strength
  group: "A"
  group_type: superset
  rest_between_sets_sec: 0  # Move immediately to next exercise in superset
```

---

### `rest_after_sec`

- **Type:** `integer`
- **Required:** No
- **Min Value:** 0
- **Unit:** Seconds

Rest period in seconds after completing all sets of this exercise (or after completing a full round of a group).

```yaml
- name: "Bent Over Row"
  modality: strength
  group: "A"
  group_type: superset
  rest_after_sec: 120  # Rest 2 minutes after completing the superset
```

## Validation Rules

| Rule | Severity | Message | Code |
|------|----------|---------|------|
| Invalid `modality` | Error | `Invalid modality: {value}` | - |
| Missing `name` | Warning | `Missing exercise name` | - |
| HTTP `link` | Warning | `HTTP URLs not allowed, use HTTPS` | - |
| Invalid URL format | Error | `Invalid URL format` | - |
| `strength` without sets/reps | Warning | `Strength exercise missing target_sets/target_reps` | - |
| `countdown` without duration | Warning | `Countdown exercise missing target_duration_sec` | - |
| `interval` without sets | Warning | `Interval exercise missing target_sets` | - |
| `target_weight_percent` without `percent_of` | Error | `target_weight_percent requires percent_of to be set` | PWF-P011 |
| `percent_of` without `target_weight_percent` | Error | `percent_of requires target_weight_percent to be set` | PWF-P012 |
| Both `target_weight_percent` and `target_load` | Error | `Cannot use both target_weight_percent and target_load - choose one` | PWF-P013 |
| `target_weight_percent` out of range | Error | `target_weight_percent must be between 0 and 200` | PWF-P014 |
| Invalid `percent_of` value | Error | `Invalid percent_of value. Must be one of: 1rm, 3rm, 5rm, 10rm` | PWF-P015 |
| `reference_exercise` not found | Warning | `reference_exercise does not match any exercise name in the plan` | PWF-P016 |
| `group` without `group_type` | Error | `group specified without group_type` | PWF-P017 |
| `group_type` without `group` | Error | `group_type specified without group` | PWF-P018 |
| Invalid `group` identifier | Error | `group identifier cannot be empty` | PWF-P019 |
| `group` identifier too long | Error | `group identifier exceeds 50 characters` | PWF-P019 |
| Invalid characters in `group` | Error | `group identifier must contain only alphanumeric characters, hyphens, or underscores` | PWF-P019 |

## Modality-Specific Examples

### Strength

```yaml
- name: "Barbell Row"
  modality: strength
  target_sets: 4
  target_reps: 8
  target_load: "135 lbs"
  target_notes: "Pull to lower chest. Squeeze shoulder blades."
```

### Countdown

```yaml
- name: "Wall Sit"
  modality: countdown
  target_duration_sec: 45
  target_notes: "Thighs parallel to ground. Back flat against wall."
```

### Stopwatch

```yaml
- name: "Mobility Flow"
  modality: stopwatch
  target_duration_sec: 300
  target_notes: "Move through each position slowly. Breathe."
```

### Interval

```yaml
- name: "Assault Bike"
  modality: interval
  target_sets: 10
  target_duration_sec: 30
  target_notes: "30 seconds max effort, 30 seconds rest"
```

## Grouping Examples

### Superset

Supersets pair two or more exercises performed back-to-back with minimal rest between exercises. Common patterns include antagonistic muscle groups (push/pull) or same muscle group exercises.

```yaml
exercises:
  # Superset A: Chest + Back
  - name: "Bench Press"
    modality: strength
    target_sets: 4
    target_reps: 8
    group: "A"
    group_type: superset
    rest_between_sets_sec: 0
    target_notes: "Move immediately to rows after completing set"

  - name: "Barbell Row"
    modality: strength
    target_sets: 4
    target_reps: 8
    group: "A"
    group_type: superset
    rest_after_sec: 120
    target_notes: "Rest 2 minutes after completing the superset"

  # Superset B: Shoulders + Arms
  - name: "Overhead Press"
    modality: strength
    target_sets: 3
    target_reps: 10
    group: "B"
    group_type: superset
    rest_between_sets_sec: 0

  - name: "Bicep Curls"
    modality: strength
    target_sets: 3
    target_reps: 12
    group: "B"
    group_type: superset
    rest_after_sec: 90
```

### Circuit

Circuits involve multiple exercises (typically 3-6+) performed consecutively. Common in metabolic conditioning, HIIT, and fat loss programs.

```yaml
exercises:
  # Full body circuit
  - name: "Goblet Squat"
    modality: strength
    target_sets: 3
    target_reps: 15
    group: "circuit1"
    group_type: circuit
    rest_between_sets_sec: 0

  - name: "Push-ups"
    modality: strength
    target_sets: 3
    target_reps: 15
    group: "circuit1"
    group_type: circuit
    rest_between_sets_sec: 0

  - name: "Kettlebell Swings"
    modality: strength
    target_sets: 3
    target_reps: 20
    group: "circuit1"
    group_type: circuit
    rest_between_sets_sec: 0

  - name: "Plank"
    modality: countdown
    target_sets: 3
    target_duration_sec: 45
    group: "circuit1"
    group_type: circuit
    rest_between_sets_sec: 0

  - name: "Jump Rope"
    modality: interval
    target_sets: 3
    target_duration_sec: 60
    group: "circuit1"
    group_type: circuit
    rest_after_sec: 120
    target_notes: "Rest 2 minutes after completing full circuit round"
```

### Mixed Grouping

You can have multiple groups in the same workout day:

```yaml
exercises:
  # Superset A
  - name: "Bench Press"
    group: "A"
    group_type: superset
  - name: "Row"
    group: "A"
    group_type: superset

  # Superset B
  - name: "Squat"
    group: "B"
    group_type: superset
  - name: "Leg Curl"
    group: "B"
    group_type: superset

  # Standalone exercise (no group)
  - name: "Core Work"
    modality: countdown
```
