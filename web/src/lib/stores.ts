/**
 * Svelte stores for global application state
 */

import { writable } from 'svelte/store';

// WASM module state
export const wasmReady = writable<boolean>(false);

// Current active tab
export type TabType = 'validate' | 'convert' | 'visualize';
export const currentTab = writable<TabType>('validate');

// File upload state
export const currentFile = writable<File | null>(null);

// Validation result state
export interface ValidationResult {
  valid: boolean;
  plan?: any;
  history?: any;
  errors: ValidationIssue[];
  warnings: ValidationIssue[];
  statistics?: any;
}

export interface ValidationIssue {
  path: string;
  message: string;
  severity: 'error' | 'warning';
  code?: string;
}

export const validationResult = writable<ValidationResult | null>(null);

// Conversion warnings state
export interface ConversionWarning {
  type: 'MissingField' | 'ValueClamped' | 'UnsupportedFeature' | 'TimeSeriesSkipped' | 'DataQualityIssue';
  message: string;
}

export const conversionWarnings = writable<ConversionWarning[]>([]);

// Dark mode theme
export const darkMode = writable<boolean>(false);

// Initialize dark mode from localStorage
if (typeof window !== 'undefined') {
  const stored = localStorage.getItem('darkMode');
  if (stored !== null) {
    darkMode.set(stored === 'true');
  } else {
    // Check system preference
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    darkMode.set(prefersDark);
  }

  // Subscribe to changes and persist
  darkMode.subscribe(value => {
    localStorage.setItem('darkMode', String(value));
    document.documentElement.classList.toggle('dark', value);
  });
}
