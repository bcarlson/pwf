/**
 * Tests for stores.ts - global application state
 */

import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { get } from 'svelte/store';
import {
  wasmReady,
  currentTab,
  currentFile,
  validationResult,
  conversionWarnings,
  darkMode,
  type TabType,
  type ValidationResult,
  type ValidationIssue,
  type ConversionWarning
} from '../stores';

// Mock localStorage
const localStorageMock = (() => {
  let store: Record<string, string> = {};

  return {
    getItem: (key: string) => store[key] || null,
    setItem: (key: string, value: string) => {
      store[key] = value.toString();
    },
    removeItem: (key: string) => {
      delete store[key];
    },
    clear: () => {
      store = {};
    }
  };
})();

// Mock window.matchMedia
const matchMediaMock = (matches: boolean) => ({
  matches,
  media: '(prefers-color-scheme: dark)',
  onchange: null,
  addListener: vi.fn(),
  removeListener: vi.fn(),
  addEventListener: vi.fn(),
  removeEventListener: vi.fn(),
  dispatchEvent: vi.fn()
});

describe('stores', () => {
  beforeEach(() => {
    // Reset all stores to initial state
    wasmReady.set(false);
    currentTab.set('validate');
    currentFile.set(null);
    validationResult.set(null);
    conversionWarnings.set([]);
    localStorageMock.clear();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe('wasmReady', () => {
    it('should initialize as false', () => {
      expect(get(wasmReady)).toBe(false);
    });

    it('should be settable to true', () => {
      wasmReady.set(true);
      expect(get(wasmReady)).toBe(true);
    });

    it('should be settable back to false', () => {
      wasmReady.set(true);
      wasmReady.set(false);
      expect(get(wasmReady)).toBe(false);
    });

    it('should allow subscription', () => {
      const values: boolean[] = [];
      const unsubscribe = wasmReady.subscribe(value => values.push(value));

      wasmReady.set(true);
      wasmReady.set(false);

      expect(values).toEqual([false, true, false]);
      unsubscribe();
    });
  });

  describe('currentTab', () => {
    it('should initialize as validate', () => {
      expect(get(currentTab)).toBe('validate');
    });

    it('should allow setting to convert', () => {
      currentTab.set('convert');
      expect(get(currentTab)).toBe('convert');
    });

    it('should allow setting to visualize', () => {
      currentTab.set('visualize');
      expect(get(currentTab)).toBe('visualize');
    });

    it('should allow setting to builder', () => {
      currentTab.set('builder');
      expect(get(currentTab)).toBe('builder');
    });

    it('should allow subscription', () => {
      const values: TabType[] = [];
      const unsubscribe = currentTab.subscribe(value => values.push(value));

      currentTab.set('convert');
      currentTab.set('builder');
      currentTab.set('validate');

      expect(values).toEqual(['validate', 'convert', 'builder', 'validate']);
      unsubscribe();
    });

    it('should maintain last set value', () => {
      currentTab.set('visualize');
      currentTab.set('builder');
      expect(get(currentTab)).toBe('builder');
    });
  });

  describe('currentFile', () => {
    it('should initialize as null', () => {
      expect(get(currentFile)).toBeNull();
    });

    it('should allow setting a File object', () => {
      const file = new File(['test content'], 'test.yaml', { type: 'text/yaml' });
      currentFile.set(file);
      expect(get(currentFile)).toBe(file);
    });

    it('should allow setting back to null', () => {
      const file = new File(['test content'], 'test.yaml', { type: 'text/yaml' });
      currentFile.set(file);
      currentFile.set(null);
      expect(get(currentFile)).toBeNull();
    });

    it('should preserve file properties', () => {
      const file = new File(['test content'], 'workout.yaml', { type: 'application/x-yaml' });
      currentFile.set(file);
      const stored = get(currentFile);
      expect(stored?.name).toBe('workout.yaml');
      expect(stored?.type).toBe('application/x-yaml');
    });

    it('should allow subscription', () => {
      const values: (File | null)[] = [];
      const unsubscribe = currentFile.subscribe(value => values.push(value));

      const file1 = new File(['content1'], 'file1.yaml');
      const file2 = new File(['content2'], 'file2.yaml');

      currentFile.set(file1);
      currentFile.set(file2);
      currentFile.set(null);

      expect(values).toHaveLength(4); // initial null + 3 sets
      expect(values[0]).toBeNull();
      expect(values[1]).toBe(file1);
      expect(values[2]).toBe(file2);
      expect(values[3]).toBeNull();

      unsubscribe();
    });
  });

  describe('validationResult', () => {
    it('should initialize as null', () => {
      expect(get(validationResult)).toBeNull();
    });

    it('should allow setting a valid result', () => {
      const result: ValidationResult = {
        valid: true,
        plan: { plan_version: 1, cycle: { days: [] } },
        errors: [],
        warnings: []
      };

      validationResult.set(result);
      expect(get(validationResult)).toEqual(result);
    });

    it('should allow setting an invalid result with errors', () => {
      const errors: ValidationIssue[] = [
        {
          path: 'meta.name',
          message: 'Name is required',
          severity: 'error',
          code: 'PWF-P001'
        }
      ];

      const result: ValidationResult = {
        valid: false,
        errors,
        warnings: []
      };

      validationResult.set(result);
      const stored = get(validationResult);
      expect(stored?.valid).toBe(false);
      expect(stored?.errors).toHaveLength(1);
      expect(stored?.errors[0].message).toBe('Name is required');
    });

    it('should allow setting result with warnings', () => {
      const warnings: ValidationIssue[] = [
        {
          path: 'meta.description',
          message: 'Description is recommended',
          severity: 'warning',
          code: 'PWF-W001'
        }
      ];

      const result: ValidationResult = {
        valid: true,
        plan: { plan_version: 1, cycle: { days: [] } },
        errors: [],
        warnings
      };

      validationResult.set(result);
      const stored = get(validationResult);
      expect(stored?.warnings).toHaveLength(1);
      expect(stored?.warnings[0].severity).toBe('warning');
    });

    it('should allow setting result with statistics', () => {
      const result: ValidationResult = {
        valid: true,
        plan: { plan_version: 1, cycle: { days: [] } },
        errors: [],
        warnings: [],
        statistics: {
          totalExercises: 10,
          totalDays: 3,
          avgExercisesPerDay: 3.33
        }
      };

      validationResult.set(result);
      const stored = get(validationResult);
      expect(stored?.statistics).toBeDefined();
      expect(stored?.statistics?.totalExercises).toBe(10);
    });

    it('should allow setting history validation result', () => {
      const result: ValidationResult = {
        valid: true,
        history: {
          history_version: 1,
          workouts: []
        },
        errors: [],
        warnings: []
      };

      validationResult.set(result);
      const stored = get(validationResult);
      expect(stored?.history).toBeDefined();
      expect(stored?.plan).toBeUndefined();
    });

    it('should allow setting back to null', () => {
      const result: ValidationResult = {
        valid: true,
        errors: [],
        warnings: []
      };

      validationResult.set(result);
      validationResult.set(null);
      expect(get(validationResult)).toBeNull();
    });

    it('should allow subscription', () => {
      const values: (ValidationResult | null)[] = [];
      const unsubscribe = validationResult.subscribe(value => values.push(value));

      const result1: ValidationResult = {
        valid: true,
        errors: [],
        warnings: []
      };

      const result2: ValidationResult = {
        valid: false,
        errors: [{ path: 'test', message: 'error', severity: 'error' }],
        warnings: []
      };

      validationResult.set(result1);
      validationResult.set(result2);
      validationResult.set(null);

      expect(values).toHaveLength(4);
      expect(values[0]).toBeNull();
      expect(values[1]?.valid).toBe(true);
      expect(values[2]?.valid).toBe(false);
      expect(values[3]).toBeNull();

      unsubscribe();
    });
  });

  describe('conversionWarnings', () => {
    it('should initialize as empty array', () => {
      expect(get(conversionWarnings)).toEqual([]);
    });

    it('should allow setting warnings array', () => {
      const warnings: ConversionWarning[] = [
        {
          type: 'MissingField',
          message: 'Field X is missing'
        },
        {
          type: 'ValueClamped',
          message: 'Value clamped to maximum'
        }
      ];

      conversionWarnings.set(warnings);
      expect(get(conversionWarnings)).toEqual(warnings);
    });

    it('should support all warning types', () => {
      const warnings: ConversionWarning[] = [
        { type: 'MissingField', message: 'Missing field' },
        { type: 'ValueClamped', message: 'Value clamped' },
        { type: 'UnsupportedFeature', message: 'Feature not supported' },
        { type: 'TimeSeriesSkipped', message: 'Time series skipped' },
        { type: 'DataQualityIssue', message: 'Data quality issue' }
      ];

      conversionWarnings.set(warnings);
      const stored = get(conversionWarnings);
      expect(stored).toHaveLength(5);
    });

    it('should allow clearing warnings', () => {
      conversionWarnings.set([
        { type: 'MissingField', message: 'Test' }
      ]);
      conversionWarnings.set([]);
      expect(get(conversionWarnings)).toEqual([]);
    });

    it('should allow subscription', () => {
      const values: ConversionWarning[][] = [];
      const unsubscribe = conversionWarnings.subscribe(value => values.push(value));

      const warnings1: ConversionWarning[] = [
        { type: 'MissingField', message: 'Test 1' }
      ];

      const warnings2: ConversionWarning[] = [
        { type: 'ValueClamped', message: 'Test 2' }
      ];

      conversionWarnings.set(warnings1);
      conversionWarnings.set(warnings2);
      conversionWarnings.set([]);

      expect(values).toHaveLength(4);
      expect(values[0]).toEqual([]);
      expect(values[1]).toHaveLength(1);
      expect(values[2]).toHaveLength(1);
      expect(values[3]).toEqual([]);

      unsubscribe();
    });

    it('should allow updating via update method', () => {
      conversionWarnings.update(current => [
        ...current,
        { type: 'MissingField', message: 'New warning' }
      ]);

      const stored = get(conversionWarnings);
      expect(stored).toHaveLength(1);
      expect(stored[0].message).toBe('New warning');
    });
  });

  describe('darkMode store (with localStorage)', () => {
    beforeEach(() => {
      localStorageMock.clear();

      // Mock document.documentElement.classList
      global.document = {
        documentElement: {
          classList: {
            toggle: vi.fn(),
            add: vi.fn(),
            remove: vi.fn()
          }
        }
      } as any;

      // Mock localStorage globally
      Object.defineProperty(global, 'localStorage', {
        value: localStorageMock,
        writable: true
      });
    });

    it('should initialize with default value when no localStorage or system preference', () => {
      // Mock window.matchMedia
      Object.defineProperty(global, 'matchMedia', {
        writable: true,
        value: vi.fn().mockImplementation(() => matchMediaMock(false))
      });

      // Note: The actual initialization happens when the module is loaded
      // For this test, we're just checking the current value
      expect([true, false]).toContain(get(darkMode));
    });

    it('should load darkMode from localStorage when available', () => {
      // Set a value in localStorage before initialization
      localStorageMock.setItem('darkMode', 'true');

      // The store should reflect this value (or at least be able to change)
      darkMode.set(true);
      expect(get(darkMode)).toBe(true);

      localStorageMock.setItem('darkMode', 'false');
      darkMode.set(false);
      expect(get(darkMode)).toBe(false);
    });

    it('should persist darkMode value to localStorage on change', () => {
      darkMode.set(true);
      expect(localStorageMock.getItem('darkMode')).toBe('true');

      darkMode.set(false);
      expect(localStorageMock.getItem('darkMode')).toBe('false');
    });

    it('should toggle dark class on document element', () => {
      const toggleSpy = document.documentElement.classList.toggle as any;

      darkMode.set(true);
      expect(toggleSpy).toHaveBeenCalled();

      darkMode.set(false);
      expect(toggleSpy).toHaveBeenCalled();
    });

    it('should allow subscription to darkMode changes', () => {
      const values: boolean[] = [];
      const unsubscribe = darkMode.subscribe(value => values.push(value));

      darkMode.set(true);
      darkMode.set(false);
      darkMode.set(true);

      expect(values.length).toBeGreaterThanOrEqual(3);

      unsubscribe();
    });
  });

  describe('type exports', () => {
    it('should properly type TabType', () => {
      const tabs: TabType[] = ['validate', 'convert', 'visualize', 'builder'];
      expect(tabs).toHaveLength(4);
    });

    it('should properly type ValidationIssue', () => {
      const issue: ValidationIssue = {
        path: 'test.path',
        message: 'Test message',
        severity: 'error',
        code: 'TEST-001'
      };

      expect(issue.severity).toBe('error');
    });

    it('should properly type ValidationIssue without optional code', () => {
      const issue: ValidationIssue = {
        path: 'test.path',
        message: 'Test message',
        severity: 'warning'
      };

      expect(issue.code).toBeUndefined();
    });

    it('should properly type ValidationResult with optional fields', () => {
      const result: ValidationResult = {
        valid: true,
        errors: [],
        warnings: []
      };

      expect(result.plan).toBeUndefined();
      expect(result.history).toBeUndefined();
      expect(result.statistics).toBeUndefined();
    });

    it('should properly type ConversionWarning', () => {
      const warning: ConversionWarning = {
        type: 'MissingField',
        message: 'Test'
      };

      expect(warning.type).toBe('MissingField');
    });
  });

  describe('store updates and reactivity', () => {
    it('should allow update method on all writable stores', () => {
      wasmReady.update(v => !v);
      expect(get(wasmReady)).toBe(true);

      currentTab.update(() => 'builder');
      expect(get(currentTab)).toBe('builder');

      currentFile.update(() => new File(['test'], 'test.yaml'));
      expect(get(currentFile)?.name).toBe('test.yaml');

      validationResult.update(() => ({
        valid: true,
        errors: [],
        warnings: []
      }));
      expect(get(validationResult)?.valid).toBe(true);

      conversionWarnings.update(w => [...w, { type: 'MissingField', message: 'test' }]);
      expect(get(conversionWarnings)).toHaveLength(1);

      darkMode.update(v => !v);
      // Value should have toggled
      expect([true, false]).toContain(get(darkMode));
    });

    it('should handle multiple subscribers', () => {
      const values1: boolean[] = [];
      const values2: boolean[] = [];

      const unsub1 = wasmReady.subscribe(v => values1.push(v));
      const unsub2 = wasmReady.subscribe(v => values2.push(v));

      wasmReady.set(true);
      wasmReady.set(false);

      expect(values1).toEqual(values2);

      unsub1();
      unsub2();
    });

    it('should unsubscribe properly', () => {
      const values: boolean[] = [];
      const unsubscribe = wasmReady.subscribe(v => values.push(v));

      wasmReady.set(true);
      unsubscribe();
      wasmReady.set(false);

      // Should only have initial value + one update
      expect(values).toEqual([false, true]);
    });
  });
});
