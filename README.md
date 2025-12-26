# PWF - Portable Workout Format

A human-readable, YAML-based specification for defining workout plans and exporting workout history.

[![CI](https://github.com/bcarlson/pwf/actions/workflows/ci.yaml/badge.svg)](https://github.com/bcarlson/pwf/actions/workflows/ci.yaml)
[![npm](https://img.shields.io/npm/v/%40pwf-dev%2Fcore)](https://www.npmjs.com/package/@pwf-dev/core)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Overview

**PWF** (Portable Workout Format) is an open standard for:

1. **Plans** - Workout templates that define exercises, sets, reps, and modalities
2. **History** - Completed workout exports including PRs and body measurements

Both formats are portable, version-controllable YAML files that can be shared across any fitness application.

### Philosophy

- **Your data, your choice.** PWF exists so you can move your workout data wherever you want.
- **Unopinionated by design.** The spec defines structure, not workflow. Adopters decide how to use it.
- **Language agnostic.** Works with any programming language (it's just YAML). Content can be in any human language (UTF-8).

## Quick Example

### Plan (Workout Template)

```yaml
plan_version: 1

meta:
  title: "Beginner Strength"
  equipment: [barbell, dumbbells]
  daysPerWeek: 3

cycle:
  days:
    - focus: "Full Body A"
      exercises:
        - name: "Squat"
          modality: strength
          target_sets: 3
          target_reps: 5

        - name: "Plank"
          modality: countdown
          target_duration_sec: 60
```

### History (Workout Export)

```yaml
history_version: 1
exported_at: "2025-01-15T10:30:00Z"

workouts:
  - date: "2025-01-15"
    title: "Push Day"
    exercises:
      - name: "Bench Press"
        sets:
          - reps: 5
            weight_kg: 100
            rpe: 8

personal_records:
  - exercise_name: "Bench Press"
    record_type: max_weight
    value: 100
    achieved_at: "2025-01-15"
```

## Why PWF?

| Problem | Solution |
|---------|----------|
| Proprietary formats lock you in | Open spec, export anywhere |
| No validation until import fails | CLI catches errors before import |
| Plans can't be version controlled | YAML files work with Git |
| Hard to share plans between apps | Standard format all apps can read |
| Workout history trapped in one app | Export your complete history |

## Web Interface

**Try it now**: [https://bcarlson.github.io/pwf/](https://bcarlson.github.io/pwf/)

The PWF Web Tools provide a browser-based interface with no installation required:

### Features

- **Validator** - Paste or upload YAML files for instant validation with syntax highlighting
- **Converter** - Convert between FIT, TCX, GPX, CSV, and PWF formats in your browser
- **Visualizer** - View GPS routes on interactive maps, telemetry charts, and plan structure
- **Plan Builder** - Visual workout plan creator featuring:
  - Step-by-step wizard interface (Plan Info → Days → Exercises → Review)
  - Pre-built templates (5×5 strength, PPL split, HIIT cardio, calisthenics)
  - Drag-and-drop exercise reordering
  - Custom template library (save/load/delete)
  - YAML import/export
  - Shareable plan links (URL-based with compression)

All tools run entirely in your browser using WebAssembly. **No data is uploaded to servers** - everything happens client-side.

## CLI Installation

### Download Binary

```bash
# macOS (Apple Silicon)
curl -sSL https://github.com/bcarlson/pwf/releases/latest/download/pwf-darwin-arm64 -o pwf
chmod +x pwf
sudo mv pwf /usr/local/bin/

# macOS (Intel)
curl -sSL https://github.com/bcarlson/pwf/releases/latest/download/pwf-darwin-amd64 -o pwf
chmod +x pwf
sudo mv pwf /usr/local/bin/

# Linux (x64)
curl -sSL https://github.com/bcarlson/pwf/releases/latest/download/pwf-linux-amd64 -o pwf
chmod +x pwf
sudo mv pwf /usr/local/bin/
```

### From Source

```bash
cargo install --git https://github.com/bcarlson/pwf.git pwf
```

## Usage

### Validate Plans

```bash
# Validate a plan file
pwf validate my-plan.yaml

# Strict mode (warnings become errors)
pwf validate --strict my-plan.yaml

# JSON output for CI/CD
pwf validate --format json plans/*.yaml
```

### Validate History Exports

```bash
# Validate a history export
pwf history my-export.yaml

# JSON output
pwf history --format json exports/*.yaml
```

### Generate Templates

```bash
# Create a new plan template
pwf init plan.yaml

# Create a history export template
pwf init --history export.yaml
```

### Convert Formats

```bash
# Convert FIT file (Garmin/COROS/Wahoo) to PWF
pwf convert --from fit --to pwf activity.fit workout.yaml

# Convert TCX file (Training Center XML) to PWF
pwf convert --from tcx --to pwf activity.tcx workout.yaml

# Summary only (skip GPS time-series data)
pwf convert --from tcx --to pwf --summary-only activity.tcx workout.yaml

# Verbose output (show conversion warnings and progress)
pwf convert --from fit --to pwf --verbose activity.fit workout.yaml

# Validate the converted file
pwf history workout.yaml
```

**Supported conversions:**
- FIT → PWF (Garmin, COROS, Wahoo, Polar, Suunto devices)
- TCX → PWF (Training Center XML - Garmin Connect, Strava exports)
- Includes: GPS routes, power metrics, swimming data, multi-sport activities, heart rate telemetry

See [`crates/pwf-converters/README.md`](crates/pwf-converters/README.md) for detailed conversion documentation.

### Example Output

```
$ pwf validate beginner-strength.yaml

✓ beginner-strength.yaml
  3 days, 12 exercises
  ⚠ cycle.days[1].exercises[2]: Strength exercise missing target_sets/target_reps

$ pwf history workout-export.yaml

✓ workout-export.yaml
  15 workouts, 187 sets, 45,230 kg total volume
  Date range: 2025-01-01 to 2025-01-31
```

## Documentation

### Specification

| Document | Description |
|----------|-------------|
| [Specification](docs/SPECIFICATION.md) | Complete format overview |

### Plan Blocks

| Block | Description |
|-------|-------------|
| [Plan (Root)](docs/blocks/plan.md) | Top-level plan structure |
| [Meta](docs/blocks/meta.md) | Plan metadata and lifecycle tracking |
| [Glossary](docs/blocks/glossary.md) | Exercise terminology definitions |
| [Cycle](docs/blocks/cycle.md) | Training cycle |
| [Day](docs/blocks/day.md) | Training day |
| [Exercise](docs/blocks/exercise.md) | Exercise definition |

### History Blocks

| Block | Description |
|-------|-------------|
| [History (Root)](docs/blocks/history.md) | Top-level export structure |
| [Workout](docs/blocks/workout.md) | Completed workout session |

### Reference

| Document | Description |
|----------|-------------|
| [Modalities](docs/modalities.md) | Exercise types (strength, countdown, etc.) |
| [Equipment](docs/equipment.md) | Standard equipment tags |

## Modalities

PWF supports four exercise modalities:

| Modality | Use Case | Key Fields |
|----------|----------|------------|
| `strength` | Sets × reps training | `target_sets`, `target_reps` |
| `countdown` | Fixed duration timer | `target_duration_sec` |
| `stopwatch` | Open-ended timing | `target_duration_sec` (optional) |
| `interval` | Repeating work periods | `target_sets`, `target_duration_sec` |

## JSON Schema

JSON Schema files are provided for editor autocompletion and programmatic validation:

- [`schema/pwf-v1.json`](schema/pwf-v1.json) - Plan schema
- [`schema/pwf-history-v1.json`](schema/pwf-history-v1.json) - History export schema

### VS Code Setup

Add to your `.vscode/settings.json`:

```json
{
  "yaml.schemas": {
    "./schema/pwf-v1.json": ["**/plans/*.yaml"],
    "./schema/pwf-history-v1.json": ["**/exports/*.yaml"]
  }
}
```

## Roadmap

### v1.4.0 (Current - Released 2025-12-25)
- [x] **Web Interface** (https://bcarlson.github.io/pwf/)
  - Browser-based validator with syntax highlighting
  - Visual plan builder with templates and drag-and-drop
  - Format converter UI (FIT/TCX/GPX/CSV ↔ PWF)
  - GPS map visualizer and telemetry charts
  - Shareable plan links with URL compression
  - Custom template library
  - WASM-powered, client-side only (no server uploads)

### v1.3.0 (Released 2025-12-23)
- [x] **GPX Format Support** (GPS Exchange Format)
  - GPX → PWF import for GPS tracks
  - PWF → GPX export for route sharing
  - Full CLI integration
- [x] **CSV Export** (Time-series data for spreadsheets)
  - Export telemetry data to CSV for analysis
  - Compatible with Excel, Google Sheets
- [x] **Expanded Sport Type Mappings**
  - 22 sport types (up from 7)
  - Comprehensive FIT, TCX, and GPX mappings

### v1.2.0 (Released 2025-12-23)
- [x] **TCX Format Conversion (Bidirectional)**
  - TCX → PWF import (Training Center XML)
  - PWF → TCX export (Garmin Connect, Strava compatible)
  - GPS routes, heart rate, power, cadence support
- [x] **FIT Export Analysis**
  - Comprehensive evaluation of FIT writing options
  - Documented alternatives and workarounds
- [x] **Enhanced Test Coverage**
  - 729 tests (93.84% coverage)

### v1.1.0 (Released 2025-12-22)
- [x] **FIT Format Import**
  - FIT → PWF conversion (Garmin, COROS, Wahoo, Polar, Suunto)
  - Multi-sport activities, swimming data, power metrics
- [x] **pwf-converters Library**
  - Format conversion infrastructure
- [x] **Superset/circuit grouping**
- [x] **Rest period specifications**

### v1.0.1 (Released 2025-12-21)
- [x] **PWF v2.1 Advanced Features**
  - Time-series telemetry, GPS route tracking
  - Multi-sport segments, pool swimming
- [x] **Multi-week periodization**
- [x] **Exercise library references**

### Future Considerations
- [ ] **FIT Export** (if production-ready Rust library emerges)
  - Monitor ecosystem for FIT writing libraries
  - Currently recommended: use TCX export
- [ ] **Mobile Apps** (iOS/Android)
  - Native mobile applications
  - Offline-first sync capabilities

See [CHANGELOG.md](CHANGELOG.md) for detailed release notes.

## Adopters

- [OwnLift](https://ownlift.app) - Privacy-first workout tracking

*Using PWF? [Open a PR](https://github.com/bcarlson/pwf/pulls) to add your app!*

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - See [LICENSE](LICENSE)

---

*Your training data should be portable and yours to own.*
