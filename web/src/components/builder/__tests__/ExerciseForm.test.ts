/**
 * Tests for ExerciseForm component
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import { get } from 'svelte/store';
import ExerciseForm from '../forms/ExerciseForm.svelte';
import { builderState } from '../../../lib/builderState';
import * as wasm from '../../../lib/wasm';

// Mock WASM module
vi.mock('../../../lib/wasm', () => ({
  getSupportedModalities: vi.fn(() => ['strength', 'countdown', 'stopwatch', 'interval']),
  getSupportedEquipment: vi.fn(() => ['barbell', 'dumbbell', 'kettlebell'])
}));

describe('ExerciseForm', () => {
  beforeEach(() => {
    builderState.reset();
    builderState.addExercise(0);
    vi.clearAllMocks();
  });

  it('should render the form', () => {
    const { container } = render(ExerciseForm, {
      props: { dayIndex: 0, exerciseIndex: 0 }
    });
    expect(container.querySelector('.exercise-form')).toBeTruthy();
  });

  it('should render form header with exercise number', () => {
    const { getByText } = render(ExerciseForm, {
      props: { dayIndex: 0, exerciseIndex: 0 }
    });
    expect(getByText('Exercise 1')).toBeTruthy();
  });

  it('should render delete button', () => {
    const { getByText } = render(ExerciseForm, {
      props: { dayIndex: 0, exerciseIndex: 0 }
    });
    expect(getByText('Delete')).toBeTruthy();
  });

  it('should delete exercise when delete button is clicked', async () => {
    const { getByText } = render(ExerciseForm, {
      props: { dayIndex: 0, exerciseIndex: 0 }
    });

    const deleteButton = getByText('Delete');
    await fireEvent.click(deleteButton);

    const state = get(builderState);
    expect(state.plan.cycle.days[0].exercises).toHaveLength(0);
  });

  it('should update exercise name', async () => {
    const { getByLabelText } = render(ExerciseForm, {
      props: { dayIndex: 0, exerciseIndex: 0 }
    });

    const nameInput = getByLabelText(/Exercise Name/) as HTMLInputElement;
    await fireEvent.input(nameInput, { target: { value: 'Squat' } });

    const state = get(builderState);
    expect(state.plan.cycle.days[0].exercises[0].name).toBe('Squat');
  });

  it('should update exercise modality', async () => {
    const { getByLabelText } = render(ExerciseForm, {
      props: { dayIndex: 0, exerciseIndex: 0 }
    });

    const modalitySelect = getByLabelText(/Modality/) as HTMLSelectElement;
    await fireEvent.change(modalitySelect, { target: { value: 'countdown' } });

    const state = get(builderState);
    expect(state.plan.cycle.days[0].exercises[0].modality).toBe('countdown');
  });

  it('should clear modality-specific fields when modality changes', async () => {
    // Set up strength exercise with fields
    builderState.updateExercise(0, 0, {
      modality: 'strength',
      target_sets: 3,
      target_reps: 10
    });

    const { getByLabelText } = render(ExerciseForm, {
      props: { dayIndex: 0, exerciseIndex: 0 }
    });

    const modalitySelect = getByLabelText(/Modality/) as HTMLSelectElement;
    await fireEvent.change(modalitySelect, { target: { value: 'countdown' } });

    const state = get(builderState);
    expect(state.plan.cycle.days[0].exercises[0].target_sets).toBeUndefined();
    expect(state.plan.cycle.days[0].exercises[0].target_reps).toBeUndefined();
  });

  it('should toggle equipment selection', async () => {
    const { getByText } = render(ExerciseForm, {
      props: { dayIndex: 0, exerciseIndex: 0 }
    });

    const barbellChip = getByText('barbell');
    await fireEvent.click(barbellChip);

    let state = get(builderState);
    expect(state.plan.cycle.days[0].exercises[0].equipment).toContain('barbell');

    // Click again to deselect
    await fireEvent.click(barbellChip);

    state = get(builderState);
    expect(state.plan.cycle.days[0].exercises[0].equipment).not.toContain('barbell');
  });

  it('should update exercise notes', async () => {
    const { getByLabelText } = render(ExerciseForm, {
      props: { dayIndex: 0, exerciseIndex: 0 }
    });

    const notesInput = getByLabelText(/Notes/) as HTMLTextAreaElement;
    await fireEvent.input(notesInput, { target: { value: 'Focus on form' } });

    const state = get(builderState);
    expect(state.plan.cycle.days[0].exercises[0].notes).toBe('Focus on form');
  });

  it('should render ModalityFields component', () => {
    const { container } = render(ExerciseForm, {
      props: { dayIndex: 0, exerciseIndex: 0 }
    });

    expect(container.querySelector('.modality-fields')).toBeTruthy();
  });

  it('should handle WASM errors gracefully', () => {
    vi.mocked(wasm.getSupportedModalities).mockImplementation(() => {
      throw new Error('WASM not ready');
    });

    // Should not throw
    expect(() => render(ExerciseForm, {
      props: { dayIndex: 0, exerciseIndex: 0 }
    })).not.toThrow();
  });

  it('should render all modality options', () => {
    const { container } = render(ExerciseForm, {
      props: { dayIndex: 0, exerciseIndex: 0 }
    });

    const select = container.querySelector('select') as HTMLSelectElement;
    const options = Array.from(select.options).map(o => o.value);

    expect(options).toContain('strength');
    expect(options).toContain('countdown');
    expect(options).toContain('stopwatch');
    expect(options).toContain('interval');
  });

  it('should render equipment grid', () => {
    const { container } = render(ExerciseForm, {
      props: { dayIndex: 0, exerciseIndex: 0 }
    });

    expect(container.querySelector('.equipment-grid')).toBeTruthy();
  });
});
