/**
 * Type definitions for PWF exercise modalities
 */

export type Modality =
  | 'strength'
  | 'countdown'
  | 'stopwatch'
  | 'interval'
  | 'cycling'
  | 'running'
  | 'rowing'
  | 'swimming';

export interface TrainingZone {
  zone: number; // 1-7
  duration_sec?: number;
  target_power_watts?: number;
  target_hr_bpm?: number;
  target_pace_sec_per_km?: number;
  cadence_rpm?: number;
}

export interface RampConfig {
  start_power_watts: number;
  end_power_watts: number;
  duration_sec: number;
  step_duration_sec?: number;
}

export interface IntervalPhase {
  name: string;
  duration_sec: number;
  target_power_watts?: number;
  target_hr_bpm?: number;
  target_pace_sec_per_km?: number;
  cadence_rpm?: number;
}

export interface Exercise {
  id?: string;
  name?: string;
  modality: Modality;

  // Strength fields
  target_sets?: number;
  target_reps?: number;
  target_load?: string;

  // Countdown/Stopwatch/Interval fields
  target_duration_sec?: number;

  // Distance fields
  target_distance_meters?: number;

  // Endurance fields
  zones?: TrainingZone[];
  ramp?: RampConfig;
  interval_phases?: IntervalPhase[];

  // Common fields
  target_notes?: string;
  cues?: string;
  link?: string;
  image?: string;
  group?: string;
  group_type?: 'superset' | 'circuit';
  rest_between_sets_sec?: number;
  rest_after_sec?: number;
}

/**
 * Get the required fields for a given modality
 */
export function getRequiredFields(modality: Modality): string[] {
  switch (modality) {
    case 'strength':
      return ['target_sets', 'target_reps'];
    case 'countdown':
      return ['target_duration_sec'];
    case 'stopwatch':
      return [];
    case 'interval':
      return ['target_sets'];
    case 'cycling':
    case 'running':
    case 'rowing':
    case 'swimming':
      return ['zones', 'ramp', 'interval_phases']; // At least one required
    default:
      return [];
  }
}

/**
 * Get the optional fields for a given modality
 */
export function getOptionalFields(modality: Modality): string[] {
  switch (modality) {
    case 'strength':
      return ['target_load'];
    case 'countdown':
      return ['target_sets'];
    case 'stopwatch':
      return ['target_duration_sec'];
    case 'interval':
      return ['target_duration_sec', 'target_distance_meters'];
    case 'cycling':
      return ['target_distance_meters'];
    case 'running':
      return ['target_distance_meters'];
    case 'rowing':
      return ['target_distance_meters'];
    case 'swimming':
      return ['target_distance_meters'];
    default:
      return [];
  }
}

/**
 * Check if modality uses endurance structures
 */
export function isEnduranceModality(modality: Modality): boolean {
  return ['cycling', 'running', 'rowing', 'swimming'].includes(modality);
}

/**
 * Check if modality supports zones
 */
export function supportsZones(modality: Modality): boolean {
  return ['cycling', 'running', 'rowing', 'swimming'].includes(modality);
}

/**
 * Check if modality supports ramps
 */
export function supportsRamps(modality: Modality): boolean {
  return ['cycling', 'rowing'].includes(modality);
}

/**
 * Check if modality supports intervals
 */
export function supportsIntervals(modality: Modality): boolean {
  return ['cycling', 'running', 'rowing', 'swimming'].includes(modality);
}

/**
 * Get field label for UI display
 */
export function getFieldLabel(field: string): string {
  const labels: Record<string, string> = {
    target_sets: 'Target Sets',
    target_reps: 'Target Reps',
    target_load: 'Target Load',
    target_duration_sec: 'Duration (seconds)',
    target_distance_meters: 'Distance (meters)',
    zones: 'Training Zones',
    ramp: 'Power Ramp',
    interval_phases: 'Interval Phases',
  };
  return labels[field] || field;
}
