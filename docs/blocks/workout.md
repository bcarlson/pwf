# Workout Block

The `workout` block represents a single completed workout session.

## Example Usage

```yaml
workouts:
  - id: "workout-2025-01-15-001"
    date: "2025-01-15"
    started_at: "2025-01-15T09:00:00Z"
    ended_at: "2025-01-15T10:15:00Z"
    duration_sec: 4500
    title: "Push Day - Week 3"
    notes: "Felt strong today. Increased bench by 5kg."
    plan_id: "beginner-strength-v1"
    plan_day_id: "day-a"
    exercises:
      - name: "Bench Press"
        sets:
          - reps: 5
            weight_kg: 100
```

## Argument Reference

### `id`

- **Type:** `string`
- **Required:** No

A unique identifier for this workout session. Useful for cross-referencing with personal records.

```yaml
- id: "workout-2025-01-15-001"
```

---

### `date`

- **Type:** `string` (ISO 8601 date)
- **Required:** Yes
- **Format:** `YYYY-MM-DD`

The date the workout was performed.

```yaml
- date: "2025-01-15"
```

---

### `started_at`

- **Type:** `string` (ISO 8601 datetime)
- **Required:** No
- **Format:** `YYYY-MM-DDTHH:MM:SSZ`

The exact time the workout started.

```yaml
- started_at: "2025-01-15T09:00:00Z"
```

---

### `ended_at`

- **Type:** `string` (ISO 8601 datetime)
- **Required:** No

The exact time the workout ended.

```yaml
- ended_at: "2025-01-15T10:15:00Z"
```

---

### `duration_sec`

- **Type:** `integer`
- **Required:** No
- **Unit:** Seconds

Total workout duration. Can be calculated from `started_at`/`ended_at` or recorded separately.

```yaml
- duration_sec: 4500  # 75 minutes
```

---

### `title`

- **Type:** `string`
- **Required:** No

A descriptive title for the workout session.

```yaml
- title: "Push Day - Heavy"
```

---

### `notes`

- **Type:** `string`
- **Required:** No

Notes about the workout (how it felt, observations, etc.).

```yaml
- notes: "Felt tired from poor sleep. Kept weights moderate."
```

---

### `plan_id`

- **Type:** `string`
- **Required:** No

Reference to the WPS plan this workout was based on.

```yaml
- plan_id: "beginner-strength-v1"
```

---

### `plan_day_id`

- **Type:** `string`
- **Required:** No

Reference to the specific day within the plan.

```yaml
- plan_day_id: "day-a"
```

---

### `exercises`

- **Type:** `array` of [CompletedExercise Block](#completed-exercise-block)
- **Required:** Yes

The exercises performed in this workout.

```yaml
- exercises:
    - name: "Squat"
      sets: [...]
```

---

## Completed Exercise Block

A completed exercise within a workout.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | `string` | No | Unique identifier |
| `name` | `string` | **Yes** | Exercise name |
| `modality` | `string` | No | Exercise type (strength, countdown, etc.) |
| `notes` | `string` | No | Exercise-level notes |
| `sets` | `array` of [Set](#completed-set-block) | **Yes** | Completed sets |

### Example

```yaml
exercises:
  - name: "Barbell Squat"
    modality: strength
    notes: "Felt good depth today"
    sets:
      - reps: 5
        weight_kg: 140
```

---

## Completed Set Block

A single completed set.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `set_number` | `integer` | No | Set order (1-indexed) |
| `set_type` | `string` | No | Type of set (see below) |
| `reps` | `integer` | Conditional | Repetitions completed |
| `weight_kg` | `number` | Conditional | Weight in kilograms |
| `weight_lb` | `number` | Conditional | Weight in pounds |
| `duration_sec` | `integer` | Conditional | Duration in seconds |
| `distance_meters` | `number` | Conditional | Distance in meters |
| `rpe` | `number` | No | Rate of Perceived Exertion (0-10) |
| `notes` | `string` | No | Set-level notes |
| `is_pr` | `boolean` | No | Whether this set was a PR |
| `completed_at` | `string` | No | ISO 8601 datetime |

### Set Types

| Type | Description |
|------|-------------|
| `working` | Standard working set (default) |
| `warmup` | Warm-up set |
| `dropset` | Drop set (reduced weight) |
| `failure` | Set to failure |
| `amrap` | As Many Reps As Possible |

### Examples

```yaml
# Strength set
sets:
  - set_number: 1
    set_type: warmup
    reps: 10
    weight_kg: 60

  - set_number: 2
    set_type: working
    reps: 5
    weight_kg: 100
    rpe: 8

  - set_number: 3
    set_type: working
    reps: 5
    weight_kg: 100
    rpe: 8.5
    is_pr: true
    notes: "New 5RM!"
```

```yaml
# Timed set (countdown/stopwatch)
sets:
  - duration_sec: 60
    notes: "Held full plank"

  - duration_sec: 45
    notes: "Dropped to knees at 45s"
```

```yaml
# Distance-based set (interval)
sets:
  - set_number: 1
    distance_meters: 400
    duration_sec: 85
    notes: "Felt fast"

  - set_number: 2
    distance_meters: 400
    duration_sec: 90
```

---

## Validation Rules

| Rule | Severity | Message |
|------|----------|---------|
| Missing `date` | Error | `Workout date is required` |
| Empty `exercises` | Warning | `Workout has no exercises` |
| Missing `exercise.name` | Error | `Exercise name is required` |
| Empty `sets` | Warning | `Exercise has no recorded sets` |
| Set with no metrics | Warning | `Set has no recorded metrics` |
| RPE out of range | Warning | `RPE should be between 0 and 10` |

---

## Full Workout Example

```yaml
workouts:
  - id: "w-20250115-001"
    date: "2025-01-15"
    started_at: "2025-01-15T09:00:00Z"
    ended_at: "2025-01-15T10:15:00Z"
    duration_sec: 4500
    title: "Push Day - Week 3"
    notes: "Great session. Hit new bench PR!"
    plan_id: "beginner-strength-v1"
    plan_day_id: "day-a"

    exercises:
      - name: "Bench Press"
        modality: strength
        sets:
          - set_type: warmup
            reps: 10
            weight_kg: 40
          - set_type: warmup
            reps: 5
            weight_kg: 60
          - set_type: warmup
            reps: 3
            weight_kg: 80
          - set_type: working
            reps: 5
            weight_kg: 100
            rpe: 7.5
          - set_type: working
            reps: 5
            weight_kg: 100
            rpe: 8
          - set_type: working
            reps: 5
            weight_kg: 100
            rpe: 9
            is_pr: true

      - name: "Overhead Press"
        modality: strength
        sets:
          - reps: 5
            weight_kg: 50
            rpe: 7
          - reps: 5
            weight_kg: 50
            rpe: 7.5
          - reps: 5
            weight_kg: 50
            rpe: 8

      - name: "Plank"
        modality: countdown
        sets:
          - duration_sec: 60
          - duration_sec: 55
          - duration_sec: 45
            notes: "Core fatigued"
```
