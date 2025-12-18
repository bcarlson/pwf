# History (Root Block)

The history root block defines a workout history export containing completed workouts, personal records, and body measurements.

## Example Usage

```yaml
history_version: 1
exported_at: "2025-01-15T10:30:00Z"

export_source:
  app_name: "OwnLift"
  app_version: "1.0.0"

units:
  weight: kg
  distance: meters

workouts:
  - date: "2025-01-15"
    title: "Push Day"
    exercises:
      - name: "Bench Press"
        sets:
          - reps: 5
            weight_kg: 100

personal_records:
  - exercise_name: "Bench Press"
    record_type: max_weight
    value: 100
    achieved_at: "2025-01-15"

body_measurements:
  - date: "2025-01-15"
    weight_kg: 85.5
```

## Argument Reference

### `history_version`

- **Type:** `integer`
- **Required:** Yes
- **Valid Values:** `1`

The specification version this history export conforms to.

```yaml
history_version: 1
```

---

### `exported_at`

- **Type:** `string` (ISO 8601 datetime)
- **Required:** Yes
- **Format:** `YYYY-MM-DDTHH:MM:SSZ`

The timestamp when this export was created.

```yaml
exported_at: "2025-01-15T10:30:00Z"
```

---

### `export_source`

- **Type:** [ExportSource Block](#export-source-block)
- **Required:** No

Information about the application that created this export.

```yaml
export_source:
  app_name: "OwnLift"
  app_version: "1.0.0"
  platform: "ios"
```

---

### `units`

- **Type:** [Units Block](#units-block)
- **Required:** No
- **Default:** `{ weight: kg, distance: meters }`

Default units used throughout the export. Individual values can override these.

```yaml
units:
  weight: kg
  distance: meters
```

---

### `workouts`

- **Type:** `array` of [Workout Block](workout.md)
- **Required:** Yes

The completed workout sessions.

```yaml
workouts:
  - date: "2025-01-15"
    exercises: [...]
```

---

### `personal_records`

- **Type:** `array` of [PersonalRecord Block](#personal-record-block)
- **Required:** No
- **Default:** `[]`

Personal records achieved during the export period.

```yaml
personal_records:
  - exercise_name: "Squat"
    record_type: max_weight
    value: 150
    achieved_at: "2025-01-15"
```

---

### `body_measurements`

- **Type:** `array` of [BodyMeasurement Block](#body-measurement-block)
- **Required:** No
- **Default:** `[]`

Body measurements recorded during the export period.

```yaml
body_measurements:
  - date: "2025-01-15"
    weight_kg: 85.5
```

---

## Export Source Block

Information about the exporting application.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_name` | `string` | No | Application name |
| `app_version` | `string` | No | Application version |
| `platform` | `string` | No | Platform (ios, android, web, desktop) |
| `preferred_units` | [Units Block](#units-block) | No | User's preferred display units |

### Example

```yaml
export_source:
  app_name: "OwnLift"
  app_version: "1.0.0"
  platform: "ios"
  preferred_units:
    weight: kg
    distance: meters
```

**Note on preferred_units**: This field indicates the user's preferred display units. PWF will validate that the preferred units match the actual data in workouts. For example, if `preferred_units.weight` is `lb` but all workout sets use `weight_kg`, a warning (PWF-H601) will be issued.



---

## Units Block

Default units for the export.

| Field | Type | Default | Valid Values |
|-------|------|---------|--------------|
| `weight` | `string` | `kg` | `kg`, `lb` |
| `distance` | `string` | `meters` | `meters`, `kilometers`, `miles`, `feet`, `yards` |

---

## Personal Record Block

A personal record achievement.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `exercise_name` | `string` | **Yes** | Exercise name |
| `record_type` | `string` | **Yes** | Type of record (see below) |
| `value` | `number` | **Yes** | Record value |
| `unit` | `string` | No | Unit for the value |
| `achieved_at` | `string` | **Yes** | ISO 8601 date |
| `workout_id` | `string` | No | Reference to workout |
| `notes` | `string` | No | Additional notes |

### Record Types

| Type | Description | Recommended Unit |
|------|-------------|------------------|
| `1rm` | Estimated one-rep max | kg or lb |
| `max_weight_3rm` | Heaviest weight for 3 reps | kg or lb |
| `max_weight_5rm` | Heaviest weight for 5 reps | kg or lb |
| `max_weight_8rm` | Heaviest weight for 8 reps | kg or lb |
| `max_weight_10rm` | Heaviest weight for 10 reps | kg or lb |
| `max_weight` | Heaviest weight lifted (any rep count) | kg or lb |
| `max_reps` | Most reps at a weight | - |
| `max_volume` | Highest total volume | kg or lb |
| `max_duration` | Longest duration | seconds |
| `max_distance` | Longest distance | meters, km, miles |
| `fastest_time` | Fastest completion time | seconds |

**Note**: Weight-based record types (`1rm`, `max_weight_*`) should include a `unit` field. PWF will warn if the unit is missing.

---

## Body Measurement Block

A body measurement entry.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `date` | `string` | **Yes** | ISO 8601 date |
| `recorded_at` | `string` | No | ISO 8601 datetime |
| `weight_kg` | `number` | No | Body weight in kg |
| `weight_lb` | `number` | No | Body weight in lb |
| `body_fat_percent` | `number` | No | Body fat percentage |
| `notes` | `string` | No | Additional notes |
| `measurements` | [BodyDimensions](#body-dimensions) | No | Body dimensions |

### Body Dimensions

All measurements in centimeters.

| Field | Description |
|-------|-------------|
| `neck_cm` | Neck circumference |
| `shoulders_cm` | Shoulder width |
| `chest_cm` | Chest circumference |
| `waist_cm` | Waist circumference |
| `hips_cm` | Hip circumference |
| `bicep_left_cm` | Left bicep |
| `bicep_right_cm` | Right bicep |
| `forearm_left_cm` | Left forearm |
| `forearm_right_cm` | Right forearm |
| `thigh_left_cm` | Left thigh |
| `thigh_right_cm` | Right thigh |
| `calf_left_cm` | Left calf |
| `calf_right_cm` | Right calf |

---

## Validation Rules

| Rule | Severity | Error Code | Message |
|------|----------|------------|---------|
| Missing `history_version` | Error | - | `history_version is required` |
| `history_version` not 1 | Error | PWF-H001 | `Unsupported history_version` |
| Missing `exported_at` | Error | PWF-H002 | `exported_at is required` |
| Missing PR exercise name | Error | PWF-H401 | `Personal record must have exercise_name` |
| Missing PR date | Error | PWF-H402 | `Personal record must have achieved_at date` |
| Weight-based PR missing unit | Warning | PWF-H403 | `Weight-based personal records should specify a unit` |
| Missing body measurement date | Error | PWF-H501 | `Body measurement must have date` |
| Body measurement with no values | Warning | PWF-H502 | `Body measurement entry has no recorded values` |
| Preferred units mismatch | Warning | PWF-H601 | `Preferred weight unit doesn't match actual data` |

See [workout.md](workout.md#validation-rules) for additional workout and set validation rules.

## Minimal Valid History

```yaml
history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    exercises:
      - name: "Push-ups"
        sets:
          - reps: 10
```
