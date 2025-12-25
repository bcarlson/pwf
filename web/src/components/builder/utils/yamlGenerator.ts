/**
 * YAML generation utilities for the Plan Builder
 */

import { stringify } from 'yaml';
import type { PlanDraft } from '../../../lib/builderState';

/**
 * Generate PWF YAML from a PlanDraft object
 */
export function generateYAML(draft: PlanDraft): string {
  // Remove empty/undefined fields to keep YAML clean
  const cleanedDraft = cleanObject(draft);

  // Generate YAML with proper formatting
  const yaml = stringify(cleanedDraft, {
    indent: 2,
    lineWidth: 0, // Disable line wrapping
    minContentWidth: 0,
    defaultKeyType: 'PLAIN',
    defaultStringType: 'PLAIN',
  });

  return yaml;
}

/**
 * Recursively remove undefined, null, and empty values from objects
 */
function cleanObject(obj: any): any {
  if (obj === null || obj === undefined) {
    return undefined;
  }

  if (Array.isArray(obj)) {
    const cleaned = obj.map(cleanObject).filter(item => item !== undefined);
    return cleaned.length > 0 ? cleaned : undefined;
  }

  if (typeof obj === 'object') {
    const cleaned: any = {};

    for (const [key, value] of Object.entries(obj)) {
      const cleanedValue = cleanObject(value);

      // Skip undefined, null, empty strings, and empty arrays
      if (cleanedValue !== undefined && cleanedValue !== null && cleanedValue !== '') {
        // Skip empty objects
        if (typeof cleanedValue === 'object' && !Array.isArray(cleanedValue)) {
          if (Object.keys(cleanedValue).length > 0) {
            cleaned[key] = cleanedValue;
          }
        } else {
          cleaned[key] = cleanedValue;
        }
      }
    }

    return Object.keys(cleaned).length > 0 ? cleaned : undefined;
  }

  return obj;
}

/**
 * Validate that a plan draft has the minimum required fields
 */
export function validateDraftStructure(draft: PlanDraft): { valid: boolean; errors: string[] } {
  const errors: string[] = [];

  if (!draft.plan_version) {
    errors.push('plan_version is required');
  }

  if (!draft.cycle) {
    errors.push('cycle is required');
  } else if (!draft.cycle.days || draft.cycle.days.length === 0) {
    errors.push('cycle must have at least one day');
  }

  return {
    valid: errors.length === 0,
    errors,
  };
}

/**
 * Create a minimal valid plan for testing
 */
export function createMinimalPlan(): PlanDraft {
  return {
    plan_version: 1,
    meta: {
      name: 'Minimal Plan',
      description: 'A minimal valid plan',
    },
    cycle: {
      days: [
        {
          exercises: [
            {
              name: 'Squats',
              modality: 'strength',
              target_sets: 3,
              target_reps: 10,
            },
          ],
        },
      ],
    },
  };
}

/**
 * Create a plan with all 8 modalities for testing
 */
export function createAllModalitiesPlan(): PlanDraft {
  return {
    plan_version: 1,
    meta: {
      name: 'All Modalities Plan',
      description: 'Demonstrates all 8 exercise modalities',
      author: 'PWF Builder',
    },
    cycle: {
      days: [
        {
          focus: 'Full Modality Demo',
          exercises: [
            {
              name: 'Squats',
              modality: 'strength',
              target_sets: 3,
              target_reps: 10,
              equipment: ['barbell'],
            },
            {
              name: 'Plank Hold',
              modality: 'countdown',
              target_duration_sec: 60,
            },
            {
              name: 'Max Distance Run',
              modality: 'stopwatch',
              target_duration_sec: 1200,
              sport: 'running',
            },
            {
              name: 'Sprint Intervals',
              modality: 'interval',
              target_sets: 8,
              target_duration_sec: 30,
              rest_between_sets_sec: 30,
              sport: 'running',
            },
            {
              name: 'EMOM Burpees',
              modality: 'emom',
              target_sets: 10,
              target_reps: 5,
            },
            {
              name: 'AMRAP Circuit',
              modality: 'amrap',
              target_duration_sec: 600,
              notes: '10 pushups, 10 situps, 10 air squats',
            },
            {
              name: 'Tabata Kettlebell Swings',
              modality: 'tabata',
              target_sets: 8,
              equipment: ['kettlebell'],
            },
            {
              name: 'Long Bike Ride',
              modality: 'endurance',
              sport: 'cycling',
              target_duration_sec: 3600,
              target_distance_meters: 40000,
            },
          ],
        },
      ],
    },
  };
}

/**
 * Create a complex endurance plan with zones
 */
export function createEndurancePlan(): PlanDraft {
  return {
    plan_version: 1,
    meta: {
      name: 'Endurance Training Plan',
      description: 'Zone-based endurance workouts',
      author: 'PWF Builder',
      equipment: ['bike', 'heart rate monitor'],
      athlete_profile: {
        ftp_watts: 250,
        threshold_hr_bpm: 165,
        max_hr_bpm: 185,
      },
    },
    glossary: {
      'Z2': 'Zone 2 - Aerobic endurance (60-70% FTP)',
      'Z4': 'Zone 4 - Lactate threshold (90-105% FTP)',
    },
    cycle: {
      days: [
        {
          focus: 'Base Endurance',
          exercises: [
            {
              name: 'Z2 Ride',
              modality: 'endurance',
              sport: 'cycling',
              target_duration_sec: 3600,
              target_power_zone: 2,
              target_hr_zone: 2,
              notes: 'Stay in Z2 for entire ride',
            },
          ],
        },
        {
          focus: 'Threshold Work',
          exercises: [
            {
              name: 'Warmup',
              modality: 'endurance',
              sport: 'cycling',
              target_duration_sec: 900,
              target_power_zone: 1,
            },
            {
              name: 'Z4 Intervals',
              modality: 'interval',
              sport: 'cycling',
              target_sets: 4,
              target_duration_sec: 300,
              rest_between_sets_sec: 180,
              target_power_zone: 4,
              target_hr_zone: 4,
            },
            {
              name: 'Cooldown',
              modality: 'endurance',
              sport: 'cycling',
              target_duration_sec: 600,
              target_power_zone: 1,
            },
          ],
        },
      ],
    },
  };
}
