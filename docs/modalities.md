# Modalities

PWF supports eight exercise modalities, each designed for different training styles.

## Overview

| Modality | Description | Primary Fields | Use Case |
|----------|-------------|----------------|----------|
| [`strength`](#strength) | Sets × reps training | `target_sets`, `target_reps` | Weight training, calisthenics |
| [`countdown`](#countdown) | Fixed duration timer | `target_duration_sec` | Isometric holds, timed sets |
| [`stopwatch`](#stopwatch) | Open-ended timing | `target_duration_sec` (optional) | Stretching, mobility, cardio |
| [`interval`](#interval) | Repeating work periods | `target_sets`, `target_duration_sec` | HIIT, sprints, circuits |
| [`cycling`](#cycling) | Structured cycling workouts | `zones`, `ramp`, `interval_phases` | Indoor/outdoor cycling, power-based training |
| [`running`](#running) | Structured running workouts | `zones`, `ramp`, `interval_phases` | Track, road, trail running |
| [`rowing`](#rowing) | Structured rowing workouts | `zones`, `ramp`, `interval_phases` | Erg training, power-based rowing |
| [`swimming`](#swimming) | Structured swimming workouts | `zones`, `interval_phases` | Pool training, technique work |

---

## Strength

Traditional resistance training with sets and repetitions.

### When to Use

- Barbell and dumbbell exercises
- Bodyweight exercises with rep targets
- Machine exercises
- Any exercise counted by repetitions

### Fields

| Field | Recommended | Description |
|-------|-------------|-------------|
| `target_sets` | Yes | Number of sets |
| `target_reps` | Yes | Reps per set |
| `target_load` | Optional | Weight or intensity |
| `target_notes` | Optional | Form cues |

### Examples

```yaml
# Basic strength exercise
- name: "Bench Press"
  modality: strength
  target_sets: 3
  target_reps: 8

# With loading guidance
- name: "Squat"
  modality: strength
  target_sets: 5
  target_reps: 5
  target_load: "185 lbs"
  target_notes: "Add 5 lbs from last session"

# Bodyweight with rep range
- name: "Pull-ups"
  modality: strength
  target_sets: 4
  target_reps: 8
  target_notes: "Use band assist if needed. Aim for 6-10 reps."

# Single set (e.g., deadlift top set)
- name: "Deadlift"
  modality: strength
  target_sets: 1
  target_reps: 5
  target_load: "RPE 9"
```

### Validation

| Condition | Severity | Message |
|-----------|----------|---------|
| Missing `target_sets` AND `target_reps` | Warning | `Strength exercise missing target_sets/target_reps` |

---

## Countdown

Fixed-duration exercise with a timer counting down to zero.

### When to Use

- Isometric holds (planks, wall sits)
- Timed sets where duration is the goal
- Static stretches with hold time
- Any exercise with a fixed time target

### Fields

| Field | Recommended | Description |
|-------|-------------|-------------|
| `target_duration_sec` | Yes | Hold/work duration in seconds |
| `target_sets` | Optional | Number of rounds |
| `target_notes` | Optional | Form cues |

### Examples

```yaml
# Basic isometric hold
- name: "Plank"
  modality: countdown
  target_duration_sec: 60

# Multiple rounds
- name: "Dead Hang"
  modality: countdown
  target_sets: 3
  target_duration_sec: 30
  target_notes: "Full grip, shoulders engaged"

# Static stretch
- name: "Hip Flexor Stretch"
  modality: countdown
  target_duration_sec: 90
  target_notes: "Each side. Keep hips square."
```

### Validation

| Condition | Severity | Message |
|-----------|----------|---------|
| Missing `target_duration_sec` | Warning | `Countdown exercise missing target_duration_sec` |

---

## Stopwatch

Open-ended timing where the user controls duration.

### When to Use

- Cardio with no fixed endpoint
- Mobility work
- Warm-up and cool-down
- "Work until done" exercises

### Fields

| Field | Recommended | Description |
|-------|-------------|-------------|
| `target_duration_sec` | Optional | Suggested minimum duration |
| `target_notes` | Optional | Guidance notes |

### Examples

```yaml
# Open-ended cardio
- name: "Treadmill Walk"
  modality: stopwatch
  target_notes: "Easy pace, conversational"

# With suggested duration
- name: "Foam Rolling"
  modality: stopwatch
  target_duration_sec: 300
  target_notes: "5+ minutes. Focus on tight areas."

# Mobility flow
- name: "Dynamic Warm-up"
  modality: stopwatch
  target_duration_sec: 600
  target_notes: "10 minutes. Include leg swings, arm circles, hip circles."
```

### Validation

No required fields. Stopwatch is the most flexible modality.

---

## Interval

Repeating work periods, optionally with distance targets.

### When to Use

- Sprint intervals
- HIIT training
- Circuit training rounds
- Any "X rounds of Y" format

### Fields

| Field | Recommended | Description |
|-------|-------------|-------------|
| `target_sets` | Yes | Number of intervals/rounds |
| `target_duration_sec` | Optional | Work period duration |
| `target_distance_meters` | Optional | Distance per interval |
| `target_notes` | Optional | Work/rest protocol |

### Examples

```yaml
# Track intervals
- name: "400m Repeats"
  modality: interval
  target_sets: 8
  target_distance_meters: 400
  target_duration_sec: 90
  target_notes: "Run in 90 seconds. Rest 90 seconds between."

# HIIT
- name: "Kettlebell Swings"
  modality: interval
  target_sets: 10
  target_duration_sec: 30
  target_notes: "30s work, 30s rest. Max effort."

# Tabata
- name: "Burpees"
  modality: interval
  target_sets: 8
  target_duration_sec: 20
  target_notes: "20s work, 10s rest (Tabata)"

# Distance-based
- name: "Rowing Intervals"
  modality: interval
  target_sets: 5
  target_distance_meters: 500
  target_notes: "500m efforts. Rest 2 min between."
```

### Validation

| Condition | Severity | Message |
|-----------|----------|---------|
| Missing `target_sets` | Warning | `Interval exercise missing target_sets` |

---

## Choosing the Right Modality

```
                    ┌─────────────────────┐
                    │  How is it tracked? │
                    └─────────┬───────────┘
                              │
            ┌─────────────────┼─────────────────┐
            │                 │                 │
            ▼                 ▼                 ▼
      ┌──────────┐     ┌───────────┐     ┌───────────┐
      │   Reps   │     │   Time    │     │  Rounds   │
      └────┬─────┘     └─────┬─────┘     └─────┬─────┘
           │                 │                 │
           ▼                 │                 ▼
      ┌──────────┐           │           ┌───────────┐
      │ STRENGTH │     ┌─────┴─────┐     │ INTERVAL  │
      └──────────┘     │           │     └───────────┘
                       ▼           ▼
                 ┌──────────┐ ┌───────────┐
                 │  Fixed?  │ │ Open-ended│
                 └────┬─────┘ └─────┬─────┘
                      │             │
                      ▼             ▼
                ┌───────────┐ ┌───────────┐
                │ COUNTDOWN │ │ STOPWATCH │
                └───────────┘ └───────────┘
```

---

## Cycling

Structured cycling workouts with power zones, heart rate targets, and interval phases.

### When to Use

- Indoor trainer sessions
- Outdoor cycling workouts
- Power-based training
- FTP tests and ramp tests

### Fields

| Field | Recommended | Description |
|-------|-------------|-------------|
| `zones` | Yes | Training zones with duration and targets |
| `ramp` | Optional | Gradual power increase configuration |
| `interval_phases` | Optional | Complex interval structures |

### Examples

```yaml
# Zone-based endurance ride
- name: "Zone 2 Base Building"
  modality: cycling
  zones:
    - zone: 2
      duration_sec: 3600
      target_power_watts: 200
      target_hr_bpm: 145

# FTP ramp test
- name: "20-min FTP Test"
  modality: cycling
  zones:
    - zone: 5
      duration_sec: 1200
      target_power_watts: 275

# Interval workout with phases
- name: "Sweet Spot Intervals"
  modality: cycling
  interval_phases:
    - name: "work"
      duration_sec: 480
      target_power_watts: 240
      cadence_rpm: 90
    - name: "recovery"
      duration_sec: 240
      target_power_watts: 120
  target_sets: 4
```

### Validation

| Condition | Severity | Message |
|-----------|----------|---------|
| Missing `zones`, `ramp`, AND `interval_phases` | Warning | `Endurance exercise should specify zones, ramp, or interval_phases` |

---

## Running

Structured running workouts with pace zones, heart rate targets, and interval phases.

### When to Use

- Track workouts
- Road running
- Trail running
- Tempo and threshold runs

### Fields

| Field | Recommended | Description |
|-------|-------------|-------------|
| `zones` | Yes | Training zones with pace/HR targets |
| `interval_phases` | Optional | Complex interval structures |
| `target_distance_meters` | Optional | Total workout distance |

### Examples

```yaml
# Tempo run
- name: "Threshold Run"
  modality: running
  zones:
    - zone: 4
      duration_sec: 1200
      target_pace_sec_per_km: 240
      target_hr_bpm: 170
  target_distance_meters: 5000

# Interval session
- name: "800m Repeats"
  modality: running
  interval_phases:
    - name: "work"
      duration_sec: 180
      target_pace_sec_per_km: 210
    - name: "recovery"
      duration_sec: 120
      target_pace_sec_per_km: 360
  target_sets: 6
```

---

## Rowing

Structured rowing workouts for indoor erg training with power and heart rate targets.

### When to Use

- Indoor rowing machine (erg)
- Power-based training
- Interval training
- Ramp tests

### Fields

| Field | Recommended | Description |
|-------|-------------|-------------|
| `zones` | Yes | Training zones with power/HR targets |
| `ramp` | Optional | Power ramp configuration |
| `interval_phases` | Optional | Complex interval structures |
| `target_distance_meters` | Optional | Total rowing distance |

### Examples

```yaml
# Steady-state rowing
- name: "Zone 2 Steady State"
  modality: rowing
  zones:
    - zone: 2
      duration_sec: 1800
      target_power_watts: 180
  target_distance_meters: 6000

# Ramp test
- name: "Power Ramp"
  modality: rowing
  ramp:
    start_power_watts: 150
    end_power_watts: 300
    duration_sec: 900
    step_duration_sec: 60
```

---

## Swimming

Structured swimming workouts with heart rate zones and interval phases.

### When to Use

- Pool training
- Technique sessions
- Endurance swims
- Interval sets

### Fields

| Field | Recommended | Description |
|-------|-------------|-------------|
| `zones` | Optional | Heart rate zones |
| `interval_phases` | Yes | Work/rest intervals |
| `target_distance_meters` | Yes | Swimming distance |

### Examples

```yaml
# Interval set
- name: "200m Repeats"
  modality: swimming
  interval_phases:
    - name: "work"
      duration_sec: 240
      target_hr_bpm: 150
    - name: "recovery"
      duration_sec: 60
      target_hr_bpm: 120
  target_sets: 8
  target_distance_meters: 1600

# Easy recovery swim
- name: "Easy Swim"
  modality: swimming
  target_distance_meters: 400
  zones:
    - zone: 1
      duration_sec: 480
      target_hr_bpm: 110
```

---

## Future Modalities (v3.0)

The following modalities are under consideration:

| Modality | Description |
|----------|-------------|
| `amrap` | As Many Rounds As Possible |
| `emom` | Every Minute On the Minute |
| `tabata` | 20s work / 10s rest protocol |
| `ladder` | Ascending/descending rep schemes |
