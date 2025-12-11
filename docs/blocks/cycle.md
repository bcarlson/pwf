# Cycle Block

The `cycle` block defines a training cycle containing one or more workout days.

## Example Usage

```yaml
cycle:
  start_date: "2025-01-06"
  notes: "Week 1 - Focus on form. Keep weights light."

  days:
    - focus: "Full Body A"
      exercises:
        - name: "Squat"
          modality: strength
          target_sets: 3
          target_reps: 5

    - focus: "Full Body B"
      exercises:
        - name: "Deadlift"
          modality: strength
          target_sets: 1
          target_reps: 5
```

## Argument Reference

### `start_date`

- **Type:** `string` (ISO 8601 date)
- **Required:** No
- **Format:** `YYYY-MM-DD`

The planned start date for this training cycle. Used for scheduling and calendar integration.

```yaml
cycle:
  start_date: "2025-01-06"
```

> **Note:** Applications may ignore this field if the user starts the plan on a different date.

---

### `notes`

- **Type:** `string`
- **Required:** No

Coaching notes that apply to the entire cycle. Displayed at the cycle level, not repeated for each day.

```yaml
cycle:
  notes: "Deload week - reduce all weights by 40%"
```

---

### `days`

- **Type:** `array` of [Day Block](day.md)
- **Required:** Yes
- **Min Items:** 1

The training days in this cycle. Days are executed in the order defined by their `order` field, or by array position if `order` is not specified.

```yaml
cycle:
  days:
    - focus: "Push"
      exercises: [...]
    - focus: "Pull"
      exercises: [...]
    - focus: "Legs"
      exercises: [...]
```

See [Day Block](day.md) for all available arguments.

## Validation Rules

| Rule | Severity | Message |
|------|----------|---------|
| Missing `days` | Error | `cycle.days is required` |
| Empty `days` array | Error | `cycle.days must have at least 1 day` |
| Invalid `start_date` format | Warning | `start_date should be YYYY-MM-DD format` |

## Nested Blocks

The `cycle` block contains the following nested blocks:

- [Day Block](day.md) - One or more training days

## Multi-Week Cycles (Future)

> **Note:** PWF v1.0 supports single-cycle plans. Multi-week periodization with multiple cycles is planned for v2.0.

Current workaround for multi-week programs:

```yaml
cycle:
  notes: "4-week program. Repeat cycle, adding 5lbs each week."
  days:
    # Week 1-4 days defined here
    - focus: "Week 1 - Day 1"
      exercises: [...]
```
