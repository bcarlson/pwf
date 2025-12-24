/**
 * WASM module loader and initialization
 */

import { wasmReady } from './stores';
import type { ValidationResult, ConversionWarning } from './stores';

let wasmModule: any = null;

/**
 * Load and initialize the WASM module
 */
export async function loadWasm(): Promise<boolean> {
  if (wasmModule) {
    return true;
  }

  try {
    // Dynamic import of WASM module
    wasmModule = await import('../wasm/pwf_wasm');
    await wasmModule.default();

    wasmReady.set(true);
    console.log('PWF WASM module loaded successfully');
    return true;
  } catch (error) {
    console.error('Failed to load WASM module:', error);
    wasmReady.set(false);
    return false;
  }
}

/**
 * Get PWF version
 */
export function getVersion(): string {
  if (!wasmModule) throw new Error('WASM not initialized');
  return wasmModule.get_version();
}

/**
 * Get supported modalities
 */
export function getSupportedModalities(): string[] {
  if (!wasmModule) throw new Error('WASM not initialized');
  return wasmModule.get_supported_modalities();
}

/**
 * Get supported sports
 */
export function getSupportedSports(): string[] {
  if (!wasmModule) throw new Error('WASM not initialized');
  return wasmModule.get_supported_sports();
}

/**
 * Get supported equipment
 */
export function getSupportedEquipment(): string[] {
  if (!wasmModule) throw new Error('WASM not initialized');
  return wasmModule.get_supported_equipment();
}

/**
 * Validate a PWF plan
 */
export function validatePlan(yaml: string): ValidationResult {
  if (!wasmModule) throw new Error('WASM not initialized');
  return wasmModule.validate_plan(yaml);
}

/**
 * Validate a PWF history file
 */
export function validateHistory(yaml: string): ValidationResult {
  if (!wasmModule) throw new Error('WASM not initialized');
  return wasmModule.validate_history(yaml);
}

/**
 * Convert FIT to PWF
 */
export function fitToPwf(bytes: Uint8Array, summaryOnly: boolean = false): {
  pwf_yaml?: string;
  warnings: ConversionWarning[];
  error?: string;
} {
  if (!wasmModule) throw new Error('WASM not initialized');
  return wasmModule.fit_to_pwf(bytes, summaryOnly);
}

/**
 * Convert TCX to PWF
 */
export function tcxToPwf(bytes: Uint8Array, summaryOnly: boolean = false): {
  pwf_yaml?: string;
  warnings: ConversionWarning[];
  error?: string;
} {
  if (!wasmModule) throw new Error('WASM not initialized');
  return wasmModule.tcx_to_pwf(bytes, summaryOnly);
}

/**
 * Convert GPX to PWF
 */
export function gpxToPwf(bytes: Uint8Array, summaryOnly: boolean = false): {
  pwf_yaml?: string;
  warnings: ConversionWarning[];
  error?: string;
} {
  if (!wasmModule) throw new Error('WASM not initialized');
  return wasmModule.gpx_to_pwf(bytes, summaryOnly);
}

/**
 * Convert PWF to TCX
 */
export function pwfToTcx(yaml: string): {
  tcx_xml?: string;
  warnings: ConversionWarning[];
  error?: string;
} {
  if (!wasmModule) throw new Error('WASM not initialized');
  return wasmModule.pwf_to_tcx(yaml);
}

/**
 * Convert PWF to GPX
 */
export function pwfToGpx(yaml: string): {
  gpx_xml?: string;
  warnings: ConversionWarning[];
  error?: string;
} {
  if (!wasmModule) throw new Error('WASM not initialized');
  return wasmModule.pwf_to_gpx(yaml);
}

/**
 * Convert PWF to CSV
 */
export function pwfToCsv(yaml: string): {
  csv_data?: string;
  warnings: ConversionWarning[];
  data_points: number;
  workouts_processed: number;
  error?: string;
} {
  if (!wasmModule) throw new Error('WASM not initialized');
  return wasmModule.pwf_to_csv(yaml);
}
