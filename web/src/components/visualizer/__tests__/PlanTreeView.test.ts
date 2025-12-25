/**
 * Tests for PlanTreeView component
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import PlanTreeView from '../PlanTreeView.svelte';

describe('PlanTreeView', () => {
  const basicPlan = {
    plan_version: 1,
    meta: {
      name: 'Test Plan',
      description: 'A test workout plan'
    },
    cycle: {
      days: [
        {
          name: 'Upper Body',
          exercises: [
            {
              name: 'Bench Press',
              modality: 'strength',
              target_sets: 3,
              target_reps: 10,
              target_load_kg: 80
            }
          ]
        }
      ]
    }
  };

  beforeEach(() => {
    // Reset any state if needed
  });

  it('should render the component', () => {
    const { container } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });
    expect(container.querySelector('.plan-tree')).toBeTruthy();
  });

  it('should display plan name from meta', () => {
    const { getByText } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });
    expect(getByText('Test Plan')).toBeTruthy();
  });

  it('should display default plan name when meta.name is missing', () => {
    const planWithoutName = {
      ...basicPlan,
      meta: {}
    };
    const { getByText } = render(PlanTreeView, {
      props: { plan: planWithoutName }
    });
    expect(getByText('Workout Plan')).toBeTruthy();
  });

  it('should display plan description', () => {
    const { getByText } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });
    expect(getByText('A test workout plan')).toBeTruthy();
  });

  it('should not display description when missing', () => {
    const planWithoutDescription = {
      ...basicPlan,
      meta: { name: 'Test Plan' }
    };
    const { container } = render(PlanTreeView, {
      props: { plan: planWithoutDescription }
    });
    expect(container.querySelector('.plan-description')).toBeFalsy();
  });

  it('should display cycle information', () => {
    const { getByText } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });
    expect(getByText(/1 days/)).toBeTruthy();
  });

  it('should display target reps badge when present', () => {
    const planWithTargetReps = {
      ...basicPlan,
      cycle: {
        ...basicPlan.cycle,
        target_reps: 3
      }
    };
    const { getByText } = render(PlanTreeView, {
      props: { plan: planWithTargetReps }
    });
    expect(getByText('Target Reps')).toBeTruthy();
    expect(getByText('3')).toBeTruthy();
  });

  it('should not display target reps badge when not present', () => {
    const { container } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });
    const badges = container.querySelectorAll('.info-badge');
    const targetRepsBadge = Array.from(badges).find(
      badge => badge.textContent?.includes('Target Reps')
    );
    expect(targetRepsBadge).toBeFalsy();
  });

  it('should render day headers', () => {
    const { getByText } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });
    expect(getByText(/Day 1/)).toBeTruthy();
    expect(getByText(/Upper Body/)).toBeTruthy();
  });

  it('should display exercise count for each day', () => {
    const { getByText } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });
    expect(getByText('1 exercise')).toBeTruthy();
  });

  it('should display correct plural for multiple exercises', () => {
    const planWithMultipleExercises = {
      ...basicPlan,
      cycle: {
        days: [
          {
            name: 'Full Body',
            exercises: [
              {
                name: 'Squats',
                modality: 'strength',
                target_sets: 3,
                target_reps: 10
              },
              {
                name: 'Deadlifts',
                modality: 'strength',
                target_sets: 3,
                target_reps: 8
              }
            ]
          }
        ]
      }
    };
    const { getByText } = render(PlanTreeView, {
      props: { plan: planWithMultipleExercises }
    });
    expect(getByText('2 exercises')).toBeTruthy();
  });

  it('should expand day when day header is clicked', async () => {
    const { container, getByText } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    expect(container.querySelector('.exercises-list')).toBeTruthy();
    expect(getByText('Bench Press')).toBeTruthy();
  });

  it('should collapse day when clicked again', async () => {
    const { container, getByText } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;

    // Expand
    await fireEvent.click(dayHeader);
    expect(container.querySelector('.exercises-list')).toBeTruthy();

    // Collapse
    await fireEvent.click(dayHeader);
    expect(container.querySelector('.exercises-list')).toBeFalsy();
  });

  it('should toggle expand icon when day is expanded/collapsed', async () => {
    const { container, getByText } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    const expandIcon = dayHeader.querySelector('.expand-icon');

    // Initially collapsed
    expect(expandIcon?.textContent).toBe('▶');

    // Expand
    await fireEvent.click(dayHeader);
    expect(expandIcon?.textContent).toBe('▼');

    // Collapse
    await fireEvent.click(dayHeader);
    expect(expandIcon?.textContent).toBe('▶');
  });

  it('should display exercise name', async () => {
    const { getByText } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    expect(getByText('Bench Press')).toBeTruthy();
  });

  it('should display modality badge', async () => {
    const { container, getByText } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const modalityBadge = container.querySelector('.modality-badge');
    expect(modalityBadge).toBeTruthy();
    expect(modalityBadge?.textContent).toBe('strength');
  });

  it('should display modality icon for strength exercises', async () => {
    const { container, getByText } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const modalityIcon = container.querySelector('.modality-icon');
    expect(modalityIcon?.textContent).toBe('💪');
  });

  it('should display modality icon for countdown exercises', async () => {
    const countdownPlan = {
      ...basicPlan,
      cycle: {
        days: [
          {
            exercises: [
              {
                name: 'Plank',
                modality: 'countdown',
                target_duration_sec: 60
              }
            ]
          }
        ]
      }
    };

    const { container, getByText } = render(PlanTreeView, {
      props: { plan: countdownPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const modalityIcon = container.querySelector('.modality-icon');
    expect(modalityIcon?.textContent).toBe('⏱️');
  });

  it('should display modality icon for stopwatch exercises', async () => {
    const stopwatchPlan = {
      ...basicPlan,
      cycle: {
        days: [
          {
            exercises: [
              {
                name: 'Run',
                modality: 'stopwatch',
                target_duration_sec: 1800
              }
            ]
          }
        ]
      }
    };

    const { container, getByText } = render(PlanTreeView, {
      props: { plan: stopwatchPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const modalityIcon = container.querySelector('.modality-icon');
    expect(modalityIcon?.textContent).toBe('⏲️');
  });

  it('should display modality icon for interval exercises', async () => {
    const intervalPlan = {
      ...basicPlan,
      cycle: {
        days: [
          {
            exercises: [
              {
                name: 'HIIT',
                modality: 'interval',
                target_sets: 8,
                target_duration_sec: 30
              }
            ]
          }
        ]
      }
    };

    const { container, getByText } = render(PlanTreeView, {
      props: { plan: intervalPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const modalityIcon = container.querySelector('.modality-icon');
    expect(modalityIcon?.textContent).toBe('🔄');
  });

  it('should display default icon for unknown modality', async () => {
    const unknownPlan = {
      ...basicPlan,
      cycle: {
        days: [
          {
            exercises: [
              {
                name: 'Unknown Exercise',
                modality: 'unknown'
              }
            ]
          }
        ]
      }
    };

    const { container, getByText } = render(PlanTreeView, {
      props: { plan: unknownPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const modalityIcon = container.querySelector('.modality-icon');
    expect(modalityIcon?.textContent).toBe('📝');
  });

  it('should expand exercise to show details', async () => {
    const { container, getByText } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });

    // Expand day
    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    // Expand exercise
    const exerciseHeader = getByText('Bench Press').closest('button') as HTMLElement;
    await fireEvent.click(exerciseHeader);

    const exerciseDetails = container.querySelector('.exercise-details');
    expect(exerciseDetails).toBeTruthy();
  });

  it('should display strength exercise details', async () => {
    const { container, getByText } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const exerciseHeader = getByText('Bench Press').closest('button') as HTMLElement;
    await fireEvent.click(exerciseHeader);

    const details = container.querySelector('.exercise-details');
    expect(details?.textContent).toContain('Sets:');
    expect(details?.textContent).toContain('3');
    expect(details?.textContent).toContain('Reps:');
    expect(details?.textContent).toContain('10');
    expect(details?.textContent).toContain('Load:');
    expect(details?.textContent).toContain('80 kg');
  });

  it('should display countdown exercise details', async () => {
    const countdownPlan = {
      ...basicPlan,
      cycle: {
        days: [
          {
            exercises: [
              {
                name: 'Plank',
                modality: 'countdown',
                target_duration_sec: 90
              }
            ]
          }
        ]
      }
    };

    const { container, getByText } = render(PlanTreeView, {
      props: { plan: countdownPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const exerciseHeader = getByText('Plank').closest('button') as HTMLElement;
    await fireEvent.click(exerciseHeader);

    const details = container.querySelector('.exercise-details');
    expect(details?.textContent).toContain('Duration:');
    expect(details?.textContent).toContain('1:30');
  });

  it('should display stopwatch exercise details', async () => {
    const stopwatchPlan = {
      ...basicPlan,
      cycle: {
        days: [
          {
            exercises: [
              {
                name: 'Run',
                modality: 'stopwatch',
                target_duration_sec: 1800
              }
            ]
          }
        ]
      }
    };

    const { container, getByText } = render(PlanTreeView, {
      props: { plan: stopwatchPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const exerciseHeader = getByText('Run').closest('button') as HTMLElement;
    await fireEvent.click(exerciseHeader);

    const details = container.querySelector('.exercise-details');
    expect(details?.textContent).toContain('Duration:');
    expect(details?.textContent).toContain('30:00');
  });

  it('should display interval exercise details', async () => {
    const intervalPlan = {
      ...basicPlan,
      cycle: {
        days: [
          {
            exercises: [
              {
                name: 'HIIT',
                modality: 'interval',
                target_sets: 8,
                target_duration_sec: 45
              }
            ]
          }
        ]
      }
    };

    const { container, getByText } = render(PlanTreeView, {
      props: { plan: intervalPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const exerciseHeader = getByText('HIIT').closest('button') as HTMLElement;
    await fireEvent.click(exerciseHeader);

    const details = container.querySelector('.exercise-details');
    expect(details?.textContent).toContain('Sets:');
    expect(details?.textContent).toContain('8');
    expect(details?.textContent).toContain('Duration:');
    expect(details?.textContent).toContain('0:45');
  });

  it('should format duration correctly', async () => {
    const durationTestPlan = {
      ...basicPlan,
      cycle: {
        days: [
          {
            exercises: [
              {
                name: 'Long Run',
                modality: 'stopwatch',
                target_duration_sec: 3665 // 61:05
              }
            ]
          }
        ]
      }
    };

    const { container, getByText } = render(PlanTreeView, {
      props: { plan: durationTestPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const exerciseHeader = getByText('Long Run').closest('button') as HTMLElement;
    await fireEvent.click(exerciseHeader);

    const details = container.querySelector('.exercise-details');
    expect(details?.textContent).toContain('61:05');
  });

  it('should pad seconds with leading zero', async () => {
    const durationTestPlan = {
      ...basicPlan,
      cycle: {
        days: [
          {
            exercises: [
              {
                name: 'Short Exercise',
                modality: 'countdown',
                target_duration_sec: 65 // 1:05
              }
            ]
          }
        ]
      }
    };

    const { container, getByText } = render(PlanTreeView, {
      props: { plan: durationTestPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const exerciseHeader = getByText('Short Exercise').closest('button') as HTMLElement;
    await fireEvent.click(exerciseHeader);

    const details = container.querySelector('.exercise-details');
    expect(details?.textContent).toContain('1:05');
  });

  it('should display equipment when present', async () => {
    const equipmentPlan = {
      ...basicPlan,
      cycle: {
        days: [
          {
            exercises: [
              {
                name: 'Bench Press',
                modality: 'strength',
                target_sets: 3,
                target_reps: 10,
                equipment: ['barbell', 'bench']
              }
            ]
          }
        ]
      }
    };

    const { container, getByText } = render(PlanTreeView, {
      props: { plan: equipmentPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const exerciseHeader = getByText('Bench Press').closest('button') as HTMLElement;
    await fireEvent.click(exerciseHeader);

    const details = container.querySelector('.exercise-details');
    expect(details?.textContent).toContain('Equipment:');
    expect(details?.textContent).toContain('barbell, bench');
  });

  it('should not display equipment when not present', async () => {
    const { container, getByText } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const exerciseHeader = getByText('Bench Press').closest('button') as HTMLElement;
    await fireEvent.click(exerciseHeader);

    const details = container.querySelector('.exercise-details');
    expect(details?.textContent).not.toContain('Equipment:');
  });

  it('should not display equipment when array is empty', async () => {
    const emptyEquipmentPlan = {
      ...basicPlan,
      cycle: {
        days: [
          {
            exercises: [
              {
                name: 'Bench Press',
                modality: 'strength',
                target_sets: 3,
                target_reps: 10,
                equipment: []
              }
            ]
          }
        ]
      }
    };

    const { container, getByText } = render(PlanTreeView, {
      props: { plan: emptyEquipmentPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const exerciseHeader = getByText('Bench Press').closest('button') as HTMLElement;
    await fireEvent.click(exerciseHeader);

    const details = container.querySelector('.exercise-details');
    expect(details?.textContent).not.toContain('Equipment:');
  });

  it('should display notes when present', async () => {
    const notesPlan = {
      ...basicPlan,
      cycle: {
        days: [
          {
            exercises: [
              {
                name: 'Bench Press',
                modality: 'strength',
                target_sets: 3,
                target_reps: 10,
                notes: 'Focus on form and control'
              }
            ]
          }
        ]
      }
    };

    const { container, getByText } = render(PlanTreeView, {
      props: { plan: notesPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const exerciseHeader = getByText('Bench Press').closest('button') as HTMLElement;
    await fireEvent.click(exerciseHeader);

    const details = container.querySelector('.exercise-details');
    expect(details?.textContent).toContain('Notes:');
    expect(details?.textContent).toContain('Focus on form and control');
  });

  it('should not display notes when not present', async () => {
    const { container, getByText } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const exerciseHeader = getByText('Bench Press').closest('button') as HTMLElement;
    await fireEvent.click(exerciseHeader);

    const details = container.querySelector('.exercise-details');
    expect(details?.textContent).not.toContain('Notes:');
  });

  it('should handle multiple days', () => {
    const multiDayPlan = {
      ...basicPlan,
      cycle: {
        days: [
          {
            name: 'Day 1',
            exercises: [{ name: 'Exercise 1', modality: 'strength', target_sets: 3, target_reps: 10 }]
          },
          {
            name: 'Day 2',
            exercises: [{ name: 'Exercise 2', modality: 'strength', target_sets: 3, target_reps: 10 }]
          },
          {
            name: 'Day 3',
            exercises: [{ name: 'Exercise 3', modality: 'strength', target_sets: 3, target_reps: 10 }]
          }
        ]
      }
    };

    const { getByText } = render(PlanTreeView, {
      props: { plan: multiDayPlan }
    });

    expect(getByText(/3 days/)).toBeTruthy();
    expect(getByText(/Day 1/)).toBeTruthy();
    expect(getByText(/Day 2/)).toBeTruthy();
    expect(getByText(/Day 3/)).toBeTruthy();
  });

  it('should handle day without name', async () => {
    const unnamedDayPlan = {
      ...basicPlan,
      cycle: {
        days: [
          {
            exercises: [
              {
                name: 'Squats',
                modality: 'strength',
                target_sets: 3,
                target_reps: 10
              }
            ]
          }
        ]
      }
    };

    const { getByText } = render(PlanTreeView, {
      props: { plan: unnamedDayPlan }
    });

    // Should only show "Day 1" without additional name
    expect(getByText(/^Day 1$/)).toBeTruthy();
  });

  it('should display empty state when no cycle', () => {
    const noCyclePlan = {
      ...basicPlan,
      cycle: null
    };

    const { getByText } = render(PlanTreeView, {
      props: { plan: noCyclePlan }
    });

    expect(getByText('No cycle data available')).toBeTruthy();
  });

  it('should handle plan without meta', () => {
    const noMetaPlan = {
      plan_version: 1,
      cycle: {
        days: [
          {
            exercises: [
              {
                name: 'Push-ups',
                modality: 'strength',
                target_sets: 3,
                target_reps: 15
              }
            ]
          }
        ]
      }
    };

    const { getByText } = render(PlanTreeView, {
      props: { plan: noMetaPlan }
    });

    expect(getByText('Workout Plan')).toBeTruthy();
  });

  it('should collapse exercise when clicked again', async () => {
    const { container, getByText } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const exerciseHeader = getByText('Bench Press').closest('button') as HTMLElement;

    // Expand
    await fireEvent.click(exerciseHeader);
    expect(container.querySelector('.exercise-details')).toBeTruthy();

    // Collapse
    await fireEvent.click(exerciseHeader);
    expect(container.querySelector('.exercise-details')).toBeFalsy();
  });

  it('should toggle exercise expand icon', async () => {
    const { container, getByText } = render(PlanTreeView, {
      props: { plan: basicPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const exerciseHeader = getByText('Bench Press').closest('button') as HTMLElement;
    const expandIcon = exerciseHeader.querySelector('.expand-icon');

    // Initially collapsed
    expect(expandIcon?.textContent).toBe('▶');

    // Expand
    await fireEvent.click(exerciseHeader);
    expect(expandIcon?.textContent).toBe('▼');

    // Collapse
    await fireEvent.click(exerciseHeader);
    expect(expandIcon?.textContent).toBe('▶');
  });

  it('should handle missing target_load_kg', async () => {
    const noLoadPlan = {
      ...basicPlan,
      cycle: {
        days: [
          {
            exercises: [
              {
                name: 'Push-ups',
                modality: 'strength',
                target_sets: 3,
                target_reps: 15
              }
            ]
          }
        ]
      }
    };

    const { container, getByText } = render(PlanTreeView, {
      props: { plan: noLoadPlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const exerciseHeader = getByText('Push-ups').closest('button') as HTMLElement;
    await fireEvent.click(exerciseHeader);

    const details = container.querySelector('.exercise-details');
    expect(details?.textContent).not.toContain('Load:');
  });

  it('should display "-" for missing sets or reps', async () => {
    const incompletePlan = {
      ...basicPlan,
      cycle: {
        days: [
          {
            exercises: [
              {
                name: 'Exercise',
                modality: 'strength'
              }
            ]
          }
        ]
      }
    };

    const { container, getByText } = render(PlanTreeView, {
      props: { plan: incompletePlan }
    });

    const dayHeader = getByText(/Day 1/).closest('button') as HTMLElement;
    await fireEvent.click(dayHeader);

    const exerciseHeader = getByText('Exercise').closest('button') as HTMLElement;
    await fireEvent.click(exerciseHeader);

    const details = container.querySelector('.exercise-details');
    expect(details?.textContent).toContain('-');
  });
});
