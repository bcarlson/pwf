/**
 * Tests for YAML generation from PlanDraft objects
 */

import { describe, it, expect } from 'vitest';
import {
  generateYAML,
  validateDraftStructure,
  createMinimalPlan,
  createAllModalitiesPlan,
  createEndurancePlan,
} from '../utils/yamlGenerator';
import type { PlanDraft } from '../../../lib/builderState';
import { parse } from 'yaml';

describe('YAML Generation', () => {
  describe('generateYAML', () => {
    it('should generate valid YAML for minimal plan', () => {
      const draft = createMinimalPlan();
      const yaml = generateYAML(draft);

      expect(yaml).toBeTruthy();
      expect(yaml).toContain('plan_version: 1');
      expect(yaml).toContain('name: Minimal Plan');
      expect(yaml).toContain('Squats');
      expect(yaml).toContain('modality: strength');

      // Should be parseable
      const parsed = parse(yaml);
      expect(parsed).toBeTruthy();
      expect(parsed.plan_version).toBe(1);
    });

    it('should generate YAML for plan with all 8 modalities', () => {
      const draft = createAllModalitiesPlan();
      const yaml = generateYAML(draft);

      expect(yaml).toBeTruthy();
      expect(yaml).toContain('modality: strength');
      expect(yaml).toContain('modality: countdown');
      expect(yaml).toContain('modality: stopwatch');
      expect(yaml).toContain('modality: interval');
      expect(yaml).toContain('modality: emom');
      expect(yaml).toContain('modality: amrap');
      expect(yaml).toContain('modality: tabata');
      expect(yaml).toContain('modality: endurance');

      // Should be parseable
      const parsed = parse(yaml);
      expect(parsed).toBeTruthy();
      expect(parsed.cycle.days[0].exercises).toHaveLength(8);
    });

    it('should generate YAML for complex endurance workouts with zones', () => {
      const draft = createEndurancePlan();
      const yaml = generateYAML(draft);

      expect(yaml).toBeTruthy();
      expect(yaml).toContain('athlete_profile:');
      expect(yaml).toContain('ftp_watts: 250');
      expect(yaml).toContain('threshold_hr_bpm: 165');
      expect(yaml).toContain('glossary:');
      expect(yaml).toContain('target_power_zone: 2');
      expect(yaml).toContain('target_hr_zone: 2');
      expect(yaml).toContain('sport: cycling');

      // Should be parseable
      const parsed = parse(yaml);
      expect(parsed).toBeTruthy();
      expect(parsed.meta.athlete_profile).toBeTruthy();
      expect(parsed.glossary).toBeTruthy();
    });

    it('should clean empty and undefined fields', () => {
      const draft: PlanDraft = {
        plan_version: 1,
        meta: {
          name: 'Test Plan',
          description: '',  // Empty string should be removed
          author: undefined,  // Undefined should be removed
        },
        cycle: {
          days: [
            {
              focus: 'Test',
              exercises: [
                {
                  name: 'Exercise',
                  modality: 'strength',
                  target_sets: 3,
                  target_reps: 10,
                  notes: '',  // Empty string should be removed
                  equipment: [],  // Empty array should be removed
                },
              ],
            },
          ],
        },
      };

      const yaml = generateYAML(draft);

      expect(yaml).not.toContain('description:');
      expect(yaml).not.toContain('author:');
      expect(yaml).not.toContain('notes:');
      expect(yaml).not.toContain('equipment:');
    });

    it('should handle glossary correctly', () => {
      const draft: PlanDraft = {
        plan_version: 1,
        meta: {
          name: 'Glossary Test',
        },
        glossary: {
          'RPE': 'Rate of Perceived Exertion',
          'AMRAP': 'As Many Rounds As Possible',
        },
        cycle: {
          days: [
            {
              exercises: [
                {
                  name: 'Test',
                  modality: 'strength',
                  target_sets: 1,
                  target_reps: 1,
                },
              ],
            },
          ],
        },
      };

      const yaml = generateYAML(draft);

      expect(yaml).toContain('glossary:');
      expect(yaml).toContain('RPE:');
      expect(yaml).toContain('Rate of Perceived Exertion');
      expect(yaml).toContain('AMRAP:');
    });

    it('should handle multiple days with varying exercises', () => {
      const draft: PlanDraft = {
        plan_version: 1,
        meta: {
          name: 'Multi-Day Plan',
        },
        cycle: {
          days: [
            {
              focus: 'Upper Body',
              exercises: [
                {
                  name: 'Bench Press',
                  modality: 'strength',
                  target_sets: 3,
                  target_reps: 8,
                },
                {
                  name: 'Pull-ups',
                  modality: 'strength',
                  target_sets: 3,
                  target_reps: 10,
                },
              ],
            },
            {
              focus: 'Lower Body',
              exercises: [
                {
                  name: 'Squats',
                  modality: 'strength',
                  target_sets: 4,
                  target_reps: 6,
                },
              ],
            },
            {
              focus: 'Rest',
              exercises: [],
            },
          ],
        },
      };

      const yaml = generateYAML(draft);
      const parsed = parse(yaml);

      expect(parsed.cycle.days).toHaveLength(3);
      expect(parsed.cycle.days[0].exercises).toHaveLength(2);
      expect(parsed.cycle.days[1].exercises).toHaveLength(1);
      // Empty exercise days should still appear
      expect(parsed.cycle.days[2]).toBeTruthy();
    });

    it('should preserve equipment array when present', () => {
      const draft: PlanDraft = {
        plan_version: 1,
        meta: {
          name: 'Equipment Test',
          equipment: ['barbell', 'dumbbells'],
        },
        cycle: {
          days: [
            {
              exercises: [
                {
                  name: 'Bench Press',
                  modality: 'strength',
                  target_sets: 3,
                  target_reps: 10,
                  equipment: ['barbell', 'bench'],
                },
              ],
            },
          ],
        },
      };

      const yaml = generateYAML(draft);

      expect(yaml).toContain('equipment:');
      expect(yaml).toContain('- barbell');
      expect(yaml).toContain('- dumbbells');
      expect(yaml).toContain('- bench');
    });
  });

  describe('validateDraftStructure', () => {
    it('should validate minimal valid plan', () => {
      const draft = createMinimalPlan();
      const result = validateDraftStructure(draft);

      expect(result.valid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    it('should reject plan without plan_version', () => {
      const draft: any = {
        cycle: {
          days: [{ exercises: [] }],
        },
      };

      const result = validateDraftStructure(draft);

      expect(result.valid).toBe(false);
      expect(result.errors).toContain('plan_version is required');
    });

    it('should reject plan without cycle', () => {
      const draft: any = {
        plan_version: 1,
      };

      const result = validateDraftStructure(draft);

      expect(result.valid).toBe(false);
      expect(result.errors).toContain('cycle is required');
    });

    it('should reject plan with empty days array', () => {
      const draft: PlanDraft = {
        plan_version: 1,
        cycle: {
          days: [],
        },
      };

      const result = validateDraftStructure(draft);

      expect(result.valid).toBe(false);
      expect(result.errors).toContain('cycle must have at least one day');
    });
  });

  describe('Template Plans', () => {
    it('should create minimal plan', () => {
      const plan = createMinimalPlan();

      expect(plan.plan_version).toBe(1);
      expect(plan.meta?.name).toBe('Minimal Plan');
      expect(plan.cycle.days).toHaveLength(1);
      expect(plan.cycle.days[0].exercises).toHaveLength(1);
    });

    it('should create all modalities plan', () => {
      const plan = createAllModalitiesPlan();

      expect(plan.plan_version).toBe(1);
      expect(plan.cycle.days).toHaveLength(1);
      expect(plan.cycle.days[0].exercises).toHaveLength(8);

      const modalities = plan.cycle.days[0].exercises.map(e => e.modality);
      expect(modalities).toContain('strength');
      expect(modalities).toContain('countdown');
      expect(modalities).toContain('stopwatch');
      expect(modalities).toContain('interval');
      expect(modalities).toContain('emom');
      expect(modalities).toContain('amrap');
      expect(modalities).toContain('tabata');
      expect(modalities).toContain('endurance');
    });

    it('should create endurance plan with zones', () => {
      const plan = createEndurancePlan();

      expect(plan.plan_version).toBe(1);
      expect(plan.meta?.athlete_profile).toBeTruthy();
      expect(plan.meta?.athlete_profile?.ftp_watts).toBe(250);
      expect(plan.glossary).toBeTruthy();
      expect(plan.cycle.days).toHaveLength(2);

      // Check zone-based exercises
      const exercises = plan.cycle.days.flatMap(d => d.exercises);
      const hasZones = exercises.some(e => e.target_power_zone || e.target_hr_zone);
      expect(hasZones).toBe(true);
    });
  });

  describe('Edge Cases', () => {
    it('should handle minimal plan with meta', () => {
      const draft: PlanDraft = {
        plan_version: 1,
        meta: {
          name: 'Meta Only',
          description: 'Just metadata',
        },
        cycle: {
          days: [
            {
              exercises: [
                {
                  name: 'Squat',
                  modality: 'strength',
                  target_sets: 3,
                  target_reps: 5,
                },
              ],
            },
          ],
        },
      };

      const yaml = generateYAML(draft);
      const parsed = parse(yaml);

      expect(parsed.meta.name).toBe('Meta Only');
      expect(parsed.cycle.days).toHaveLength(1);
      expect(parsed.cycle.days[0].exercises).toHaveLength(1);
    });

    it('should handle exercise with all optional fields', () => {
      const draft: PlanDraft = {
        plan_version: 1,
        cycle: {
          days: [
            {
              exercises: [
                {
                  name: 'Complex Exercise',
                  modality: 'endurance',
                  sport: 'cycling',
                  target_duration_sec: 3600,
                  target_distance_meters: 50000,
                  target_power_watts: 200,
                  target_power_zone: 3,
                  target_hr_bpm: 150,
                  target_hr_zone: 3,
                  target_pace_sec_per_km: 240,
                  notes: 'Detailed notes',
                  equipment: ['bike', 'power meter'],
                },
              ],
            },
          ],
        },
      };

      const yaml = generateYAML(draft);
      const parsed = parse(yaml);

      const exercise = parsed.cycle.days[0].exercises[0];
      expect(exercise.name).toBe('Complex Exercise');
      expect(exercise.target_duration_sec).toBe(3600);
      expect(exercise.target_power_zone).toBe(3);
      expect(exercise.equipment).toContain('bike');
    });
  });
});
