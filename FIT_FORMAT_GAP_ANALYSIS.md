# FIT File Format - Complete Gap Analysis for PWF

> **⚠️ SUPERSEDED:** This document was created during planning and is now outdated.
> **See instead:**
> - `FIT_EXPORT_ANALYSIS.md` - Current analysis of FIT export feasibility
> - `crates/pwf-converters/README.md` - Implemented FIT import features
> - Issue #23 - Completed implementation details
>
> **Status:** All critical gaps identified in this document have been addressed in PWF v2.1 and the FIT converter (v1.1.0).

**Document Version:** 1.0 (Archived)
**Date:** 2025-12-20
**Author:** PWF Analysis
**Purpose:** Identify all features required for complete FIT ↔ PWF conversion parity

---

## Executive Summary

This document provides a comprehensive gap analysis between the FIT (Flexible and Interoperable Data Transfer) file format and PWF (Portable Workout Format) history exports. The analysis categorizes gaps by priority to guide implementation for lossless FIT file conversion.

**Current PWF Coverage:** ~40% of FIT features
**Critical Gaps:** 18 major features
**Important Gaps:** 22 features
**Optional Gaps:** 15+ advanced features

---

## 1. File Structure & Hierarchy

### 1.1 Multi-Sport Activity Support ⚠️ CRITICAL

**FIT Capability:**
- Single activity file can contain multiple sport sessions (e.g., triathlon: swim → T1 → bike → T2 → run)
- Each session has its own sport type, metrics, and summary data
- Sessions are chronologically ordered and linked
- Support for 5+ sessions in a single activity (triathlon + transitions)

**PWF Current State:**
- Single workout = single sport/activity type
- No concept of transitions between sports
- No session-based organization
- Each workout is standalone

**Gap Severity:** CRITICAL
**Use Cases:**
- Triathlons (swim/bike/run + 2 transitions)
- Brick workouts (bike-to-run)
- Duathlons
- Multi-sport training days

**Implementation Requirements:**
```yaml
# Proposed PWF extension
workouts:
  - id: "triathlon-20251220-001"
    activity_type: multisport
    sport_sequence:
      - swim
      - transition
      - cycling
      - transition
      - running
    sessions:
      - session_number: 1
        sport: swimming
        started_at: "2025-12-20T07:00:00Z"
        duration_sec: 1800
        exercises: [...]
      - session_number: 2
        sport: transition
        started_at: "2025-12-20T07:30:00Z"
        duration_sec: 120
      - session_number: 3
        sport: cycling
        started_at: "2025-12-20T07:32:00Z"
        duration_sec: 3600
        exercises: [...]
```

**References:**
- [FIT Multi-Sport Structure](https://apizone.suunto.com/fit-description)
- [Triathlon Session Examples](https://www.fitfilerepairtool.info/quick-tour/multi-session-files/)

---

### 1.2 Length Messages (Pool Swimming) ⚠️ CRITICAL

**FIT Capability:**
- Length-by-length tracking for pool swimming
- Hierarchy: Session → Lap → Length → Record
- Each length has: stroke type, duration, strokes, SWOLF score

**PWF Current State:**
- Only set-level tracking
- No length-specific data
- No automatic lap/length detection

**Gap Severity:** CRITICAL
**Use Cases:**
- Pool swimming workouts
- Swim training analysis
- Stroke efficiency tracking

**Implementation Requirements:**
```yaml
exercises:
  - name: "Pool Swim"
    modality: swimming
    pool_length_m: 25
    sets:
      - set_number: 1  # One lap = multiple lengths
        duration_sec: 120
        distance_meters: 100
        lengths:
          - length_number: 1
            stroke_type: freestyle
            duration_sec: 28
            strokes: 16
            swolf: 44
          - length_number: 2
            stroke_type: freestyle
            duration_sec: 30
            strokes: 18
            swolf: 48
```

**References:**
- [Swimming Metrics Explained](https://sporttracks.mobi/blog/what-is-swolf-swimming-metrics-explained)
- [Pool Swim Analysis](https://www.wareable.com/swimming/swimming-metrics-explained-understand-stats-4430)

---

### 1.3 Event Messages ⚠️ IMPORTANT

**FIT Capability:**
- Discrete events during activity: lap button press, timer start/stop, recovery heart rate
- Events have: timestamp, event type, event group, data field
- Common events: lap, session start/end, recovery_hr, battery level

**PWF Current State:**
- No event tracking
- Only start/end timestamps at workout level

**Gap Severity:** IMPORTANT
**Use Cases:**
- Automatic pause/resume detection
- Battery warnings during workout
- Manual lap markers
- Recovery heart rate measurements

**Implementation Requirements:**
```yaml
workouts:
  - events:
      - timestamp: "2025-12-20T06:30:00Z"
        event_type: timer_start
      - timestamp: "2025-12-20T06:45:32Z"
        event_type: lap_button
        data: 1
      - timestamp: "2025-12-20T07:00:00Z"
        event_type: timer_pause
      - timestamp: "2025-12-20T07:25:00Z"
        event_type: recovery_hr
        data: 125
```

**References:**
- [FIT Activity File Structure](https://docs.fileformat.com/gis/fit/)

---

## 2. Advanced Telemetry & Metrics

### 2.1 Time-Series Record Data ⚠️ CRITICAL

**FIT Capability:**
- Second-by-second (or smart recording) data points
- Each record contains: timestamp, GPS position, HR, power, cadence, altitude, speed
- Records can be 1Hz, 10Hz, or variable (smart recording)
- Fractional timestamps for high-frequency data (VIRB cameras: 10Hz)

**PWF Current State:**
- Only aggregate/summary metrics (avg, max, min)
- No time-series data storage
- No second-by-second tracking
- `gps_route_id` reference exists but not defined

**Gap Severity:** CRITICAL
**Use Cases:**
- Detailed performance analysis
- Power curve analysis
- Heart rate recovery analysis
- GPS track visualization
- Segment efforts
- Interval analysis

**Implementation Requirements:**
```yaml
# Option 1: Inline time-series (small datasets)
sets:
  - set_number: 1
    duration_sec: 600
    time_series:
      recording_interval: 1  # seconds
      timestamps: [0, 1, 2, 3, ...]
      heart_rate: [120, 122, 125, 128, ...]
      power: [200, 205, 210, 215, ...]
      cadence: [85, 86, 87, 88, ...]

# Option 2: External file reference (large datasets)
sets:
  - set_number: 1
    time_series_file: "workout-20251220-001-set-1.csv"
    time_series_format: "csv"  # or "gpx", "tcx"

# Option 3: Embedded GPX/TCX
workouts:
  - gpx_data: |
      <?xml version="1.0"?>
      <gpx>...</gpx>
```

**References:**
- [FIT Record Messages](https://msmith.de/FITfileR/articles/FITfileR.html)
- [Second-by-Second Recording](https://www.pinns.co.uk/osm/fit-for-dummies.html)

---

### 2.2 GPS Position Data ⚠️ CRITICAL

**FIT Capability:**
- Latitude/longitude in semicircles (32-bit signed int: degrees × 2³¹/180)
- Per-record GPS coordinates
- Enhanced altitude (0.2m precision)
- GPS accuracy metrics: HDOP, VDOP, GPS fix quality
- Position precision to 7 decimal places (~1cm)

**PWF Current State:**
- `gps_route_id` string reference only
- No coordinate storage
- No GPS metadata
- No altitude tracking per position

**Gap Severity:** CRITICAL
**Use Cases:**
- Route visualization
- GPS track export
- Segment matching (Strava KOMs)
- Elevation profile analysis
- Location-based insights

**Implementation Requirements:**
```yaml
# Embedded approach
sets:
  - gps_track:
      coordinates:
        - lat: 37.7749295
          lon: -122.4194155
          alt: 52.4
          timestamp: "2025-12-20T06:30:00Z"
        - lat: 37.7750123
          lon: -122.4195887
          alt: 52.6
          timestamp: "2025-12-20T06:30:01Z"
      metadata:
        recording_method: gps
        accuracy_meters: 3.5
        hdop: 1.2

# External file approach
workouts:
  - gps_routes:
      - id: "route-001"
        format: gpx
        file: "route-001.gpx"
        encoding: base64_inline  # or external_file
```

**References:**
- [GPS Coordinates in FIT](https://forums.garmin.com/developer/fit-sdk/f/discussion/280125/record-the-latitude-and-longitude-format-of-the-message)
- [FIT Position Storage](https://www.pinns.co.uk/osm/fit-for-dummies.html)

---

### 2.3 Training Effect Metrics ⚠️ CRITICAL

**FIT Capability:**
- **Aerobic Training Effect** (0.0-5.0): Impact on aerobic fitness/VO2max
- **Anaerobic Training Effect** (0.0-5.0): Impact on high-intensity capacity
- **VO2max estimate** (ml/kg/min with scale factor)
- **Recovery Time** (minutes until next hard workout)
- **Performance Condition** (-20 to +20): Real-time performance vs. fitness
- **Lactate Threshold Heart Rate** (bpm)
- **Lactate Threshold Speed** (m/s)

**PWF Current State:**
- None of these metrics supported
- Only basic telemetry (HR, power, calories)

**Gap Severity:** CRITICAL
**Use Cases:**
- Training load management
- Fitness tracking over time
- Recovery planning
- Adaptive training programs

**Implementation Requirements:**
```yaml
workouts:
  - advanced_metrics:
      aerobic_training_effect: 3.8
      anaerobic_training_effect: 1.2
      vo2max_estimate: 52.3
      recovery_time_hours: 36
      performance_condition: 5
      lactate_threshold:
        heart_rate_bpm: 165
        speed_mps: 3.8
        power_watts: 285  # For cycling
```

**References:**
- [Training Effect Documentation](https://www.dcrainmaker.com/images/2017/03/FirstBeat-GarminFeatures-DCR.pdf)
- [VO2max in FIT Files](https://forums.garmin.com/sports-fitness/running-multisport/f/forerunner-945/226862/extracting-precise-vo2max-from-fit-file/1166716)

---

### 2.4 Power-Based Metrics ⚠️ IMPORTANT

**FIT Capability:**
- **Normalized Power (NP)®** (watts): Physiological cost-adjusted power
- **Intensity Factor (IF)®** (0.0-2.0): NP/FTP ratio
- **Training Stress Score (TSS)®** (0-500+): Workout difficulty quantification
- **Work** (kilojoules): Total mechanical work
- **Average Power** (raw watts)
- **Variability Index**: NP/AP ratio
- **FTP** (Functional Threshold Power)
- **Power-to-Weight Ratio** (watts/kg)
- **Left/Right Balance** (percentage)
- **Pedal Smoothness** (percentage)
- **Torque Effectiveness** (percentage)

**PWF Current State:**
- Average power (workout and set level)
- Max power (workout and set level)
- Min power (set level)
- No derived power metrics
- No FTP tracking
- No power meter advanced metrics

**Gap Severity:** IMPORTANT
**Use Cases:**
- Cycling training analysis
- Power-based training plans
- Fitness benchmarking
- Race pacing strategies

**Implementation Requirements:**
```yaml
workouts:
  - power_metrics:
      normalized_power: 265
      intensity_factor: 0.88
      training_stress_score: 95
      work_kj: 1250
      variability_index: 1.08
      power_to_weight: 3.8
      left_right_balance: 49.5  # Left percentage
      pedal_smoothness_left: 68
      pedal_smoothness_right: 72
      torque_effectiveness_left: 92
      torque_effectiveness_right: 94
    athlete_thresholds:
      ftp_watts: 300
      ftp_tested_at: "2025-12-01"
```

**References:**
- [Normalized Power Explained](https://www.trainingpeaks.com/learn/articles/normalized-power-intensity-factor-training-stress/)
- [TSS and IF](https://support.rouvy.com/hc/en-us/articles/4481820359313-TSS-and-IF)

---

### 2.5 Running Dynamics ⚠️ IMPORTANT

**FIT Capability:**
- **Ground Contact Time (GCT)** (ms): Time foot contacts ground per step
- **Ground Contact Time Balance** (percentage L/R)
- **Vertical Oscillation** (cm): Torso bounce per stride
- **Vertical Ratio** (percentage): Vertical oscillation / stride length
- **Stride Length** (meters)
- **Step Speed Loss** (NEW 2025, with HRM-600): Torso deceleration on landing
- **Stance Time** (ms)
- **Stance Time Balance** (percentage L/R)
- **Leg Spring Stiffness** (kN/m)

**PWF Current State:**
- Only cadence (avg/max)
- No biomechanical metrics
- No left/right balance data

**Gap Severity:** IMPORTANT
**Use Cases:**
- Running form analysis
- Injury prevention
- Efficiency optimization
- Fatigue monitoring

**Implementation Requirements:**
```yaml
sets:
  - telemetry:
      running_dynamics:
        ground_contact_time_ms: 245
        ground_contact_balance_percent: 50.5
        vertical_oscillation_cm: 8.2
        vertical_ratio_percent: 7.3
        stride_length_m: 1.12
        step_speed_loss_percent: 3.2  # New 2025
        stance_time_ms: 248
        stance_time_balance_percent: 51.2
        leg_spring_stiffness_knm: 12.4
```

**References:**
- [Running Dynamics](https://thewiredrunner.com/ground-contact-vertical-oscillation/)
- [2025 Garmin Updates](https://uk.amazfit.com/blogs/product-updates/active-2-square-october-2025-software-updates)

---

### 2.6 Swimming-Specific Metrics ⚠️ IMPORTANT

**FIT Capability:**
- **SWOLF Score**: Strokes + seconds per length (efficiency metric)
- **Stroke Type**: freestyle, backstroke, breaststroke, butterfly, drill, mixed, IM
- **Stroke Count**: Per length and per lap
- **Pool Length**: Configurable (25m, 50m, 25yd, etc.)
- **Number of Lengths**: Auto-detected
- **Active Lengths**: Excludes rest periods
- **Drill Mode**: Boolean flag for drill vs. swim
- **Stroke Rate**: Strokes per minute

**PWF Current State:**
- `stroke_rate` (set level only, generic)
- `modality: swimming` (generic)
- No stroke type differentiation
- No SWOLF
- No drill mode
- No pool configuration

**Gap Severity:** IMPORTANT
**Use Cases:**
- Pool swim training
- Technique analysis
- Efficiency tracking
- Workout structure (drills vs. swimming)

**Implementation Requirements:**
```yaml
exercises:
  - name: "Pool Swim Workout"
    modality: swimming
    swimming_config:
      pool_length_m: 25
      pool_length_units: meters
    sets:
      - set_number: 1
        set_type: working
        distance_meters: 400
        duration_sec: 420
        stroke_type: freestyle
        stroke_count: 64
        active_lengths: 16
        total_lengths: 16
        swolf_avg: 40
        drill_mode: false
        telemetry:
          stroke_rate: 32

      - set_number: 2
        set_type: working
        distance_meters: 100
        duration_sec: 180
        stroke_type: drill
        drill_mode: true
        notes: "Catch-up drill"
```

**References:**
- [SWOLF Explained](https://sporttracks.mobi/blog/what-is-swolf-swimming-metrics-explained)
- [Swimming Metrics](https://www.wareable.com/swimming/swimming-metrics-explained-understand-stats-4430)

---

### 2.7 Time in Zone Distribution ⚠️ IMPORTANT

**FIT Capability:**
- **Heart Rate Zones**: Duration in each of 5+ HR zones (seconds per zone)
- **Power Zones**: Duration in each of 7 power zones (cycling)
- **Pace/Speed Zones**: Duration in each zone (running)
- User-configurable zone boundaries
- Per-session and per-lap zone time
- Zone-based targets for structured workouts

**PWF Current State:**
- No zone configuration
- No time-in-zone tracking
- Only aggregate metrics (avg, max, min)

**Gap Severity:** IMPORTANT
**Use Cases:**
- Training intensity distribution
- Polarized training analysis
- Zone-based training plans
- Intensity monitoring

**Implementation Requirements:**
```yaml
# Athlete profile (separate from workout)
athlete_profile:
  heart_rate_zones:
    zone_system: 5_zone
    max_hr: 190
    zones:
      - zone: 1
        name: "Recovery"
        min_bpm: 0
        max_bpm: 123
      - zone: 2
        name: "Aerobic"
        min_bpm: 123
        max_bpm: 142
      - zone: 3
        name: "Tempo"
        min_bpm: 142
        max_bpm: 161
      - zone: 4
        name: "Threshold"
        min_bpm: 161
        max_bpm: 180
      - zone: 5
        name: "VO2max"
        min_bpm: 180
        max_bpm: 250

# Workout with time-in-zone
workouts:
  - telemetry:
      time_in_hr_zones:
        zone_1_sec: 300
        zone_2_sec: 1800
        zone_3_sec: 900
        zone_4_sec: 420
        zone_5_sec: 180
      time_in_power_zones:
        zone_1_sec: 600
        zone_2_sec: 1200
        zone_3_sec: 800
        # ... etc
```

**References:**
- [FIT Time in Zones](https://apizone.suunto.com/fit-description)
- [Training Zones Guide](https://www.trainingpeaks.com/learn/articles/joe-friel-s-quick-guide-to-setting-zones/)

---

## 3. Physiological & Recovery Metrics

### 3.1 HRV and Stress Metrics ⚠️ IMPORTANT

**FIT Capability:**
- **HRV Messages**: R-R interval data (ms between heartbeats)
- **Stress Score**: 0-100 scale
- **Respiration Rate**: Breaths per minute
- **Stress Level**: Real-time and daily average
- **Body Battery** (Garmin): Energy reserve score 0-100
- **Readiness Score**: Recovery/training readiness

**PWF Current State:**
- None of these metrics

**Gap Severity:** IMPORTANT
**Use Cases:**
- Recovery monitoring
- Overtraining prevention
- Readiness assessment
- Sleep quality correlation

**Implementation Requirements:**
```yaml
workouts:
  - physiological_metrics:
      hrv_data:
        format: rr_intervals
        intervals_ms: [850, 870, 845, 880, ...]  # R-R intervals
      stress_score: 42
      respiration_rate_bpm: 16
      body_battery_start: 85
      body_battery_end: 62
      body_battery_drain: 23

# Separate daily tracking
daily_metrics:
  - date: "2025-12-20"
    stress_avg: 38
    hrv_rmssd: 52.3
    readiness_score: 78
    body_battery_low: 55
    body_battery_high: 95
```

**References:**
- [Body Battery](https://fitstraps.co.uk/blogs/news/body-battery-explained-how-does-garmin-work-out-body-battery)
- [HRV Analysis](https://www.firstbeat.com/wp-content/uploads/2015/10/How-to-Analyze-Stress-from-Heart-Rate-Variability.pdf)

---

### 3.2 Sleep Metrics ⚠️ OPTIONAL

**FIT Capability:**
- **Sleep Monitoring File Type**: Separate FIT file type
- **Sleep Stages**: Awake, Light, Deep, REM (duration in each)
- **Sleep Score**: 0-100
- **Sleep Start/End**: Timestamps
- **Average Respiration**: Breaths per minute during sleep
- **Average Stress**: During sleep
- **Movement Detection**: Restlessness

**PWF Current State:**
- No sleep tracking

**Gap Severity:** OPTIONAL
**Use Cases:**
- Recovery monitoring
- Training adaptation
- Health tracking
- Holistic athlete monitoring

**Implementation Requirements:**
```yaml
# Separate from workouts
sleep_tracking:
  - date: "2025-12-20"
    sleep_start: "2025-12-19T22:30:00Z"
    sleep_end: "2025-12-20T06:15:00Z"
    total_sleep_sec: 27900  # 7h 45m
    sleep_stages:
      awake_sec: 900
      light_sec: 13500
      deep_sec: 9000
      rem_sec: 4500
    sleep_score: 82
    respiration_avg_bpm: 14
    stress_avg: 18
    movement_score: 25
    hrv_avg_rmssd: 58.2
```

**References:**
- [Wearable Sleep Tracking](https://pmc.ncbi.nlm.nih.gov/articles/PMC10195711/)

---

### 3.3 Body Composition Extended ⚠️ OPTIONAL

**FIT Capability:**
- **Weight Scale File Type**: Separate FIT file type
- **Bone Mass** (kg)
- **Muscle Mass** (kg)
- **Hydration Percentage**
- **Body Fat Percentage**
- **Metabolic Age**
- **Visceral Fat Rating**
- **BMI**
- **BMR** (Basal Metabolic Rate)
- **Impedance** (ohms)

**PWF Current State:**
- `body_fat_percent` only
- Basic weight tracking
- Manual body dimensions (neck, chest, etc.)

**Gap Severity:** OPTIONAL
**Use Cases:**
- Body composition tracking
- Smart scale integration
- Longitudinal health monitoring

**Implementation Requirements:**
```yaml
body_measurements:
  - date: "2025-12-20"
    weight_kg: 75.2
    body_composition:
      body_fat_percent: 15.2
      muscle_mass_kg: 62.8
      bone_mass_kg: 3.2
      hydration_percent: 58.5
      visceral_fat_rating: 5
      metabolic_age: 28
      bmi: 23.1
      bmr_calories: 1685
      impedance_ohms: 485
```

---

## 4. Device & Equipment Tracking

### 4.1 Device Information ⚠️ IMPORTANT

**FIT Capability:**
- **Device Info Messages**: Multiple devices per activity
- **Serial Number**: Unique device identifier
- **Manufacturer**: Garmin, Wahoo, COROS, Polar, etc.
- **Product**: Specific model (Fenix 7, Edge 1030, etc.)
- **Software Version**: Firmware version
- **Hardware Version**
- **Battery Status**: Voltage, percentage, time remaining
- **Cumulative Operating Time**: Device usage hours
- **Device Type**: Bike computer, watch, HRM, power meter, etc.
- **Device Index**: For multiple devices (e.g., HRM=1, Power=2)
- **ANT+ Device Number**: Sensor pairing ID
- **Source Type**: Local device, ANT+, Bluetooth, etc.

**PWF Current State:**
- `export_source` has basic app info only
- No device tracking
- No sensor information
- No battery data

**Gap Severity:** IMPORTANT
**Use Cases:**
- Sensor calibration tracking
- Device lifecycle management
- Troubleshooting data quality issues
- Multi-device workouts (watch + HRM + power meter)

**Implementation Requirements:**
```yaml
workouts:
  - devices:
      - device_index: 0
        device_type: watch
        manufacturer: garmin
        product: fenix_7x
        serial_number: "3954872193"
        software_version: "18.24"
        hardware_version: "A"
        battery_start_percent: 85
        battery_end_percent: 72
        cumulative_operating_time_hours: 1247.5

      - device_index: 1
        device_type: heart_rate_monitor
        manufacturer: garmin
        product: hrm_pro_plus
        serial_number: "2947182736"
        ant_device_number: 12345
        source_type: ant_plus
        battery_status: good

      - device_index: 2
        device_type: power_meter
        manufacturer: wahoo
        product: powrlink_zero
        serial_number: "WF-PM-8471928"
        ant_device_number: 67890
        calibration_factor: 1.02
        last_calibrated: "2025-12-15T10:00:00Z"
```

**References:**
- [FIT Device Tracking](https://andrewcooke.github.io/choochoo/fit-files.html)
- [Device Info Messages](https://www.thisisant.com/forum/viewthread/4275)

---

### 4.2 Equipment Tracking (Bikes, Shoes) ⚠️ OPTIONAL

**FIT Capability:**
- **Equipment/Gear Tracking**: Not a standard FIT message, but many platforms track this
- Platforms like Strava/TrainingPeaks link activities to specific equipment
- Mileage/usage tracking per equipment item

**PWF Current State:**
- No equipment tracking

**Gap Severity:** OPTIONAL
**Use Cases:**
- Bike/shoe mileage tracking
- Maintenance scheduling
- Equipment lifecycle management
- Multi-bike athletes

**Implementation Requirements:**
```yaml
# Athlete equipment catalog (separate file)
equipment:
  - id: "bike-001"
    type: bike
    name: "Canyon Ultimate CF SLX"
    brand: canyon
    model: "Ultimate CF SLX Disc"
    added_date: "2024-01-15"
    retired_date: null
    initial_mileage_km: 0
    current_mileage_km: 2847.3
    maintenance_log:
      - date: "2025-12-01"
        type: chain_replacement
        mileage_km: 2500

  - id: "shoes-001"
    type: running_shoes
    name: "Hoka Clifton 9"
    brand: hoka
    model: "Clifton 9"
    added_date: "2025-09-01"
    recommended_lifespan_km: 800
    current_mileage_km: 342.1

# Reference in workout
workouts:
  - equipment_used:
      - equipment_id: "bike-001"
        session_mileage_km: 45.2
```

---

## 5. Workout Structure & Planning

### 5.1 Structured Workout Definition ⚠️ IMPORTANT

**FIT Capability:**
- **Workout File Type**: Separate FIT file for planned workouts
- **Workout Steps**: Hierarchical structure with repeats
- **Step Targets**: Power zones, HR zones, pace, cadence, duration, distance
- **Step Intensity**: Warmup, active, recovery, cooldown, rest
- **Repeat Steps**: Nested repeats (e.g., 4x(8min @ FTP + 4min recovery))
- **Step Notes**: Instructions per step
- **Open/Closed Duration**: Time-based vs. lap button press
- **Target Ranges**: Min/max for each metric

**PWF Current State:**
- Plan format exists but is for templates only
- No structured workout targets in history
- No step-by-step execution tracking
- No target adherence metrics

**Gap Severity:** IMPORTANT
**Use Cases:**
- Structured interval workouts
- Workout execution compliance
- Coach-prescribed workouts
- Indoor trainer workouts (Zwift, TrainerRoad)

**Implementation Requirements:**
```yaml
# In workout history
workouts:
  - structured_workout:
      workout_name: "VO2max Intervals"
      planned_steps:
        - step: 1
          type: warmup
          duration_sec: 600
          target_power_zone: 2
        - step: 2
          type: repeat
          repeat_count: 4
          steps:
            - step: 2.1
              type: active
              duration_sec: 480
              target_power_watts_min: 315
              target_power_watts_max: 330
              target_cadence_rpm_min: 90
              target_cadence_rpm_max: 100
            - step: 2.2
              type: recovery
              duration_sec: 240
              target_power_watts_max: 150
        - step: 3
          type: cooldown
          duration_sec: 600
          target_power_zone: 1

      # Actual execution linked to sets
      execution:
        - step: 1
          actual_set_id: "set-1"
          compliance_percent: 98
        - step: 2.1
          actual_set_id: "set-2"
          compliance_percent: 94
        # ...
```

**References:**
- [Encoding Workout Files](https://developer.garmin.com/fit/cookbook/encoding-workout-files/)
- [Structured Workouts](https://forum.intervals.icu/t/create-workout-and-export-as-fit-file/33323)

---

### 5.2 Course/Navigation Data ⚠️ OPTIONAL

**FIT Capability:**
- **Course File Type**: Pre-planned route
- **Course Points**: Waypoints, summits, turns, POIs (max 50)
- **Course Name**: Route name
- **Turn-by-Turn Navigation**: Alerts and directions
- **Climb Categories**: HC, Cat 1-5 climbs
- **Course Records**: GPS track for the route
- **Cue Sheets**: Text instructions

**PWF Current State:**
- No course/route planning
- No navigation

**Gap Severity:** OPTIONAL
**Use Cases:**
- Race course planning
- Group ride routes
- Adventure planning
- Climb tracking

**Implementation Requirements:**
```yaml
# Separate course file
courses:
  - id: "course-alpe-d-huez"
    name: "Alpe d'Huez Climb"
    distance_m: 13800
    elevation_gain_m: 1071
    course_points:
      - point_id: 1
        name: "Start - Bourg d'Oisans"
        type: generic
        position_lat: 45.0556
        position_lon: 6.0283
      - point_id: 2
        name: "Hairpin 21"
        type: left_turn
        position_lat: 45.0612
        position_lon: 6.0345
        distance_m: 1200
      - point_id: 3
        name: "Summit"
        type: summit
        position_lat: 45.0928
        position_lon: 6.0719
        distance_m: 13800
    climb_category: hc
    gps_track_file: "alpe-dhuez.gpx"

# Reference in workout
workouts:
  - course_id: "course-alpe-d-huez"
```

**References:**
- [Course Points](https://muddytweed.com/2024/05/13/garmin-tips-1-the-power-of-course-points/)
- [FIT Course Files](https://forum.locusmap.eu/index.php?topic=6505.0)

---

### 5.3 Segment Efforts ⚠️ OPTIONAL

**FIT Capability:**
- Not directly in FIT format
- Platforms like Strava detect segments from GPS data
- Segment matching requires GPS coordinates + segment database

**PWF Current State:**
- No segment tracking

**Gap Severity:** OPTIONAL
**Use Cases:**
- Strava KOM/QOM hunting
- Performance benchmarking
- Course familiarity

**Implementation Requirements:**
```yaml
workouts:
  - segments:
      - segment_id: "strava-12345678"
        segment_name: "Hawk Hill Climb"
        start_time: "2025-12-20T06:45:32Z"
        elapsed_time_sec: 382
        moving_time_sec: 382
        distance_m: 2100
        elevation_gain_m: 152
        average_power: 310
        average_hr: 172
        pr_rank: 2  # 2nd best effort
        kom_time_sec: 298
```

---

## 6. Environmental & Context Data

### 6.1 Weather Data ⚠️ OPTIONAL

**FIT Capability:**
- **Temperature**: From device sensor or weather service
- **Apparent Temperature**: Wind chill / heat index
- **Humidity**: Percentage
- **Barometric Pressure**: hPa/mmHg
- **Wind Speed**: m/s or km/h
- **Wind Direction**: Degrees
- **Precipitation**: Type and intensity
- **Weather Condition**: Clear, cloudy, rain, snow, etc.
- **Location**: GPS-based weather lookup

**PWF Current State:**
- Temperature (Celsius/Fahrenheit)
- Humidity (percentage)
- No other weather data

**Gap Severity:** OPTIONAL
**Use Cases:**
- Performance analysis (heat/cold impact)
- Outdoor workout planning
- Historical context

**Implementation Requirements:**
```yaml
workouts:
  - environmental_conditions:
      temperature_c: 14.4
      temperature_f: 57.9
      apparent_temperature_c: 12.1
      humidity_percent: 72
      pressure_hpa: 1013.2
      wind_speed_kph: 15
      wind_direction_deg: 270
      precipitation_type: none
      weather_condition: partly_cloudy
      uv_index: 3
      air_quality_index: 45
      location:
        name: "San Francisco, CA"
        country: "USA"
```

---

### 6.2 Sport-Specific Settings ⚠️ OPTIONAL

**FIT Capability:**
- **Auto Lap**: Distance or position-based
- **Auto Pause**: Enabled/disabled
- **Pool Length**: For swimming
- **Bike Weight**: For power calculations
- **Position Calibration**: Indoor vs. outdoor
- **Data Screening**: Enabled/disabled
- **Sensor Calibration**: Zero offset values

**PWF Current State:**
- None

**Gap Severity:** OPTIONAL
**Use Cases:**
- Data quality understanding
- Troubleshooting anomalies
- Reproducibility

**Implementation Requirements:**
```yaml
workouts:
  - activity_settings:
      auto_lap_distance_m: 1000
      auto_pause_enabled: true
      gps_enabled: true
      pool_length_m: 25  # Swimming
      bike_weight_kg: 7.2  # Cycling
      calibration_offset: 0
      data_screening_enabled: true
```

---

## 7. Training Load & Periodization

### 7.1 Chronic Training Load (CTL/ATL/TSB) ⚠️ OPTIONAL

**FIT Capability:**
- Not directly in FIT files
- Calculated by platforms using historical TSS data
- **CTL** (Chronic Training Load / Fitness): 42-day weighted average
- **ATL** (Acute Training Load / Fatigue): 7-day weighted average
- **TSB** (Training Stress Balance / Form): CTL - ATL

**PWF Current State:**
- No training load tracking
- No historical fitness metrics

**Gap Severity:** OPTIONAL
**Use Cases:**
- Long-term fitness tracking
- Peak performance timing
- Overtraining detection
- Training plan periodization

**Implementation Requirements:**
```yaml
# Athlete fitness tracking (separate from workouts)
fitness_metrics:
  - date: "2025-12-20"
    ctl: 65.3  # Fitness (42-day avg)
    atl: 72.1  # Fatigue (7-day avg)
    tsb: -6.8  # Form (negative = fatigued)
    ramp_rate: 5.2  # CTL change per week
```

**References:**
- [TrainingPeaks Metrics](https://www.procyclingcoaching.com/post/core-trainingpeaks-metrics-fitness-form-fatigue)

---

## 8. Developer & Custom Data

### 8.1 Developer Data Fields ⚠️ OPTIONAL

**FIT Capability:**
- **Developer Fields**: Custom fields from Connect IQ apps
- **Application ID**: Unique app identifier (UUID)
- **Field Definition**: Name, units, data type
- **Field Data**: Per-record, per-lap, or per-session
- Custom metrics not in FIT spec (e.g., muscle oxygen, core temperature)

**PWF Current State:**
- No custom field support

**Gap Severity:** OPTIONAL
**Use Cases:**
- Third-party sensor data (Moxy, CORE, Stryd)
- Custom training metrics
- Research data collection
- Platform-specific features

**Implementation Requirements:**
```yaml
workouts:
  - custom_fields:
      - field_name: "muscle_oxygen_saturation"
        field_units: "percent"
        field_type: uint8
        source_app: "BSXinsight"
        source_app_id: "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
        values: [72, 74, 68, 65, ...]  # Per-record

      - field_name: "core_temperature"
        field_units: "celsius"
        field_type: float
        source_app: "CORE"
        source_app_id: "f9e8d7c6-b5a4-3210-9876-543210fedcba"
        avg_value: 37.8
        max_value: 38.5
```

**References:**
- [Developer Data Fields](https://developer.garmin.com/fit/cookbook/developer-data/)
- [FitContributor API](https://developer.garmin.com/connect-iq/api-docs/Toybox/FitContributor.html)

---

## 9. Summary Tables

### 9.1 Critical Gaps (Must-Have for Lossless Conversion)

| # | Feature | Impact | Effort | Priority |
|---|---------|--------|--------|----------|
| 1 | Multi-sport session support | High | High | P0 |
| 2 | Time-series record data | High | High | P0 |
| 3 | GPS position tracking | High | High | P0 |
| 4 | Training Effect metrics | High | Medium | P0 |
| 5 | Length messages (swimming) | Medium | Medium | P1 |
| 6 | Time in zone distribution | Medium | Medium | P1 |

### 9.2 Important Gaps (Commonly Used Features)

| # | Feature | Impact | Effort | Priority |
|---|---------|--------|--------|----------|
| 7 | Power-based metrics (NP/IF/TSS) | High | Medium | P1 |
| 8 | Running dynamics | Medium | Medium | P1 |
| 9 | Swimming metrics (SWOLF, strokes) | Medium | Medium | P1 |
| 10 | Device information tracking | Medium | Low | P2 |
| 11 | Event messages | Low | Low | P2 |
| 12 | Structured workout execution | Medium | High | P2 |
| 13 | HRV and stress metrics | Medium | Medium | P2 |

### 9.3 Optional Gaps (Advanced/Specialized Features)

| # | Feature | Impact | Effort | Priority |
|---|---------|--------|--------|----------|
| 14 | Sleep tracking | Low | Medium | P3 |
| 15 | Equipment tracking | Low | Low | P3 |
| 16 | Course/navigation data | Low | Medium | P3 |
| 17 | Weather data (extended) | Low | Low | P3 |
| 18 | Segment efforts | Low | Medium | P3 |
| 19 | Body composition (extended) | Low | Low | P3 |
| 20 | Developer custom fields | Low | Medium | P3 |
| 21 | CTL/ATL/TSB tracking | Low | Medium | P3 |
| 22 | Sport-specific settings | Low | Low | P3 |

---

## 10. Implementation Roadmap

### Phase 1: Core Telemetry (v2.1) - CRITICAL
**Goal:** Enable basic FIT activity import with telemetry

- [ ] Time-series record data structure
- [ ] GPS coordinate storage (inline or external)
- [ ] Enhanced session/lap hierarchy
- [ ] Device information tracking
- [ ] Basic event messages

**Estimated effort:** 4-6 weeks
**Coverage improvement:** 40% → 60%

---

### Phase 2: Advanced Metrics (v2.2) - IMPORTANT
**Goal:** Support performance analysis metrics

- [ ] Training Effect metrics (aerobic/anaerobic/VO2max)
- [ ] Power-based metrics (NP/IF/TSS)
- [ ] Running dynamics
- [ ] Swimming-specific metrics (SWOLF, stroke types)
- [ ] Time in zone distribution
- [ ] Zone configuration schema

**Estimated effort:** 3-4 weeks
**Coverage improvement:** 60% → 80%

---

### Phase 3: Multi-Sport & Structure (v2.3) - IMPORTANT
**Goal:** Full multi-sport and workout structure support

- [ ] Multi-sport session model
- [ ] Length messages for swimming
- [ ] Structured workout execution tracking
- [ ] Target compliance metrics
- [ ] HRV data storage

**Estimated effort:** 4-5 weeks
**Coverage improvement:** 80% → 90%

---

### Phase 4: Ecosystem Features (v3.0) - OPTIONAL
**Goal:** Complete FIT parity and ecosystem integration

- [ ] Equipment tracking
- [ ] Course/navigation data
- [ ] Segment efforts
- [ ] Extended weather data
- [ ] Sleep tracking integration
- [ ] Developer custom fields
- [ ] Fitness metrics (CTL/ATL/TSB)

**Estimated effort:** 6-8 weeks
**Coverage improvement:** 90% → 98%

---

## 11. Technical Considerations

### 11.1 File Size Impact

**Time-series data will significantly increase file sizes:**

- Current PWF history: ~5-20 KB per workout (text YAML)
- With second-by-second GPS/telemetry: 500 KB - 5 MB per workout
- 1-hour workout @ 1Hz = 3,600 records × 10-15 fields = ~36,000 data points

**Mitigation strategies:**
1. **External file references**: Store time-series in separate CSV/GPX/TCX files
2. **Compression**: YAML → gzip (70-80% reduction)
3. **Binary format**: Consider optional binary storage alongside YAML
4. **Sampling**: Allow configurable sampling rates (1Hz, 10Hz, smart)
5. **Selective export**: User chooses summary vs. full telemetry

---

### 11.2 Backward Compatibility

**All new features must be optional:**

```yaml
history_version: 2  # Existing v2 files remain valid
```

**Extended v2 with optional fields:**
```yaml
history_version: 2
workouts:
  - date: "2025-12-20"
    exercises: [...]
    # New optional fields
    sessions: [...]  # Multi-sport (optional)
    telemetry: [...]  # Existing
    time_series_file: "..."  # New (optional)
```

**Major version bump for breaking changes:**
```yaml
history_version: 3  # For incompatible changes
```

---

### 11.3 Schema Versioning Strategy

**Recommendation: Incremental v2.x releases**

- v2.1: Time-series + GPS + device info
- v2.2: Advanced metrics + zones
- v2.3: Multi-sport + structured workouts
- v3.0: Breaking changes (if needed)

**Schema evolution:**
```json
{
  "history_version": 2,
  "schema_version": "2.2.0",  // Semantic versioning
  "extensions": {
    "time_series": "enabled",
    "multi_sport": "disabled",
    "developer_fields": "disabled"
  }
}
```

---

## 12. Testing Strategy

### 12.1 FIT File Test Suite

**Required test files:**
1. Simple running activity (5 km steady state)
2. Cycling with power meter (intervals)
3. Pool swimming (mixed strokes, drills)
4. Multi-sport triathlon (swim/bike/run)
5. Indoor trainer (ERG mode, structured workout)
6. Trail running (elevation, GPS tracks)
7. Long ride with multiple devices (HRM, power, cadence)
8. Open water swimming (GPS, no pool length)
9. Strength training (if recorded via Garmin/COROS)

**Test data sources:**
- Garmin sample FIT files (official SDK)
- Real-world user contributions
- Generated test files via FIT SDK

---

### 12.2 Conversion Testing

**Bi-directional conversion tests:**

```bash
# FIT → PWF → FIT
fit_to_pwf input.fit output.yaml
pwf_to_fit output.yaml roundtrip.fit
compare_fit_files input.fit roundtrip.fit --tolerance=0.1%

# Verify lossless conversion
assert_equal(input.fit.records.length, roundtrip.fit.records.length)
assert_near(input.fit.session.avg_power, roundtrip.fit.session.avg_power, delta: 1)
```

**Coverage targets:**
- Critical fields: 100% lossless
- Important fields: 95% lossless
- Optional fields: Best-effort, document known gaps

---

## 13. Migration Guide for Existing Apps

### 13.1 For PWF Consumers

**Apps reading PWF v2 will continue to work:**
```rust
// Existing code works
let workout = parse_pwf_v2(file)?;
println!("Date: {}", workout.date);

// New optional fields
if let Some(sessions) = workout.sessions {
    println!("Multi-sport activity: {} sessions", sessions.len());
}
```

**New features are additive, not breaking.**

---

### 13.2 For PWF Exporters

**Apps can incrementally adopt new features:**

```python
# Minimal v2 export (existing)
pwf = {
    'history_version': 2,
    'exported_at': datetime.now().isoformat(),
    'workouts': [...]
}

# Enhanced v2.1 export
pwf = {
    'history_version': 2,
    'schema_version': '2.1.0',
    'workouts': [
        {
            'date': '2025-12-20',
            'exercises': [...],
            'time_series_file': 'workout-001.csv',  # NEW
            'devices': [...]  # NEW
        }
    ]
}
```

---

## 14. References & Sources

### Official Documentation
- [FIT SDK - Garmin Developers](https://developer.garmin.com/fit/)
- [FIT Protocol Specification](https://developer.garmin.com/fit/protocol)
- [FIT File Types](https://developer.garmin.com/fit/file-types/)
- [FIT Cookbook](https://developer.garmin.com/fit/cookbook/decoding-activity-files/)

### Technical Resources
- [FIT File Structure for Dummies](https://www.pinns.co.uk/osm/fit-for-dummies.html)
- [FIT File Description - Suunto](https://apizone.suunto.com/fit-description)
- [Reading FIT Files - FITfileR](https://msmith.de/FITfileR/articles/FITfileR.html)
- [FIT Format Specification - THIS IS ANT](https://www.thisisant.com/forum/viewthread/4275)

### Training Metrics
- [Normalized Power, IF, TSS - TrainingPeaks](https://www.trainingpeaks.com/learn/articles/normalized-power-intensity-factor-training-stress/)
- [Training Zones Guide - Joe Friel](https://www.trainingpeaks.com/learn/articles/joe-friel-s-quick-guide-to-setting-zones/)
- [Core TrainingPeaks Metrics](https://www.procyclingcoaching.com/post/core-trainingpeaks-metrics-fitness-form-fatigue)

### Sport-Specific
- [SWOLF Swimming Metrics](https://sporttracks.mobi/blog/what-is-swolf-swimming-metrics-explained)
- [Swimming Metrics Explained](https://www.wareable.com/swimming/swimming-metrics-explained-understand-stats-4430)
- [Running Dynamics - Garmin](https://thewiredrunner.com/ground-contact-vertical-oscillation/)
- [Advanced Pool Swim Analysis](https://sporttracks.mobi/blog/advanced-pool-swim-analysis-using-sporttracks)

### Device Integration
- [Developer Data Fields - FIT SDK](https://developer.garmin.com/fit/cookbook/developer-data/)
- [FitContributor API](https://developer.garmin.com/connect-iq/api-docs/Toybox/FitContributor.html)
- [Course Points Guide](https://muddytweed.com/2024/05/13/garmin-tips-1-the-power-of-course-points/)

### Recovery & Physiology
- [Body Battery Explained](https://fitstraps.co.uk/blogs/news/body-battery-explained-how-does-garmin-work-out-body-battery)
- [HRV Analysis - Firstbeat](https://www.firstbeat.com/wp-content/uploads/2015/10/How-to-Analyze-Stress-from-Heart-Rate-Variability.pdf)
- [Wearable Sleep Tracking Research](https://pmc.ncbi.nlm.nih.gov/articles/PMC10195711/)

---

## 15. Appendix: Example FIT → PWF Mappings

### A. Simple Running Activity

**FIT Structure:**
```
Activity:
  Sessions: 1
  Laps: 5
  Records: 1847 (second-by-second)

Session:
  sport: running
  total_distance: 5000m
  avg_heart_rate: 165 bpm
  avg_speed: 3.47 m/s
  total_calories: 425

Record (×1847):
  timestamp, position_lat, position_long, heart_rate,
  speed, cadence, altitude, distance
```

**PWF Mapping:**
```yaml
workouts:
  - date: "2025-12-20"
    duration_sec: 1847
    telemetry:
      total_distance_m: 5000
      heart_rate_avg: 165
      speed_avg_mps: 3.47
      cadence_avg: 172
      total_calories: 425
    exercises:
      - name: "Run"
        modality: running
        sets:  # Laps mapped to sets
          - set_number: 1
            duration_sec: 369
            distance_meters: 1000
            telemetry: {...}
          # ... 4 more laps

    # Time-series data (external file)
    time_series_file: "run-20251220.csv"
    # CSV: timestamp,lat,lon,hr,speed,cadence,altitude,distance
```

---

### B. Cycling Power Intervals

**FIT Structure:**
```
Session:
  sport: cycling
  normalized_power: 265W
  training_stress_score: 95
  intensity_factor: 0.88
  time_in_power_zone_1: 600s
  time_in_power_zone_2: 1200s
  time_in_power_zone_3: 480s
  time_in_power_zone_4: 720s
```

**PWF Mapping:**
```yaml
workouts:
  - telemetry:
      power_avg: 245
      power_max: 420
    power_metrics:
      normalized_power: 265
      intensity_factor: 0.88
      training_stress_score: 95
      time_in_power_zones:
        zone_1_sec: 600
        zone_2_sec: 1200
        zone_3_sec: 480
        zone_4_sec: 720
```

---

### C. Triathlon Multi-Sport

**FIT Structure:**
```
Activity: multisport
  Session 1: swimming (1800s, 1500m)
  Session 2: transition (120s)
  Session 3: cycling (3600s, 40km)
  Session 4: transition (90s)
  Session 5: running (2400s, 10km)
```

**PWF Mapping:**
```yaml
workouts:
  - activity_type: multisport
    sessions:
      - session_number: 1
        sport: swimming
        duration_sec: 1800
        exercises: [...]
      - session_number: 2
        sport: transition
        duration_sec: 120
      - session_number: 3
        sport: cycling
        duration_sec: 3600
        exercises: [...]
      - session_number: 4
        sport: transition
        duration_sec: 90
      - session_number: 5
        sport: running
        duration_sec: 2400
        exercises: [...]
```

---

## 16. Next Steps

1. **Community Review** (2 weeks)
   - Share this analysis with PWF users/contributors
   - Gather feedback on priorities
   - Identify missing use cases

2. **Specification Design** (3 weeks)
   - Draft schema changes for Phase 1
   - Create example YAML files
   - Update JSON Schema

3. **Prototype Implementation** (4 weeks)
   - Build FIT parser in pwf-core
   - Implement Phase 1 features
   - Test with real FIT files

4. **Beta Release** (v2.1-beta)
   - Gather real-world feedback
   - Iterate on schema
   - Fix edge cases

5. **Production Release** (v2.1)
   - Finalize documentation
   - Update validation library
   - Release FIT conversion tools

---

**Document Status:** Draft for Discussion
**Feedback Welcome:** Open GitHub issue or PR to suggest changes
