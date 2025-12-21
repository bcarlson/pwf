# History (Root Block)

The history root block defines a workout history export containing completed workouts, personal records, and body measurements.

## Table of Contents

### Core Features
- [Example Usage](#example-usage)
- [Argument Reference](#argument-reference)
- [Export Source Block](#export-source-block)
- [Units Block](#units-block)
- [Personal Record Block](#personal-record-block)
- [Body Measurement Block](#body-measurement-block)

### PWF v2.1 Features
- [Sport Classification](#sport-classification-v21)
- [Swimming Features](#swimming-features-v21)
  - [Pool Configuration](#pool-configuration)
  - [Stroke Types](#stroke-types)
  - [Length-by-Length Tracking](#length-by-length-tracking)
  - [SWOLF Score](#swolf-score)
- [Multi-Sport Sessions](#multi-sport-sessions-v21)
  - [Sport Segments](#sport-segments)
  - [Transitions](#transitions)
- [Advanced Metrics](#advanced-metrics-v21)
  - [Training Effect](#training-effect)
  - [Training Status](#training-status)
  - [Lactate Threshold](#lactate-threshold)
- [Power Metrics](#power-metrics-v21)
  - [Key Power Metrics](#key-power-metrics)
  - [Pedal Dynamics](#pedal-dynamics)
- [Time in Zones](#time-in-zones-v21)
- [GPS Routes](#gps-routes-v21)
  - [GPS Positions](#gps-positions)
  - [GPS Fix Quality](#gps-fix-quality)
- [Device Tracking](#device-tracking-v21)
  - [Device Types](#device-types)
  - [Battery Information](#battery-information)
  - [Connection Information](#connection-information)
- [Time-Series Data](#time-series-data-v21)
  - [Columnar Storage](#columnar-storage)
  - [Sport-Specific Fields](#sport-specific-time-series-fields)
- [Migration from v2.0 to v2.1](#migration-from-v20-to-v21)
- [Validation Rules](#validation-rules)

---

**PWF v2.1** introduces comprehensive FIT format feature parity including:
- Sport classification and multi-sport sessions
- Swimming features (pool config, length tracking, SWOLF, stroke types)
- Advanced physiological metrics (Training Effect, VO2 Max, lactate threshold)
- Power-based cycling metrics (NP, TSS, IF, VI, pedal dynamics)
- GPS route tracking with full position data
- Device tracking and telemetry
- Time-series data for second-by-second metrics

## Example Usage

### Basic Strength Training (v1.x)

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

### Swimming with Length Tracking (v2.1)

```yaml
history_version: 2
exported_at: "2025-12-20T10:00:00Z"

workouts:
  - date: "2025-12-20"
    sport: swimming
    title: "Pool Swim"
    exercises:
      - name: "Freestyle"
        modality: swimming
        pool_config:
          pool_length: 25
          pool_length_unit: meters
        sets:
          - duration_sec: 120
            distance_meters: 100
            swimming:
              stroke_type: freestyle
              swolf_avg: 45
              lengths:
                - length_number: 1
                  stroke_type: freestyle
                  duration_sec: 30
                  stroke_count: 15
                  swolf: 45
```

### Multi-Sport Session (v2.1)

```yaml
history_version: 2
workouts:
  - date: "2025-12-20"
    title: "Sprint Triathlon"
    sport_segments:
      - segment_id: "swim"
        sport: swimming
        segment_index: 0
        duration_sec: 980
        distance_m: 750
        transition:
          transition_id: "T1"
          from_sport: swimming
          to_sport: cycling
          duration_sec: 95
      - segment_id: "bike"
        sport: cycling
        segment_index: 1
        duration_sec: 2640
        distance_m: 20000
```

## Argument Reference

### `history_version`

- **Type:** `integer`
- **Required:** Yes
- **Valid Values:** `1`, `2`

The specification version this history export conforms to.

- **Version 1**: Basic workout history, telemetry, and personal records
- **Version 2** (v2.1): Adds sport classification, swimming features, advanced metrics, GPS routes, device tracking, and multi-sport sessions

```yaml
history_version: 2
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

## PWF v2.1 Features

The following sections document features added in PWF v2.1 for comprehensive FIT format compatibility.

---

## Sport Classification (v2.1)

PWF v2.1 adds the `sport` field to classify workouts and exercises by activity type.

### Workout-Level Sport

The `sport` field can be set at the workout level to classify the primary activity:

```yaml
workouts:
  - date: "2025-12-20"
    sport: cycling
    title: "Interval Ride"
```

**Valid Sport Values:**

- `generic` - Default/unspecified sport
- `running` - Road/trail/track running
- `cycling` - Road/mountain/indoor cycling
- `swimming` - Pool or open water swimming
- `walking` - Walking/hiking
- `strength_training` - Weight lifting, resistance training
- `triathlon` - Multi-sport events (see [Multi-Sport Sessions](#multi-sport-sessions-v21))
- `rowing` - Indoor or outdoor rowing
- `yoga` - Yoga practice
- `pilates` - Pilates practice
- `other` - Other activities

### Exercise-Level Sport

Exercises can override the workout-level sport for mixed-modality sessions:

```yaml
workouts:
  - date: "2025-12-20"
    sport: strength_training
    exercises:
      - name: "Treadmill Run"
        sport: running  # Overrides workout sport
        modality: stopwatch
```

**Migration from v2.0**: The `sport` field is optional and backward compatible. Existing v2.0 exports will validate without modification.

---

## Swimming Features (v2.1)

PWF v2.1 adds comprehensive swimming support with pool configuration, length-by-length tracking, stroke types, and SWOLF efficiency scores.

### Pool Configuration

The `pool_config` field specifies the pool dimensions for swimming exercises:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pool_length` | `number` | **Yes** | Length of pool |
| `pool_length_unit` | `string` | No | `meters` (default) or `yards` |

```yaml
exercises:
  - name: "Freestyle Technique"
    modality: swimming
    pool_config:
      pool_length: 25
      pool_length_unit: meters
```

**Common Pool Sizes:**
- **25m**: Short course meters (SCM), Olympic standard
- **50m**: Long course meters (LCM), Olympic racing
- **25yd**: Short course yards (SCY), common in US

### Swimming Set Data

The `swimming` field within a `CompletedSet` contains swimming-specific metrics:

| Field | Type | Description |
|-------|------|-------------|
| `stroke_type` | `string` | Primary stroke for the set (see below) |
| `total_lengths` | `number` | Total number of pool lengths |
| `active_lengths` | `number` | Active swimming lengths (excludes rest) |
| `swolf_avg` | `number` | Average SWOLF score (lower is better) |
| `drill_mode` | `boolean` | Whether this was technique drill work |
| `lengths` | `array` | Length-by-length breakdown (see below) |

```yaml
sets:
  - duration_sec: 420
    distance_meters: 400
    swimming:
      stroke_type: freestyle
      total_lengths: 16
      active_lengths: 16
      swolf_avg: 45
      drill_mode: false
```

### Stroke Types

Valid values for `stroke_type`:

| Value | Description |
|-------|-------------|
| `freestyle` | Front crawl |
| `backstroke` | Backstroke |
| `breaststroke` | Breaststroke |
| `butterfly` | Butterfly |
| `drill` | Technique drills (not racing stroke) |
| `mixed` | Multiple strokes in same length |
| `im` | Individual Medley (all four strokes) |

### Length-by-Length Tracking

The `lengths` array provides detailed metrics for each pool length:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `length_number` | `number` | **Yes** | Length number (1-indexed) |
| `stroke_type` | `string` | **Yes** | Stroke used for this length |
| `duration_sec` | `number` | **Yes** | Time for this length (seconds) |
| `stroke_count` | `number` | No | Number of strokes taken |
| `swolf` | `number` | No | SWOLF score (duration + stroke_count) |
| `started_at` | `string` | No | ISO 8601 timestamp when length started |
| `active` | `boolean` | No | Whether this was an active length (vs. rest) |

```yaml
swimming:
  lengths:
    - length_number: 1
      stroke_type: freestyle
      duration_sec: 26
      stroke_count: 18
      swolf: 44
      started_at: "2025-12-20T06:05:00Z"
      active: true

    - length_number: 2
      stroke_type: freestyle
      duration_sec: 27
      stroke_count: 19
      swolf: 46
      active: true
```

### SWOLF Score

**SWOLF** (Swimming + Golf) is an efficiency metric: `duration_sec + stroke_count`

- **Lower is better** - indicates swimming efficiency
- Typical values: 35-45 for competitive swimmers, 45-60 for fitness swimmers
- Only valid when `stroke_count` is present
- If provided, PWF validates: `swolf == duration_sec + stroke_count`

```yaml
# SWOLF = 30s + 15 strokes = 45
- length_number: 1
  duration_sec: 30
  stroke_count: 15
  swolf: 45
```

### Active vs. Rest Lengths

The `active` flag distinguishes swimming lengths from rest at the wall:

```yaml
lengths:
  - length_number: 1
    stroke_type: backstroke
    duration_sec: 35
    active: false  # Drill/rest length
    notes: "Single arm drill"

  - length_number: 2
    stroke_type: backstroke
    duration_sec: 31
    active: true  # Full-effort length
```

### Individual Medley (IM)

For IM sets, use `stroke_type: im` at the set level, then specify each stroke per length:

```yaml
swimming:
  stroke_type: im
  lengths:
    # Butterfly leg (50m)
    - length_number: 1
      stroke_type: butterfly
      duration_sec: 28
    - length_number: 2
      stroke_type: butterfly
      duration_sec: 29

    # Backstroke leg (50m)
    - length_number: 3
      stroke_type: backstroke
      duration_sec: 25

    # ... breaststroke and freestyle
```

---

## Multi-Sport Sessions (v2.1)

PWF v2.1 supports multi-sport workouts (e.g., triathlon, duathlon, brick workouts) with sport segments and transitions.

### Sport Segments

When a workout contains `sport_segments`, it's treated as a multi-sport session:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `segment_id` | `string` | **Yes** | Unique identifier for segment |
| `sport` | `string` | **Yes** | Sport for this segment |
| `segment_index` | `number` | **Yes** | Sequence number (0-indexed) |
| `started_at` | `string` | No | ISO 8601 timestamp when segment started |
| `duration_sec` | `number` | No | Segment duration (seconds) |
| `distance_m` | `number` | No | Distance covered (meters) |
| `exercise_ids` | `array[string]` | No | IDs of exercises in this segment |
| `telemetry` | `object` | No | Segment-specific telemetry |
| `transition` | `object` | No | Transition after this segment |
| `notes` | `string` | No | Segment notes |

```yaml
workouts:
  - date: "2025-12-20"
    title: "Sprint Triathlon"

    sport_segments:
      - segment_id: "swim"
        sport: swimming
        segment_index: 0
        started_at: "2025-12-20T07:00:00Z"
        duration_sec: 980
        distance_m: 750
        exercise_ids: ["swim-1"]

        telemetry:
          heart_rate_avg: 145
          pace_avg_sec_per_km: 1307
```

### Transitions

Transitions track the changeover between sports (e.g., T1, T2 in triathlon):

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `transition_id` | `string` | **Yes** | Identifier (e.g., "T1", "T2") |
| `from_sport` | `string` | **Yes** | Sport transitioning from |
| `to_sport` | `string` | **Yes** | Sport transitioning to |
| `duration_sec` | `number` | No | Transition time (seconds) |
| `started_at` | `string` | No | ISO 8601 timestamp |
| `heart_rate_avg` | `number` | No | Average HR during transition |
| `notes` | `string` | No | Transition notes |

```yaml
sport_segments:
  - segment_id: "swim"
    sport: swimming
    # ... segment data

    transition:
      transition_id: "T1"
      from_sport: swimming
      to_sport: cycling
      duration_sec: 95
      started_at: "2025-12-20T07:16:20Z"
      heart_rate_avg: 138
      notes: "Wetsuit removal, helmet/shoes on"
```

### Triathlon Example

```yaml
workouts:
  - title: "Sprint Triathlon"
    sport_segments:
      # Swim (750m)
      - segment_id: "swim"
        sport: swimming
        segment_index: 0
        duration_sec: 980
        distance_m: 750
        transition:
          transition_id: "T1"
          from_sport: swimming
          to_sport: cycling
          duration_sec: 95

      # Bike (20km)
      - segment_id: "bike"
        sport: cycling
        segment_index: 1
        duration_sec: 2640
        distance_m: 20000
        transition:
          transition_id: "T2"
          from_sport: cycling
          to_sport: running
          duration_sec: 42

      # Run (5km)
      - segment_id: "run"
        sport: running
        segment_index: 2
        duration_sec: 1380
        distance_m: 5000
```

---

## Advanced Metrics (v2.1)

PWF v2.1 supports advanced physiological metrics primarily from Garmin/Firstbeat algorithms.

### Advanced Metrics Block

Add `advanced_metrics` to workout or segment telemetry:

| Field | Type | Range | Description |
|-------|------|-------|-------------|
| `training_effect` | `number` | 0.0 - 5.0 | Aerobic training stimulus |
| `anaerobic_training_effect` | `number` | 0.0 - 5.0 | Anaerobic training stimulus |
| `recovery_time_hours` | `number` | 0+ | Recommended recovery (hours) |
| `vo2_max_estimate` | `number` | 0+ | Estimated VO2 Max (ml/kg/min) |
| `performance_condition` | `number` | -20 to +20 | Real-time performance assessment |
| `training_load` | `number` | 0 - 1000+ | Cumulative training stress |
| `training_status` | `string` | - | Training status classification |
| `lactate_threshold` | `object` | - | Lactate threshold data |

```yaml
telemetry:
  advanced_metrics:
    training_effect: 3.8  # Highly improving
    anaerobic_training_effect: 2.1  # Maintaining
    recovery_time_hours: 36
    vo2_max_estimate: 52.3
    performance_condition: 5  # Feeling strong
    training_load: 285
    training_status: productive
```

### Training Effect

- **0.0 - 0.9**: None
- **1.0 - 1.9**: Minor
- **2.0 - 2.9**: Maintaining
- **3.0 - 3.9**: Improving
- **4.0 - 4.9**: Highly improving
- **5.0**: Overreaching

### Training Status

Valid values for `training_status`:

- `detraining` - Fitness declining
- `recovery` - Recovering from training
- `maintaining` - Maintaining current fitness
- `productive` - Building fitness effectively
- `peaking` - Reaching peak fitness
- `overreaching` - Risk of overtraining
- `unknown` - Cannot determine status

### Lactate Threshold

The `lactate_threshold` object tracks threshold metrics:

| Field | Type | Description |
|-------|------|-------------|
| `heart_rate_bpm` | `number` | HR at lactate threshold |
| `speed_mps` | `number` | Speed at threshold (m/s) |
| `power_watts` | `number` | Power at threshold (cycling) |
| `detected_at` | `string` | ISO 8601 timestamp |

```yaml
advanced_metrics:
  lactate_threshold:
    heart_rate_bpm: 168
    power_watts: 265
    detected_at: "2025-12-20T10:15:00Z"
```

---

## Power Metrics (v2.1)

PWF v2.1 supports power-based cycling metrics for training with power meters.

### Power Metrics Block

Add `power_metrics` to workout or segment telemetry:

| Field | Type | Description |
|-------|------|-------------|
| `normalized_power` | `number` | Normalized Power (NP) in watts |
| `training_stress_score` | `number` | Training Stress Score (TSS) |
| `intensity_factor` | `number` | Intensity Factor (IF) |
| `variability_index` | `number` | Variability Index (VI) |
| `ftp_watts` | `number` | Functional Threshold Power used |
| `total_work_kj` | `number` | Total work in kilojoules |
| `left_right_balance` | `number` | Left power balance (percentage) |
| `left_pedal_smoothness` | `number` | Left pedal smoothness (%) |
| `right_pedal_smoothness` | `number` | Right pedal smoothness (%) |
| `left_torque_effectiveness` | `number` | Left torque effectiveness (%) |
| `right_torque_effectiveness` | `number` | Right torque effectiveness (%) |

```yaml
telemetry:
  power_metrics:
    normalized_power: 268
    training_stress_score: 142.5
    intensity_factor: 0.89
    variability_index: 1.09
    ftp_watts: 300
    total_work_kj: 1985
```

### Key Power Metrics

**Normalized Power (NP)**:
- Weighted average power accounting for variability
- Better represents physiological cost than average power
- Typically higher than average power for variable efforts

**Training Stress Score (TSS)**:
- Quantifies training load from a workout
- Based on duration, intensity, and FTP
- 100 TSS = 1 hour at FTP

**Intensity Factor (IF)**:
- Ratio of NP to FTP: `IF = NP / FTP`
- 0.75-0.85: Endurance ride
- 0.85-0.95: Tempo ride
- 0.95-1.05: Threshold intervals
- 1.05+: VO2 Max intervals

**Variability Index (VI)**:
- Ratio of NP to average power: `VI = NP / Avg Power`
- 1.00-1.05: Very steady (TT, indoor trainer)
- 1.05-1.10: Moderately variable
- 1.10+: Highly variable (racing, group rides)

### Pedal Dynamics

Dual-sided power meters provide pedal metrics:

```yaml
power_metrics:
  left_right_balance: 51.2  # 51.2% left, 48.8% right
  left_pedal_smoothness: 28.5
  right_pedal_smoothness: 26.8
  left_torque_effectiveness: 92.3
  right_torque_effectiveness: 90.7
```

**Pedal Smoothness**: How smoothly power is applied (0-100%)
- Higher is smoother

**Torque Effectiveness**: How much of pedal stroke produces forward motion (0-100%)
- Higher is more efficient

---

## Time in Zones (v2.1)

PWF v2.1 tracks time spent in training zones for heart rate, power, and pace.

### Time in Zones Block

Add `time_in_zones` to workout or segment telemetry:

| Field | Type | Description |
|-------|------|-------------|
| `hr_zones_sec` | `array[number]` | Time in each HR zone (seconds) |
| `power_zones_sec` | `array[number]` | Time in each power zone (seconds) |
| `pace_zones_sec` | `array[number]` | Time in each pace zone (seconds) |
| `hr_zone_boundaries` | `array[number]` | HR zone boundaries (bpm) |
| `power_zone_boundaries` | `array[number]` | Power zone boundaries (watts) |
| `pace_zone_boundaries` | `array[number]` | Pace zone boundaries (sec/km) |

```yaml
telemetry:
  time_in_zones:
    # Time spent in each zone (seconds)
    hr_zones_sec: [180, 420, 1200, 2400, 3600, 300]
    power_zones_sec: [240, 600, 1500, 2700, 2400, 660]

    # Zone boundaries
    hr_zone_boundaries: [120, 140, 155, 165, 175, 185]
    power_zone_boundaries: [150, 210, 240, 270, 315, 360]
```

### Zone Arrays

- Zone arrays are 0-indexed: `[Z1, Z2, Z3, Z4, Z5, Z6]`
- Boundary arrays define upper limit of each zone
- Length of zone arrays must match number of zones
- All values in seconds for time-in-zone

### Heart Rate Zones (Typical)

| Zone | % Max HR | Training Focus |
|------|----------|----------------|
| Z1 | <60% | Recovery |
| Z2 | 60-70% | Endurance base |
| Z3 | 70-80% | Tempo |
| Z4 | 80-90% | Lactate threshold |
| Z5 | 90-95% | VO2 Max |
| Z6 | 95%+ | Anaerobic |

### Power Zones (Based on FTP)

| Zone | % FTP | Training Focus |
|------|-------|----------------|
| Z1 | <55% | Active recovery |
| Z2 | 55-75% | Endurance |
| Z3 | 75-90% | Tempo |
| Z4 | 90-105% | Lactate threshold |
| Z5 | 105-120% | VO2 Max |
| Z6 | 120%+ | Anaerobic capacity |

---

## GPS Routes (v2.1)

PWF v2.1 supports full GPS route tracking with position data, elevation, and per-point metrics.

### GPS Route Reference

Use `gps_route_id` in telemetry to reference a route:

```yaml
telemetry:
  gps_route_id: "route-20251220-ride"
```

### GPS Route Object

The `gps_route` object contains full route data:

| Field | Type | Description |
|-------|------|-------------|
| `route_id` | `string` | **Required** - Unique route identifier |
| `name` | `string` | Human-readable route name |
| `positions` | `array` | **Required** - GPS positions (see below) |
| `total_distance_m` | `number` | Total route distance (meters) |
| `total_ascent_m` | `number` | Total elevation gain (meters) |
| `total_descent_m` | `number` | Total elevation loss (meters) |
| `min_elevation_m` | `number` | Minimum elevation (meters) |
| `max_elevation_m` | `number` | Maximum elevation (meters) |
| `bbox_sw_lat` | `number` | Bounding box - SW latitude |
| `bbox_sw_lng` | `number` | Bounding box - SW longitude |
| `bbox_ne_lat` | `number` | Bounding box - NE latitude |
| `bbox_ne_lng` | `number` | Bounding box - NE longitude |
| `recording_mode` | `string` | GPS recording mode |
| `gps_fix` | `string` | GPS fix quality |

```yaml
telemetry:
  gps_route:
    route_id: "route-20251220-ride"
    name: "River Road Intervals"
    total_distance_m: 62543
    total_ascent_m: 422
    total_descent_m: 418
    min_elevation_m: 125
    max_elevation_m: 312
    recording_mode: "1s"
    gps_fix: fix_3d
    positions: [...]
```

### GPS Positions

Each position in the `positions` array:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `latitude_deg` | `number` | **Yes** | Latitude (WGS84 decimal degrees) |
| `longitude_deg` | `number` | **Yes** | Longitude (WGS84 decimal degrees) |
| `timestamp` | `string` | **Yes** | ISO 8601 timestamp |
| `elevation_m` | `number` | No | Elevation/altitude (meters) |
| `accuracy_m` | `number` | No | Horizontal accuracy (meters) |
| `speed_mps` | `number` | No | Speed (meters per second) |
| `heading_deg` | `number` | No | Heading/bearing (0-360°) |
| `heart_rate_bpm` | `number` | No | Heart rate at position |
| `power_watts` | `number` | No | Power at position |
| `cadence` | `number` | No | Cadence at position |
| `temperature_c` | `number` | No | Temperature at position (°C) |

```yaml
positions:
  - latitude_deg: 45.5230
    longitude_deg: -122.6765
    timestamp: "2025-12-20T09:00:00Z"
    elevation_m: 125
    accuracy_m: 3.2
    speed_mps: 6.5
    heading_deg: 92
    heart_rate_bpm: 165
    power_watts: 302
    cadence: 92
    temperature_c: 18.5
```

### GPS Fix Quality

Valid values for `gps_fix`:

- `none` - No GPS fix
- `fix_2d` - 2D fix (lat/lon only, no elevation)
- `fix_3d` - 3D fix (lat/lon/elevation)
- `dgps` - Differential GPS (enhanced accuracy)
- `unknown` - Unknown fix quality

### Recording Modes

Common values for `recording_mode`:

- `1s` - 1 Hz recording (1 point per second)
- `auto` - Smart recording (variable rate)
- `smart` - Smart recording (Garmin)
- `gps_only` - GPS only (no GLONASS/Galileo)

### Bounding Box

The bounding box defines the geographic extent of the route:

```yaml
# Southwest and northeast corners
bbox_sw_lat: 45.4982
bbox_sw_lng: -122.7215
bbox_ne_lat: 45.5892
bbox_ne_lng: -122.6105
```

Useful for map rendering and route previews.

---

## Device Tracking (v2.1)

PWF v2.1 tracks devices used during workouts (watches, sensors, power meters, etc.).

### Device Info

Add devices to the `devices` array in a workout:

| Field | Type | Description |
|-------|------|-------------|
| `device_index` | `number` | Device index for multi-device workouts |
| `device_type` | `string` | **Required** - Type of device |
| `manufacturer` | `string` | **Required** - Manufacturer name |
| `product` | `string` | Product/model name |
| `serial_number` | `string` | Device serial number |
| `software_version` | `string` | Software/firmware version |
| `hardware_version` | `string` | Hardware version |
| `battery` | `object` | Battery information |
| `cumulative_operating_time_hours` | `number` | Total device operating time |
| `connection` | `object` | Connection information |
| `calibration` | `object` | Calibration information |

```yaml
devices:
  - device_type: bike_computer
    manufacturer: garmin
    product: "Edge 1040"
    serial_number: "3985624781"
    software_version: "15.20"

    battery:
      start_percent: 87
      end_percent: 62
      status: good
```

### Device Types

Valid values for `device_type`:

- `watch` - GPS sports watch
- `bike_computer` - Cycling head unit
- `heart_rate_monitor` - HR chest strap/armband
- `power_meter` - Cycling power meter
- `speed_sensor` - Speed sensor
- `cadence_sensor` - Cadence sensor
- `speed_cadence_sensor` - Combined sensor
- `foot_pod` - Running dynamics pod
- `smart_trainer` - Indoor cycling trainer
- `camera` - Action camera
- `phone` - Smartphone app
- `other` - Other device type

### Manufacturers

Common manufacturer values:

- `garmin`, `wahoo`, `polar`, `suunto`, `coros`
- `hammerhead`, `stages`, `sram`, `shimano`
- `quarq`, `powertap`, `stryd`
- `whoop`, `apple`, `samsung`, `fitbit`

Or use a custom string for other manufacturers.

### Battery Information

The `battery` object tracks battery status:

| Field | Type | Description |
|-------|------|-------------|
| `start_percent` | `number` | Battery at workout start (0-100) |
| `end_percent` | `number` | Battery at workout end (0-100) |
| `voltage` | `number` | Battery voltage |
| `status` | `string` | Battery status |

**Battery Status Values**: `good`, `low`, `critical`, `charging`, `unknown`

```yaml
battery:
  start_percent: 87
  end_percent: 62
  status: good
```

### Connection Information

The `connection` object describes sensor connectivity:

| Field | Type | Description |
|-------|------|-------------|
| `connection_type` | `string` | **Required** - Connection type |
| `ant_device_number` | `number` | ANT+ device number |
| `bluetooth_id` | `string` | Bluetooth MAC address/ID |

**Connection Types**: `local`, `ant_plus`, `bluetooth_le`, `bluetooth`, `wifi`, `usb`, `unknown`

```yaml
connection:
  connection_type: ant_plus
  ant_device_number: 58392
```

### Calibration Information

The `calibration` object tracks sensor calibration (primarily for power meters):

| Field | Type | Description |
|-------|------|-------------|
| `calibration_factor` | `number` | Calibration factor/zero offset |
| `last_calibrated` | `string` | ISO 8601 timestamp of last calibration |
| `auto_zero_enabled` | `boolean` | Whether auto-zero is enabled |

```yaml
calibration:
  calibration_factor: 1.02
  last_calibrated: "2025-12-15T18:30:00Z"
  auto_zero_enabled: true
```

### Multi-Device Example

```yaml
devices:
  # Primary device (watch)
  - device_index: 0
    device_type: watch
    manufacturer: garmin
    product: "Forerunner 965"
    serial_number: "4028571930"
    software_version: "18.26"
    battery:
      start_percent: 92
      end_percent: 84

  # Heart rate monitor
  - device_index: 1
    device_type: heart_rate_monitor
    manufacturer: garmin
    product: "HRM-Pro Plus"
    connection:
      connection_type: ant_plus
      ant_device_number: 3928475

  # Power meter
  - device_index: 2
    device_type: power_meter
    manufacturer: garmin
    product: "Rally XC200"
    connection:
      connection_type: ant_plus
      ant_device_number: 58392
    calibration:
      auto_zero_enabled: true
```

---

## Time-Series Data (v2.1)

PWF v2.1 supports high-frequency time-series data for second-by-second telemetry using columnar storage.

### Time-Series Block

Add `time_series` to set telemetry for detailed metrics:

| Field | Type | Description |
|-------|------|-------------|
| `timestamps` | `array[string]` | **Required** - ISO 8601 timestamps |
| `elapsed_sec` | `array[number]` | Elapsed seconds since start |
| `heart_rate` | `array[number]` | Heart rate (bpm) |
| `power` | `array[number]` | Power (watts) |
| `cadence` | `array[number]` | Cadence (RPM/SPM) |
| `speed_mps` | `array[number]` | Speed (meters per second) |
| `distance_m` | `array[number]` | Cumulative distance (meters) |
| `elevation_m` | `array[number]` | Elevation/altitude (meters) |
| `temperature_c` | `array[number]` | Temperature (Celsius) |
| `latitude` | `array[number]` | Latitude (decimal degrees) |
| `longitude` | `array[number]` | Longitude (decimal degrees) |
| `grade_percent` | `array[number]` | Grade/slope (-100 to +100%) |

**Plus sport-specific fields** (see below).

```yaml
telemetry:
  heart_rate_avg: 165
  power_avg: 250

  time_series:
    timestamps:
      - "2025-12-20T10:00:00Z"
      - "2025-12-20T10:00:01Z"
      - "2025-12-20T10:00:02Z"

    elapsed_sec: [0, 1, 2]
    heart_rate: [160, 165, 170]
    power: [245, 255, 260]
    cadence: [88, 90, 92]
```

### Columnar Storage

Time-series data uses **columnar format**:
- All arrays must have the same length as `timestamps`
- Each index represents the same point in time
- Missing data: use `null` in array or omit the field entirely
- More efficient than per-second objects for large datasets

### Sport-Specific Time-Series Fields

**Cycling**:
- `power_balance` - Left power balance (%)
- `left_pedal_smoothness` - Left pedal smoothness (%)
- `right_pedal_smoothness` - Right pedal smoothness (%)
- `left_torque_effectiveness` - Left torque effectiveness (%)
- `right_torque_effectiveness` - Right torque effectiveness (%)

**Running**:
- `stride_length_m` - Stride length (meters)
- `vertical_oscillation_cm` - Vertical oscillation (cm)
- `ground_contact_time_ms` - Ground contact time (milliseconds)
- `ground_contact_balance` - Ground contact balance (% left)

**Swimming**:
- `stroke_rate` - Stroke rate (strokes per minute)
- `stroke_count` - Cumulative stroke count
- `swolf` - SWOLF score per length
- `stroke_type` - Stroke type at each point

**Advanced Sensors**:
- `respiration_rate` - Breaths per minute
- `core_temperature_c` - Core body temperature (°C)
- `muscle_oxygen_percent` - Muscle oxygen saturation (%)

### Example: Cycling with Power and Running Dynamics

```yaml
time_series:
  timestamps:
    - "2025-12-20T10:00:00Z"
    - "2025-12-20T10:00:01Z"
    - "2025-12-20T10:00:02Z"

  elapsed_sec: [0, 1, 2]
  heart_rate: [165, 168, 170]
  power: [250, 255, 260]
  cadence: [88, 90, 92]

  # Cycling-specific
  power_balance: [51.2, 51.0, 51.3]
  left_pedal_smoothness: [28.5, 29.0, 28.8]
  right_pedal_smoothness: [26.8, 27.2, 27.0]

  # Position data
  latitude: [45.5230, 45.5231, 45.5232]
  longitude: [-122.6765, -122.6766, -122.6767]
  elevation_m: [125, 126, 127]
  grade_percent: [2.5, 2.8, 3.0]
```

### Validation

PWF validates that all arrays match `timestamps` length:

```yaml
# INVALID - mismatched lengths
time_series:
  timestamps: ["2025-12-20T10:00:00Z", "2025-12-20T10:00:01Z"]
  heart_rate: [160, 165, 170]  # ERROR: 3 values but only 2 timestamps
```

---

## Migration from v2.0 to v2.1

PWF v2.1 is **fully backward compatible** with v2.0. All v2.1 fields are optional.

### What Changed

**New in v2.1**:
- Sport classification (`sport` field)
- Swimming features (`pool_config`, `swimming`, `lengths`, `swolf`)
- Multi-sport sessions (`sport_segments`, `transition`)
- Advanced metrics (`advanced_metrics`, `training_effect`, `vo2_max_estimate`)
- Power metrics (`power_metrics`, `normalized_power`, `training_stress_score`)
- Time in zones (`time_in_zones`)
- GPS routes (`gps_route`, `positions`)
- Device tracking (`devices`, `battery`, `connection`, `calibration`)
- Time-series data (`time_series` in telemetry)

**Unchanged**:
- All v1.x and v2.0 fields remain valid
- No breaking changes to existing exports
- `history_version: 2` accepts both v2.0 and v2.1 features

### Migration Strategy

1. **Update `history_version`** to `2` (if not already)
2. **Add sport classification** for better categorization
3. **For swimming**: Add `pool_config` and `swimming` data
4. **For multi-sport**: Use `sport_segments` and `transition`
5. **For power training**: Add `power_metrics` and `time_in_zones`
6. **For GPS workouts**: Add `gps_route` with positions
7. **For device tracking**: Add `devices` array
8. **For detailed analysis**: Add `time_series` data

### Example Migration

**v2.0 Export**:
```yaml
history_version: 2
workouts:
  - date: "2025-12-20"
    title: "Bike Ride"
    telemetry:
      heart_rate_avg: 165
      power_avg: 250
```

**v2.1 Enhanced Export**:
```yaml
history_version: 2
workouts:
  - date: "2025-12-20"
    sport: cycling  # NEW
    title: "Bike Ride"
    telemetry:
      heart_rate_avg: 165
      power_avg: 250

      # NEW in v2.1
      power_metrics:
        normalized_power: 268
        training_stress_score: 142.5
        ftp_watts: 300

      time_in_zones:
        power_zones_sec: [240, 600, 1500, 2700, 2400, 660]

      gps_route_id: "route-20251220"

    devices:  # NEW
      - device_type: bike_computer
        manufacturer: garmin
        product: "Edge 1040"
```

---

## Validation Rules

### Core Validation

| Rule | Severity | Error Code | Message |
|------|----------|------------|---------|
| Missing `history_version` | Error | - | `history_version is required` |
| `history_version` not 1 or 2 | Error | PWF-H001 | `Unsupported history_version` |
| Missing `exported_at` | Error | PWF-H002 | `exported_at is required` |
| Missing PR exercise name | Error | PWF-H401 | `Personal record must have exercise_name` |
| Missing PR date | Error | PWF-H402 | `Personal record must have achieved_at date` |
| Weight-based PR missing unit | Warning | PWF-H403 | `Weight-based personal records should specify a unit` |
| Missing body measurement date | Error | PWF-H501 | `Body measurement must have date` |
| Body measurement with no values | Warning | PWF-H502 | `Body measurement entry has no recorded values` |
| Preferred units mismatch | Warning | PWF-H601 | `Preferred weight unit doesn't match actual data` |

### PWF v2.1 Validation

| Rule | Severity | Error Code | Message |
|------|----------|------------|---------|
| Invalid sport value | Error | PWF-H701 | `Invalid sport: must be one of valid Sport enum values` |
| SWOLF mismatch | Warning | PWF-H702 | `SWOLF score doesn't match duration + stroke_count` |
| Invalid stroke type | Error | PWF-H703 | `Invalid stroke_type: must be valid StrokeType enum` |
| Time-series length mismatch | Error | PWF-H704 | `Time-series array length doesn't match timestamps length` |
| Invalid GPS fix | Error | PWF-H705 | `Invalid gps_fix: must be valid GpsFix enum` |
| Invalid device type | Error | PWF-H706 | `Invalid device_type: must be valid DeviceType enum` |
| Invalid training status | Error | PWF-H707 | `Invalid training_status: must be valid TrainingStatus enum` |
| Training Effect out of range | Warning | PWF-H708 | `Training Effect should be 0.0-5.0` |
| Performance Condition out of range | Warning | PWF-H709 | `Performance Condition should be -20 to +20` |
| Battery percent out of range | Warning | PWF-H710 | `Battery percent should be 0-100` |
| Invalid connection type | Error | PWF-H711 | `Invalid connection_type: must be valid ConnectionType enum` |
| Invalid battery status | Error | PWF-H712 | `Invalid battery status: must be valid BatteryStatus enum` |
| Multi-sport without segments | Warning | PWF-H713 | `Workout has sport: triathlon but no sport_segments` |
| Segment index not sequential | Warning | PWF-H714 | `Sport segment indices should be sequential starting from 0` |
| Transition sport mismatch | Error | PWF-H715 | `Transition from_sport doesn't match current segment sport` |
| Pool config without swimming | Warning | PWF-H716 | `Exercise has pool_config but modality is not swimming` |
| Swimming without pool config | Warning | PWF-H717 | `Swimming exercise missing pool_config` |
| Length number not sequential | Warning | PWF-H718 | `Swimming length numbers should be sequential starting from 1` |

See [workout.md](workout.md#validation-rules) for additional workout and set validation rules.

## Minimal Valid History

### v1.x Minimal

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

### v2.1 Swimming Minimal

```yaml
history_version: 2
exported_at: "2025-12-20T10:00:00Z"
workouts:
  - date: "2025-12-20"
    sport: swimming
    exercises:
      - name: "Freestyle"
        modality: swimming
        pool_config:
          pool_length: 25
          pool_length_unit: meters
        sets:
          - duration_sec: 120
            distance_meters: 100
```

---

## See Also

### Related Documentation
- [Workout Block](workout.md) - Detailed workout structure and validation
- [Exercise Block](exercise.md) - Exercise configuration and modalities
- [Modalities](../modalities.md) - Exercise types (strength, swimming, interval, etc.)
- [Equipment Tags](../equipment.md) - Standard equipment classification

### Example Files
- `examples/history-swimming-v2.1.yaml` - Swimming with length tracking
- `examples/history-cycling-power-v2.1.yaml` - Cycling with power metrics
- `examples/history-triathlon-v2.1.yaml` - Multi-sport session

### Specification
- [PWF Specification](../SPECIFICATION.md) - Complete format specification
- [Plan Block](plan.md) - Workout plan structure (for linking to history)
