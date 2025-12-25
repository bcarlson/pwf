/**
 * Tests for builderStore.ts
 */

import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { get } from 'svelte/store';
import {
  builderStore,
  loadFromLocalStorage,
  saveToLocalStorage,
  type BuilderState,
  type PlanDraft
} from '../../../lib/builderStore';

describe('builderStore', () => {
  beforeEach(() => {
    // Clear localStorage before each test
    localStorage.clear();
    // Reset the store
    builderStore.reset();
    // Clear all timers
    vi.clearAllTimers();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe('initialization', () => {
    it('should initialize with empty plan', () => {
      const state = get(builderStore);

      expect(state.currentPlan).toEqual({
        plan_version: 1,
        cycle: {
          days: []
        }
      });
      expect(state.currentDayIndex).toBeNull();
      expect(state.isDirty).toBe(false);
    });

    it('should load from localStorage on mount if available', () => {
      const savedState: BuilderState = {
        currentPlan: {
          plan_version: 1,
          meta: {
            title: 'Test Plan'
          },
          cycle: {
            days: [
              {
                focus: 'Test Day',
                exercises: []
              }
            ]
          }
        },
        currentDayIndex: 0,
        isDirty: true
      };

      localStorage.setItem('pwf-builder-state', JSON.stringify(savedState));

      const loaded = loadFromLocalStorage();
      expect(loaded).toEqual(savedState);
    });

    it('should return null when localStorage is empty', () => {
      const loaded = loadFromLocalStorage();
      expect(loaded).toBeNull();
    });

    it('should handle corrupted localStorage data gracefully', () => {
      localStorage.setItem('pwf-builder-state', 'invalid-json{');

      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
      const loaded = loadFromLocalStorage();

      expect(loaded).toBeNull();
      expect(consoleSpy).toHaveBeenCalledWith(
        'Failed to load builder state from localStorage:',
        expect.any(Error)
      );

      consoleSpy.mockRestore();
    });
  });

  describe('localStorage persistence', () => {
    beforeEach(() => {
      vi.useFakeTimers();
    });

    afterEach(() => {
      vi.useRealTimers();
    });

    it('should save to localStorage when state changes', () => {
      const testPlan: PlanDraft = {
        plan_version: 1,
        meta: {
          title: 'My Workout Plan'
        },
        cycle: {
          days: []
        }
      };

      builderStore.loadPlan(testPlan);

      // Fast-forward time to trigger debounced save
      vi.advanceTimersByTime(500);

      const saved = localStorage.getItem('pwf-builder-state');
      expect(saved).toBeTruthy();

      const parsed = JSON.parse(saved!);
      expect(parsed.currentPlan.meta.title).toBe('My Workout Plan');
      expect(parsed.isDirty).toBe(true);
    });

    it('should debounce saves to localStorage', () => {
      builderStore.markDirty();
      vi.advanceTimersByTime(250);

      // Should not have saved yet
      let saved = localStorage.getItem('pwf-builder-state');
      expect(saved).toBeNull();

      builderStore.markDirty();
      vi.advanceTimersByTime(250);

      // Still should not have saved (timer was reset)
      saved = localStorage.getItem('pwf-builder-state');
      expect(saved).toBeNull();

      // Now advance past the debounce time
      vi.advanceTimersByTime(250);

      saved = localStorage.getItem('pwf-builder-state');
      expect(saved).toBeTruthy();
    });

    it('should handle localStorage errors gracefully', () => {
      const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
      const setItemSpy = vi.spyOn(Storage.prototype, 'setItem').mockImplementation(() => {
        throw new Error('Storage quota exceeded');
      });

      const state: BuilderState = {
        currentPlan: {
          plan_version: 1,
          cycle: { days: [] }
        },
        currentDayIndex: null,
        isDirty: true
      };

      saveToLocalStorage(state);
      vi.advanceTimersByTime(500);

      expect(consoleSpy).toHaveBeenCalledWith(
        'Failed to save builder state to localStorage:',
        expect.any(Error)
      );

      consoleSpy.mockRestore();
      setItemSpy.mockRestore();
    });

    it('should not save when isDirty is false', () => {
      builderStore.update((state) => ({
        ...state,
        isDirty: false
      }));

      vi.advanceTimersByTime(500);

      const saved = localStorage.getItem('pwf-builder-state');
      expect(saved).toBeNull();
    });
  });

  describe('store methods', () => {
    it('should reset the store', () => {
      builderStore.loadPlan({
        plan_version: 1,
        meta: { title: 'Test' },
        cycle: { days: [] }
      });

      builderStore.reset();

      const state = get(builderStore);
      expect(state.currentPlan.meta).toBeUndefined();
      expect(state.isDirty).toBe(false);
    });

    it('should load a plan and mark as dirty', () => {
      const testPlan: PlanDraft = {
        plan_version: 1,
        meta: {
          title: 'New Plan',
          description: 'Test description'
        },
        glossary: {
          'Term': 'Definition'
        },
        cycle: {
          notes: 'Cycle notes',
          days: [
            {
              focus: 'Day 1',
              exercises: [
                {
                  name: 'Squat',
                  modality: 'strength' as const,
                  target_sets: 3,
                  target_reps: 5
                }
              ]
            }
          ]
        }
      };

      builderStore.loadPlan(testPlan);

      const state = get(builderStore);
      expect(state.currentPlan).toEqual(testPlan);
      expect(state.currentDayIndex).toBeNull();
      expect(state.isDirty).toBe(true);
    });

    it('should mark state as dirty', () => {
      builderStore.markDirty();

      const state = get(builderStore);
      expect(state.isDirty).toBe(true);
    });

    it('should allow direct updates', () => {
      builderStore.update((state) => ({
        ...state,
        currentDayIndex: 1
      }));

      const state = get(builderStore);
      expect(state.currentDayIndex).toBe(1);
    });

    it('should allow direct set', () => {
      const newState: BuilderState = {
        currentPlan: {
          plan_version: 1,
          cycle: { days: [] }
        },
        currentDayIndex: 2,
        isDirty: false
      };

      builderStore.set(newState);

      const state = get(builderStore);
      expect(state).toEqual(newState);
    });
  });

  describe('complex scenarios', () => {
    beforeEach(() => {
      vi.useFakeTimers();
    });

    afterEach(() => {
      vi.useRealTimers();
    });

    it('should handle multiple rapid updates with debouncing', () => {
      builderStore.markDirty();
      vi.advanceTimersByTime(100);

      builderStore.update((state) => ({
        ...state,
        currentDayIndex: 0,
        isDirty: true
      }));
      vi.advanceTimersByTime(100);

      builderStore.update((state) => ({
        ...state,
        currentDayIndex: 1,
        isDirty: true
      }));
      vi.advanceTimersByTime(100);

      // Should not have saved yet
      let saved = localStorage.getItem('pwf-builder-state');
      expect(saved).toBeNull();

      // Advance past debounce time from last update
      vi.advanceTimersByTime(400);

      saved = localStorage.getItem('pwf-builder-state');
      expect(saved).toBeTruthy();

      const parsed = JSON.parse(saved!);
      expect(parsed.currentDayIndex).toBe(1);
    });

    it('should preserve all plan data through save/load cycle', () => {
      const complexPlan: PlanDraft = {
        plan_version: 1,
        meta: {
          id: 'test-plan',
          title: 'Complex Test Plan',
          description: 'A comprehensive test',
          author: 'Test Author',
          status: 'active' as const,
          equipment: ['barbell', 'dumbbells'],
          daysPerWeek: 3,
          recommendedFirst: true,
          tags: ['strength', 'beginner']
        },
        glossary: {
          'Term1': 'Definition1',
          'Term2': 'Definition2'
        },
        cycle: {
          notes: 'Test cycle notes',
          days: [
            {
              id: 'day-1',
              order: 0,
              focus: 'Strength',
              target_session_length_min: 60,
              notes: 'Day notes',
              exercises: [
                {
                  id: 'ex-1',
                  name: 'Squat',
                  modality: 'strength' as const,
                  target_sets: 3,
                  target_reps: 5,
                  target_notes: 'Heavy weight'
                }
              ]
            }
          ]
        }
      };

      builderStore.loadPlan(complexPlan);
      vi.advanceTimersByTime(500);

      const saved = localStorage.getItem('pwf-builder-state');
      const parsed = JSON.parse(saved!);

      expect(parsed.currentPlan).toEqual(complexPlan);
    });
  });
});
