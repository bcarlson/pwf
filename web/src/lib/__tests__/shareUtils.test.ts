import { describe, it, expect, beforeEach, vi } from 'vitest';
import { encodePlanToUrl, decodePlanFromUrl, getPlanFromCurrentUrl, copyShareLink } from '../shareUtils';
import type { PlanDraft } from '../builderState';

describe('shareUtils', () => {
  const mockPlan: PlanDraft = {
    plan_version: 1,
    meta: {
      name: 'Test Plan',
      description: 'A test workout plan'
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
              target_reps: 10
            }
          ]
        }
      ]
    }
  };

  beforeEach(() => {
    // Clear any existing location state
    delete (window as any).location;
  });

  describe('encodePlanToUrl', () => {
    beforeEach(() => {
      (window as any).location = { origin: 'https://test.com' };
    });

    it('should encode a plan to a URL with compressed data', () => {
      const url = encodePlanToUrl(mockPlan, 'https://example.com');

      expect(url).toContain('https://example.com/#/builder?plan=');
      expect(url.length).toBeGreaterThan('https://example.com/#/builder?plan='.length);
    });

    it('should use window.location.origin as default base URL', () => {
      (window as any).location = { origin: 'https://test.com' };

      const url = encodePlanToUrl(mockPlan);

      expect(url).toContain('https://test.com/#/builder?plan=');
    });

    it('should throw error for invalid plan', () => {
      const circularPlan = {} as any;
      circularPlan.self = circularPlan; // Create circular reference

      expect(() => encodePlanToUrl(circularPlan, 'https://example.com')).toThrow('Failed to create share link');
    });
  });

  describe('decodePlanFromUrl', () => {
    it('should decode a plan from a valid URL', () => {
      const encodedUrl = encodePlanToUrl(mockPlan, 'https://example.com');
      const decoded = decodePlanFromUrl(encodedUrl);

      expect(decoded).toEqual(mockPlan);
    });

    it('should return null for URL without plan parameter', () => {
      const result = decodePlanFromUrl('https://example.com/#/builder');

      expect(result).toBeNull();
    });

    it('should handle hash-based routing', () => {
      const encodedUrl = encodePlanToUrl(mockPlan, 'https://example.com');
      const decoded = decodePlanFromUrl(encodedUrl);

      expect(decoded).not.toBeNull();
      expect(decoded?.meta?.name).toBe('Test Plan');
    });

    it('should return null for invalid compressed data', () => {
      const result = decodePlanFromUrl('https://example.com/#/builder?plan=invaliddata');

      expect(result).toBeNull();
    });

    it('should handle errors gracefully', () => {
      const result = decodePlanFromUrl('not-a-valid-url');

      expect(result).toBeNull();
    });
  });

  describe('getPlanFromCurrentUrl', () => {
    it('should get plan from current window location', () => {
      const encodedUrl = encodePlanToUrl(mockPlan, 'https://example.com');
      (window as any).location = { href: encodedUrl };

      const result = getPlanFromCurrentUrl();

      expect(result).toEqual(mockPlan);
    });

    it('should return null if no plan in current URL', () => {
      (window as any).location = { href: 'https://example.com/#/builder' };

      const result = getPlanFromCurrentUrl();

      expect(result).toBeNull();
    });
  });

  describe('copyShareLink', () => {
    beforeEach(() => {
      (window as any).location = { origin: 'https://test.com' };
    });

    it('should copy share link to clipboard', async () => {
      const writeTextMock = vi.fn().mockResolvedValue(undefined);
      Object.assign(navigator, {
        clipboard: {
          writeText: writeTextMock
        }
      });

      await copyShareLink(mockPlan);

      expect(writeTextMock).toHaveBeenCalledTimes(1);
      const calledUrl = writeTextMock.mock.calls[0][0];
      expect(calledUrl).toContain('/#/builder?plan=');
    });

    it('should throw error if clipboard write fails', async () => {
      const writeTextMock = vi.fn().mockRejectedValue(new Error('Clipboard error'));
      Object.assign(navigator, {
        clipboard: {
          writeText: writeTextMock
        }
      });

      await expect(copyShareLink(mockPlan)).rejects.toThrow('Clipboard error');
    });
  });

  describe('round-trip encoding/decoding', () => {
    it('should preserve plan data through encode and decode', () => {
      const encoded = encodePlanToUrl(mockPlan, 'https://example.com');
      const decoded = decodePlanFromUrl(encoded);

      expect(decoded).toEqual(mockPlan);
    });

    it('should handle plans with no metadata', () => {
      const minimalPlan: PlanDraft = {
        plan_version: 1,
        cycle: {
          days: []
        }
      };

      const encoded = encodePlanToUrl(minimalPlan, 'https://example.com');
      const decoded = decodePlanFromUrl(encoded);

      expect(decoded).toEqual(minimalPlan);
    });

    it('should handle complex plans with multiple days and exercises', () => {
      const complexPlan: PlanDraft = {
        plan_version: 1,
        meta: {
          name: 'Complex Plan',
          days_per_week: 3,
          tags: ['strength', 'hypertrophy']
        },
        cycle: {
          days: [
            {
              focus: 'Push',
              exercises: [
                { name: 'Bench Press', modality: 'strength', target_sets: 4, target_reps: 8 },
                { name: 'OHP', modality: 'strength', target_sets: 3, target_reps: 10 }
              ]
            },
            {
              focus: 'Pull',
              exercises: [
                { name: 'Deadlift', modality: 'strength', target_sets: 5, target_reps: 5 }
              ]
            }
          ]
        }
      };

      const encoded = encodePlanToUrl(complexPlan, 'https://example.com');
      const decoded = decodePlanFromUrl(encoded);

      expect(decoded).toEqual(complexPlan);
    });
  });
});
