/**
 * Tests for DaysStep component
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import { get } from 'svelte/store';
import DaysStep from '../steps/DaysStep.svelte';
import { builderState } from '../../../lib/builderState';

describe('DaysStep', () => {
  beforeEach(() => {
    builderState.reset();
  });

  it('should render the component', () => {
    const { container } = render(DaysStep);
    expect(container.querySelector('.days-step')).toBeTruthy();
  });

  it('should render step header', () => {
    const { getByText } = render(DaysStep);
    expect(getByText('Workout Days')).toBeTruthy();
    expect(getByText(/Define the days/)).toBeTruthy();
  });

  it('should render default day', () => {
    const { getByText } = render(DaysStep);
    expect(getByText('Day 1')).toBeTruthy();
  });

  it('should render Add Day button', () => {
    const { getByText } = render(DaysStep);
    expect(getByText('+ Add Day')).toBeTruthy();
  });

  it('should add a new day when Add Day button is clicked', async () => {
    const { getByText } = render(DaysStep);
    const addButton = getByText('+ Add Day');

    await fireEvent.click(addButton);

    const state = get(builderState);
    expect(state.plan.cycle.days).toHaveLength(2);
    expect(getByText('Day 2')).toBeTruthy();
  });

  it('should remove a day when Remove button is clicked', async () => {
    // Add a second day first
    builderState.addDay();

    const { getAllByText, queryByText } = render(DaysStep);
    const removeButtons = getAllByText('Remove');

    await fireEvent.click(removeButtons[1]); // Remove second day

    const state = get(builderState);
    expect(state.plan.cycle.days).toHaveLength(1);
    expect(queryByText('Day 2')).toBeNull();
  });

  it('should not allow removing the last day', async () => {
    const { queryByText } = render(DaysStep);

    // With only one day, remove button should not be present
    const removeButton = queryByText('Remove');
    expect(removeButton).toBeNull();

    const state = get(builderState);
    expect(state.plan.cycle.days).toHaveLength(1);
  });

  it('should display exercise count for each day', () => {
    builderState.addExercise(0);
    builderState.addExercise(0);

    const { getByText } = render(DaysStep);
    expect(getByText('2 exercises')).toBeTruthy();
  });

  it('should display singular "exercise" for one exercise', () => {
    builderState.addExercise(0);

    const { getByText } = render(DaysStep);
    expect(getByText('1 exercise')).toBeTruthy();
  });

  it('should display focus badge when day has focus', () => {
    builderState.updateDay(0, { focus: 'Upper Body' });

    const { getByText } = render(DaysStep);
    expect(getByText('Upper Body')).toBeTruthy();
  });

  it('should render DayForm for each day', () => {
    builderState.addDay();
    builderState.addDay();

    const { container } = render(DaysStep);
    const dayForms = container.querySelectorAll('.day-form');

    expect(dayForms).toHaveLength(3);
  });

  it('should not show validation warning when days exist', () => {
    const { queryByText } = render(DaysStep);
    expect(queryByText('At least one day is required')).toBeNull();
  });

  it('should have proper day card structure', () => {
    const { container } = render(DaysStep);
    const dayCard = container.querySelector('.day-card');

    expect(dayCard).toBeTruthy();
    expect(dayCard?.querySelector('.day-card-header')).toBeTruthy();
    expect(dayCard?.querySelector('.day-meta')).toBeTruthy();
  });
});
