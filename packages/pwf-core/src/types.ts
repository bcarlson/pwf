import type {
  Cycle,
  Day,
  Exercise,
  Meta,
  PWFPlanV1
} from './generated/plan';
import type {
  CompletedExercise,
  PWFHistoryExportV1,
  Workout
} from './generated/history';

export type Severity = 'error' | 'warning';

export interface ValidationIssue {
  path: string;
  message: string;
  severity: Severity;
  code?: string;
}

export type ValidationError = ValidationIssue;
export type ValidationWarning = ValidationIssue;

export interface ValidationOptions {
  strict?: boolean;
}

export interface ParseOptions extends ValidationOptions {}

export type PlanMeta = Meta;
export type PlanExercise = Exercise;
export type PlanDay = Day;
export type PlanCycle = Cycle;
export type PwfPlan = PWFPlanV1;
export type Modality = PlanExercise['modality'];

export interface PlanDocument extends PwfPlan {
  toYAML(): string;
}

export type HistoryExercise = CompletedExercise;
export type HistoryWorkout = Workout;
export type PwfHistory = PWFHistoryExportV1;
