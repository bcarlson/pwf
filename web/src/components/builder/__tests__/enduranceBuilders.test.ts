/**
 * Tests for endurance modality builders (zones, intervals, ramps)
 */

import { describe, it, expect, vi } from 'vitest';
import { render, fireEvent, screen } from '@testing-library/svelte';
import ZoneBuilder from '../forms/ZoneBuilder.svelte';
import IntervalBuilder from '../forms/IntervalBuilder.svelte';
import RampBuilder from '../forms/RampBuilder.svelte';
import type { TrainingZone, IntervalPhase, RampConfig } from '../../../lib/builderState';

describe('ZoneBuilder', () => {
  describe('empty state', () => {
    it('should render empty state when no zones', () => {
      render(ZoneBuilder, {
        props: {
          zones: [],
          modality: 'cycling',
          onChange: vi.fn(),
        },
      });

      expect(screen.getByText(/No training zones defined/i)).toBeTruthy();
      expect(screen.getByRole('button', { name: /Add Zone/i })).toBeTruthy();
    });
  });

  describe('adding zones', () => {
    it('should add a new zone for cycling with power and cadence', async () => {
      const onChange = vi.fn();

      render(ZoneBuilder, {
        props: {
          zones: [],
          modality: 'cycling',
          onChange,
        },
      });

      const addButton = screen.getByRole('button', { name: /Add Zone/i });
      await fireEvent.click(addButton);

      expect(onChange).toHaveBeenCalledWith(
        expect.arrayContaining([
          expect.objectContaining({
            zone: 2,
            duration_sec: 300,
            target_power_watts: 200,
            cadence_rpm: 90,
          }),
        ])
      );
    });

    it('should add a new zone for running with pace', async () => {
      const onChange = vi.fn();

      render(ZoneBuilder, {
        props: {
          zones: [],
          modality: 'running',
          onChange,
        },
      });

      const addButton = screen.getByRole('button', { name: /Add Zone/i });
      await fireEvent.click(addButton);

      expect(onChange).toHaveBeenCalledWith(
        expect.arrayContaining([
          expect.objectContaining({
            zone: 2,
            duration_sec: 300,
            target_pace_sec_per_km: 300,
          }),
        ])
      );
    });

    it('should add multiple zones', async () => {
      const onChange = vi.fn();

      render(ZoneBuilder, {
        props: {
          zones: [],
          modality: 'cycling',
          onChange,
        },
      });

      const addButton = screen.getByRole('button', { name: /Add Zone/i });
      await fireEvent.click(addButton);
      await fireEvent.click(addButton);

      expect(onChange).toHaveBeenLastCalledWith(
        expect.arrayContaining([
          expect.objectContaining({ zone: 2 }),
          expect.objectContaining({ zone: 2 }),
        ])
      );
    });
  });

  describe('displaying zones', () => {
    it('should display existing zones', () => {
      const zones: TrainingZone[] = [
        {
          zone: 2,
          duration_sec: 600,
          target_power_watts: 180,
          target_hr_bpm: 140,
        },
        {
          zone: 4,
          duration_sec: 300,
          target_power_watts: 250,
        },
      ];

      render(ZoneBuilder, {
        props: {
          zones,
          modality: 'cycling',
          onChange: vi.fn(),
        },
      });

      expect(screen.getByText(/Zone 2/i)).toBeTruthy();
      expect(screen.getByText(/Zone 4/i)).toBeTruthy();
    });

    it('should display zone values in inputs', () => {
      const zones: TrainingZone[] = [
        {
          zone: 3,
          duration_sec: 1200,
          target_power_watts: 220,
          target_hr_bpm: 155,
        },
      ];

      const { container } = render(ZoneBuilder, {
        props: {
          zones,
          modality: 'cycling',
          onChange: vi.fn(),
        },
      });

      const inputs = container.querySelectorAll('input');
      const values = Array.from(inputs).map((i) => i.value);

      expect(values).toContain('3');
      expect(values).toContain('1200');
      expect(values).toContain('220');
      expect(values).toContain('155');
    });
  });

  describe('removing zones', () => {
    it('should remove a zone when remove button is clicked', async () => {
      const zones: TrainingZone[] = [
        { zone: 2, duration_sec: 600 },
        { zone: 3, duration_sec: 300 },
      ];
      const onChange = vi.fn();

      render(ZoneBuilder, {
        props: {
          zones,
          modality: 'running',
          onChange,
        },
      });

      const removeButtons = screen.getAllByLabelText(/Remove zone/i);
      await fireEvent.click(removeButtons[0]);

      expect(onChange).toHaveBeenCalledWith([{ zone: 3, duration_sec: 300 }]);
    });

    it('should remove the correct zone', async () => {
      const zones: TrainingZone[] = [
        { zone: 1, duration_sec: 100 },
        { zone: 2, duration_sec: 200 },
        { zone: 3, duration_sec: 300 },
      ];
      const onChange = vi.fn();

      render(ZoneBuilder, {
        props: {
          zones,
          modality: 'rowing',
          onChange,
        },
      });

      const removeButtons = screen.getAllByLabelText(/Remove zone/i);
      await fireEvent.click(removeButtons[1]); // Remove middle zone

      expect(onChange).toHaveBeenCalledWith([
        { zone: 1, duration_sec: 100 },
        { zone: 3, duration_sec: 300 },
      ]);
    });
  });

  describe('modality-specific fields', () => {
    it('should show power and cadence fields for cycling', () => {
      const zones: TrainingZone[] = [{ zone: 2, duration_sec: 600 }];

      render(ZoneBuilder, {
        props: {
          zones,
          modality: 'cycling',
          onChange: vi.fn(),
        },
      });

      expect(screen.getByLabelText(/Target Power \(watts\)/i)).toBeTruthy();
      expect(screen.getByLabelText(/Cadence \(rpm\)/i)).toBeTruthy();
    });

    it('should show pace field for running', () => {
      const zones: TrainingZone[] = [{ zone: 2, duration_sec: 600 }];

      render(ZoneBuilder, {
        props: {
          zones,
          modality: 'running',
          onChange: vi.fn(),
        },
      });

      expect(screen.getByLabelText(/Target Pace \(sec\/km\)/i)).toBeTruthy();
    });

    it('should not show power field for running', () => {
      const zones: TrainingZone[] = [{ zone: 2, duration_sec: 600 }];

      const { container } = render(ZoneBuilder, {
        props: {
          zones,
          modality: 'running',
          onChange: vi.fn(),
        },
      });

      const labels = container.querySelectorAll('label');
      const hasPowerField = Array.from(labels).some((label) =>
        label.textContent?.includes('Target Power')
      );
      expect(hasPowerField).toBe(false);
    });

    it('should show power field for rowing', () => {
      const zones: TrainingZone[] = [{ zone: 2, duration_sec: 600 }];

      render(ZoneBuilder, {
        props: {
          zones,
          modality: 'rowing',
          onChange: vi.fn(),
        },
      });

      expect(screen.getByLabelText(/Target Power \(watts\)/i)).toBeTruthy();
    });

    it('should not show cadence for running', () => {
      const zones: TrainingZone[] = [{ zone: 2, duration_sec: 600 }];

      const { container } = render(ZoneBuilder, {
        props: {
          zones,
          modality: 'running',
          onChange: vi.fn(),
        },
      });

      const labels = container.querySelectorAll('label');
      const hasCadenceField = Array.from(labels).some((label) =>
        label.textContent?.includes('Cadence')
      );
      expect(hasCadenceField).toBe(false);
    });
  });

  describe('updating zones', () => {
    it('should update zone number', async () => {
      const zones: TrainingZone[] = [{ zone: 2, duration_sec: 600 }];
      const onChange = vi.fn();

      render(ZoneBuilder, {
        props: {
          zones,
          modality: 'cycling',
          onChange,
        },
      });

      const zoneInput = screen.getByDisplayValue('2') as HTMLInputElement;
      await fireEvent.input(zoneInput, { target: { value: '4' } });

      expect(onChange).toHaveBeenCalledWith([
        expect.objectContaining({
          zone: 4,
          duration_sec: 600,
        }),
      ]);
    });

    it('should update duration', async () => {
      const zones: TrainingZone[] = [{ zone: 2, duration_sec: 600 }];
      const onChange = vi.fn();

      render(ZoneBuilder, {
        props: {
          zones,
          modality: 'cycling',
          onChange,
        },
      });

      const durationInput = screen.getByDisplayValue('600') as HTMLInputElement;
      await fireEvent.input(durationInput, { target: { value: '1200' } });

      expect(onChange).toHaveBeenCalledWith([
        expect.objectContaining({
          zone: 2,
          duration_sec: 1200,
        }),
      ]);
    });
  });
});

describe('IntervalBuilder', () => {
  describe('empty state', () => {
    it('should render empty state when no phases', () => {
      render(IntervalBuilder, {
        props: {
          interval_phases: [],
          modality: 'cycling',
          onChange: vi.fn(),
        },
      });

      expect(screen.getByText(/No interval phases defined/i)).toBeTruthy();
      expect(screen.getByRole('button', { name: /Add Phase/i })).toBeTruthy();
    });
  });

  describe('adding phases', () => {
    it('should add first phase as "work"', async () => {
      const onChange = vi.fn();

      render(IntervalBuilder, {
        props: {
          interval_phases: [],
          modality: 'cycling',
          onChange,
        },
      });

      const addButton = screen.getByRole('button', { name: /Add Phase/i });
      await fireEvent.click(addButton);

      expect(onChange).toHaveBeenCalledWith(
        expect.arrayContaining([
          expect.objectContaining({
            name: 'work',
            duration_sec: 60,
          }),
        ])
      );
    });

    it('should add second phase as "recovery"', async () => {
      const onChange = vi.fn();
      const phases: IntervalPhase[] = [{ name: 'work', duration_sec: 120 }];

      render(IntervalBuilder, {
        props: {
          interval_phases: phases,
          modality: 'cycling',
          onChange,
        },
      });

      const addButton = screen.getByRole('button', { name: /Add Phase/i });
      await fireEvent.click(addButton);

      expect(onChange).toHaveBeenCalledWith(
        expect.arrayContaining([
          expect.objectContaining({ name: 'work' }),
          expect.objectContaining({ name: 'recovery' }),
        ])
      );
    });

    it('should add power and cadence for cycling', async () => {
      const onChange = vi.fn();

      render(IntervalBuilder, {
        props: {
          interval_phases: [],
          modality: 'cycling',
          onChange,
        },
      });

      const addButton = screen.getByRole('button', { name: /Add Phase/i });
      await fireEvent.click(addButton);

      expect(onChange).toHaveBeenCalledWith(
        expect.arrayContaining([
          expect.objectContaining({
            target_power_watts: 200,
            cadence_rpm: 90,
          }),
        ])
      );
    });

    it('should add pace for running', async () => {
      const onChange = vi.fn();

      render(IntervalBuilder, {
        props: {
          interval_phases: [],
          modality: 'running',
          onChange,
        },
      });

      const addButton = screen.getByRole('button', { name: /Add Phase/i });
      await fireEvent.click(addButton);

      expect(onChange).toHaveBeenCalledWith(
        expect.arrayContaining([
          expect.objectContaining({
            target_pace_sec_per_km: 300,
          }),
        ])
      );
    });
  });

  describe('displaying phases', () => {
    it('should display existing phases', () => {
      const phases: IntervalPhase[] = [
        {
          name: 'work',
          duration_sec: 120,
          target_power_watts: 250,
        },
        {
          name: 'recovery',
          duration_sec: 60,
          target_power_watts: 100,
        },
      ];

      render(IntervalBuilder, {
        props: {
          interval_phases: phases,
          modality: 'cycling',
          onChange: vi.fn(),
        },
      });

      expect(screen.getByText(/Phase 1: work/i)).toBeTruthy();
      expect(screen.getByText(/Phase 2: recovery/i)).toBeTruthy();
    });
  });

  describe('removing phases', () => {
    it('should remove a phase when remove button is clicked', async () => {
      const phases: IntervalPhase[] = [
        { name: 'work', duration_sec: 120 },
        { name: 'recovery', duration_sec: 60 },
      ];
      const onChange = vi.fn();

      render(IntervalBuilder, {
        props: {
          interval_phases: phases,
          modality: 'running',
          onChange,
        },
      });

      const removeButtons = screen.getAllByLabelText(/Remove phase/i);
      await fireEvent.click(removeButtons[0]);

      expect(onChange).toHaveBeenCalledWith([{ name: 'recovery', duration_sec: 60 }]);
    });
  });

  describe('updating phases', () => {
    it('should update phase name', async () => {
      const phases: IntervalPhase[] = [{ name: 'work', duration_sec: 120 }];
      const onChange = vi.fn();

      render(IntervalBuilder, {
        props: {
          interval_phases: phases,
          modality: 'cycling',
          onChange,
        },
      });

      const nameInput = screen.getByDisplayValue('work') as HTMLInputElement;
      await fireEvent.input(nameInput, { target: { value: 'sprint' } });

      expect(onChange).toHaveBeenCalledWith([
        expect.objectContaining({
          name: 'sprint',
          duration_sec: 120,
        }),
      ]);
    });

    it('should update duration', async () => {
      const phases: IntervalPhase[] = [{ name: 'work', duration_sec: 60 }];
      const onChange = vi.fn();

      render(IntervalBuilder, {
        props: {
          interval_phases: phases,
          modality: 'cycling',
          onChange,
        },
      });

      const durationInput = screen.getByDisplayValue('60') as HTMLInputElement;
      await fireEvent.input(durationInput, { target: { value: '120' } });

      expect(onChange).toHaveBeenCalledWith([
        expect.objectContaining({
          name: 'work',
          duration_sec: 120,
        }),
      ]);
    });
  });

  describe('modality-specific fields', () => {
    it('should show power and cadence for cycling', () => {
      const phases: IntervalPhase[] = [{ name: 'work', duration_sec: 60 }];

      render(IntervalBuilder, {
        props: {
          interval_phases: phases,
          modality: 'cycling',
          onChange: vi.fn(),
        },
      });

      expect(screen.getByLabelText(/Target Power \(watts\)/i)).toBeTruthy();
      expect(screen.getByLabelText(/Cadence \(rpm\)/i)).toBeTruthy();
    });

    it('should show pace for running', () => {
      const phases: IntervalPhase[] = [{ name: 'work', duration_sec: 60 }];

      render(IntervalBuilder, {
        props: {
          interval_phases: phases,
          modality: 'running',
          onChange: vi.fn(),
        },
      });

      expect(screen.getByLabelText(/Target Pace \(sec\/km\)/i)).toBeTruthy();
    });
  });
});

describe('RampBuilder', () => {
  describe('empty state', () => {
    it('should render empty state when no ramp', () => {
      render(RampBuilder, {
        props: {
          ramp: undefined,
          onChange: vi.fn(),
        },
      });

      expect(screen.getByText(/No ramp configuration/i)).toBeTruthy();
      expect(screen.getByText(/Add Power Ramp/i)).toBeTruthy();
    });
  });

  describe('adding ramp', () => {
    it('should add a new ramp with default values', async () => {
      const onChange = vi.fn();

      render(RampBuilder, {
        props: {
          ramp: undefined,
          onChange,
        },
      });

      const addButton = screen.getByText(/Add Power Ramp/i);
      await fireEvent.click(addButton);

      expect(onChange).toHaveBeenCalledWith(
        expect.objectContaining({
          start_power_watts: 100,
          end_power_watts: 300,
          duration_sec: 600,
        })
      );
    });
  });

  describe('displaying ramp', () => {
    it('should display existing ramp configuration', () => {
      const ramp: RampConfig = {
        start_power_watts: 150,
        end_power_watts: 350,
        duration_sec: 900,
      };

      render(RampBuilder, {
        props: {
          ramp,
          onChange: vi.fn(),
        },
      });

      expect(screen.getByText(/Power Ramp Configuration/i)).toBeTruthy();
      expect(screen.getByDisplayValue('150')).toBeTruthy();
      expect(screen.getByDisplayValue('350')).toBeTruthy();
      expect(screen.getByDisplayValue('900')).toBeTruthy();
    });

    it('should display optional step_duration_sec', () => {
      const ramp: RampConfig = {
        start_power_watts: 100,
        end_power_watts: 300,
        duration_sec: 600,
        step_duration_sec: 60,
      };

      render(RampBuilder, {
        props: {
          ramp,
          onChange: vi.fn(),
        },
      });

      expect(screen.getByLabelText(/Step Duration/i)).toBeTruthy();
      expect(screen.getByDisplayValue('60')).toBeTruthy();
    });
  });

  describe('removing ramp', () => {
    it('should remove ramp when Remove Ramp button is clicked', async () => {
      const ramp: RampConfig = {
        start_power_watts: 150,
        end_power_watts: 350,
        duration_sec: 900,
      };
      const onChange = vi.fn();

      render(RampBuilder, {
        props: {
          ramp,
          onChange,
        },
      });

      const removeButton = screen.getByText(/Remove Ramp/i);
      await fireEvent.click(removeButton);

      expect(onChange).toHaveBeenCalledWith(undefined);
    });
  });

  describe('updating ramp', () => {
    it('should update start_power_watts', async () => {
      const ramp: RampConfig = {
        start_power_watts: 100,
        end_power_watts: 300,
        duration_sec: 600,
      };
      const onChange = vi.fn();

      render(RampBuilder, {
        props: {
          ramp,
          onChange,
        },
      });

      const startPowerInput = screen.getByLabelText(/Start Power/i) as HTMLInputElement;
      await fireEvent.input(startPowerInput, { target: { value: '150' } });

      expect(onChange).toHaveBeenCalledWith(
        expect.objectContaining({
          start_power_watts: 150,
          end_power_watts: 300,
          duration_sec: 600,
        })
      );
    });

    it('should update end_power_watts', async () => {
      const ramp: RampConfig = {
        start_power_watts: 100,
        end_power_watts: 300,
        duration_sec: 600,
      };
      const onChange = vi.fn();

      render(RampBuilder, {
        props: {
          ramp,
          onChange,
        },
      });

      const endPowerInput = screen.getByLabelText(/End Power/i) as HTMLInputElement;
      await fireEvent.input(endPowerInput, { target: { value: '400' } });

      expect(onChange).toHaveBeenCalledWith(
        expect.objectContaining({
          start_power_watts: 100,
          end_power_watts: 400,
          duration_sec: 600,
        })
      );
    });

    it('should update duration_sec', async () => {
      const ramp: RampConfig = {
        start_power_watts: 100,
        end_power_watts: 300,
        duration_sec: 600,
      };
      const onChange = vi.fn();

      render(RampBuilder, {
        props: {
          ramp,
          onChange,
        },
      });

      const durationInput = screen.getAllByLabelText(/Duration \(seconds\)/i)[0] as HTMLInputElement;
      await fireEvent.input(durationInput, { target: { value: '900' } });

      expect(onChange).toHaveBeenCalledWith(
        expect.objectContaining({
          start_power_watts: 100,
          end_power_watts: 300,
          duration_sec: 900,
        })
      );
    });
  });

  describe('validation', () => {
    it('should show validation error when start >= end power', () => {
      const ramp: RampConfig = {
        start_power_watts: 300,
        end_power_watts: 200,
        duration_sec: 600,
      };

      const { container } = render(RampBuilder, {
        props: {
          ramp,
          onChange: vi.fn(),
        },
      });

      expect(container.textContent).toContain('Start power must be less than end power');
    });

    it('should not show validation error when start < end power', () => {
      const ramp: RampConfig = {
        start_power_watts: 100,
        end_power_watts: 300,
        duration_sec: 600,
      };

      const { container } = render(RampBuilder, {
        props: {
          ramp,
          onChange: vi.fn(),
        },
      });

      expect(container.textContent).not.toContain('Start power must be less than end power');
    });
  });
});
