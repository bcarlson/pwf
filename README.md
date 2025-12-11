# WPS - Workout Plan Specification

A human-readable, YAML-based specification for defining workout plans and exporting workout history.

[![CI](https://github.com/bcarlson/workout-plan-spec/actions/workflows/ci.yaml/badge.svg)](https://github.com/bcarlson/workout-plan-spec/actions/workflows/ci.yaml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Overview

**WPS** (Workout Plan Specification) is an open standard for:

1. **Plans** - Workout templates that define exercises, sets, reps, and modalities
2. **History** - Completed workout exports including PRs and body measurements

Both formats are portable, version-controllable YAML files that can be shared across any fitness application.

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

## Why WPS?

| Problem | Solution |
|---------|----------|
| Proprietary formats lock you in | Open spec, export anywhere |
| No validation until import fails | CLI catches errors before import |
| Plans can't be version controlled | YAML files work with Git |
| Hard to share plans between apps | Standard format all apps can read |
| Workout history trapped in one app | Export your complete history |

## Installation

### Download Binary

```bash
# macOS (Apple Silicon)
curl -sSL https://github.com/bcarlson/workout-plan-spec/releases/latest/download/wps-darwin-arm64 -o wps
chmod +x wps
sudo mv wps /usr/local/bin/

# macOS (Intel)
curl -sSL https://github.com/bcarlson/workout-plan-spec/releases/latest/download/wps-darwin-amd64 -o wps
chmod +x wps
sudo mv wps /usr/local/bin/

# Linux (x64)
curl -sSL https://github.com/bcarlson/workout-plan-spec/releases/latest/download/wps-linux-amd64 -o wps
chmod +x wps
sudo mv wps /usr/local/bin/
```

### From Source

```bash
cargo install --git https://github.com/bcarlson/workout-plan-spec.git wps
```

## Usage

### Validate Plans

```bash
# Validate a plan file
wps validate my-plan.yaml

# Strict mode (warnings become errors)
wps validate --strict my-plan.yaml

# JSON output for CI/CD
wps validate --format json plans/*.yaml
```

### Validate History Exports

```bash
# Validate a history export
wps history my-export.yaml

# JSON output
wps history --format json exports/*.yaml
```

### Generate Templates

```bash
# Create a new plan template
wps init plan.yaml

# Create a history export template
wps init --history export.yaml
```

### Example Output

```
$ wps validate beginner-strength.yaml

✓ beginner-strength.yaml
  3 days, 12 exercises
  ⚠ cycle.days[1].exercises[2]: Strength exercise missing target_sets/target_reps

$ wps history workout-export.yaml

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
| [Meta](docs/blocks/meta.md) | Plan metadata |
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

WPS supports four exercise modalities:

| Modality | Use Case | Key Fields |
|----------|----------|------------|
| `strength` | Sets × reps training | `target_sets`, `target_reps` |
| `countdown` | Fixed duration timer | `target_duration_sec` |
| `stopwatch` | Open-ended timing | `target_duration_sec` (optional) |
| `interval` | Repeating work periods | `target_sets`, `target_duration_sec` |

## JSON Schema

JSON Schema files are provided for editor autocompletion and programmatic validation:

- [`schema/wps-v1.json`](schema/wps-v1.json) - Plan schema
- [`schema/wps-history-v1.json`](schema/wps-history-v1.json) - History export schema

### VS Code Setup

Add to your `.vscode/settings.json`:

```json
{
  "yaml.schemas": {
    "./schema/wps-v1.json": ["**/plans/*.yaml"],
    "./schema/wps-history-v1.json": ["**/exports/*.yaml"]
  }
}
```

## Roadmap

### v1.0 (Current)
- [x] Plan specification
- [x] History export specification
- [x] Four modalities (strength, countdown, stopwatch, interval)
- [x] Personal records and body measurements
- [x] JSON Schema
- [x] Rust CLI validator

### v1.1 (Planned)
- [ ] Superset/circuit grouping
- [ ] Rest period specifications
- [ ] RPE/RIR targets
- [ ] Percentage-based loading

### v2.0 (Future)
- [ ] Multi-week periodization
- [ ] Progressive overload rules
- [ ] Exercise library references
- [ ] Workout templates (reusable exercise groups)

## Adopters

- [OwnLift](https://ownlift.com) - Privacy-first workout tracking

*Using WPS? [Open a PR](https://github.com/bcarlson/workout-plan-spec/pulls) to add your app!*

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - See [LICENSE](LICENSE)

---

*Your training data should be portable and yours to own.*
