# PWF - Portable Workout Format

A human-readable, YAML-based specification for defining workout plans and exporting workout history.

[![CI](https://github.com/bcarlson/pwf/actions/workflows/ci.yaml/badge.svg)](https://github.com/bcarlson/pwf/actions/workflows/ci.yaml)
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

## Installation

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

### v1.0.1 (Current)
- [x] Plan specification with lifecycle tracking
- [x] History export specification
- [x] Four modalities (strength, countdown, stopwatch, interval)
- [x] Personal records and body measurements
- [x] Rep-specific PR types (1RM, 3RM, 5RM, 8RM, 10RM)
- [x] RPE and RIR tracking on sets
- [x] Plan status with timestamps (draft/active/completed/archived)
- [x] Plan lifecycle timestamps (activated_at, completed_at)
- [x] Glossary block for terminology definitions
- [x] Preferred units in exports with validation
- [x] Comprehensive error codes (PWF-P### and PWF-H###)
- [x] JSON Schema validation
- [x] Rust CLI validator

### v1.1 (Planned)
- [ ] Superset/circuit grouping
- [ ] Rest period specifications
- [ ] Percentage-based loading

### v2.0 (Future)
- [ ] Multi-week periodization
- [ ] Progressive overload rules
- [ ] Exercise library references
- [ ] Workout templates (reusable exercise groups)

## Adopters

- [OwnLift](https://ownlift.com) - Privacy-first workout tracking

*Using PWF? [Open a PR](https://github.com/bcarlson/pwf/pulls) to add your app!*

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - See [LICENSE](LICENSE)

---

*Your training data should be portable and yours to own.*
