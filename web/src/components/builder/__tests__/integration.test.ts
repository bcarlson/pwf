/**
 * Integration tests for Builder components
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { get } from 'svelte/store';
import { builderState } from '../../../lib/builderState';
import { generateYAML } from '../utils/yamlGenerator';
import type { PlanDraft } from '../../../lib/builderState';

// Mock WASM module
vi.mock('../../../lib/wasm', () => ({
  validatePlan: vi.fn((yaml: string) => {
    // Simple mock validation
    if (!yaml || yaml.length === 0) {
      return {
        valid: false,
        errors: [{ path: 'root', message: 'Empty YAML', severity: 'error' }],
        warnings: [],
      };
    }

    // Check for required fields
    const hasVersion = yaml.includes('plan_version');
    const hasCycle = yaml.includes('cycle');

    if (!hasVersion || !hasCycle) {
      return {
        valid: false,
        errors: [{ path: 'root', message: 'Missing required fields', severity: 'error' }],
        warnings: [],
      };
    }

    return {
      valid: true,
      errors: [],
      warnings: [],
      plan: {
        plan_version: 1,
        cycle: { days: [] },
      },
    };
  }),
}));

describe('Builder Integration Tests', () => {
  beforeEach(() => {
    // Reset builder state before each test
    builderState.reset();
  });

  describe('Wizard Navigation', () => {
    it('should start at step 0', () => {
      const state = get(builderState);
      expect(state.currentStep).toBe(0);
    });

    it('should navigate forward through steps', () => {
      builderState.nextStep();
      expect(get(builderState).currentStep).toBe(1);

      builderState.nextStep();
      expect(get(builderState).currentStep).toBe(2);

      builderState.nextStep();
      expect(get(builderState).currentStep).toBe(3);
    });

    it('should navigate backward through steps', () => {
      builderState.goToStep(3);
      expect(get(builderState).currentStep).toBe(3);

      builderState.prevStep();
      expect(get(builderState).currentStep).toBe(2);

      builderState.prevStep();
      expect(get(builderState).currentStep).toBe(1);

      builderState.prevStep();
      expect(get(builderState).currentStep).toBe(0);
    });

    it('should not go below step 0', () => {
      builderState.reset();
      expect(get(builderState).currentStep).toBe(0);

      builderState.prevStep();
      expect(get(builderState).currentStep).toBe(0);
    });

    it('should jump to specific step', () => {
      builderState.goToStep(2);
      expect(get(builderState).currentStep).toBe(2);

      builderState.goToStep(0);
      expect(get(builderState).currentStep).toBe(0);
    });
  });

  describe('Meta Step Data Management', () => {
    it('should update plan metadata', () => {
      builderState.updateMeta({
        name: 'Test Plan',
        description: 'A test workout plan',
        author: 'Test Author',
      });

      const state = get(builderState);
      expect(state.plan.meta?.name).toBe('Test Plan');
      expect(state.plan.meta?.description).toBe('A test workout plan');
      expect(state.plan.meta?.author).toBe('Test Author');
    });

    it('should merge partial meta updates', () => {
      builderState.updateMeta({ name: 'Initial Name' });
      builderState.updateMeta({ description: 'Added description' });

      const state = get(builderState);
      expect(state.plan.meta?.name).toBe('Initial Name');
      expect(state.plan.meta?.description).toBe('Added description');
    });
  });

  describe('Days Step Data Management', () => {
    it('should add days to plan', () => {
      builderState.addDay();
      const state = get(builderState);

      expect(state.plan.cycle.days).toHaveLength(2); // Starts with 1 day
    });

    it('should update day properties', () => {
      builderState.updateDay(0, { focus: 'Strength Training' });

      const state = get(builderState);
      expect(state.plan.cycle.days[0].focus).toBe('Strength Training');
    });

    it('should remove days', () => {
      builderState.addDay();
      builderState.addDay();
      expect(get(builderState).plan.cycle.days).toHaveLength(3);

      builderState.removeDay(1);
      expect(get(builderState).plan.cycle.days).toHaveLength(2);
    });

    it('should always maintain at least one day', () => {
      builderState.removeDay(0);
      const state = get(builderState);

      expect(state.plan.cycle.days).toHaveLength(1);
    });
  });

  describe('Exercises Step Data Management', () => {
    it('should add exercises to days', () => {
      builderState.addExercise(0);
      const state = get(builderState);

      expect(state.plan.cycle.days[0].exercises).toHaveLength(1);
    });

    it('should update exercise properties', () => {
      builderState.addExercise(0);
      builderState.updateExercise(0, 0, {
        name: 'Squats',
        modality: 'strength',
        target_sets: 3,
        target_reps: 10,
      });

      const state = get(builderState);
      const exercise = state.plan.cycle.days[0].exercises[0];

      expect(exercise.name).toBe('Squats');
      expect(exercise.modality).toBe('strength');
      expect(exercise.target_sets).toBe(3);
      expect(exercise.target_reps).toBe(10);
    });

    it('should remove exercises', () => {
      builderState.addExercise(0);
      builderState.addExercise(0);
      expect(get(builderState).plan.cycle.days[0].exercises).toHaveLength(2);

      builderState.removeExercise(0, 0);
      expect(get(builderState).plan.cycle.days[0].exercises).toHaveLength(1);
    });

    it('should handle exercises across multiple days', () => {
      builderState.addDay();

      builderState.addExercise(0);
      builderState.updateExercise(0, 0, { name: 'Day 1 Exercise' });

      builderState.addExercise(1);
      builderState.updateExercise(1, 0, { name: 'Day 2 Exercise' });

      const state = get(builderState);
      expect(state.plan.cycle.days[0].exercises[0].name).toBe('Day 1 Exercise');
      expect(state.plan.cycle.days[1].exercises[0].name).toBe('Day 2 Exercise');
    });
  });

  describe('Live Preview Updates', () => {
    it('should generate YAML from empty plan', () => {
      const state = get(builderState);
      const yaml = generateYAML(state.plan);

      expect(yaml).toBeTruthy();
      expect(yaml).toContain('plan_version: 1');
    });

    it('should update YAML when meta changes', () => {
      builderState.updateMeta({ name: 'Dynamic Plan' });

      const state = get(builderState);
      const yaml = generateYAML(state.plan);

      expect(yaml).toContain('name: Dynamic Plan');
    });

    it('should update YAML when exercises are added', () => {
      builderState.addExercise(0);
      builderState.updateExercise(0, 0, {
        name: 'Bench Press',
        modality: 'strength',
        target_sets: 3,
        target_reps: 8,
      });

      const state = get(builderState);
      const yaml = generateYAML(state.plan);

      expect(yaml).toContain('Bench Press');
      expect(yaml).toContain('modality: strength');
      expect(yaml).toContain('target_sets: 3');
    });

    it('should reactively update YAML for complex plan', () => {
      // Step 1: Add metadata
      builderState.updateMeta({
        name: 'Complete Plan',
        description: 'A full workout plan',
        author: 'Test Author',
      });

      // Step 2: Add multiple days
      builderState.addDay();

      // Step 3: Add exercises
      builderState.addExercise(0);
      builderState.updateExercise(0, 0, {
        name: 'Squats',
        modality: 'strength',
        target_sets: 4,
        target_reps: 6,
      });

      builderState.addExercise(1);
      builderState.updateExercise(1, 0, {
        name: 'Running',
        modality: 'endurance',
        sport: 'running',
        target_duration_sec: 1800,
      });

      const state = get(builderState);
      const yaml = generateYAML(state.plan);

      expect(yaml).toContain('Complete Plan');
      expect(yaml).toContain('Squats');
      expect(yaml).toContain('Running');
      expect(yaml).toContain('sport: running');
    });
  });

  describe('WASM Validation Integration', () => {
    it('should validate minimal plan', async () => {
      const { validatePlan } = await import('../../../lib/wasm');

      builderState.updateMeta({ name: 'Test' });
      builderState.addExercise(0);
      builderState.updateExercise(0, 0, {
        name: 'Test Exercise',
        modality: 'strength',
        target_sets: 1,
        target_reps: 1,
      });

      const state = get(builderState);
      const yaml = generateYAML(state.plan);
      const result = validatePlan(yaml);

      expect(result.valid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    it('should detect validation errors', async () => {
      const { validatePlan } = await import('../../../lib/wasm');

      const yaml = ''; // Empty YAML should fail
      const result = validatePlan(yaml);

      expect(result.valid).toBe(false);
      expect(result.errors.length).toBeGreaterThan(0);
    });
  });

  describe('State Persistence and Reset', () => {
    it('should reset to initial state', () => {
      // Make changes
      builderState.updateMeta({ name: 'Test' });
      builderState.addDay();
      builderState.addExercise(0);
      builderState.nextStep();

      // Reset
      builderState.reset();

      const state = get(builderState);
      expect(state.currentStep).toBe(0);
      expect(state.plan.meta?.name).toBeUndefined();
      expect(state.plan.cycle.days).toHaveLength(1);
      expect(state.plan.cycle.days[0].exercises).toHaveLength(0);
    });

    it('should maintain state across navigation', () => {
      builderState.updateMeta({ name: 'Persistent Plan' });
      builderState.nextStep();
      builderState.nextStep();
      builderState.prevStep();
      builderState.prevStep();

      const state = get(builderState);
      expect(state.plan.meta?.name).toBe('Persistent Plan');
    });
  });

  describe('Current Day Management', () => {
    it('should track current day index', () => {
      builderState.addDay();
      builderState.setCurrentDay(1);

      const state = get(builderState);
      expect(state.currentDayIndex).toBe(1);
    });

    it('should adjust current day when days are removed', () => {
      builderState.addDay();
      builderState.addDay();
      builderState.setCurrentDay(2);

      builderState.removeDay(2);

      const state = get(builderState);
      expect(state.currentDayIndex).toBeLessThan(state.plan.cycle.days.length);
    });
  });

  describe('Complete Workflow Simulation', () => {
    it('should simulate complete plan creation workflow', () => {
      // Step 0: Meta
      builderState.updateMeta({
        name: '4-Week Beginner Program',
        description: 'A beginner-friendly workout program',
        author: 'PWF Builder',
        days_per_week: 3,
      });
      expect(get(builderState).currentStep).toBe(0);

      // Move to Days step
      builderState.nextStep();
      expect(get(builderState).currentStep).toBe(1);

      // Add 3 days
      builderState.updateDay(0, { focus: 'Upper Body' });
      builderState.addDay();
      builderState.updateDay(1, { focus: 'Lower Body' });
      builderState.addDay();
      builderState.updateDay(2, { focus: 'Full Body' });

      // Move to Exercises step
      builderState.nextStep();
      expect(get(builderState).currentStep).toBe(2);

      // Add exercises to each day
      builderState.addExercise(0);
      builderState.updateExercise(0, 0, {
        name: 'Bench Press',
        modality: 'strength',
        target_sets: 3,
        target_reps: 10,
      });

      builderState.addExercise(1);
      builderState.updateExercise(1, 0, {
        name: 'Squats',
        modality: 'strength',
        target_sets: 3,
        target_reps: 10,
      });

      builderState.addExercise(2);
      builderState.updateExercise(2, 0, {
        name: 'Burpees',
        modality: 'amrap',
        target_duration_sec: 600,
      });

      // Move to Review step
      builderState.nextStep();
      expect(get(builderState).currentStep).toBe(3);

      // Validate final plan
      const state = get(builderState);
      const yaml = generateYAML(state.plan);

      expect(yaml).toContain('4-Week Beginner Program');
      expect(yaml).toContain('Upper Body');
      expect(yaml).toContain('Lower Body');
      expect(yaml).toContain('Full Body');
      expect(yaml).toContain('Bench Press');
      expect(yaml).toContain('Squats');
      expect(yaml).toContain('Burpees');
    });
  });
});
