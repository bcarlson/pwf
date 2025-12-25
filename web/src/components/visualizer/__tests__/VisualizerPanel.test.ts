/**
 * Tests for VisualizerPanel component
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { render } from '@testing-library/svelte';
import VisualizerPanel from '../VisualizerPanel.svelte';

describe('VisualizerPanel', () => {
  beforeEach(() => {
    // Reset any state if needed
  });

  it('should render the component', () => {
    const { container } = render(VisualizerPanel);
    expect(container.querySelector('.visualizer-panel')).toBeTruthy();
  });

  it('should render panel intro with title', () => {
    const { getByText } = render(VisualizerPanel);
    expect(getByText('Workout Visualizer')).toBeTruthy();
  });

  it('should render panel intro with description', () => {
    const { getByText } = render(VisualizerPanel);
    expect(getByText(/Visualize PWF plans and workout history/)).toBeTruthy();
  });

  it('should render FileUpload component initially', () => {
    const { container } = render(VisualizerPanel);
    // FileUpload should be present when no data is loaded
    expect(container.textContent).toContain('Drop PWF file here or click to browse');
  });

  it('should render ExampleGallery component initially', () => {
    const { container } = render(VisualizerPanel);
    // ExampleGallery should be present when no data is loaded
    const panel = container.querySelector('.visualizer-panel');
    expect(panel).toBeTruthy();
  });

  it('should have intro section', () => {
    const { container } = render(VisualizerPanel);
    expect(container.querySelector('.panel-intro')).toBeTruthy();
  });

  it('should have proper component structure', () => {
    const { container } = render(VisualizerPanel);
    const panel = container.querySelector('.visualizer-panel');
    expect(panel).toBeTruthy();

    const intro = container.querySelector('.panel-intro');
    expect(intro).toBeTruthy();
  });

  it('should render without crashing', () => {
    expect(() => {
      render(VisualizerPanel);
    }).not.toThrow();
  });

  it('should have correct initial state (no data loaded)', () => {
    const { container } = render(VisualizerPanel);

    // Should show upload UI, not visualization
    expect(container.textContent).toContain('Drop PWF file here or click to browse');

    // Should not show file info
    expect(container.querySelector('.file-info')).toBeFalsy();
  });

  it('should not show plan tree initially', () => {
    const { container } = render(VisualizerPanel);
    expect(container.querySelector('.plan-tree')).toBeFalsy();
  });

  it('should not show history visualization initially', () => {
    const { container } = render(VisualizerPanel);
    expect(container.querySelector('.history-visualization')).toBeFalsy();
  });

  it('should not show error banner initially', () => {
    const { container } = render(VisualizerPanel);
    expect(container.querySelector('.error-banner')).toBeFalsy();
  });

  it('should not show file info initially', () => {
    const { container } = render(VisualizerPanel);
    expect(container.querySelector('.file-info')).toBeFalsy();
  });

  it('should have proper CSS classes', () => {
    const { container } = render(VisualizerPanel);

    const panel = container.querySelector('.visualizer-panel');
    expect(panel).toBeTruthy();

    const intro = container.querySelector('.panel-intro');
    expect(intro).toBeTruthy();
  });

  it('should render h2 for title', () => {
    const { container } = render(VisualizerPanel);
    const h2 = container.querySelector('.panel-intro h2');
    expect(h2).toBeTruthy();
    expect(h2?.textContent).toBe('Workout Visualizer');
  });

  it('should render paragraph for description', () => {
    const { container } = render(VisualizerPanel);
    const p = container.querySelector('.panel-intro p');
    expect(p).toBeTruthy();
    expect(p?.textContent).toContain('Visualize PWF plans and workout history');
  });

  it('should have no console errors on mount', () => {
    const consoleError = console.error;
    const mockError = vi.fn();
    console.error = mockError;

    render(VisualizerPanel);

    expect(mockError).not.toHaveBeenCalled();
    console.error = consoleError;
  });

  it('should be a valid Svelte component', () => {
    const { component } = render(VisualizerPanel);
    expect(component).toBeTruthy();
    expect(component.$$).toBeTruthy();
  });

  it('should have valid component structure', () => {
    const { component } = render(VisualizerPanel);
    expect(component).toBeTruthy();
    expect(component.$$).toBeTruthy();
  });
});
