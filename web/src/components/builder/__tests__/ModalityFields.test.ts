/**
 * Tests for ModalityFields component
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import ModalityFields from '../forms/ModalityFields.svelte';
import type { Exercise } from '../../../lib/builderState';

describe('ModalityFields', () => {
  let onUpdate: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    onUpdate = vi.fn();
  });

  describe('strength modality', () => {
    it('should render sets and reps fields', () => {
      const exercise: Exercise = {
        name: 'Squat',
        modality: 'strength'
      };

      const { getByLabelText } = render(ModalityFields, {
        props: { exercise, onUpdate }
      });

      expect(getByLabelText(/Sets/)).toBeTruthy();
      expect(getByLabelText(/Reps/)).toBeTruthy();
    });

    it('should render optional load field', () => {
      const exercise: Exercise = {
        name: 'Squat',
        modality: 'strength'
      };

      const { getByLabelText } = render(ModalityFields, {
        props: { exercise, onUpdate }
      });

      expect(getByLabelText(/Load/)).toBeTruthy();
    });

    it('should update sets field', async () => {
      const exercise: Exercise = {
        name: 'Squat',
        modality: 'strength'
      };

      const { getByLabelText } = render(ModalityFields, {
        props: { exercise, onUpdate }
      });

      const setsInput = getByLabelText(/Sets/) as HTMLInputElement;
      await fireEvent.input(setsInput, { target: { value: '5' } });

      expect(onUpdate).toHaveBeenCalledWith('target_sets', 5);
    });

    it('should update reps field', async () => {
      const exercise: Exercise = {
        name: 'Squat',
        modality: 'strength'
      };

      const { getByLabelText } = render(ModalityFields, {
        props: { exercise, onUpdate }
      });

      const repsInput = getByLabelText(/Reps/) as HTMLInputElement;
      await fireEvent.input(repsInput, { target: { value: '10' } });

      expect(onUpdate).toHaveBeenCalledWith('target_reps', 10);
    });
  });

  describe('countdown modality', () => {
    it('should render duration field', () => {
      const exercise: Exercise = {
        name: 'Plank',
        modality: 'countdown'
      };

      const { getByLabelText } = render(ModalityFields, {
        props: { exercise, onUpdate }
      });

      expect(getByLabelText(/Duration \(seconds\)/)).toBeTruthy();
    });

    it('should update duration field', async () => {
      const exercise: Exercise = {
        name: 'Plank',
        modality: 'countdown'
      };

      const { getByLabelText } = render(ModalityFields, {
        props: { exercise, onUpdate }
      });

      const durationInput = getByLabelText(/Duration \(seconds\)/) as HTMLInputElement;
      await fireEvent.input(durationInput, { target: { value: '60' } });

      expect(onUpdate).toHaveBeenCalledWith('target_duration_sec', 60);
    });
  });

  describe('stopwatch modality', () => {
    it('should render optional duration field', () => {
      const exercise: Exercise = {
        name: 'Run',
        modality: 'stopwatch'
      };

      const { getByLabelText, container } = render(ModalityFields, {
        props: { exercise, onUpdate }
      });

      const input = getByLabelText(/Duration \(seconds\)/);
      expect(input).toBeTruthy();

      // Check that the label text contains "optional"
      const label = container.querySelector('label[for="target-duration"]');
      expect(label?.textContent).toContain('optional');
    });
  });

  describe('interval modality', () => {
    it('should render sets and duration fields', () => {
      const exercise: Exercise = {
        name: 'HIIT',
        modality: 'interval'
      };

      const { getByLabelText } = render(ModalityFields, {
        props: { exercise, onUpdate }
      });

      expect(getByLabelText(/Sets \(intervals\)/)).toBeTruthy();
      expect(getByLabelText(/Duration per interval/)).toBeTruthy();
    });
  });

  describe('cycling modality (endurance)', () => {
    it('should render endurance configuration section', () => {
      const exercise: Exercise = {
        name: 'Zone 2 Ride',
        modality: 'cycling'
      };

      const { container } = render(ModalityFields, {
        props: { exercise, onUpdate }
      });

      expect(container.textContent).toContain('Cycling Configuration');
      expect(container.textContent).toContain('Training Zones');
      expect(container.textContent).toContain('Power Ramp');
      expect(container.textContent).toContain('Interval Phases');
    });
  });

  describe('running modality (endurance)', () => {
    it('should render endurance configuration section', () => {
      const exercise: Exercise = {
        name: 'Tempo Run',
        modality: 'running'
      };

      const { container, getByLabelText } = render(ModalityFields, {
        props: { exercise, onUpdate }
      });

      expect(container.textContent).toContain('Running Configuration');
      expect(container.textContent).toContain('Training Zones');
      expect(getByLabelText(/Target Distance \(meters\)/)).toBeTruthy();
    });
  });

  describe('rowing modality (endurance)', () => {
    it('should render endurance configuration section', () => {
      const exercise: Exercise = {
        name: '2K Row',
        modality: 'rowing'
      };

      const { container, getByLabelText } = render(ModalityFields, {
        props: { exercise, onUpdate }
      });

      expect(container.textContent).toContain('Rowing Configuration');
      expect(container.textContent).toContain('Training Zones');
      expect(container.textContent).toContain('Power Ramp');
      expect(getByLabelText(/Target Distance \(meters\)/)).toBeTruthy();
    });
  });

  describe('swimming modality (endurance)', () => {
    it('should render endurance configuration section', () => {
      const exercise: Exercise = {
        name: '200m Repeats',
        modality: 'swimming'
      };

      const { container, getByLabelText } = render(ModalityFields, {
        props: { exercise, onUpdate }
      });

      expect(container.textContent).toContain('Swimming Configuration');
      expect(container.textContent).toContain('Training Zones');
      expect(getByLabelText(/Target Distance \(meters\)/)).toBeTruthy();
    });
  });

  describe('field value handling', () => {
    it('should handle empty input as undefined', async () => {
      const exercise: Exercise = {
        name: 'Squat',
        modality: 'strength',
        target_sets: 5
      };

      const { getByLabelText } = render(ModalityFields, {
        props: { exercise, onUpdate }
      });

      const setsInput = getByLabelText(/Sets/) as HTMLInputElement;
      await fireEvent.input(setsInput, { target: { value: '' } });

      expect(onUpdate).toHaveBeenCalledWith('target_sets', undefined);
    });

    it('should handle string values for load', async () => {
      const exercise: Exercise = {
        name: 'Squat',
        modality: 'strength'
      };

      const { getByLabelText } = render(ModalityFields, {
        props: { exercise, onUpdate }
      });

      const loadInput = getByLabelText(/Load/) as HTMLInputElement;
      await fireEvent.input(loadInput, { target: { value: '185 lbs' } });

      expect(onUpdate).toHaveBeenCalledWith('target_load', '185 lbs');
    });

    it('should handle invalid number input', async () => {
      const exercise: Exercise = {
        name: 'Squat',
        modality: 'strength'
      };

      const { getByLabelText } = render(ModalityFields, {
        props: { exercise, onUpdate }
      });

      const setsInput = getByLabelText(/Sets/) as HTMLInputElement;
      await fireEvent.input(setsInput, { target: { value: 'abc' } });

      expect(onUpdate).toHaveBeenCalledWith('target_sets', undefined);
    });
  });

  describe('displayed values', () => {
    it('should display existing values', () => {
      const exercise: Exercise = {
        name: 'Squat',
        modality: 'strength',
        target_sets: 5,
        target_reps: 10,
        target_load: '100 lbs'
      };

      const { container } = render(ModalityFields, {
        props: { exercise, onUpdate }
      });

      const inputs = container.querySelectorAll('input');
      const values = Array.from(inputs).map(i => i.value);

      expect(values).toContain('5');
      expect(values).toContain('10');
      expect(values).toContain('100 lbs');
    });
  });

  it('should render container with proper class', () => {
    const exercise: Exercise = {
      name: 'Squat',
      modality: 'strength'
    };

    const { container } = render(ModalityFields, {
      props: { exercise, onUpdate }
    });

    expect(container.querySelector('.modality-fields')).toBeTruthy();
  });
});
