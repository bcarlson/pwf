/**
 * Tests for ExercisesStep component
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import { get } from 'svelte/store';
import ExercisesStep from '../steps/ExercisesStep.svelte';
import { builderState } from '../../../lib/builderState';

describe('ExercisesStep', () => {
  beforeEach(() => {
    builderState.reset();
  });

  it('should render the component', () => {
    const { container } = render(ExercisesStep);
    expect(container.querySelector('.exercises-step')).toBeTruthy();
  });

  it('should render step header', () => {
    const { getByText } = render(ExercisesStep);
    expect(getByText('Exercises')).toBeTruthy();
    expect(getByText(/Add exercises to each day/)).toBeTruthy();
  });

  it('should show validation warning when day has no exercises', () => {
    const { getByText } = render(ExercisesStep);
    expect(getByText('All days must have at least one exercise')).toBeTruthy();
  });

  it('should not show validation warning when all days have exercises', () => {
    builderState.addExercise(0);

    const { queryByText } = render(ExercisesStep);
    expect(queryByText('All days must have at least one exercise')).toBeNull();
  });

  it('should render day selector tabs', () => {
    builderState.addDay();
    builderState.addDay();

    const { getAllByText } = render(ExercisesStep);
    // Day text appears in both the tab button and the h3 heading
    expect(getAllByText('Day 1').length).toBeGreaterThan(0);
    expect(getAllByText('Day 2').length).toBeGreaterThan(0);
    expect(getAllByText('Day 3').length).toBeGreaterThan(0);
  });

  it('should switch days when tab is clicked', async () => {
    builderState.addDay();

    const { getAllByText } = render(ExercisesStep);
    const day2Tab = getAllByText('Day 2')[0];

    await fireEvent.click(day2Tab);

    const state = get(builderState);
    expect(state.currentDayIndex).toBe(1);
  });

  it('should highlight active day tab', () => {
    builderState.addDay();
    builderState.setCurrentDay(1);

    const { container } = render(ExercisesStep);
    const dayTabs = container.querySelectorAll('.day-tab');

    expect(dayTabs[0].classList.contains('active')).toBe(false);
    expect(dayTabs[1].classList.contains('active')).toBe(true);
  });

  it('should show exercise count in day tabs', () => {
    builderState.addExercise(0);
    builderState.addExercise(0);

    const { getAllByText } = render(ExercisesStep);
    expect(getAllByText('2')).toBeTruthy();
  });

  it('should show error badge for days with 0 exercises', () => {
    const { container } = render(ExercisesStep);
    const exerciseCount = container.querySelector('.exercise-count.error');

    expect(exerciseCount).toBeTruthy();
  });

  it('should render current day info', () => {
    builderState.updateDay(0, { focus: 'Strength' });

    const { getByText } = render(ExercisesStep);
    expect(getByText(/Day 1 - Strength/)).toBeTruthy();
  });

  it('should show day notes if present', () => {
    builderState.updateDay(0, { notes: 'Focus on form' });

    const { getByText } = render(ExercisesStep);
    expect(getByText('Focus on form')).toBeTruthy();
  });

  it('should show empty state when no exercises', () => {
    const { getByText } = render(ExercisesStep);
    expect(getByText('No exercises added yet')).toBeTruthy();
    expect(getByText(/Click "Add Exercise"/)).toBeTruthy();
  });

  it('should add exercise when Add Exercise button is clicked', async () => {
    const { getByText } = render(ExercisesStep);
    const addButton = getByText('+ Add Exercise');

    await fireEvent.click(addButton);

    const state = get(builderState);
    expect(state.plan.cycle.days[0].exercises).toHaveLength(1);
  });

  it('should render ExerciseForms for each exercise', () => {
    builderState.addExercise(0);
    builderState.addExercise(0);

    const { container } = render(ExercisesStep);
    const exerciseForms = container.querySelectorAll('.exercise-form');

    expect(exerciseForms).toHaveLength(2);
  });

  it('should hide empty state when exercises exist', () => {
    builderState.addExercise(0);

    const { queryByText } = render(ExercisesStep);
    expect(queryByText('No exercises added yet')).toBeNull();
  });

  it('should show day focus in tab if present', () => {
    builderState.addDay();
    builderState.updateDay(1, { focus: 'Cardio' });

    const { getByText } = render(ExercisesStep);
    expect(getByText('Cardio')).toBeTruthy();
  });
});
