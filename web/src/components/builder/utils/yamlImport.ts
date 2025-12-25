/**
 * Utilities for importing YAML plans and converting them to draft format
 */

import { parse as parseYAML } from 'yaml';
import type { PlanDraft, PlanDayDraft } from '../../../lib/builderStore';
import type { PlanExercise } from '@pwf-dev/core';

/**
 * Convert parsed YAML to PlanDraft format
 * Handles all exercise fields including zones, ramps, and interval_phases
 */
export function convertToDraft(parsedYAML: unknown): PlanDraft {
  if (!parsedYAML || typeof parsedYAML !== 'object' || Array.isArray(parsedYAML)) {
    throw new Error('Invalid YAML: expected an object');
  }

  const yaml = parsedYAML as Record<string, unknown>;

  // Validate required fields
  if (typeof yaml.plan_version !== 'number') {
    throw new Error('Invalid YAML: plan_version must be a number');
  }

  const draft: PlanDraft = {
    plan_version: yaml.plan_version
  };

  // Handle meta (optional)
  if (yaml.meta !== undefined) {
    if (typeof yaml.meta !== 'object' || yaml.meta === null) {
      throw new Error('Invalid YAML: meta must be an object');
    }
    draft.meta = yaml.meta as PlanDraft['meta'];
  }

  // Handle glossary (optional)
  if (yaml.glossary !== undefined) {
    if (typeof yaml.glossary !== 'object' || yaml.glossary === null || Array.isArray(yaml.glossary)) {
      throw new Error('Invalid YAML: glossary must be an object');
    }
    draft.glossary = yaml.glossary as Record<string, string>;
  }

  // Handle cycle (optional but recommended)
  if (yaml.cycle !== undefined) {
    if (typeof yaml.cycle !== 'object' || yaml.cycle === null || Array.isArray(yaml.cycle)) {
      throw new Error('Invalid YAML: cycle must be an object');
    }

    const cycle = yaml.cycle as Record<string, unknown>;
    const cycleDraft: PlanDraft['cycle'] = {
      days: []
    };

    // Handle cycle notes
    if (cycle.notes !== undefined) {
      if (typeof cycle.notes !== 'string') {
        throw new Error('Invalid YAML: cycle.notes must be a string');
      }
      cycleDraft.notes = cycle.notes;
    }

    // Handle days
    if (cycle.days !== undefined) {
      if (!Array.isArray(cycle.days)) {
        throw new Error('Invalid YAML: cycle.days must be an array');
      }

      cycleDraft.days = cycle.days.map((day, index) => {
        if (typeof day !== 'object' || day === null) {
          throw new Error(`Invalid YAML: day at index ${index} must be an object`);
        }

        return convertDayToDraft(day as Record<string, unknown>, index);
      });
    }

    draft.cycle = cycleDraft;
  }

  return draft;
}

/**
 * Convert a day object to PlanDayDraft format
 */
function convertDayToDraft(day: Record<string, unknown>, index: number): PlanDayDraft {
  const dayDraft: PlanDayDraft = {
    exercises: []
  };

  // Handle optional fields
  if (day.id !== undefined) {
    if (typeof day.id !== 'string') {
      throw new Error(`Invalid YAML: day ${index} id must be a string`);
    }
    dayDraft.id = day.id;
  }

  if (day.order !== undefined) {
    if (typeof day.order !== 'number') {
      throw new Error(`Invalid YAML: day ${index} order must be a number`);
    }
    dayDraft.order = day.order;
  }

  if (day.focus !== undefined) {
    if (typeof day.focus !== 'string') {
      throw new Error(`Invalid YAML: day ${index} focus must be a string`);
    }
    dayDraft.focus = day.focus;
  }

  if (day.target_session_length_min !== undefined) {
    if (typeof day.target_session_length_min !== 'number') {
      throw new Error(`Invalid YAML: day ${index} target_session_length_min must be a number`);
    }
    dayDraft.target_session_length_min = day.target_session_length_min;
  }

  if (day.notes !== undefined) {
    if (typeof day.notes !== 'string') {
      throw new Error(`Invalid YAML: day ${index} notes must be a string`);
    }
    dayDraft.notes = day.notes;
  }

  // Handle exercises
  if (day.exercises !== undefined) {
    if (!Array.isArray(day.exercises)) {
      throw new Error(`Invalid YAML: day ${index} exercises must be an array`);
    }

    dayDraft.exercises = day.exercises.map((exercise, exIndex) => {
      if (typeof exercise !== 'object' || exercise === null) {
        throw new Error(`Invalid YAML: day ${index} exercise ${exIndex} must be an object`);
      }

      return convertExerciseToDraft(exercise as Record<string, unknown>, index, exIndex);
    });
  }

  return dayDraft;
}

/**
 * Convert an exercise object to PlanExercise format
 * Preserves all fields including zones, ramps, interval_phases
 */
function convertExerciseToDraft(
  exercise: Record<string, unknown>,
  dayIndex: number,
  exIndex: number
): PlanExercise {
  if (typeof exercise.name !== 'string') {
    throw new Error(`Invalid YAML: day ${dayIndex} exercise ${exIndex} must have a name string`);
  }

  if (typeof exercise.modality !== 'string') {
    throw new Error(`Invalid YAML: day ${dayIndex} exercise ${exIndex} must have a modality string`);
  }

  // Start with required fields
  const exerciseDraft: Record<string, unknown> = {
    name: exercise.name,
    modality: exercise.modality
  };

  // Copy all other fields, preserving their structure
  // This includes: zones, ramps, interval_phases, target_*, id, notes, etc.
  for (const [key, value] of Object.entries(exercise)) {
    if (key !== 'name' && key !== 'modality') {
      exerciseDraft[key] = value;
    }
  }

  return exerciseDraft as unknown as PlanExercise;
}

/**
 * Parse YAML string and convert to PlanDraft
 */
export function parseAndConvert(yamlString: string): PlanDraft {
  try {
    const parsed = parseYAML(yamlString);
    return convertToDraft(parsed);
  } catch (error) {
    if (error instanceof Error) {
      throw error;
    }
    throw new Error('Failed to parse YAML');
  }
}
