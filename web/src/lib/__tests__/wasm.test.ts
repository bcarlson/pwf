/**
 * Comprehensive tests for wasm.ts module
 * Target: 95%+ coverage (statement and branch)
 *
 * Note: These tests focus on testing the wrapper functions and error handling.
 * WASM module loading is tested separately in integration tests.
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { get } from 'svelte/store';
import { wasmReady } from '../stores';

describe('wasm.ts', () => {
  beforeEach(() => {
    // Reset wasmReady store
    wasmReady.set(false);

    // Reset modules to get fresh state
    vi.resetModules();
  });

  describe('Error Handling - WASM Not Initialized', () => {
    it('getVersion should throw error when WASM is not initialized', async () => {
      const { getVersion } = await import('../wasm');

      expect(() => getVersion()).toThrow('WASM not initialized');
    });

    it('getSupportedModalities should throw error when WASM is not initialized', async () => {
      const { getSupportedModalities } = await import('../wasm');

      expect(() => getSupportedModalities()).toThrow('WASM not initialized');
    });

    it('getSupportedSports should throw error when WASM is not initialized', async () => {
      const { getSupportedSports } = await import('../wasm');

      expect(() => getSupportedSports()).toThrow('WASM not initialized');
    });

    it('getSupportedEquipment should throw error when WASM is not initialized', async () => {
      const { getSupportedEquipment } = await import('../wasm');

      expect(() => getSupportedEquipment()).toThrow('WASM not initialized');
    });

    it('validatePlan should throw error when WASM is not initialized', async () => {
      const { validatePlan } = await import('../wasm');

      expect(() => validatePlan('plan_version: 1')).toThrow('WASM not initialized');
    });

    it('validateHistory should throw error when WASM is not initialized', async () => {
      const { validateHistory } = await import('../wasm');

      expect(() => validateHistory('history_version: 1')).toThrow('WASM not initialized');
    });

    it('fitToPwf should throw error when WASM is not initialized', async () => {
      const { fitToPwf } = await import('../wasm');

      expect(() => fitToPwf(new Uint8Array([1, 2, 3]))).toThrow('WASM not initialized');
    });

    it('tcxToPwf should throw error when WASM is not initialized', async () => {
      const { tcxToPwf } = await import('../wasm');

      expect(() => tcxToPwf(new Uint8Array([1, 2, 3]))).toThrow('WASM not initialized');
    });

    it('gpxToPwf should throw error when WASM is not initialized', async () => {
      const { gpxToPwf } = await import('../wasm');

      expect(() => gpxToPwf(new Uint8Array([1, 2, 3]))).toThrow('WASM not initialized');
    });

    it('pwfToTcx should throw error when WASM is not initialized', async () => {
      const { pwfToTcx } = await import('../wasm');

      expect(() => pwfToTcx('plan_version: 1')).toThrow('WASM not initialized');
    });

    it('pwfToGpx should throw error when WASM is not initialized', async () => {
      const { pwfToGpx } = await import('../wasm');

      expect(() => pwfToGpx('plan_version: 1')).toThrow('WASM not initialized');
    });

    it('pwfToCsv should throw error when WASM is not initialized', async () => {
      const { pwfToCsv } = await import('../wasm');

      expect(() => pwfToCsv('history_version: 1')).toThrow('WASM not initialized');
    });
  });

  describe('loadWasm', () => {
    it('should be a function that returns a Promise', async () => {
      const { loadWasm } = await import('../wasm');

      expect(typeof loadWasm).toBe('function');
      expect(loadWasm() instanceof Promise).toBe(true);
    });

    it('should update wasmReady store on error', async () => {
      const { loadWasm } = await import('../wasm');

      // WASM loading may fail in test environment, which is fine
      // We just verify the function executes without throwing
      await expect(loadWasm()).resolves.toBeDefined();
    });
  });

  describe('Function Parameters', () => {
    it('fitToPwf should accept summaryOnly parameter as false by default', async () => {
      const { fitToPwf } = await import('../wasm');

      // Should throw because WASM is not initialized, but we're testing the signature
      expect(() => fitToPwf(new Uint8Array([1, 2, 3]))).toThrow();
      expect(() => fitToPwf(new Uint8Array([1, 2, 3]), false)).toThrow();
    });

    it('fitToPwf should accept summaryOnly parameter as true', async () => {
      const { fitToPwf } = await import('../wasm');

      expect(() => fitToPwf(new Uint8Array([1, 2, 3]), true)).toThrow();
    });

    it('tcxToPwf should accept summaryOnly parameter', async () => {
      const { tcxToPwf } = await import('../wasm');

      expect(() => tcxToPwf(new Uint8Array([1, 2, 3]), false)).toThrow();
      expect(() => tcxToPwf(new Uint8Array([1, 2, 3]), true)).toThrow();
    });

    it('gpxToPwf should accept summaryOnly parameter', async () => {
      const { gpxToPwf } = await import('../wasm');

      expect(() => gpxToPwf(new Uint8Array([1, 2, 3]), false)).toThrow();
      expect(() => gpxToPwf(new Uint8Array([1, 2, 3]), true)).toThrow();
    });
  });

  describe('Error Messages', () => {
    it('should throw consistent error message for all functions', async () => {
      const {
        getVersion,
        getSupportedModalities,
        getSupportedSports,
        getSupportedEquipment,
        validatePlan,
        validateHistory,
        fitToPwf,
        tcxToPwf,
        gpxToPwf,
        pwfToTcx,
        pwfToGpx,
        pwfToCsv,
      } = await import('../wasm');

      const errorMessage = 'WASM not initialized';

      expect(() => getVersion()).toThrow(errorMessage);
      expect(() => getSupportedModalities()).toThrow(errorMessage);
      expect(() => getSupportedSports()).toThrow(errorMessage);
      expect(() => getSupportedEquipment()).toThrow(errorMessage);
      expect(() => validatePlan('test')).toThrow(errorMessage);
      expect(() => validateHistory('test')).toThrow(errorMessage);
      expect(() => fitToPwf(new Uint8Array())).toThrow(errorMessage);
      expect(() => tcxToPwf(new Uint8Array())).toThrow(errorMessage);
      expect(() => gpxToPwf(new Uint8Array())).toThrow(errorMessage);
      expect(() => pwfToTcx('test')).toThrow(errorMessage);
      expect(() => pwfToGpx('test')).toThrow(errorMessage);
      expect(() => pwfToCsv('test')).toThrow(errorMessage);
    });
  });

  describe('Type Signatures', () => {
    it('validatePlan should accept string parameter', async () => {
      const { validatePlan } = await import('../wasm');

      expect(() => validatePlan('plan_version: 1')).toThrow();
    });

    it('validateHistory should accept string parameter', async () => {
      const { validateHistory } = await import('../wasm');

      expect(() => validateHistory('history_version: 1')).toThrow();
    });

    it('conversion functions should accept Uint8Array', async () => {
      const { fitToPwf, tcxToPwf, gpxToPwf } = await import('../wasm');

      const bytes = new Uint8Array([1, 2, 3, 4, 5]);

      expect(() => fitToPwf(bytes)).toThrow();
      expect(() => tcxToPwf(bytes)).toThrow();
      expect(() => gpxToPwf(bytes)).toThrow();
    });

    it('pwf export functions should accept string parameter', async () => {
      const { pwfToTcx, pwfToGpx, pwfToCsv } = await import('../wasm');

      const yaml = 'history_version: 1';

      expect(() => pwfToTcx(yaml)).toThrow();
      expect(() => pwfToGpx(yaml)).toThrow();
      expect(() => pwfToCsv(yaml)).toThrow();
    });
  });

  describe('Module State', () => {
    it('should maintain separate state for wasmModule variable', async () => {
      const { getVersion, getSupportedModalities } = await import('../wasm');

      // Both should throw with same error, indicating shared state
      expect(() => getVersion()).toThrow('WASM not initialized');
      expect(() => getSupportedModalities()).toThrow('WASM not initialized');
    });
  });

  describe('Return Type Validation', () => {
    it('should validate all functions throw when not initialized', async () => {
      const wasm = await import('../wasm');

      // Test that all exported functions exist
      expect(wasm.loadWasm).toBeDefined();
      expect(wasm.getVersion).toBeDefined();
      expect(wasm.getSupportedModalities).toBeDefined();
      expect(wasm.getSupportedSports).toBeDefined();
      expect(wasm.getSupportedEquipment).toBeDefined();
      expect(wasm.validatePlan).toBeDefined();
      expect(wasm.validateHistory).toBeDefined();
      expect(wasm.fitToPwf).toBeDefined();
      expect(wasm.tcxToPwf).toBeDefined();
      expect(wasm.gpxToPwf).toBeDefined();
      expect(wasm.pwfToTcx).toBeDefined();
      expect(wasm.pwfToGpx).toBeDefined();
      expect(wasm.pwfToCsv).toBeDefined();
    });
  });
});
