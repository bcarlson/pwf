/**
 * Svelte store for PWF Plan Builder state
 */

import { writable } from 'svelte/store';

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
  name: string;
  modality: Modality;

  // Strength fields
  target_sets?: number;
  target_reps?: number;
  target_load?: string;

  // Countdown/Stopwatch/Interval fields
  target_duration_sec?: number;

  // Distance fields (interval, running, rowing, swimming)
  target_distance_meters?: number;

  // Endurance fields (cycling, running, rowing, swimming)
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

export interface Day {
  focus?: string;
  notes?: string;
  exercises: Exercise[];
}

export interface Cycle {
  days: Day[];
  target_reps?: number;
}

export interface Meta {
  name?: string;
  description?: string;
  author?: string;
  equipment?: string[];
  days_per_week?: number;
  tags?: string[];
}

export interface PlanDraft {
  plan_version: number;
  meta?: Meta;
  cycle: Cycle;
}

export interface BuilderState {
  currentStep: number;
  plan: PlanDraft;
  currentDayIndex: number;
}

function createInitialPlan(): PlanDraft {
  return {
    plan_version: 1,
    cycle: {
      days: [
        {
          exercises: []
        }
      ]
    }
  };
}

function createBuilderState() {
  const { subscribe, set, update } = writable<BuilderState>({
    currentStep: 0,
    plan: createInitialPlan(),
    currentDayIndex: 0
  });

  return {
    subscribe,
    set,
    update,

    // Navigation
    nextStep: () => update(state => ({ ...state, currentStep: state.currentStep + 1 })),
    prevStep: () => update(state => ({ ...state, currentStep: Math.max(0, state.currentStep - 1) })),
    goToStep: (step: number) => update(state => ({ ...state, currentStep: step })),

    // Meta operations
    updateMeta: (meta: Partial<Meta>) => update(state => ({
      ...state,
      plan: {
        ...state.plan,
        meta: { ...state.plan.meta, ...meta }
      }
    })),

    // Day operations
    setCurrentDay: (index: number) => update(state => ({ ...state, currentDayIndex: index })),

    addDay: () => update(state => ({
      ...state,
      plan: {
        ...state.plan,
        cycle: {
          ...state.plan.cycle,
          days: [
            ...state.plan.cycle.days,
            { exercises: [] }
          ]
        }
      }
    })),

    removeDay: (index: number) => update(state => {
      const newDays = [...state.plan.cycle.days];
      newDays.splice(index, 1);

      // Ensure at least one day exists
      if (newDays.length === 0) {
        newDays.push({ exercises: [] });
      }

      return {
        ...state,
        plan: {
          ...state.plan,
          cycle: {
            ...state.plan.cycle,
            days: newDays
          }
        },
        currentDayIndex: Math.min(state.currentDayIndex, newDays.length - 1)
      };
    }),

    updateDay: (index: number, day: Partial<Day>) => update(state => {
      const newDays = [...state.plan.cycle.days];
      newDays[index] = { ...newDays[index], ...day };

      return {
        ...state,
        plan: {
          ...state.plan,
          cycle: {
            ...state.plan.cycle,
            days: newDays
          }
        }
      };
    }),

    // Exercise operations
    addExercise: (dayIndex: number) => update(state => {
      const newDays = [...state.plan.cycle.days];
      newDays[dayIndex].exercises.push({
        name: '',
        modality: 'strength'
      });

      return {
        ...state,
        plan: {
          ...state.plan,
          cycle: {
            ...state.plan.cycle,
            days: newDays
          }
        }
      };
    }),

    removeExercise: (dayIndex: number, exerciseIndex: number) => update(state => {
      const newDays = [...state.plan.cycle.days];
      newDays[dayIndex].exercises.splice(exerciseIndex, 1);

      return {
        ...state,
        plan: {
          ...state.plan,
          cycle: {
            ...state.plan.cycle,
            days: newDays
          }
        }
      };
    }),

    updateExercise: (dayIndex: number, exerciseIndex: number, exercise: Partial<Exercise>) => update(state => {
      const newDays = [...state.plan.cycle.days];
      newDays[dayIndex].exercises[exerciseIndex] = {
        ...newDays[dayIndex].exercises[exerciseIndex],
        ...exercise
      };

      return {
        ...state,
        plan: {
          ...state.plan,
          cycle: {
            ...state.plan.cycle,
            days: newDays
          }
        }
      };
    }),

    // Load plan
    loadPlan: (plan: PlanDraft) => set({
      currentStep: 0,
      plan: {
        ...plan,
        cycle: plan.cycle || { days: [{ exercises: [] }] }
      },
      currentDayIndex: 0
    }),

    // Reset
    reset: () => set({
      currentStep: 0,
      plan: createInitialPlan(),
      currentDayIndex: 0
    })
  };
}

export const builderState = createBuilderState();
