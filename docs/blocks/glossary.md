# Glossary Block

The `glossary` block is an optional root-level block that defines exercise-specific terminology, abbreviations, and concepts used in the plan. This helps users understand training concepts without requiring external documentation.

## Example Usage

```yaml
plan_version: 1

meta:
  title: "Powerlifting Program"

glossary:
  "1RM": "One Rep Max - the maximum weight you can lift for a single repetition"
  "RPE": "Rate of Perceived Exertion on a 1-10 scale, where 10 is maximal effort"
  "RIR": "Reps in Reserve - how many more reps you could complete before failure"
  "AMRAP": "As Many Reps As Possible - perform reps with good form until exhaustion"
  "Autoregulation": "Adjusting training load based on daily readiness and bar speed"

cycle:
  days: [...]
```

## Structure

The glossary is a simple key-value mapping where:

- **Key**: The term or abbreviation being defined (1-50 characters)
- **Value**: The definition or explanation (1-500 characters)

```yaml
glossary:
  "Term": "Definition of the term"
  "Abbreviation": "What this abbreviation means"
```

## Validation Rules

| Rule | Severity | Error Code | Message |
|------|----------|------------|---------|
| Glossary exceeds 100 entries | Error | PWF-P006 | `Glossary has N entries but maximum is 100` |
| Term length < 1 or > 50 characters | Error | PWF-P007 | `Glossary term must be 1-50 characters` |
| Term contains invalid characters | Error | PWF-P008 | `Glossary term must contain only alphanumeric characters, spaces, hyphens, and apostrophes` |
| Empty definition | Error | PWF-P009 | `Glossary definition cannot be empty` |
| Definition exceeds 500 characters | Error | PWF-P010 | `Glossary definition exceeds 500 characters` |

## Term Requirements

Terms must follow these rules:

- **Length**: 1-50 characters
- **Characters**: Alphanumeric, spaces, hyphens (`-`), and apostrophes (`'`) only
- **Case**: Case-sensitive (e.g., "1RM" and "1rm" are different terms)

### Valid Terms

```yaml
glossary:
  "1RM": "One Rep Max"
  "RPE": "Rate of Perceived Exertion"
  "AMRAP": "As Many Reps As Possible"
  "Smith's Method": "A training technique developed by Coach Smith"
  "Self-Myofascial Release": "Foam rolling and trigger point therapy"
```

### Invalid Terms

```yaml
glossary:
  "":  # Error: Empty term (PWF-P007)
  "A very long term that exceeds the fifty character limit for terms":  # Error: Too long (PWF-P007)
  "Special@Chars#!": "Definition"  # Error: Invalid characters (PWF-P008)
```

## Definition Requirements

Definitions must follow these rules:

- **Length**: 1-500 characters
- **Content**: Free-form text (any characters allowed)

### Valid Definitions

```yaml
glossary:
  "Progressive Overload": "Gradually increasing training stimulus over time through weight, reps, or sets to promote continued adaptation."
  "Deload Week": "A planned recovery week with reduced volume (typically 40-60% of normal) and intensity to manage fatigue and promote supercompensation."
```

### Invalid Definitions

```yaml
glossary:
  "Term": ""  # Error: Empty definition (PWF-P009)
  "Term": "A definition that is so incredibly long it exceeds the five hundred character limit which is enforced to keep definitions concise and readable..."  # Error: Exceeds 500 chars (PWF-P010)
```

## Usage Guidelines

### When to Use a Glossary

Include a glossary when your plan:

- Uses specialized terminology or abbreviations (e.g., "RIR", "1RM", "AMRAP")
- References specific training methodologies (e.g., "RPE-based autoregulation")
- Includes program-specific concepts (e.g., "Top Set", "Back-off Sets")
- Targets beginners who may not know common fitness terms

### Best Practices

1. **Be Concise**: Keep definitions clear and brief (aim for 1-2 sentences)
2. **Use Plain Language**: Avoid jargon in definitions unless absolutely necessary
3. **Alphabetize**: Consider ordering terms alphabetically for easier reference
4. **Focus on Relevance**: Only define terms actually used in your plan
5. **Stay Under 100 Entries**: If you need more, consider splitting into multiple plans

### Size Limits

- **Maximum entries**: 100 terms
- **Maximum term length**: 50 characters
- **Maximum definition length**: 500 characters

If you need more than 100 terms, consider creating separate glossary documents or linking to external documentation.

## Complete Example

```yaml
plan_version: 1

meta:
  id: "intermediate-powerlifting"
  title: "Intermediate Powerlifting Program"
  description: "12-week program focused on competition lifts with RPE-based autoregulation"
  status: draft

glossary:
  "1RM": "One Rep Max - the maximum weight you can lift for a single repetition"
  "3RM": "Three Rep Max - the maximum weight you can lift for three consecutive reps"
  "5RM": "Five Rep Max - the maximum weight you can lift for five consecutive reps"
  "AMRAP": "As Many Reps As Possible - perform reps with good form until exhaustion"
  "Autoregulation": "Adjusting training load based on daily readiness and bar speed"
  "Back-off Sets": "Lighter sets performed after the top set, typically 85-90% of top set weight"
  "Competition Lifts": "Squat, bench press, and deadlift as performed in powerlifting meets"
  "Deload": "A planned recovery week with reduced volume and intensity"
  "Progressive Overload": "Gradually increasing training stimulus over time through weight, reps, or sets"
  "RIR": "Reps in Reserve - how many more reps you could complete before failure"
  "RPE": "Rate of Perceived Exertion on a 1-10 scale, where 10 is maximal effort"
  "Top Set": "The heaviest working set of the day"

cycle:
  start_date: "2025-01-06"
  days:
    - focus: "Squat Day"
      exercises:
        - name: "Competition Squat"
          target_sets: 5
          target_reps: 5
          target_notes: "Work up to top set at RPE 8"
```

## Rendering Guidelines

Applications displaying PWF plans should:

1. **Show glossary in sidebar or help panel**: Make definitions easily accessible without cluttering the main view
2. **Highlight defined terms**: Mark terms in the plan that have glossary entries (e.g., with underline or tooltip)
3. **Provide search/filter**: Allow users to search glossary terms
4. **Sort alphabetically**: Display terms in alphabetical order by default
5. **Link to external resources**: Consider linking standard terms (like "RPE") to authoritative sources

## Integration with Other Blocks

The glossary is a standalone block and doesn't directly reference other blocks. However, terms defined in the glossary are typically used in:

- `meta.description`: Program methodology descriptions
- `cycle.notes`: Overall program guidance
- `day.notes`: Day-specific instructions
- `exercise.target_notes`: Exercise guidance
- Example: A plan might reference "RPE" in exercise notes, with the glossary providing the definition

---

For more information on plan structure, see:
- [Meta Block](meta.md) - Plan metadata and status
- [Cycle Block](cycle.md) - Training cycle structure
- [Exercise Block](exercise.md) - Exercise definitions
