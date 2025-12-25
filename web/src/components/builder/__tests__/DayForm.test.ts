/**
 * Tests for DayForm component
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import { get } from 'svelte/store';
import DayForm from '../forms/DayForm.svelte';
import { builderState } from '../../../lib/builderState';

describe('DayForm', () => {
  beforeEach(() => {
    builderState.reset();
  });

  it('should render the form', () => {
    const { container } = render(DayForm, {
      props: { dayIndex: 0 }
    });
    expect(container.querySelector('.day-form')).toBeTruthy();
  });

  it('should render focus input field', () => {
    const { getByLabelText } = render(DayForm, {
      props: { dayIndex: 0 }
    });
    expect(getByLabelText(/Focus/)).toBeTruthy();
  });

  it('should render notes textarea', () => {
    const { getByLabelText } = render(DayForm, {
      props: { dayIndex: 0 }
    });
    expect(getByLabelText(/Notes/)).toBeTruthy();
  });

  it('should update day focus when input changes', async () => {
    const { getByLabelText } = render(DayForm, {
      props: { dayIndex: 0 }
    });

    const focusInput = getByLabelText(/Focus/) as HTMLInputElement;
    await fireEvent.input(focusInput, { target: { value: 'Upper Body' } });

    const state = get(builderState);
    expect(state.plan.cycle.days[0].focus).toBe('Upper Body');
  });

  it('should update day notes when textarea changes', async () => {
    const { getByLabelText } = render(DayForm, {
      props: { dayIndex: 0 }
    });

    const notesInput = getByLabelText(/Notes/) as HTMLTextAreaElement;
    await fireEvent.input(notesInput, { target: { value: 'Focus on form' } });

    const state = get(builderState);
    expect(state.plan.cycle.days[0].notes).toBe('Focus on form');
  });

  it('should clear focus when input is empty', async () => {
    builderState.updateDay(0, { focus: 'Test' });

    const { getByLabelText } = render(DayForm, {
      props: { dayIndex: 0 }
    });

    const focusInput = getByLabelText(/Focus/) as HTMLInputElement;
    await fireEvent.input(focusInput, { target: { value: '' } });

    const state = get(builderState);
    expect(state.plan.cycle.days[0].focus).toBeUndefined();
  });

  it('should clear notes when textarea is empty', async () => {
    builderState.updateDay(0, { notes: 'Test notes' });

    const { getByLabelText } = render(DayForm, {
      props: { dayIndex: 0 }
    });

    const notesInput = getByLabelText(/Notes/) as HTMLTextAreaElement;
    await fireEvent.input(notesInput, { target: { value: '' } });

    const state = get(builderState);
    expect(state.plan.cycle.days[0].notes).toBeUndefined();
  });

  it('should display existing focus value', () => {
    builderState.updateDay(0, { focus: 'Lower Body' });

    const { getByLabelText } = render(DayForm, {
      props: { dayIndex: 0 }
    });

    const focusInput = getByLabelText(/Focus/) as HTMLInputElement;
    expect(focusInput.value).toBe('Lower Body');
  });

  it('should display existing notes value', () => {
    builderState.updateDay(0, { notes: 'Test notes' });

    const { getByLabelText } = render(DayForm, {
      props: { dayIndex: 0 }
    });

    const notesInput = getByLabelText(/Notes/) as HTMLTextAreaElement;
    expect(notesInput.value).toBe('Test notes');
  });

  it('should have unique IDs for different day indices', () => {
    builderState.addDay();

    const { container: container1 } = render(DayForm, {
      props: { dayIndex: 0 }
    });

    const { container: container2 } = render(DayForm, {
      props: { dayIndex: 1 }
    });

    const focus1 = container1.querySelector('#day-focus-0');
    const focus2 = container2.querySelector('#day-focus-1');

    expect(focus1).toBeTruthy();
    expect(focus2).toBeTruthy();
  });

  it('should mark fields as optional', () => {
    const { container } = render(DayForm, {
      props: { dayIndex: 0 }
    });

    const optionalLabels = container.querySelectorAll('.optional');
    expect(optionalLabels.length).toBeGreaterThan(0);
  });

  it('should have correct form structure', () => {
    const { container } = render(DayForm, {
      props: { dayIndex: 0 }
    });

    const formGroups = container.querySelectorAll('.form-group');
    expect(formGroups.length).toBe(2); // focus and notes
  });
});
