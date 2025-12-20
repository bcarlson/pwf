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

Reference to the PWF plan this workout was based on.

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

### `telemetry`

- **Type:** [WorkoutTelemetry Block](#workout-telemetry-block)
- **Required:** No
- **Version:** PWF v2

Workout-level telemetry metrics such as heart rate, power, distance, elevation, and environmental data.

```yaml
- telemetry:
    heart_rate_avg: 155
    heart_rate_max: 182
    total_distance_km: 45.2
    total_elevation_gain_m: 250
    total_calories: 680
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
| `rpe` | `number` | No | Rate of Perceived Exertion (0-10 scale) |
| `rir` | `integer` | No | Reps in Reserve (how many more reps possible) |
| `notes` | `string` | No | Set-level notes |
| `is_pr` | `boolean` | No | Whether this set was a PR |
| `completed_at` | `string` | No | ISO 8601 datetime |
| `telemetry` | [SetTelemetry](#set-telemetry-block) | No | Set-level telemetry metrics (v2) |

### Set Types

| Type | Description |
|------|-------------|
| `working` | Standard working set (default) |
| `warmup` | Warm-up set |
| `dropset` | Drop set (reduced weight) |
| `failure` | Set to failure |
| `amrap` | As Many Reps As Possible |

### RPE vs RIR

RPE (Rate of Perceived Exertion) and RIR (Reps in Reserve) are two methods for quantifying set difficulty:

- **RPE (0-10 scale)**: How hard the set felt overall. 10 = maximum effort, unable to do more reps.
- **RIR (integer)**: How many more reps you could have performed with good form. RIR 2 = could have done 2 more reps.

These metrics are inversely related: RPE 8 ≈ RIR 2, RPE 9 ≈ RIR 1, RPE 10 ≈ RIR 0.

**Best Practice**: Use one metric or the other, not both on the same set. PWF will warn if both are present.

### Examples

```yaml
# Strength set with RPE
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
# Strength set with RIR (alternative to RPE)
sets:
  - set_number: 1
    set_type: working
    reps: 8
    weight_kg: 80
    rir: 3
    notes: "Easy warmup, 3 reps in reserve"

  - set_number: 2
    set_type: working
    reps: 8
    weight_kg: 90
    rir: 1
    notes: "Hard set, only 1 rep left"
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

| Rule | Severity | Error Code | Message |
|------|----------|------------|---------|
| Missing `date` | Error | PWF-H101 | `Workout date is required` |
| Empty `exercises` | Warning | PWF-H102 | `Workout has no exercises` |
| Missing `exercise.name` | Error | PWF-H201 | `Exercise name is required` |
| Empty `sets` | Warning | PWF-H202 | `Exercise has no recorded sets` |
| Set with no metrics | Warning | PWF-H301 | `Set has no recorded metrics` |
| RPE out of range (0-10) | Warning | PWF-H302 | `RPE should be between 0 and 10` |
| RIR > 10 | Warning | PWF-H303 | `RIR typically ranges 0-10` |
| Both RPE and RIR set | Warning | PWF-H304 | `Both RPE and RIR are set. Typically only one should be used.` |

---

## Set Telemetry Block

Telemetry metrics for individual sets (PWF v2).

| Field | Type | Description |
|-------|------|-------------|
| `heart_rate_avg` | `integer` | Average heart rate (bpm) |
| `heart_rate_max` | `integer` | Maximum heart rate (bpm) |
| `heart_rate_min` | `integer` | Minimum heart rate (bpm) |
| `power_avg` | `integer` | Average power (watts) |
| `power_max` | `integer` | Maximum power (watts) |
| `power_min` | `integer` | Minimum power (watts) |
| `elevation_gain_m` | `number` | Elevation gain in meters |
| `elevation_gain_ft` | `number` | Elevation gain in feet |
| `elevation_loss_m` | `number` | Elevation loss in meters |
| `elevation_loss_ft` | `number` | Elevation loss in feet |
| `speed_avg_mps` | `number` | Average speed in m/s |
| `speed_avg_kph` | `number` | Average speed in km/h |
| `speed_avg_mph` | `number` | Average speed in mph |
| `speed_max_mps` | `number` | Maximum speed in m/s |
| `speed_max_kph` | `number` | Maximum speed in km/h |
| `speed_max_mph` | `number` | Maximum speed in mph |
| `pace_avg_sec_per_km` | `integer` | Average pace (sec/km) |
| `pace_avg_sec_per_mi` | `integer` | Average pace (sec/mi) |
| `cadence_avg` | `integer` | Average cadence (steps/min or RPM) |
| `cadence_max` | `integer` | Maximum cadence |
| `temperature_c` | `number` | Temperature in Celsius |
| `temperature_f` | `number` | Temperature in Fahrenheit |
| `humidity_percent` | `number` | Humidity percentage (0-100) |
| `calories` | `integer` | Calories burned |
| `stroke_rate` | `integer` | Stroke rate (swimming/rowing) |
| `gps_route_id` | `string` | Reference to GPS route data |

### Example

```yaml
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
```

---

## Workout Telemetry Block

Telemetry metrics for entire workout sessions (PWF v2).

| Field | Type | Description |
|-------|------|-------------|
| `heart_rate_avg` | `integer` | Average heart rate (bpm) |
| `heart_rate_max` | `integer` | Maximum heart rate (bpm) |
| `heart_rate_min` | `integer` | Minimum heart rate (bpm) |
| `power_avg` | `integer` | Average power (watts) |
| `power_max` | `integer` | Maximum power (watts) |
| `total_distance_m` | `number` | Total distance in meters |
| `total_distance_km` | `number` | Total distance in kilometers |
| `total_distance_mi` | `number` | Total distance in miles |
| `total_elevation_gain_m` | `number` | Total elevation gain in meters |
| `total_elevation_gain_ft` | `number` | Total elevation gain in feet |
| `total_elevation_loss_m` | `number` | Total elevation loss in meters |
| `total_elevation_loss_ft` | `number` | Total elevation loss in feet |
| `speed_avg_kph` | `number` | Average speed in km/h |
| `speed_avg_mph` | `number` | Average speed in mph |
| `speed_max_kph` | `number` | Maximum speed in km/h |
| `speed_max_mph` | `number` | Maximum speed in mph |
| `pace_avg_sec_per_km` | `integer` | Average pace (sec/km) |
| `pace_avg_sec_per_mi` | `integer` | Average pace (sec/mi) |
| `cadence_avg` | `integer` | Average cadence |
| `temperature_c` | `number` | Temperature in Celsius |
| `temperature_f` | `number` | Temperature in Fahrenheit |
| `humidity_percent` | `number` | Humidity percentage (0-100) |
| `total_calories` | `integer` | Total calories burned |
| `gps_route_id` | `string` | Reference to GPS route data |

### Example

```yaml
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
```

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

## Full Workout with Telemetry Example (PWF v2)

```yaml
workouts:
  - id: "run-20251220-morning"
    date: "2025-12-20"
    started_at: "2025-12-20T06:30:00Z"
    ended_at: "2025-12-20T07:25:00Z"
    duration_sec: 3300
    title: "Morning Trail Run"

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
            notes: "Outbound - uphill section"
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

          - duration_sec: 1320
            distance_meters: 5440
            notes: "Return - mostly downhill"
            telemetry:
              heart_rate_avg: 150
              heart_rate_max: 172
              elevation_loss_m: 213
              speed_avg_kph: 14.3
              pace_avg_sec_per_km: 251
              cadence_avg: 176
              temperature_c: 15.6
              humidity_percent: 68
              calories: 360
```
