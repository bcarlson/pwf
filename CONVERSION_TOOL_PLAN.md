# PWF Conversion Tool - Implementation Plan

**Issue:** #23 - Conversion tool to/from FIT and TCX formats
**Status:** Planning (Awaiting v2.1 Feature Parity)
**Date:** 2025-12-20

---

## Prerequisites

**CRITICAL:** This conversion tool requires PWF v2.1 feature parity to be implemented first.

Without the following P0 features, conversions will be lossy:
- ✅ Time-series record data (issue #24 P0-1)
- ✅ GPS position data (issue #24 P0-2)
- ✅ Multi-sport activity support (issue #24 P0-3)
- ✅ Length messages for pool swimming (issue #24 P0-4)

**Current Status:** v2.1 P0 features are being implemented now. This conversion tool should be started AFTER those features are completed and tested.

---

## Overview

The conversion tool will enable bidirectional conversion between PWF and industry-standard fitness file formats:
- **FIT** (.fit) - Binary format from Garmin, COROS, Wahoo (PRIMARY)
- **TCX** (.tcx) - XML format from Garmin (SECONDARY)
- **GPX** (.gpx) - GPS-focused format (TERTIARY)
- **CSV** (.csv) - Spreadsheet format for inspection (OPTIONAL)

## Architecture

### Crate Structure

```
pwf-converters/           # New crate in workspace
├── src/
│   ├── lib.rs           # Public API
│   ├── fit/             # FIT format support
│   │   ├── mod.rs
│   │   ├── parser.rs    # FIT → PWF
│   │   ├── writer.rs    # PWF → FIT
│   │   ├── types.rs     # FIT type mappings
│   │   └── validation.rs
│   ├── tcx/             # TCX format support
│   │   ├── mod.rs
│   │   ├── parser.rs    # TCX → PWF
│   │   ├── writer.rs    # PWF → TCX
│   │   └── types.rs
│   ├── gpx/             # GPX format support
│   │   ├── mod.rs
│   │   ├── parser.rs    # GPX → PWF
│   │   └── writer.rs    # PWF → GPX
│   └── csv/             # CSV export support
│       ├── mod.rs
│       └── exporter.rs  # PWF → CSV
├── Cargo.toml
└── tests/
    ├── fit_roundtrip_tests.rs
    ├── tcx_roundtrip_tests.rs
    └── conversion_accuracy_tests.rs
```

### Dependencies

```toml
[dependencies]
fitparser = "0.5"           # FIT SDK wrapper
quick-xml = "0.31"          # XML parsing for TCX
gpx = "0.9"                 # GPX parsing
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"          # PWF parsing
thiserror = "1.0"           # Error handling
chrono = "0.4"              # Timestamp conversion
csv = "1.3"                 # CSV export

[dev-dependencies]
assert_cmd = "2.0"          # CLI testing
tempfile = "3.8"            # Test files
```

---

## CLI Interface

### Command Structure

```bash
# Convert FIT → PWF
pwf convert --from fit --to pwf input.fit output.yaml

# Convert PWF → FIT
pwf convert --from pwf --to fit input.yaml output.fit

# Convert TCX → PWF
pwf convert --from tcx --to pwf workout.tcx output.yaml

# Convert PWF → GPX (GPS tracks only)
pwf convert --from pwf --to gpx workout.yaml track.gpx

# Export to CSV for inspection
pwf convert --from pwf --to csv workout.yaml data.csv
```

### Options

```bash
# Strict mode - fail on data loss warnings
pwf convert --from fit --to pwf --strict input.fit output.yaml

# Verbose mode - show conversion mappings
pwf convert --from fit --to pwf --verbose input.fit output.yaml

# Dry run - validate without writing
pwf convert --from fit --to pwf --dry-run input.fit output.yaml

# Include time-series data (default: true)
pwf convert --from fit --to pwf --no-timeseries input.fit output.yaml

# Output format for telemetry (inline, external, gpx)
pwf convert --from fit --to pwf --telemetry-format external input.fit output.yaml
```

---

## Conversion Mappings

### FIT → PWF Mapping

#### Activity/Session Level
| FIT Field | PWF Field | Notes |
|-----------|-----------|-------|
| `session.start_time` | `workout.started_at` | ISO 8601 timestamp |
| `session.total_elapsed_time` | `workout.duration_sec` | Seconds |
| `session.sport` | `workout.activity_type` or `session.sport` | Single or multi-sport |
| `session.sub_sport` | `workout.notes` | Freeform text |
| `session.total_distance` | `workout.telemetry.total_distance_m` | Meters |
| `session.total_calories` | `workout.telemetry.total_calories` | kcal |
| `session.avg_heart_rate` | `workout.telemetry.heart_rate_avg` | bpm |
| `session.max_heart_rate` | `workout.telemetry.heart_rate_max` | bpm |
| `session.avg_power` | `workout.telemetry.power_avg` | watts |
| `session.max_power` | `workout.telemetry.power_max` | watts |
| `session.total_ascent` | `workout.telemetry.total_elevation_gain_m` | meters |
| `session.total_descent` | `workout.telemetry.total_elevation_loss_m` | meters |

#### Lap → Set Mapping
| FIT Field | PWF Field | Notes |
|-----------|-----------|-------|
| `lap.start_time` | `set.completed_at` | ISO 8601 |
| `lap.total_elapsed_time` | `set.duration_sec` | Seconds |
| `lap.total_distance` | `set.distance_meters` | Meters |
| `lap.avg_heart_rate` | `set.telemetry.heart_rate_avg` | bpm |
| `lap.avg_power` | `set.telemetry.power_avg` | watts |
| `lap.avg_cadence` | `set.telemetry.cadence_avg` | steps/min or RPM |

#### Record → Time-Series Mapping
| FIT Field | PWF Field | Notes |
|-----------|-----------|-------|
| `record.timestamp` | `time_series.start_timestamp` + offset | Calculated |
| `record.heart_rate` | `time_series.heart_rate[i]` | Array index |
| `record.power` | `time_series.power[i]` | Array index |
| `record.cadence` | `time_series.cadence[i]` | Array index |
| `record.speed` | `time_series.speed_mps[i]` | m/s |
| `record.altitude` | `time_series.altitude_m[i]` | meters |
| `record.distance` | `time_series.distance_m[i]` | Cumulative meters |
| `record.position_lat` | `gps_track.coordinates[i].lat` | Degrees |
| `record.position_long` | `gps_track.coordinates[i].lon` | Degrees |
| `record.enhanced_altitude` | `gps_track.coordinates[i].alt` | Meters (0.2m precision) |

#### Swimming Length → PWF Length
| FIT Field | PWF Field | Notes |
|-----------|-----------|-------|
| `length.start_time` | `lengths[i].timestamp` | Calculated |
| `length.total_elapsed_time` | `lengths[i].duration_sec` | Seconds |
| `length.swim_stroke` | `lengths[i].stroke_type` | Map to enum |
| `length.total_strokes` | `lengths[i].stroke_count` | Count |
| Calculated | `lengths[i].swolf` | duration_sec + stroke_count |
| `length.length_type` | `set.drill_mode` | active vs. rest |

#### Multi-Sport Sessions
| FIT Field | PWF Field | Notes |
|-----------|-----------|-------|
| `session[0].sport` | `sessions[0].sport` | Swim |
| `session[1].sport` | `sessions[1].sport` | Transition |
| `session[2].sport` | `sessions[2].sport` | Bike |
| `session[3].sport` | `sessions[3].sport` | Transition |
| `session[4].sport` | `sessions[4].sport` | Run |

### PWF → FIT Mapping

Reverse mapping follows the same table above. Key considerations:
- **Exercise → Lap**: PWF exercises map to FIT laps (1 exercise = 1+ laps)
- **Set → Lap**: PWF sets map to FIT laps (1:1 mapping)
- **Time-series → Records**: Generate 1Hz records from PWF time-series arrays

---

## Conversion Strategies

### Strategy 1: Full Fidelity (Default)

```bash
pwf convert --from fit --to pwf input.fit output.yaml
```

**Behavior:**
- Convert ALL FIT messages to PWF
- Store time-series data in external CSV files
- Store GPS tracks in external GPX files
- Result: ~100% data preservation, multiple output files

**Output Structure:**
```
output.yaml                    # Workout metadata
output-set-1-timeseries.csv    # Time-series for set 1
output-set-2-timeseries.csv    # Time-series for set 2
output-set-1-gps.gpx           # GPS track for set 1
```

### Strategy 2: Summary Only

```bash
pwf convert --from fit --to pwf --summary-only input.fit output.yaml
```

**Behavior:**
- Convert aggregate metrics only (avg, max, min)
- Skip time-series and GPS data
- Result: Single compact YAML file (~5-20 KB)

**Use Case:** Training log without detailed analysis

### Strategy 3: Inline (Small Workouts)

```bash
pwf convert --from fit --to pwf --telemetry-format inline input.fit output.yaml
```

**Behavior:**
- Embed time-series and GPS data in YAML
- Result: Single large YAML file (500KB - 5MB)

**Use Case:** Short workouts (<30 min), easy sharing

---

## Data Loss Handling

### Warning System

When converting between formats, warn users about potential data loss:

```
⚠️ Conversion Warnings:
  - FIT field 'normalized_power' has no PWF equivalent (will be lost in roundtrip)
  - FIT field 'training_effect' has no PWF equivalent (will be lost in roundtrip)
  - PWF field 'rpe' has no FIT equivalent (will be lost in roundtrip)

✅ Converted successfully with 3 warnings
```

### Strict Mode

```bash
pwf convert --from fit --to pwf --strict input.fit output.yaml
```

In strict mode, warnings become errors. Conversion fails if ANY data would be lost.

**Exit codes:**
- `0` - Success, no data loss
- `1` - Success, but with warnings (lossy conversion)
- `2` - Failure, validation error
- `3` - Failure, strict mode blocked lossy conversion

---

## Testing Strategy

### Unit Tests

Test individual converters in isolation:

```rust
#[test]
fn test_fit_session_to_pwf_workout() {
    let fit_session = FitSession { /* ... */ };
    let pwf_workout = convert_session_to_workout(&fit_session);
    assert_eq!(pwf_workout.duration_sec, Some(3600));
}
```

### Roundtrip Tests

Ensure lossless conversion for critical fields:

```rust
#[test]
fn test_fit_to_pwf_to_fit_roundtrip() {
    let original_fit = load_test_fit("examples/cycling.fit");
    let pwf = fit_to_pwf(&original_fit);
    let roundtrip_fit = pwf_to_fit(&pwf);

    assert_eq_within_tolerance(
        original_fit.session.avg_heart_rate,
        roundtrip_fit.session.avg_heart_rate,
        1.0  // ±1 bpm tolerance
    );
}
```

### Integration Tests

Test full CLI commands:

```rust
#[test]
fn test_cli_fit_to_pwf_conversion() {
    Command::cargo_bin("pwf")
        .unwrap()
        .args(&["convert", "--from", "fit", "--to", "pwf", "test.fit", "output.yaml"])
        .assert()
        .success();

    // Validate output
    let pwf: WpsHistory = read_yaml("output.yaml");
    assert_eq!(pwf.history_version, 2);
    assert!(pwf.workouts.len() > 0);
}
```

### Test Dataset

Collect real-world FIT files for testing:
1. **Cycling with power** (5+ data streams)
2. **Running with GPS** (lat/lon/ele + HR)
3. **Pool swimming** (25m pool, multiple strokes)
4. **Open water swimming** (GPS-based)
5. **Triathlon** (3 sports + transitions)
6. **Indoor cycling** (ERG mode, no GPS)
7. **Strength training** (if supported by device)

---

## Implementation Phases

### Phase 1: FIT → PWF (Read-Only)

**Duration:** 2-3 weeks

**Tasks:**
1. Set up `pwf-converters` crate
2. Integrate `fitparser` library
3. Implement FIT→PWF mappings for:
   - Session → Workout
   - Lap → Set
   - Record → Time-series
   - Length → Swimming lengths
   - Multi-sport sessions
4. Write comprehensive tests
5. Add `pwf convert --from fit` CLI command

**Deliverable:** Can import FIT files to PWF format

### Phase 2: PWF → FIT (Write Support)

**Duration:** 2-3 weeks

**Tasks:**
1. Implement PWF→FIT reverse mappings
2. Generate FIT files using `fitparser` SDK
3. Handle edge cases (missing fields, type conversions)
4. Add roundtrip tests
5. Add `pwf convert --to fit` CLI command

**Deliverable:** Can export PWF to FIT format

### Phase 3: TCX Support

**Duration:** 1-2 weeks

**Tasks:**
1. Implement TCX XML parsing (quick-xml)
2. Map TCX → PWF (similar to FIT)
3. Map PWF → TCX
4. Add tests
5. Add CLI commands

**Deliverable:** TCX ↔ PWF conversion

### Phase 4: GPX & CSV Support

**Duration:** 1 week

**Tasks:**
1. Implement GPX → PWF (GPS tracks only)
2. Implement PWF → GPX (extract GPS data)
3. Implement PWF → CSV (time-series export)
4. Add tests
5. Add CLI commands

**Deliverable:** GPX ↔ PWF and CSV export

### Phase 5: Polish & Documentation

**Duration:** 1 week

**Tasks:**
1. Add progress indicators for large files
2. Optimize performance (stream processing)
3. Write user documentation
4. Create conversion examples
5. Add error recovery

**Deliverable:** Production-ready conversion tool

---

## Performance Targets

**Conversion Speed:**
- Small workout (<100 records): <100ms
- Medium workout (1000 records): <500ms
- Large workout (10,000 records): <5 seconds
- Very large workout (100,000 records): <30 seconds

**Memory Usage:**
- Streaming parser: <50 MB RAM for any file size
- Avoid loading entire file into memory

**File Size:**
- External time-series CSV: 70-80% smaller with gzip
- Recommend automatic compression for files >100KB

---

## Error Handling

### Recoverable Errors

Try to continue conversion even with errors:

```
⚠️ Warning: Record 1523 has invalid heart rate (300 bpm), clamping to 220 bpm
⚠️ Warning: GPS coordinate out of range (lat: 95.0), skipping point
✅ Conversion completed with 2 warnings
```

### Non-Recoverable Errors

Fail fast for critical issues:

```
❌ Error: FIT file is corrupted (invalid CRC checksum)
❌ Error: Missing required field: session.start_time
```

### Validation Integration

Always validate output PWF files:

```bash
pwf convert --from fit --to pwf input.fit output.yaml
# Automatically runs: pwf validate output.yaml
```

---

## Documentation

### User Guide

Create `docs/conversion-guide.md`:
- Supported formats
- Command examples
- Data loss warnings
- Best practices
- Troubleshooting

### Developer Guide

Create `docs/converter-development.md`:
- Architecture overview
- Adding new formats
- Mapping reference
- Testing guidelines

---

## Success Criteria

**v1.0 Release:**
- ✅ FIT ↔ PWF conversion (>95% data preservation)
- ✅ TCX ↔ PWF conversion (>90% data preservation)
- ✅ GPX → PWF conversion (GPS data only)
- ✅ PWF → CSV export (time-series data)
- ✅ Comprehensive test suite (>90% coverage)
- ✅ CLI with --from/--to flags
- ✅ User documentation
- ✅ Roundtrip tests passing

**Performance:**
- 10,000 record FIT file converts in <5 seconds
- 100MB FIT file uses <100MB RAM

**Quality:**
- No panics/crashes on malformed input
- Helpful error messages
- Progress indicators for large files

---

## Timeline

**Estimated Total Effort:** 8-10 weeks

| Phase | Duration | Dependencies |
|-------|----------|--------------|
| Phase 1: FIT→PWF | 2-3 weeks | v2.1 P0 features complete ✅ |
| Phase 2: PWF→FIT | 2-3 weeks | Phase 1 complete |
| Phase 3: TCX | 1-2 weeks | Phase 2 complete |
| Phase 4: GPX/CSV | 1 week | Phase 3 complete |
| Phase 5: Polish | 1 week | Phase 4 complete |

**Target Release:** Q2 2026 (after PWF v2.1 stable)

---

## Open Questions

1. **FIT SDK Licensing:** Is `fitparser` crate sufficient, or do we need official Garmin FIT SDK?
2. **Binary vs. Text Output:** Should we support binary FIT output directly, or only via external tools?
3. **Compression:** Automatically compress large external files, or leave to user?
4. **Batch Conversion:** Support `pwf convert --batch *.fit` for bulk imports?
5. **GUI Tool:** Is a graphical converter needed, or CLI sufficient?

---

## Next Steps

1. ✅ Complete PWF v2.1 P0 features (in progress)
2. ⏳ Test and validate v2.1 features
3. ⏳ Start Phase 1: FIT→PWF converter
4. ⏳ Create test dataset with real FIT files
5. ⏳ Write conversion guide documentation

---

**Document Status:** Draft Plan (Awaiting v2.1 Feature Completion)
**Feedback:** Open GitHub issue for discussion
