# @pwf/core

TypeScript/JavaScript core library for parsing, validating, and generating PWF files in Node.js and browser environments.

## Install (local workspace)

```bash
npm install
npm run build
```

Generate API docs:
```bash
npm run docs
```

## Usage

```ts
import {
  parsePlan,
  parseHistory,
  validatePlan,
  validateHistory,
  PlanBuilder,
  isValidationIssueList,
  toYAML
} from '@pwf/core';

const plan = parsePlan(yamlString);
const history = parseHistory(yamlString);

if (!isValidationIssueList(plan)) {
  const planErrors = validatePlan(plan);
  console.log(planErrors);
}

if (!isValidationIssueList(history)) {
  const historyErrors = validateHistory(history, { strict: false });
  console.log(historyErrors);
}

const builtPlan = new PlanBuilder()
  .version(1)
  .meta({ title: 'My Plan', daysPerWeek: 3 })
  .addDay('Push Day')
  .addExercise('Bench Press', {
    modality: 'strength',
    target_sets: 3,
    target_reps: 5
  })
  .build();

const yaml = builtPlan.toYAML();
const yamlAlt = toYAML(builtPlan);
```

IIFE bundles expose `PwfCore` when loaded via a `<script>` tag.

## Notes

- Validation uses the PWF JSON Schemas in `schema/`.
- Parser methods return either a parsed object or a list of validation issues.
- Error paths are formatted in a dotted/array syntax (e.g. `cycle.days[0].exercises[0].name`).
- Regenerate TypeScript types from schema with `npm run generate:types`.
