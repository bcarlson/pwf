import { describe, expect, it } from 'vitest';
import {
  fromYAML,
  isValidationIssueList,
  parseHistory,
  parsePlan,
  PlanBuilder,
  toYAML,
  validateHistory,
  validatePlan
} from './index';

const validPlanYaml = `plan_version: 1
cycle:
  days:
    - exercises:
        - name: Squat
          modality: strength
          target_sets: 3
          target_reps: 5
`;

const validHistoryYaml = `history_version: 1
exported_at: "2025-01-15T10:30:00Z"
workouts:
  - date: "2025-01-15"
    title: "Push Day"
    exercises:
      - name: "Bench Press"
        sets:
          - reps: 5
            weight_kg: 100
`;

describe('pwf-core validation', () => {
  it('parses and validates a plan', () => {
    const result = parsePlan(validPlanYaml);
    expect(Array.isArray(result)).toBe(false);
  });

  it('returns validation issues for invalid plan data', () => {
    const issues = validatePlan({});
    expect(issues.length).toBeGreaterThan(0);
    expect(issues[0]?.severity).toBe('error');
  });

  it('parses and validates a history export', () => {
    const result = parseHistory(validHistoryYaml);
    expect(Array.isArray(result)).toBe(false);
  });

  it('returns validation issues for invalid history data', () => {
    const issues = validateHistory({});
    expect(issues.length).toBeGreaterThan(0);
  });

  it('handles invalid yaml', () => {
    const result = parsePlan('plan_version: [1');
    expect(Array.isArray(result)).toBe(true);
  });

  it('formats missing required paths for plans', () => {
    const issues = validatePlan({ plan_version: 1, cycle: { days: [{}] } });
    expect(issues.some((issue) => issue.path === 'cycle.days[0].exercises')).toBe(true);
  });

  it('formats additional properties in plan paths', () => {
    const issues = validatePlan({
      plan_version: 1,
      cycle: {
        days: [
          {
            exercises: [
              {
                name: 'Squat',
                modality: 'strength'
              }
            ],
            'extra-field': true
          }
        ]
      }
    });

    expect(issues.some((issue) => issue.path === "cycle.days[0]['extra-field']")).toBe(true);
  });

  it('formats root additional properties paths', () => {
    const issues = validatePlan({
      plan_version: 1,
      cycle: {
        days: [
          {
            exercises: [
              {
                name: 'Squat',
                modality: 'strength'
              }
            ]
          }
        ]
      },
      'foo-bar': true
    });

    expect(issues.some((issue) => issue.path === "['foo-bar']")).toBe(true);
  });

  it('detects validation issue lists', () => {
    const issues = validatePlan({});
    expect(isValidationIssueList(issues)).toBe(true);

    const plan = parsePlan(validPlanYaml);
    if (!isValidationIssueList(plan)) {
      expect(isValidationIssueList(plan)).toBe(false);
    }
  });

  it('parses invalid history yaml as issues', () => {
    const result = parseHistory('history_version: [1');
    expect(Array.isArray(result)).toBe(true);
  });

  it('returns validation issues for invalid history yaml content', () => {
    const result = parseHistory('history_version: 1\nexported_at: "2025-01-15T10:30:00Z"');
    expect(isValidationIssueList(result)).toBe(true);
  });

  it('parses and stringifies yaml', () => {
    const parsed = fromYAML(validPlanYaml);
    const yaml = toYAML(parsed);
    expect(yaml).toContain('plan_version');
  });

  it('builds a plan and serializes to yaml', () => {
    const builder = new PlanBuilder()
      .version(1)
      .meta({ title: 'Plan' })
      .addDay('Day 1')
      .addExercise('Bench Press', {
        modality: 'strength',
        target_sets: 3,
        target_reps: 5
      });

    const plan = builder.build();
    const yaml = plan.toYAML();

    expect(yaml).toContain('plan_version');
    expect(yaml).toContain('Bench Press');
  });

  it('throws when adding an exercise without a day', () => {
    const builder = new PlanBuilder();
    expect(() =>
      builder.addExercise('Bench Press', {
        modality: 'strength',
        target_sets: 3,
        target_reps: 5
      })
    ).toThrow();
  });

  it('returns no issues for a valid plan', () => {
    const plan = parsePlan(validPlanYaml);
    if (isValidationIssueList(plan)) {
      throw new Error('Expected a valid plan');
    }

    const issues = validatePlan(plan);
    expect(issues).toEqual([]);
  });

  it('returns no issues for a valid history', () => {
    const history = parseHistory(validHistoryYaml);
    if (isValidationIssueList(history)) {
      throw new Error('Expected a valid history');
    }

    const issues = validateHistory(history);
    expect(issues).toEqual([]);
  });

  it('returns validation issues for invalid plan yaml content', () => {
    const result = parsePlan('plan_version: 1');
    expect(isValidationIssueList(result)).toBe(true);
  });

  it('uses builder glossary and resets cycle when missing', () => {
    const builder = new PlanBuilder();
    builder.glossary({ tempo: 'Controlled pacing' });

    const internals = builder as unknown as { plan: { cycle?: { days?: unknown[] } } };
    internals.plan.cycle = undefined;

    builder
      .addDay('Day 1')
      .addExercise('Tempo Squat', {
        modality: 'strength',
        target_sets: 3,
        target_reps: 5
      });
    const plan = builder.build();
    expect(plan.glossary).toEqual({ tempo: 'Controlled pacing' });
  });

  it('serializes from builder directly', () => {
    const yaml = new PlanBuilder()
      .addDay('Day 1')
      .addExercise('Squat', {
        modality: 'strength',
        target_sets: 3,
        target_reps: 5
      })
      .toYAML();

    expect(yaml).toContain('Squat');
  });

  it('flags non-issue arrays as invalid issue lists', () => {
    expect(isValidationIssueList([{ message: 'Missing severity' }])).toBe(false);
  });

  it('adds a day without focus', () => {
    const builder = new PlanBuilder().addDay();
    expect(() => builder.build()).toThrow('requires at least one exercise');
  });

  it('initializes exercises when missing', () => {
    const builder = new PlanBuilder();
    builder.addDay('Day 1', { exercises: undefined });
    builder.addExercise('Row', {
      modality: 'strength',
      target_sets: 3,
      target_reps: 8
    });

    const plan = builder.build();
    expect(plan.cycle.days[0]?.exercises?.length).toBe(1);
  });

  it('creates a new cycle day list when missing', () => {
    const builder = new PlanBuilder();
    const internals = builder as unknown as { plan: { cycle?: { days?: unknown[] } } };
    internals.plan.cycle = {};

    builder
      .addDay('Day 1')
      .addExercise('Bench Press', {
        modality: 'strength',
        target_sets: 3,
        target_reps: 5
      });
    const plan = builder.build();
    expect(plan.cycle.days.length).toBe(1);
  });

  it('throws when building with no days', () => {
    const builder = new PlanBuilder();
    expect(() => builder.build()).toThrow('at least one day');
  });
});
