# Meta Block

The `meta` block contains metadata about the plan used for display, search, and organization.

## Example Usage

```yaml
meta:
  id: "strength-program-v2"
  title: "Intermediate Strength Program"
  description: "A 4-day upper/lower split for lifters with 6+ months experience."
  author: "Coach Atlas"
  equipment: [barbell, dumbbells, bench, pullup_bar]
  daysPerWeek: 4
  recommendedFirst: false
  tags: [intermediate, strength, upper-lower]
```

## Argument Reference

### `id`

- **Type:** `string`
- **Required:** No
- **Default:** Auto-generated UUID

A unique identifier for the plan. If not provided, a UUID will be generated during import. Providing an explicit ID enables deterministic imports and updates.

```yaml
meta:
  id: "my-custom-plan-id"
```

> **Best Practice:** Use kebab-case identifiers like `beginner-strength-v1` for readability.

---

### `title`

- **Type:** `string`
- **Required:** Yes (when `meta` is present)
- **Max Length:** 80 characters

The display name of the plan. This is shown in plan browsers and selection screens.

```yaml
meta:
  title: "30-Minute Full Body"
```

---

### `description`

- **Type:** `string`
- **Required:** No

A brief description of the plan's purpose, target audience, or methodology.

```yaml
meta:
  description: "Quick and effective full-body routine for busy schedules."
```

---

### `author`

- **Type:** `string`
- **Required:** No

The coach, trainer, or creator of the plan.

```yaml
meta:
  author: "Coach Smith"
```

---

### `equipment`

- **Type:** `array` of `string`
- **Required:** No
- **Default:** `[]`

Equipment required to complete the plan. Using [standard equipment tags](../equipment.md) ensures consistent display across applications.

```yaml
meta:
  equipment: [barbell, dumbbells, bench]
```

See [Equipment Tags](../equipment.md) for the complete list of standard tags.

---

### `daysPerWeek`

- **Type:** `integer`
- **Required:** No
- **Default:** Count of days in `cycle.days`
- **Valid Range:** `1` to `7`

The intended training frequency. This may differ from the actual day count if some days are optional or alternating.

```yaml
meta:
  daysPerWeek: 3
```

---

### `recommendedFirst`

- **Type:** `boolean`
- **Required:** No
- **Default:** `false`

Whether this plan should be suggested to new users. Useful for marking beginner-friendly or introductory plans.

```yaml
meta:
  recommendedFirst: true
```

---

### `tags`

- **Type:** `array` of `string`
- **Required:** No
- **Default:** `[]`

Searchable tags for categorization. Non-string values are filtered out during parsing.

```yaml
meta:
  tags: [strength, hypertrophy, beginner, home-workout]
```

## Validation Rules

| Rule | Severity | Message |
|------|----------|---------|
| Empty `title` | Error | `meta.title cannot be empty` |
| `title` > 80 chars | Error | `meta.title exceeds 80 characters` |
| `daysPerWeek` < 1 or > 7 | Warning | `daysPerWeek should be between 1 and 7` |

## Complete Example

```yaml
meta:
  id: "kettlebell-fundamentals"
  title: "Kettlebell Fundamentals"
  description: "Master the basics of kettlebell training with this 3-day program."
  author: "OwnLift"
  equipment: [kettlebell]
  daysPerWeek: 3
  recommendedFirst: true
  tags: [kettlebell, beginner, conditioning, home-workout]
```
