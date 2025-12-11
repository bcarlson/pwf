# Plan (Root Block)

The root block defines the top-level structure of a PWF document.

## Example Usage

```yaml
plan_version: 1

meta:
  title: "My Training Plan"
  equipment: [dumbbells]

cycle:
  days:
    - focus: "Day 1"
      exercises:
        - name: "Push-ups"
          modality: strength
          target_sets: 3
          target_reps: 10
```

## Argument Reference

The following arguments are supported:

### `plan_version`

- **Type:** `integer`
- **Required:** Yes
- **Valid Values:** `1`

The specification version this plan conforms to. Currently, only version `1` is supported.

```yaml
plan_version: 1
```

> **Note:** Future versions of PWF may introduce breaking changes. The `plan_version` field ensures validators and importers handle plans correctly.

---

### `meta`

- **Type:** [Meta Block](meta.md)
- **Required:** No

Metadata about the plan including title, author, and equipment requirements. While optional, including `meta` is strongly recommended for discoverability.

```yaml
meta:
  title: "Beginner Strength"
  author: "Coach Smith"
```

See [Meta Block](meta.md) for all available arguments.

---

### `cycle`

- **Type:** [Cycle Block](cycle.md)
- **Required:** Yes

The training cycle containing all workout days. Every plan must have exactly one `cycle` block.

```yaml
cycle:
  days:
    - focus: "Day 1"
      exercises:
        - name: "Squat"
          modality: strength
```

See [Cycle Block](cycle.md) for all available arguments.

## Validation Rules

| Rule | Severity | Message |
|------|----------|---------|
| Missing `plan_version` | Error | `plan_version is required` |
| `plan_version` not 1 | Error | `Unsupported plan_version: {n}` |
| Missing `cycle` | Error | `cycle is required` |
| Missing `meta` | Warning | `Missing meta section - plan will have no title` |

## Minimal Valid Plan

The smallest valid PWF document:

```yaml
plan_version: 1
cycle:
  days:
    - exercises:
        - name: "Push-ups"
          modality: strength
```
