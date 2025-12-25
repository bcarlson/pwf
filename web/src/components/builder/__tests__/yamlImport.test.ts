/**
 * Tests for yamlImport.ts
 */

import { describe, it, expect } from 'vitest';
import { convertToDraft, parseAndConvert } from '../utils/yamlImport';
import type { PlanDraft } from '../../../lib/builderStore';

describe('yamlImport', () => {
  describe('convertToDraft', () => {
    it('should convert a minimal valid plan', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: [
            {
              focus: 'Test Day',
              exercises: [
                {
                  name: 'Squat',
                  modality: 'strength',
                  target_sets: 3,
                  target_reps: 5
                }
              ]
            }
          ]
        }
      };

      const result = convertToDraft(yaml);

      expect(result.plan_version).toBe(1);
      expect(result.cycle?.days).toHaveLength(1);
      expect(result.cycle?.days[0].focus).toBe('Test Day');
      expect(result.cycle?.days[0].exercises).toHaveLength(1);
      expect(result.cycle?.days[0].exercises[0].name).toBe('Squat');
    });

    it('should convert a plan with meta information', () => {
      const yaml = {
        plan_version: 1,
        meta: {
          title: 'Test Plan',
          description: 'A test workout plan',
          author: 'Test Author',
          status: 'active',
          equipment: ['barbell', 'dumbbells'],
          daysPerWeek: 3,
          tags: ['strength', 'beginner']
        },
        cycle: {
          days: []
        }
      };

      const result = convertToDraft(yaml);

      expect(result.meta?.title).toBe('Test Plan');
      expect(result.meta?.description).toBe('A test workout plan');
      expect(result.meta?.author).toBe('Test Author');
      expect(result.meta?.equipment).toEqual(['barbell', 'dumbbells']);
      expect(result.meta?.daysPerWeek).toBe(3);
      expect(result.meta?.tags).toEqual(['strength', 'beginner']);
    });

    it('should convert a plan with glossary', () => {
      const yaml = {
        plan_version: 1,
        glossary: {
          'Term1': 'Definition 1',
          'Term2': 'Definition 2',
          'AMRAP': 'As Many Reps As Possible'
        },
        cycle: {
          days: []
        }
      };

      const result = convertToDraft(yaml);

      expect(result.glossary).toEqual({
        'Term1': 'Definition 1',
        'Term2': 'Definition 2',
        'AMRAP': 'As Many Reps As Possible'
      });
    });

    it('should convert a plan with cycle notes', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          notes: 'Focus on progressive overload',
          days: []
        }
      };

      const result = convertToDraft(yaml);

      expect(result.cycle?.notes).toBe('Focus on progressive overload');
    });

    it('should convert exercises with zones', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: [
            {
              focus: 'Cycling',
              exercises: [
                {
                  name: 'FTP Test',
                  modality: 'cycling',
                  zones: [
                    {
                      zone: 5,
                      duration_sec: 1200,
                      target_power_watts: 275,
                      target_hr_bpm: 170
                    }
                  ]
                }
              ]
            }
          ]
        }
      };

      const result = convertToDraft(yaml);

      const exercise = result.cycle?.days[0].exercises[0];
      expect(exercise?.name).toBe('FTP Test');
      expect(exercise?.modality).toBe('cycling');
      expect((exercise as any).zones).toEqual([
        {
          zone: 5,
          duration_sec: 1200,
          target_power_watts: 275,
          target_hr_bpm: 170
        }
      ]);
    });

    it('should convert exercises with ramps', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: [
            {
              focus: 'Rowing',
              exercises: [
                {
                  name: 'Ramp Test',
                  modality: 'rowing',
                  ramp: {
                    start_power_watts: 150,
                    end_power_watts: 300,
                    duration_sec: 900,
                    step_duration_sec: 60
                  },
                  target_notes: 'Increase 10W every minute'
                }
              ]
            }
          ]
        }
      };

      const result = convertToDraft(yaml);

      const exercise = result.cycle?.days[0].exercises[0];
      expect(exercise?.name).toBe('Ramp Test');
      expect(exercise?.modality).toBe('rowing');
      expect((exercise as any).ramp).toEqual({
        start_power_watts: 150,
        end_power_watts: 300,
        duration_sec: 900,
        step_duration_sec: 60
      });
      expect((exercise as any).target_notes).toBe('Increase 10W every minute');
    });

    it('should convert exercises with interval_phases', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: [
            {
              focus: 'Running Intervals',
              exercises: [
                {
                  name: 'Interval Set - 6x800m',
                  modality: 'running',
                  interval_phases: [
                    {
                      name: 'work',
                      duration_sec: 180,
                      target_pace_sec_per_km: 210,
                      target_hr_bpm: 175
                    },
                    {
                      name: 'recovery',
                      duration_sec: 120,
                      target_pace_sec_per_km: 360,
                      target_hr_bpm: 135
                    }
                  ],
                  target_sets: 6,
                  target_notes: '6 rounds of work + recovery'
                }
              ]
            }
          ]
        }
      };

      const result = convertToDraft(yaml);

      const exercise = result.cycle?.days[0].exercises[0];
      expect(exercise?.name).toBe('Interval Set - 6x800m');
      expect(exercise?.modality).toBe('running');
      expect((exercise as any).interval_phases).toHaveLength(2);
      expect((exercise as any).interval_phases[0].name).toBe('work');
      expect((exercise as any).interval_phases[1].name).toBe('recovery');
      expect((exercise as any).target_sets).toBe(6);
    });

    it('should convert complete day with all optional fields', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: [
            {
              id: 'day-a',
              order: 0,
              focus: 'Full Body A',
              target_session_length_min: 60,
              notes: 'Rest 2-3 minutes between sets',
              exercises: [
                {
                  id: 'squat',
                  name: 'Barbell Back Squat',
                  modality: 'strength',
                  target_sets: 3,
                  target_reps: 5,
                  target_notes: 'Depth: hip crease below knee'
                }
              ]
            }
          ]
        }
      };

      const result = convertToDraft(yaml);
      const day = result.cycle?.days[0];

      expect(day?.id).toBe('day-a');
      expect(day?.order).toBe(0);
      expect(day?.focus).toBe('Full Body A');
      expect(day?.target_session_length_min).toBe(60);
      expect(day?.notes).toBe('Rest 2-3 minutes between sets');

      const exercise = day?.exercises[0];
      expect((exercise as any).id).toBe('squat');
      expect((exercise as any).target_notes).toBe('Depth: hip crease below knee');
    });

    it('should convert plan with multiple days and exercises', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: [
            {
              focus: 'Day 1',
              exercises: [
                { name: 'Exercise 1', modality: 'strength', target_sets: 3, target_reps: 5 },
                { name: 'Exercise 2', modality: 'countdown', target_duration_sec: 60 }
              ]
            },
            {
              focus: 'Day 2',
              exercises: [
                { name: 'Exercise 3', modality: 'interval', target_sets: 5, target_duration_sec: 30 }
              ]
            }
          ]
        }
      };

      const result = convertToDraft(yaml);

      expect(result.cycle?.days).toHaveLength(2);
      expect(result.cycle?.days[0].exercises).toHaveLength(2);
      expect(result.cycle?.days[1].exercises).toHaveLength(1);
    });

    it('should handle plan without cycle', () => {
      const yaml = {
        plan_version: 1
      };

      const result = convertToDraft(yaml);

      expect(result.plan_version).toBe(1);
      expect(result.cycle).toBeUndefined();
    });

    it('should handle day without exercises', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: [
            {
              focus: 'Rest Day'
            }
          ]
        }
      };

      const result = convertToDraft(yaml);

      expect(result.cycle?.days[0].exercises).toEqual([]);
    });
  });

  describe('error handling', () => {
    it('should throw error for null input', () => {
      expect(() => convertToDraft(null)).toThrow('Invalid YAML: expected an object');
    });

    it('should throw error for non-object input', () => {
      expect(() => convertToDraft('string')).toThrow('Invalid YAML: expected an object');
      expect(() => convertToDraft(123)).toThrow('Invalid YAML: expected an object');
      expect(() => convertToDraft([])).toThrow('Invalid YAML: expected an object');
    });

    it('should throw error for missing plan_version', () => {
      const yaml = {
        cycle: { days: [] }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: plan_version must be a number');
    });

    it('should throw error for invalid plan_version type', () => {
      const yaml = {
        plan_version: '1',
        cycle: { days: [] }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: plan_version must be a number');
    });

    it('should throw error for invalid meta type', () => {
      const yaml = {
        plan_version: 1,
        meta: 'invalid',
        cycle: { days: [] }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: meta must be an object');
    });

    it('should throw error for null meta', () => {
      const yaml = {
        plan_version: 1,
        meta: null,
        cycle: { days: [] }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: meta must be an object');
    });

    it('should throw error for invalid glossary type', () => {
      const yaml = {
        plan_version: 1,
        glossary: 'invalid',
        cycle: { days: [] }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: glossary must be an object');
    });

    it('should throw error for array glossary', () => {
      const yaml = {
        plan_version: 1,
        glossary: [],
        cycle: { days: [] }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: glossary must be an object');
    });

    it('should throw error for invalid cycle type', () => {
      const yaml = {
        plan_version: 1,
        cycle: 'invalid'
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: cycle must be an object');
    });

    it('should throw error for array cycle', () => {
      const yaml = {
        plan_version: 1,
        cycle: []
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: cycle must be an object');
    });

    it('should throw error for invalid cycle.notes type', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          notes: 123,
          days: []
        }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: cycle.notes must be a string');
    });

    it('should throw error for non-array days', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: 'invalid'
        }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: cycle.days must be an array');
    });

    it('should throw error for invalid day object', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: ['invalid']
        }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: day at index 0 must be an object');
    });

    it('should throw error for invalid day id type', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: [
            {
              id: 123,
              exercises: []
            }
          ]
        }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: day 0 id must be a string');
    });

    it('should throw error for invalid day order type', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: [
            {
              order: 'zero',
              exercises: []
            }
          ]
        }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: day 0 order must be a number');
    });

    it('should throw error for invalid day focus type', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: [
            {
              focus: 123,
              exercises: []
            }
          ]
        }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: day 0 focus must be a string');
    });

    it('should throw error for invalid day target_session_length_min type', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: [
            {
              target_session_length_min: '60',
              exercises: []
            }
          ]
        }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: day 0 target_session_length_min must be a number');
    });

    it('should throw error for invalid day notes type', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: [
            {
              notes: 123,
              exercises: []
            }
          ]
        }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: day 0 notes must be a string');
    });

    it('should throw error for non-array exercises', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: [
            {
              exercises: 'invalid'
            }
          ]
        }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: day 0 exercises must be an array');
    });

    it('should throw error for invalid exercise object', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: [
            {
              exercises: ['invalid']
            }
          ]
        }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: day 0 exercise 0 must be an object');
    });

    it('should throw error for missing exercise name', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: [
            {
              exercises: [
                {
                  modality: 'strength'
                }
              ]
            }
          ]
        }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: day 0 exercise 0 must have a name string');
    });

    it('should throw error for invalid exercise name type', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: [
            {
              exercises: [
                {
                  name: 123,
                  modality: 'strength'
                }
              ]
            }
          ]
        }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: day 0 exercise 0 must have a name string');
    });

    it('should throw error for missing exercise modality', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: [
            {
              exercises: [
                {
                  name: 'Squat'
                }
              ]
            }
          ]
        }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: day 0 exercise 0 must have a modality string');
    });

    it('should throw error for invalid exercise modality type', () => {
      const yaml = {
        plan_version: 1,
        cycle: {
          days: [
            {
              exercises: [
                {
                  name: 'Squat',
                  modality: 123
                }
              ]
            }
          ]
        }
      };

      expect(() => convertToDraft(yaml)).toThrow('Invalid YAML: day 0 exercise 0 must have a modality string');
    });
  });

  describe('parseAndConvert', () => {
    it('should parse valid YAML string and convert to draft', () => {
      const yamlString = `
plan_version: 1
meta:
  title: Test Plan
cycle:
  days:
    - focus: Day 1
      exercises:
        - name: Squat
          modality: strength
          target_sets: 3
          target_reps: 5
      `;

      const result = parseAndConvert(yamlString);

      expect(result.plan_version).toBe(1);
      expect(result.meta?.title).toBe('Test Plan');
      expect(result.cycle?.days[0].focus).toBe('Day 1');
    });

    it('should throw error for invalid YAML syntax', () => {
      const yamlString = `
plan_version: 1
  invalid indentation:
    broken
      `;

      expect(() => parseAndConvert(yamlString)).toThrow();
    });

    it('should throw error for empty string', () => {
      expect(() => parseAndConvert('')).toThrow();
    });

    it('should propagate validation errors', () => {
      const yamlString = `
plan_version: "1"
      `;

      expect(() => parseAndConvert(yamlString)).toThrow('Invalid YAML: plan_version must be a number');
    });
  });
});
