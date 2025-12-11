# PWF Specification

This document provides a complete reference for the Portable Workout Format (PWF) format.

## Version

This documentation covers **PWF v1.0**.

## File Format

PWF plans are written in [YAML 1.2](https://yaml.org/spec/1.2.2/). Files should use the `.yaml` or `.yml` extension.

## Document Structure

A PWF document consists of three top-level elements:

```yaml
plan_version: 1          # Required - Specification version
meta: { ... }            # Optional - Plan metadata
cycle: { ... }           # Required - Training cycle
```

## Block Reference

| Block | Description | Documentation |
|-------|-------------|---------------|
| Root | Top-level plan structure | [plan.md](blocks/plan.md) |
| `meta` | Plan metadata and display info | [meta.md](blocks/meta.md) |
| `cycle` | Training cycle container | [cycle.md](blocks/cycle.md) |
| `day` | Single training day | [day.md](blocks/day.md) |
| `exercise` | Individual exercise | [exercise.md](blocks/exercise.md) |

## Modalities

PWF supports four exercise modalities:

| Modality | Use Case | Documentation |
|----------|----------|---------------|
| `strength` | Sets × reps training | [modalities.md#strength](modalities.md#strength) |
| `countdown` | Fixed duration timer | [modalities.md#countdown](modalities.md#countdown) |
| `stopwatch` | Open-ended timing | [modalities.md#stopwatch](modalities.md#stopwatch) |
| `interval` | Repeating work periods | [modalities.md#interval](modalities.md#interval) |

## Validation

### Errors vs Warnings

- **Errors** indicate invalid plans that cannot be imported
- **Warnings** indicate suboptimal plans that will import but may not work as expected

### Strict Mode

When validating with `--strict`, warnings are treated as errors. This is recommended for CI/CD pipelines.

## Type Reference

| Type | Description | Example |
|------|-------------|---------|
| `string` | UTF-8 text | `"Bench Press"` |
| `integer` | Whole number | `3`, `60` |
| `number` | Decimal number | `400.5` |
| `boolean` | True or false | `true`, `false` |
| `date` | ISO 8601 date | `"2025-01-06"` |
| `url` | HTTPS URL | `"https://example.com"` |
| `array` | List of values | `[a, b, c]` |

## Complete Example

```yaml
plan_version: 1

meta:
  id: "beginner-strength-v1"
  title: "Beginner Strength Program"
  description: "A 3-day full body routine for new lifters."
  author: "Coach Atlas"
  equipment: [barbell, dumbbells, bench]
  daysPerWeek: 3
  recommendedFirst: true
  tags: [beginner, strength, full-body]

cycle:
  start_date: "2025-01-06"
  notes: "Focus on form over weight. Rest 2-3 minutes between sets."

  days:
    - id: "day-a"
      order: 0
      focus: "Full Body A"
      target_session_length_min: 60
      exercises:
        - name: "Barbell Back Squat"
          modality: strength
          target_sets: 3
          target_reps: 5
          target_notes: "Depth: hip crease below knee"

        - name: "Bench Press"
          modality: strength
          target_sets: 3
          target_reps: 5

        - name: "Plank"
          modality: countdown
          target_duration_sec: 45

    - id: "day-b"
      order: 1
      focus: "Full Body B"
      exercises:
        - name: "Deadlift"
          modality: strength
          target_sets: 1
          target_reps: 5

        - name: "Overhead Press"
          modality: strength
          target_sets: 3
          target_reps: 5
```

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-01 | Initial release |
