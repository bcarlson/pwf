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

Loading guidance for the exercise. Can be absolute weight, RPE, or percentage.

```yaml
# Absolute weight
- name: "Squat"
  target_load: "225 lbs"

# RPE-based
- name: "Deadlift"
  target_load: "RPE 8"

# Percentage (future: may be parsed)
- name: "Bench Press"
  target_load: "70% 1RM"
```

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

## Validation Rules

| Rule | Severity | Message |
|------|----------|---------|
| Invalid `modality` | Error | `Invalid modality: {value}` |
| Missing `name` | Warning | `Missing exercise name` |
| HTTP `link` | Warning | `HTTP URLs not allowed, use HTTPS` |
| Invalid URL format | Error | `Invalid URL format` |
| `strength` without sets/reps | Warning | `Strength exercise missing target_sets/target_reps` |
| `countdown` without duration | Warning | `Countdown exercise missing target_duration_sec` |
| `interval` without sets | Warning | `Interval exercise missing target_sets` |

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
