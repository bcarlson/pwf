/**
 * Tests for TelemetryCharts component
 * Note: Full chart rendering tests are skipped due to complexity of mocking svelte-chartjs.
 * These tests focus on component structure, empty states, and logic that doesn't require chart rendering.
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { render } from '@testing-library/svelte';
import TelemetryCharts from '../TelemetryCharts.svelte';

describe('TelemetryCharts', () => {
  beforeEach(() => {
    // Reset any state
  });

  it('should render the component', () => {
    const { container } = render(TelemetryCharts, {
      props: {
        workout: {}
      }
    });
    expect(container.querySelector('.telemetry-charts')).toBeTruthy();
  });

  it('should display empty state when no telemetry data', () => {
    const { getByText } = render(TelemetryCharts, {
      props: {
        workout: {}
      }
    });
    expect(getByText('No telemetry data available for this workout')).toBeTruthy();
  });

  it('should display empty state when telemetry array is empty', () => {
    const { container } = render(TelemetryCharts, {
      props: {
        workout: {
          name: 'Empty Telemetry',
          telemetry: []
        }
      }
    });
    expect(container.querySelector('.empty-state')).toBeTruthy();
  });

  it('should handle undefined workout', () => {
    const { container } = render(TelemetryCharts, {
      props: {
        workout: undefined
      }
    });
    expect(container.querySelector('.empty-state')).toBeTruthy();
  });

  it('should handle null workout', () => {
    const { container } = render(TelemetryCharts, {
      props: {
        workout: null
      }
    });
    expect(container.querySelector('.empty-state')).toBeTruthy();
  });

  it('should handle undefined telemetry', () => {
    const { container } = render(TelemetryCharts, {
      props: {
        workout: {
          name: 'No Telemetry',
          telemetry: undefined
        }
      }
    });
    expect(container.querySelector('.empty-state')).toBeTruthy();
  });

  it('should handle null telemetry', () => {
    const { container } = render(TelemetryCharts, {
      props: {
        workout: {
          name: 'Null Telemetry',
          telemetry: null
        }
      }
    });
    expect(container.querySelector('.empty-state')).toBeTruthy();
  });

  it('should have empty state message', () => {
    const { container } = render(TelemetryCharts, {
      props: {
        workout: {}
      }
    });

    const emptyState = container.querySelector('.empty-state');
    expect(emptyState).toBeTruthy();
    expect(emptyState?.textContent).toContain('No telemetry data available');
  });

  it('should not show charts header when no telemetry', () => {
    const { container } = render(TelemetryCharts, {
      props: {
        workout: {}
      }
    });

    expect(container.querySelector('.charts-header')).toBeFalsy();
  });

  it('should not show charts grid when no telemetry', () => {
    const { container } = render(TelemetryCharts, {
      props: {
        workout: {}
      }
    });

    expect(container.querySelector('.charts-grid')).toBeFalsy();
  });

  it('should not show stats section when no telemetry', () => {
    const { container } = render(TelemetryCharts, {
      props: {
        workout: {}
      }
    });

    expect(container.querySelector('.stats-section')).toBeFalsy();
  });

  it('should render without crashing', () => {
    expect(() => {
      render(TelemetryCharts, {
        props: { workout: {} }
      });
    }).not.toThrow();
  });

  it('should have correct component structure', () => {
    const { component } = render(TelemetryCharts, {
      props: { workout: {} }
    });
    expect(component).toBeTruthy();
    expect(component.$$).toBeTruthy();
  });

  it('should handle workout with only name', () => {
    const { container } = render(TelemetryCharts, {
      props: {
        workout: {
          name: 'Test Workout'
        }
      }
    });

    expect(container.querySelector('.empty-state')).toBeTruthy();
  });

  it('should have proper CSS class on main container', () => {
    const { container } = render(TelemetryCharts, {
      props: {
        workout: {}
      }
    });

    const mainContainer = container.querySelector('.telemetry-charts');
    expect(mainContainer).toBeTruthy();
  });

  it('should have proper CSS class on empty state', () => {
    const { container } = render(TelemetryCharts, {
      props: {
        workout: {}
      }
    });

    const emptyState = container.querySelector('.empty-state');
    expect(emptyState).toBeTruthy();
  });

  it('should not crash with empty object workout', () => {
    const { container } = render(TelemetryCharts, {
      props: {
        workout: {}
      }
    });

    expect(container).toBeTruthy();
  });

  it('should handle workout with empty string name', () => {
    const { container } = render(TelemetryCharts, {
      props: {
        workout: {
          name: '',
          telemetry: []
        }
      }
    });

    expect(container.querySelector('.empty-state')).toBeTruthy();
  });

  it('should handle multiple renders', () => {
    const { component, container } = render(TelemetryCharts, {
      props: {
        workout: {}
      }
    });

    expect(container.querySelector('.empty-state')).toBeTruthy();

    // Update props
    component.$set({
      workout: {
        name: 'Updated',
        telemetry: []
      }
    });

    expect(container.querySelector('.empty-state')).toBeTruthy();
  });

  it('should handle prop updates', () => {
    const { component, container } = render(TelemetryCharts, {
      props: {
        workout: null
      }
    });

    expect(container.querySelector('.empty-state')).toBeTruthy();

    component.$set({
      workout: {}
    });

    expect(container.querySelector('.empty-state')).toBeTruthy();
  });
});
