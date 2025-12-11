# Day Block

The `day` block defines a single training day containing one or more exercises.

## Example Usage

```yaml
days:
  - id: "day-a"
    order: 0
    focus: "Full Body A"
    notes: "Compound movements focus. Rest 2-3 min between sets."
    scheduled_date: "2025-01-06"
    target_session_length_min: 60
    exercises:
      - name: "Barbell Back Squat"
        modality: strength
        target_sets: 3
        target_reps: 5

      - name: "Bench Press"
        modality: strength
        target_sets: 3
        target_reps: 5

      - name: "Plank"
        modality: countdown
        target_duration_sec: 60
```

## Argument Reference

### `id`

- **Type:** `string`
- **Required:** No
- **Default:** Auto-generated UUID

A unique identifier for this day within the plan. Useful for tracking completion and referencing specific days.

```yaml
- id: "push-day-1"
  focus: "Push"
```

---

### `order`

- **Type:** `integer`
- **Required:** No
- **Default:** Array index (0-based)
- **Constraint:** Must be unique, non-negative

The sequence position of this day in the cycle. Days are sorted by `order` when displaying the plan.

```yaml
days:
  - order: 0
    focus: "Day A"
  - order: 1
    focus: "Day B"
  - order: 2
    focus: "Day C"
```

> **Note:** If `order` is not specified, days are executed in array order.

---

### `focus`

- **Type:** `string`
- **Required:** No

The training focus or theme for this day. Commonly used values:

- Muscle groups: `"Upper Body"`, `"Lower Body"`, `"Push"`, `"Pull"`, `"Legs"`
- Session types: `"Strength"`, `"Conditioning"`, `"Recovery"`
- Day names: `"Day A"`, `"Day B"`, `"Monday"`

```yaml
- focus: "Upper Body - Push"
```

---

### `notes`

- **Type:** `string`
- **Required:** No

Coaching notes specific to this training day.

```yaml
- focus: "Heavy Day"
  notes: "Work up to a heavy triple. Take extra rest between sets."
```

---

### `scheduled_date`

- **Type:** `string` (ISO 8601 date)
- **Required:** No
- **Format:** `YYYY-MM-DD`

The planned date to perform this workout. Used for calendar scheduling.

```yaml
- scheduled_date: "2025-01-06"
  focus: "Day 1"
```

---

### `target_session_length_min`

- **Type:** `integer`
- **Required:** No
- **Unit:** Minutes

The expected duration of this training session. Useful for time-constrained athletes.

```yaml
- focus: "Quick Workout"
  target_session_length_min: 30
```

---

### `exercises`

- **Type:** `array` of [Exercise Block](exercise.md)
- **Required:** Yes
- **Min Items:** 1

The exercises to perform in this training day, in order.

```yaml
- focus: "Full Body"
  exercises:
    - name: "Squat"
      modality: strength
      target_sets: 3
      target_reps: 5
    - name: "Press"
      modality: strength
      target_sets: 3
      target_reps: 5
```

See [Exercise Block](exercise.md) for all available arguments.

## Validation Rules

| Rule | Severity | Message |
|------|----------|---------|
| Empty `exercises` | Error | `Day must have at least 1 exercise` |
| Duplicate `order` | Error | `Duplicate day order: {n}` |
| Negative `order` | Error | `order must be non-negative` |
| Missing `focus` | Warning | `Day has no focus - will display as "Day {n}"` |

## Nested Blocks

The `day` block contains the following nested blocks:

- [Exercise Block](exercise.md) - One or more exercises

## Day Status (Runtime)

When a plan is activated, days track their completion status:

| Status | Description |
|--------|-------------|
| `pending` | Not yet started |
| `completed` | Finished successfully |
| `skipped` | User chose to skip |
| `rescheduled` | Moved to a different date |

> **Note:** Status is tracked at runtime, not in the YAML specification.
