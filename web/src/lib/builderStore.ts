/**
 * State management for the PWF Visual Plan Builder
 * Provides a Svelte writable store with localStorage persistence
 */

import { writable } from 'svelte/store';
import type { PlanMeta, PlanExercise } from '@pwf-dev/core';

/**
 * Draft representation of a plan day
 */
export interface PlanDayDraft {
  id?: string;
  order?: number;
  focus?: string;
  target_session_length_min?: number;
  notes?: string;
  exercises: PlanExercise[];
}

/**
 * Draft representation of a plan cycle
 */
export interface PlanCycleDraft {
  notes?: string;
  days: PlanDayDraft[];
}

/**
 * Draft representation of a complete PWF plan
 */
export interface PlanDraft {
  plan_version: number;
  meta?: PlanMeta;
  glossary?: Record<string, string>;
  cycle?: PlanCycleDraft;
}

/**
 * Builder state interface
 */
export interface BuilderState {
  currentPlan: PlanDraft;
  currentDayIndex: number | null;
  isDirty: boolean;
}

const STORAGE_KEY = 'pwf-builder-state';
const DEBOUNCE_MS = 500;

/**
 * Load state from localStorage
 */
export function loadFromLocalStorage(): BuilderState | null {
  if (typeof window === 'undefined') {
    return null;
  }

  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (!stored) {
      return null;
    }

    const parsed = JSON.parse(stored);
    return parsed;
  } catch (error) {
    console.error('Failed to load builder state from localStorage:', error);
    return null;
  }
}

/**
 * Save state to localStorage (debounced)
 */
let saveTimeout: ReturnType<typeof setTimeout> | null = null;

export function saveToLocalStorage(state: BuilderState): void {
  if (typeof window === 'undefined') {
    return;
  }

  // Clear existing timeout
  if (saveTimeout !== null) {
    clearTimeout(saveTimeout);
  }

  // Debounce the save
  saveTimeout = setTimeout(() => {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
    } catch (error) {
      console.error('Failed to save builder state to localStorage:', error);
    }
    saveTimeout = null;
  }, DEBOUNCE_MS);
}

/**
 * Create initial builder state
 */
function createInitialState(): BuilderState {
  return {
    currentPlan: {
      plan_version: 1,
      cycle: {
        days: []
      }
    },
    currentDayIndex: null,
    isDirty: false
  };
}

/**
 * Initialize the builder store
 */
function createBuilderStore() {
  const storedState = loadFromLocalStorage();
  const initialState = storedState || createInitialState();

  const { subscribe, set, update } = writable<BuilderState>(initialState);

  // Subscribe to changes and auto-save
  subscribe((state) => {
    if (state.isDirty) {
      saveToLocalStorage(state);
    }
  });

  return {
    subscribe,
    set,
    update,

    /**
     * Reset the store to initial state
     */
    reset: () => {
      set(createInitialState());
    },

    /**
     * Load a plan draft into the store
     */
    loadPlan: (plan: PlanDraft) => {
      update((state) => ({
        ...state,
        currentPlan: plan,
        currentDayIndex: null,
        isDirty: true
      }));
    },

    /**
     * Mark the current state as dirty
     */
    markDirty: () => {
      update((state) => ({
        ...state,
        isDirty: true
      }));
    }
  };
}

export const builderStore = createBuilderStore();
