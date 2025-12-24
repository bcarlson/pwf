import { toYAML } from './parse';
import type { PlanDocument, PlanExercise, PlanMeta, PwfPlan } from './types';

type PlanDayDraft = Omit<NonNullable<PwfPlan['cycle']>['days'][number], 'exercises'> & {
  exercises?: PlanExercise[];
};

type PlanCycleDraft = Omit<PwfPlan['cycle'], 'days'> & {
  days: PlanDayDraft[];
};

type PlanDraft = Omit<PwfPlan, 'cycle'> & {
  cycle?: PlanCycleDraft;
};

export class PlanBuilder {
  private plan: PlanDraft;
  private currentDayIndex: number | null = null;

  constructor() {
    this.plan = {
      plan_version: 1,
      cycle: {
        days: []
      }
    };
  }

  version(version: PwfPlan['plan_version']): this {
    this.plan.plan_version = version;
    return this;
  }

  meta(meta: PlanMeta): this {
    this.plan.meta = meta;
    return this;
  }

  glossary(glossary: Record<string, string>): this {
    this.plan.glossary = glossary;
    return this;
  }

  addDay(focus?: string, dayOverrides: Partial<PlanDayDraft> = {}): this {
    const day: PlanDayDraft = {
      ...(focus ? { focus } : {}),
      ...dayOverrides
    };

    day.exercises = day.exercises ?? [];

    if (!this.plan.cycle) {
      this.plan.cycle = { days: [] };
    }

    this.plan.cycle.days = this.plan.cycle.days ?? [];
    this.plan.cycle.days.push(day);
    this.currentDayIndex = this.plan.cycle.days.length - 1;
    return this;
  }

  addExercise(name: string, exercise: Omit<PlanExercise, 'name'>): this {
    if (this.currentDayIndex === null || !this.plan.cycle?.days?.[this.currentDayIndex]) {
      throw new Error('addExercise requires an active day. Call addDay first.');
    }

    const day = this.plan.cycle.days[this.currentDayIndex];
    day.exercises = day.exercises ?? [];
    day.exercises.push({ name, ...exercise });
    return this;
  }

  toYAML(): string {
    return toYAML(this.build());
  }

  build(): PlanDocument {
    if (!this.plan.cycle || this.plan.cycle.days.length === 0) {
      throw new Error('Plan requires at least one day.');
    }

    for (const [index, day] of this.plan.cycle.days.entries()) {
      if (!day.exercises || day.exercises.length === 0) {
        throw new Error(`Day ${index + 1} requires at least one exercise.`);
      }
    }

    const document = this.plan as unknown as PlanDocument;

    Object.defineProperty(document, 'toYAML', {
      value: () => toYAML(document),
      enumerable: false
    });

    return document;
  }
}
