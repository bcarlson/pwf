/**
 * Tests for ProgressIndicator component
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { render } from '@testing-library/svelte';
import { tick } from 'svelte';
import ProgressIndicator from '../shared/ProgressIndicator.svelte';

describe('ProgressIndicator', () => {
  const mockSteps = [
    { label: 'Plan Info', valid: false },
    { label: 'Days', valid: false },
    { label: 'Exercises', valid: false },
    { label: 'Review', valid: false },
  ];

  describe('Initial Render', () => {
    it('should render the component without crashing', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 0,
          steps: mockSteps,
        },
      });
      expect(container).toBeTruthy();
      expect(container.firstChild).toBeTruthy();
    });

    it('should render all steps', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 0,
          steps: mockSteps,
        },
      });

      const stepElements = container.querySelectorAll('.step');
      expect(stepElements.length).toBe(4);
    });

    it('should render step labels correctly', () => {
      const { getByText } = render(ProgressIndicator, {
        props: {
          currentStep: 0,
          steps: mockSteps,
        },
      });

      expect(getByText('Plan Info')).toBeTruthy();
      expect(getByText('Days')).toBeTruthy();
      expect(getByText('Exercises')).toBeTruthy();
      expect(getByText('Review')).toBeTruthy();
    });

    it('should render progress bar', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 0,
          steps: mockSteps,
        },
      });

      const progressBar = container.querySelector('.progress-bar');
      expect(progressBar).toBeTruthy();
    });

    it('should render progress fill', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 0,
          steps: mockSteps,
        },
      });

      const progressFill = container.querySelector('.progress-fill');
      expect(progressFill).toBeTruthy();
    });
  });

  describe('Step Status Computation', () => {
    it('should mark current step with active class', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 1,
          steps: mockSteps,
        },
      });

      const steps = container.querySelectorAll('.step');
      expect(steps[1].classList.contains('active')).toBe(true);
    });

    it('should mark completed valid steps with completed class', () => {
      const steps = [
        { label: 'Plan Info', valid: true },
        { label: 'Days', valid: true },
        { label: 'Exercises', valid: false },
        { label: 'Review', valid: false },
      ];

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 2,
          steps,
        },
      });

      const stepElements = container.querySelectorAll('.step');
      expect(stepElements[0].classList.contains('completed')).toBe(true);
      expect(stepElements[1].classList.contains('completed')).toBe(true);
    });

    it('should mark completed invalid steps with error class', () => {
      const steps = [
        { label: 'Plan Info', valid: false },
        { label: 'Days', valid: true },
        { label: 'Exercises', valid: false },
        { label: 'Review', valid: false },
      ];

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 2,
          steps,
        },
      });

      const stepElements = container.querySelectorAll('.step');
      expect(stepElements[0].classList.contains('error')).toBe(true);
    });

    it('should not mark future steps as completed or error', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 1,
          steps: mockSteps,
        },
      });

      const stepElements = container.querySelectorAll('.step');
      expect(stepElements[2].classList.contains('completed')).toBe(false);
      expect(stepElements[2].classList.contains('error')).toBe(false);
      expect(stepElements[2].classList.contains('active')).toBe(false);
    });

    it('should not mark current step as completed', () => {
      const steps = [
        { label: 'Plan Info', valid: true },
        { label: 'Days', valid: true },
        { label: 'Exercises', valid: false },
        { label: 'Review', valid: false },
      ];

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 1,
          steps,
        },
      });

      const stepElements = container.querySelectorAll('.step');
      expect(stepElements[1].classList.contains('completed')).toBe(false);
      expect(stepElements[1].classList.contains('active')).toBe(true);
    });
  });

  describe('Step Icons', () => {
    it('should show checkmark icon for completed valid steps', () => {
      const steps = [
        { label: 'Plan Info', valid: true },
        { label: 'Days', valid: false },
        { label: 'Exercises', valid: false },
        { label: 'Review', valid: false },
      ];

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 1,
          steps,
        },
      });

      const stepNumbers = container.querySelectorAll('.step-number');
      expect(stepNumbers[0].textContent).toBe('✓');
    });

    it('should show exclamation icon for completed invalid steps', () => {
      const steps = [
        { label: 'Plan Info', valid: false },
        { label: 'Days', valid: false },
        { label: 'Exercises', valid: false },
        { label: 'Review', valid: false },
      ];

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 1,
          steps,
        },
      });

      const stepNumbers = container.querySelectorAll('.step-number');
      expect(stepNumbers[0].textContent).toBe('!');
    });

    it('should show step number for current step', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 2,
          steps: mockSteps,
        },
      });

      const stepNumbers = container.querySelectorAll('.step-number');
      expect(stepNumbers[2].textContent).toBe('3'); // currentStep 2 = display number 3
    });

    it('should show step number for pending steps', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 1,
          steps: mockSteps,
        },
      });

      const stepNumbers = container.querySelectorAll('.step-number');
      expect(stepNumbers[2].textContent).toBe('3');
      expect(stepNumbers[3].textContent).toBe('4');
    });

    it('should display correct icons based on step validity at render time', () => {
      const stepsWithInvalid = [
        { label: 'Plan Info', valid: false },
        { label: 'Days', valid: false },
        { label: 'Exercises', valid: false },
        { label: 'Review', valid: false },
      ];

      const { container: containerInvalid } = render(ProgressIndicator, {
        props: {
          currentStep: 1,
          steps: stepsWithInvalid,
        },
      });

      const invalidStepNumbers = containerInvalid.querySelectorAll('.step-number');
      expect(invalidStepNumbers[0].textContent).toBe('!');

      // Render with valid step
      const stepsWithValid = [
        { label: 'Plan Info', valid: true },
        { label: 'Days', valid: false },
        { label: 'Exercises', valid: false },
        { label: 'Review', valid: false },
      ];

      const { container: containerValid } = render(ProgressIndicator, {
        props: {
          currentStep: 1,
          steps: stepsWithValid,
        },
      });

      const validStepNumbers = containerValid.querySelectorAll('.step-number');
      expect(validStepNumbers[0].textContent).toBe('✓');
    });
  });

  describe('Step Connectors', () => {
    it('should render connectors between steps', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 0,
          steps: mockSteps,
        },
      });

      const connectors = container.querySelectorAll('.step-connector');
      expect(connectors.length).toBe(3); // 4 steps = 3 connectors
    });

    it('should mark connectors as completed for past steps', () => {
      const steps = [
        { label: 'Plan Info', valid: true },
        { label: 'Days', valid: true },
        { label: 'Exercises', valid: false },
        { label: 'Review', valid: false },
      ];

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 2,
          steps,
        },
      });

      const connectors = container.querySelectorAll('.step-connector');
      expect(connectors[0].classList.contains('completed')).toBe(true);
      expect(connectors[1].classList.contains('completed')).toBe(true);
      expect(connectors[2].classList.contains('completed')).toBe(false);
    });

    it('should render connectors for all but the last step', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 0,
          steps: mockSteps,
        },
      });

      const stepContainers = container.querySelectorAll('.step-container');
      // There are 4 steps, so 3 connectors should exist (but last one may not render or be hidden)
      expect(stepContainers.length).toBe(4);

      // Verify connectors exist in first 3 step containers
      for (let i = 0; i < stepContainers.length - 1; i++) {
        const connector = stepContainers[i].querySelector('.step-connector');
        expect(connector).toBeTruthy();
      }
    });
  });

  describe('Progress Bar Width', () => {
    it('should be 0% on step 0', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 0,
          steps: mockSteps,
        },
      });

      const progressFill = container.querySelector('.progress-fill') as HTMLElement;
      expect(progressFill.style.width).toBe('0%');
    });

    it('should be 33.33% on step 1 (with 4 steps)', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 1,
          steps: mockSteps,
        },
      });

      const progressFill = container.querySelector('.progress-fill') as HTMLElement;
      const expectedWidth = (1 / 3) * 100;
      expect(progressFill.style.width).toBe(`${expectedWidth}%`);
    });

    it('should be 66.66% on step 2 (with 4 steps)', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 2,
          steps: mockSteps,
        },
      });

      const progressFill = container.querySelector('.progress-fill') as HTMLElement;
      const expectedWidth = (2 / 3) * 100;
      expect(progressFill.style.width).toBe(`${expectedWidth}%`);
    });

    it('should be 100% on step 3 (last step with 4 steps)', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 3,
          steps: mockSteps,
        },
      });

      const progressFill = container.querySelector('.progress-fill') as HTMLElement;
      expect(progressFill.style.width).toBe('100%');
    });

    it('should calculate progress correctly with different number of steps', () => {
      const twoSteps = [
        { label: 'Step 1', valid: false },
        { label: 'Step 2', valid: false },
      ];

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 1,
          steps: twoSteps,
        },
      });

      const progressFill = container.querySelector('.progress-fill') as HTMLElement;
      expect(progressFill.style.width).toBe('100%');
    });
  });

  describe('Props Validation', () => {
    it('should handle empty steps array gracefully', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 0,
          steps: [],
        },
      });

      expect(container.querySelector('.progress-indicator')).toBeTruthy();
    });

    it('should handle single step', () => {
      const singleStep = [{ label: 'Only Step', valid: true }];

      const { container, getByText } = render(ProgressIndicator, {
        props: {
          currentStep: 0,
          steps: singleStep,
        },
      });

      expect(getByText('Only Step')).toBeTruthy();
      const progressFill = container.querySelector('.progress-fill') as HTMLElement;
      // With one step, division by (steps.length - 1) = 0, would cause NaN
      // The component should handle this edge case
      expect(progressFill).toBeTruthy();
    });

    it('should handle currentStep out of bounds (negative)', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: -1,
          steps: mockSteps,
        },
      });

      // Should not crash
      expect(container.querySelector('.progress-indicator')).toBeTruthy();
    });

    it('should handle currentStep out of bounds (too large)', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 10,
          steps: mockSteps,
        },
      });

      // Should not crash
      expect(container.querySelector('.progress-indicator')).toBeTruthy();
    });

    it('should update when currentStep prop changes', async () => {
      const { container, component } = render(ProgressIndicator, {
        props: {
          currentStep: 0,
          steps: mockSteps,
        },
      });

      let activeSteps = container.querySelectorAll('.step.active');
      expect(activeSteps.length).toBe(1);

      component.$set({ currentStep: 2 });

      // Wait for reactivity
      await new Promise(resolve => setTimeout(resolve, 50));

      activeSteps = container.querySelectorAll('.step.active');
      expect(activeSteps.length).toBe(1);

      // Check that the third step (index 2) is now active
      const allSteps = container.querySelectorAll('.step');
      expect(allSteps[2].classList.contains('active')).toBe(true);
    });

    it('should render correctly with different step validities', () => {
      const validSteps = [
        { label: 'Plan Info', valid: true },
        { label: 'Days', valid: true },
        { label: 'Exercises', valid: false },
        { label: 'Review', valid: false },
      ];

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 1,
          steps: validSteps,
        },
      });

      const stepNumbers = container.querySelectorAll('.step-number');
      expect(stepNumbers[0].textContent).toBe('✓');
      expect(stepNumbers[1].textContent).toBe('2'); // Current step
      expect(stepNumbers[2].textContent).toBe('3'); // Future step
      expect(stepNumbers[3].textContent).toBe('4'); // Future step
    });
  });

  describe('getStepStatus Function Coverage', () => {
    it('should return "completed" for past valid step', () => {
      const steps = [
        { label: 'Plan Info', valid: true },
        { label: 'Days', valid: false },
        { label: 'Exercises', valid: false },
        { label: 'Review', valid: false },
      ];

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 2,
          steps,
        },
      });

      const stepElements = container.querySelectorAll('.step');
      expect(stepElements[0].classList.contains('completed')).toBe(true);
    });

    it('should return "error" for past invalid step', () => {
      const steps = [
        { label: 'Plan Info', valid: false },
        { label: 'Days', valid: false },
        { label: 'Exercises', valid: false },
        { label: 'Review', valid: false },
      ];

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 2,
          steps,
        },
      });

      const stepElements = container.querySelectorAll('.step');
      expect(stepElements[0].classList.contains('error')).toBe(true);
      expect(stepElements[1].classList.contains('error')).toBe(true);
    });

    it('should return "current" for current step', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 1,
          steps: mockSteps,
        },
      });

      const stepElements = container.querySelectorAll('.step');
      expect(stepElements[1].classList.contains('active')).toBe(true);
    });

    it('should return "pending" for future steps', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 1,
          steps: mockSteps,
        },
      });

      const stepElements = container.querySelectorAll('.step');
      expect(stepElements[2].classList.contains('active')).toBe(false);
      expect(stepElements[2].classList.contains('completed')).toBe(false);
      expect(stepElements[2].classList.contains('error')).toBe(false);
    });
  });

  describe('getStepIcon Function Coverage', () => {
    it('should cover completed status returning checkmark', () => {
      const steps = [
        { label: 'Plan Info', valid: true },
        { label: 'Days', valid: false },
        { label: 'Exercises', valid: false },
        { label: 'Review', valid: false },
      ];

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 1,
          steps,
        },
      });

      const stepNumbers = container.querySelectorAll('.step-number');
      expect(stepNumbers[0].textContent?.trim()).toBe('✓');
    });

    it('should cover error status returning exclamation', () => {
      const steps = [
        { label: 'Plan Info', valid: false },
        { label: 'Days', valid: false },
        { label: 'Exercises', valid: false },
        { label: 'Review', valid: false },
      ];

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 1,
          steps,
        },
      });

      const stepNumbers = container.querySelectorAll('.step-number');
      expect(stepNumbers[0].textContent?.trim()).toBe('!');
    });

    it('should cover current status returning step number', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 1,
          steps: mockSteps,
        },
      });

      const stepNumbers = container.querySelectorAll('.step-number');
      expect(stepNumbers[1].textContent?.trim()).toBe('2');
    });

    it('should cover default case (pending) returning step number', () => {
      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 0,
          steps: mockSteps,
        },
      });

      const stepNumbers = container.querySelectorAll('.step-number');
      expect(stepNumbers[1].textContent?.trim()).toBe('2');
      expect(stepNumbers[2].textContent?.trim()).toBe('3');
      expect(stepNumbers[3].textContent?.trim()).toBe('4');
    });
  });

  describe('Edge Cases and Error States', () => {
    it('should handle steps with undefined valid property', () => {
      const stepsWithUndefined = [
        { label: 'Plan Info', valid: undefined as any },
        { label: 'Days', valid: false },
        { label: 'Exercises', valid: false },
        { label: 'Review', valid: false },
      ];

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 1,
          steps: stepsWithUndefined,
        },
      });

      // Should not crash
      expect(container.querySelector('.progress-indicator')).toBeTruthy();
    });

    it('should handle steps with missing label', () => {
      const stepsWithMissingLabel = [
        { label: '', valid: false },
        { label: 'Days', valid: false },
        { label: 'Exercises', valid: false },
        { label: 'Review', valid: false },
      ];

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 0,
          steps: stepsWithMissingLabel,
        },
      });

      expect(container.querySelector('.progress-indicator')).toBeTruthy();
    });

    it('should handle very long step labels', () => {
      const stepsWithLongLabels = [
        { label: 'This is a very long step label that might cause layout issues', valid: false },
        { label: 'Days', valid: false },
        { label: 'Exercises', valid: false },
        { label: 'Review', valid: false },
      ];

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 0,
          steps: stepsWithLongLabels,
        },
      });

      expect(container.querySelector('.progress-indicator')).toBeTruthy();
    });

    it('should handle many steps', () => {
      const manySteps = Array.from({ length: 10 }, (_, i) => ({
        label: `Step ${i + 1}`,
        valid: i % 2 === 0,
      }));

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 5,
          steps: manySteps,
        },
      });

      const stepElements = container.querySelectorAll('.step');
      expect(stepElements.length).toBe(10);
    });
  });

  describe('Visual State Combinations', () => {
    it('should handle all steps valid up to current', () => {
      const steps = [
        { label: 'Plan Info', valid: true },
        { label: 'Days', valid: true },
        { label: 'Exercises', valid: false },
        { label: 'Review', valid: false },
      ];

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 2,
          steps,
        },
      });

      const stepElements = container.querySelectorAll('.step');
      expect(stepElements[0].classList.contains('completed')).toBe(true);
      expect(stepElements[1].classList.contains('completed')).toBe(true);
      expect(stepElements[2].classList.contains('active')).toBe(true);
    });

    it('should handle mixed valid/invalid completed steps', () => {
      const steps = [
        { label: 'Plan Info', valid: true },
        { label: 'Days', valid: false },
        { label: 'Exercises', valid: true },
        { label: 'Review', valid: false },
      ];

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 3,
          steps,
        },
      });

      const stepElements = container.querySelectorAll('.step');
      expect(stepElements[0].classList.contains('completed')).toBe(true);
      expect(stepElements[1].classList.contains('error')).toBe(true);
      expect(stepElements[2].classList.contains('completed')).toBe(true);
      expect(stepElements[3].classList.contains('active')).toBe(true);
    });

    it('should handle all steps invalid', () => {
      const steps = [
        { label: 'Plan Info', valid: false },
        { label: 'Days', valid: false },
        { label: 'Exercises', valid: false },
        { label: 'Review', valid: false },
      ];

      const { container } = render(ProgressIndicator, {
        props: {
          currentStep: 3,
          steps,
        },
      });

      const stepElements = container.querySelectorAll('.step');
      expect(stepElements[0].classList.contains('error')).toBe(true);
      expect(stepElements[1].classList.contains('error')).toBe(true);
      expect(stepElements[2].classList.contains('error')).toBe(true);
      expect(stepElements[3].classList.contains('active')).toBe(true);
    });
  });
});
