/**
 * Tests for modalityTypes.ts utility functions
 */

import { describe, it, expect } from 'vitest';
import {
  type Modality,
  getRequiredFields,
  getOptionalFields,
  isEnduranceModality,
  supportsZones,
  supportsRamps,
  supportsIntervals,
  getFieldLabel
} from '../modalityTypes';

describe('modalityTypes', () => {
  describe('getRequiredFields', () => {
    it('should return required fields for strength modality', () => {
      const fields = getRequiredFields('strength');
      expect(fields).toEqual(['target_sets', 'target_reps']);
    });

    it('should return required fields for countdown modality', () => {
      const fields = getRequiredFields('countdown');
      expect(fields).toEqual(['target_duration_sec']);
    });

    it('should return empty array for stopwatch modality', () => {
      const fields = getRequiredFields('stopwatch');
      expect(fields).toEqual([]);
    });

    it('should return required fields for interval modality', () => {
      const fields = getRequiredFields('interval');
      expect(fields).toEqual(['target_sets']);
    });

    it('should return endurance fields for cycling modality', () => {
      const fields = getRequiredFields('cycling');
      expect(fields).toEqual(['zones', 'ramp', 'interval_phases']);
    });

    it('should return endurance fields for running modality', () => {
      const fields = getRequiredFields('running');
      expect(fields).toEqual(['zones', 'ramp', 'interval_phases']);
    });

    it('should return endurance fields for rowing modality', () => {
      const fields = getRequiredFields('rowing');
      expect(fields).toEqual(['zones', 'ramp', 'interval_phases']);
    });

    it('should return endurance fields for swimming modality', () => {
      const fields = getRequiredFields('swimming');
      expect(fields).toEqual(['zones', 'ramp', 'interval_phases']);
    });
  });

  describe('getOptionalFields', () => {
    it('should return optional fields for strength modality', () => {
      const fields = getOptionalFields('strength');
      expect(fields).toEqual(['target_load']);
    });

    it('should return optional fields for countdown modality', () => {
      const fields = getOptionalFields('countdown');
      expect(fields).toEqual(['target_sets']);
    });

    it('should return optional fields for stopwatch modality', () => {
      const fields = getOptionalFields('stopwatch');
      expect(fields).toEqual(['target_duration_sec']);
    });

    it('should return optional fields for interval modality', () => {
      const fields = getOptionalFields('interval');
      expect(fields).toEqual(['target_duration_sec', 'target_distance_meters']);
    });

    it('should return optional fields for cycling modality', () => {
      const fields = getOptionalFields('cycling');
      expect(fields).toEqual(['target_distance_meters']);
    });

    it('should return optional fields for running modality', () => {
      const fields = getOptionalFields('running');
      expect(fields).toEqual(['target_distance_meters']);
    });

    it('should return optional fields for rowing modality', () => {
      const fields = getOptionalFields('rowing');
      expect(fields).toEqual(['target_distance_meters']);
    });

    it('should return optional fields for swimming modality', () => {
      const fields = getOptionalFields('swimming');
      expect(fields).toEqual(['target_distance_meters']);
    });
  });

  describe('isEnduranceModality', () => {
    it('should return true for cycling', () => {
      expect(isEnduranceModality('cycling')).toBe(true);
    });

    it('should return true for running', () => {
      expect(isEnduranceModality('running')).toBe(true);
    });

    it('should return true for rowing', () => {
      expect(isEnduranceModality('rowing')).toBe(true);
    });

    it('should return true for swimming', () => {
      expect(isEnduranceModality('swimming')).toBe(true);
    });

    it('should return false for strength', () => {
      expect(isEnduranceModality('strength')).toBe(false);
    });

    it('should return false for countdown', () => {
      expect(isEnduranceModality('countdown')).toBe(false);
    });

    it('should return false for stopwatch', () => {
      expect(isEnduranceModality('stopwatch')).toBe(false);
    });

    it('should return false for interval', () => {
      expect(isEnduranceModality('interval')).toBe(false);
    });
  });

  describe('supportsZones', () => {
    it('should return true for all endurance modalities', () => {
      const enduranceModalities: Modality[] = ['cycling', 'running', 'rowing', 'swimming'];
      enduranceModalities.forEach(modality => {
        expect(supportsZones(modality)).toBe(true);
      });
    });

    it('should return false for non-endurance modalities', () => {
      const nonEnduranceModalities: Modality[] = ['strength', 'countdown', 'stopwatch', 'interval'];
      nonEnduranceModalities.forEach(modality => {
        expect(supportsZones(modality)).toBe(false);
      });
    });
  });

  describe('supportsRamps', () => {
    it('should return true for cycling', () => {
      expect(supportsRamps('cycling')).toBe(true);
    });

    it('should return true for rowing', () => {
      expect(supportsRamps('rowing')).toBe(true);
    });

    it('should return false for running', () => {
      expect(supportsRamps('running')).toBe(false);
    });

    it('should return false for swimming', () => {
      expect(supportsRamps('swimming')).toBe(false);
    });

    it('should return false for strength', () => {
      expect(supportsRamps('strength')).toBe(false);
    });

    it('should return false for countdown', () => {
      expect(supportsRamps('countdown')).toBe(false);
    });

    it('should return false for stopwatch', () => {
      expect(supportsRamps('stopwatch')).toBe(false);
    });

    it('should return false for interval', () => {
      expect(supportsRamps('interval')).toBe(false);
    });
  });

  describe('supportsIntervals', () => {
    it('should return true for all endurance modalities', () => {
      const enduranceModalities: Modality[] = ['cycling', 'running', 'rowing', 'swimming'];
      enduranceModalities.forEach(modality => {
        expect(supportsIntervals(modality)).toBe(true);
      });
    });

    it('should return false for non-endurance modalities', () => {
      const nonEnduranceModalities: Modality[] = ['strength', 'countdown', 'stopwatch', 'interval'];
      nonEnduranceModalities.forEach(modality => {
        expect(supportsIntervals(modality)).toBe(false);
      });
    });
  });

  describe('getFieldLabel', () => {
    it('should return correct label for target_sets', () => {
      expect(getFieldLabel('target_sets')).toBe('Target Sets');
    });

    it('should return correct label for target_reps', () => {
      expect(getFieldLabel('target_reps')).toBe('Target Reps');
    });

    it('should return correct label for target_load', () => {
      expect(getFieldLabel('target_load')).toBe('Target Load');
    });

    it('should return correct label for target_duration_sec', () => {
      expect(getFieldLabel('target_duration_sec')).toBe('Duration (seconds)');
    });

    it('should return correct label for target_distance_meters', () => {
      expect(getFieldLabel('target_distance_meters')).toBe('Distance (meters)');
    });

    it('should return correct label for zones', () => {
      expect(getFieldLabel('zones')).toBe('Training Zones');
    });

    it('should return correct label for ramp', () => {
      expect(getFieldLabel('ramp')).toBe('Power Ramp');
    });

    it('should return correct label for interval_phases', () => {
      expect(getFieldLabel('interval_phases')).toBe('Interval Phases');
    });

    it('should return the field name itself for unknown fields', () => {
      expect(getFieldLabel('unknown_field')).toBe('unknown_field');
    });

    it('should handle empty string', () => {
      expect(getFieldLabel('')).toBe('');
    });
  });

  describe('type safety', () => {
    it('should handle all defined modality types', () => {
      const allModalities: Modality[] = [
        'strength',
        'countdown',
        'stopwatch',
        'interval',
        'cycling',
        'running',
        'rowing',
        'swimming'
      ];

      // Ensure all modalities can be processed without errors
      allModalities.forEach(modality => {
        expect(() => getRequiredFields(modality)).not.toThrow();
        expect(() => getOptionalFields(modality)).not.toThrow();
        expect(() => isEnduranceModality(modality)).not.toThrow();
        expect(() => supportsZones(modality)).not.toThrow();
        expect(() => supportsRamps(modality)).not.toThrow();
        expect(() => supportsIntervals(modality)).not.toThrow();
      });
    });
  });

  describe('default cases', () => {
    it('should handle unknown modality in getRequiredFields', () => {
      // Cast to any to test the default case
      const fields = getRequiredFields('unknown' as any);
      expect(fields).toEqual([]);
    });

    it('should handle unknown modality in getOptionalFields', () => {
      // Cast to any to test the default case
      const fields = getOptionalFields('unknown' as any);
      expect(fields).toEqual([]);
    });
  });

  describe('edge cases and consistency', () => {
    it('should have non-overlapping required and optional fields for strength', () => {
      const required = getRequiredFields('strength');
      const optional = getOptionalFields('strength');
      const overlap = required.filter(field => optional.includes(field));
      expect(overlap).toHaveLength(0);
    });

    it('should have target_duration_sec as required for countdown but optional for stopwatch', () => {
      const countdownRequired = getRequiredFields('countdown');
      const stopwatchOptional = getOptionalFields('stopwatch');
      expect(countdownRequired).toContain('target_duration_sec');
      expect(stopwatchOptional).toContain('target_duration_sec');
    });

    it('should have consistent endurance modality behavior', () => {
      const enduranceModalities: Modality[] = ['cycling', 'running', 'rowing', 'swimming'];

      enduranceModalities.forEach(modality => {
        expect(isEnduranceModality(modality)).toBe(true);
        expect(supportsZones(modality)).toBe(true);
        expect(supportsIntervals(modality)).toBe(true);
      });
    });

    it('should only cycling and rowing support ramps among endurance modalities', () => {
      expect(supportsRamps('cycling')).toBe(true);
      expect(supportsRamps('rowing')).toBe(true);
      expect(supportsRamps('running')).toBe(false);
      expect(supportsRamps('swimming')).toBe(false);
    });
  });
});
