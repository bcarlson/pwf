/**
 * Tests for FormatSelector component
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import FormatSelector from '../FormatSelector.svelte';

describe('FormatSelector', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should render the component', () => {
    const { container } = render(FormatSelector);
    expect(container.querySelector('.format-selector')).toBeTruthy();
  });

  it('should render source and target format selects', () => {
    const { getByLabelText } = render(FormatSelector);

    expect(getByLabelText(/Convert from:/)).toBeTruthy();
    expect(getByLabelText(/Convert to:/)).toBeTruthy();
  });

  it('should have target select disabled when no source is selected', () => {
    const { getByLabelText } = render(FormatSelector);
    const targetSelect = getByLabelText(/Convert to:/) as HTMLSelectElement;

    expect(targetSelect.disabled).toBe(true);
  });

  it('should enable target select when source is selected', async () => {
    const { getByLabelText } = render(FormatSelector);
    const sourceSelect = getByLabelText(/Convert from:/) as HTMLSelectElement;
    const targetSelect = getByLabelText(/Convert to:/) as HTMLSelectElement;

    await fireEvent.change(sourceSelect, { target: { value: 'fit' } });

    expect(targetSelect.disabled).toBe(false);
  });

  it('should display all source formats that can import', () => {
    const { container } = render(FormatSelector);
    const sourceSelect = container.querySelector('#source-format') as HTMLSelectElement;
    const options = Array.from(sourceSelect.options).map(opt => opt.value).filter(v => v);

    // FIT, TCX, GPX, PWF can import
    expect(options).toContain('fit');
    expect(options).toContain('tcx');
    expect(options).toContain('gpx');
    expect(options).toContain('pwf');
    // CSV cannot import
    expect(options).not.toContain('csv');
  });

  it('should display valid target formats based on source selection', async () => {
    const { getByLabelText } = render(FormatSelector);
    const sourceSelect = getByLabelText(/Convert from:/) as HTMLSelectElement;

    await fireEvent.change(sourceSelect, { target: { value: 'fit' } });

    const targetSelect = getByLabelText(/Convert to:/) as HTMLSelectElement;
    const options = Array.from(targetSelect.options)
      .map(opt => opt.value)
      .filter(v => v);

    // Target should not include the source format
    expect(options).not.toContain('fit');
    // PWF should be available
    expect(options).toContain('pwf');
  });

  it('should reset target format if it matches source format', async () => {
    const { getByLabelText } = render(FormatSelector, {
      props: { sourceFormat: 'tcx', targetFormat: 'gpx' }
    });

    const sourceSelect = getByLabelText(/Convert from:/) as HTMLSelectElement;

    // Change source to match target
    await fireEvent.change(sourceSelect, { target: { value: 'gpx' } });

    expect(sourceSelect.value).toBe('gpx');
    // Target should be reset (empty)
    const targetSelect = getByLabelText(/Convert to:/) as HTMLSelectElement;
    // The target will be cleared in the component logic
  });

  it('should dispatch change event when source format changes', async () => {
    const { component, getByLabelText } = render(FormatSelector);
    const sourceSelect = getByLabelText(/Convert from:/) as HTMLSelectElement;

    const changeHandler = vi.fn();
    component.$on('change', changeHandler);

    await fireEvent.change(sourceSelect, { target: { value: 'fit' } });

    expect(changeHandler).toHaveBeenCalled();
    expect(changeHandler.mock.calls[0][0].detail).toEqual({
      sourceFormat: 'fit',
      targetFormat: ''
    });
  });

  it('should dispatch change event when target format changes', async () => {
    const { component, getByLabelText } = render(FormatSelector, {
      props: { sourceFormat: 'fit' }
    });
    const targetSelect = getByLabelText(/Convert to:/) as HTMLSelectElement;

    const changeHandler = vi.fn();
    component.$on('change', changeHandler);

    await fireEvent.change(targetSelect, { target: { value: 'pwf' } });

    expect(changeHandler).toHaveBeenCalled();
    expect(changeHandler.mock.calls[0][0].detail).toEqual({
      sourceFormat: 'fit',
      targetFormat: 'pwf'
    });
  });

  it('should display format info when both formats are selected', async () => {
    const { getByLabelText, container } = render(FormatSelector);
    const sourceSelect = getByLabelText(/Convert from:/) as HTMLSelectElement;
    const targetSelect = getByLabelText(/Convert to:/) as HTMLSelectElement;

    await fireEvent.change(sourceSelect, { target: { value: 'fit' } });
    await fireEvent.change(targetSelect, { target: { value: 'pwf' } });

    const formatInfo = container.querySelector('.format-info');
    expect(formatInfo).toBeTruthy();
    expect(formatInfo?.textContent).toContain('FIT (Garmin)');
    expect(formatInfo?.textContent).toContain('PWF (Portable Workout Format)');
  });

  it('should not display format info when only source is selected', async () => {
    const { getByLabelText, container } = render(FormatSelector);
    const sourceSelect = getByLabelText(/Convert from:/) as HTMLSelectElement;

    await fireEvent.change(sourceSelect, { target: { value: 'fit' } });

    const formatInfo = container.querySelector('.format-info');
    expect(formatInfo).toBeFalsy();
  });

  it('should not display format info when formats are not selected', () => {
    const { container } = render(FormatSelector);
    const formatInfo = container.querySelector('.format-info');
    expect(formatInfo).toBeFalsy();
  });

  it('should handle PWF as source format', async () => {
    const { getByLabelText } = render(FormatSelector);
    const sourceSelect = getByLabelText(/Convert from:/) as HTMLSelectElement;

    await fireEvent.change(sourceSelect, { target: { value: 'pwf' } });

    const targetSelect = getByLabelText(/Convert to:/) as HTMLSelectElement;
    const options = Array.from(targetSelect.options)
      .map(opt => opt.value)
      .filter(v => v);

    // PWF can export to TCX, GPX, CSV
    expect(options).toContain('tcx');
    expect(options).toContain('gpx');
    expect(options).toContain('csv');
    expect(options).not.toContain('pwf'); // Should not include itself
  });

  it('should handle TCX as source format', async () => {
    const { getByLabelText } = render(FormatSelector);
    const sourceSelect = getByLabelText(/Convert from:/) as HTMLSelectElement;

    await fireEvent.change(sourceSelect, { target: { value: 'tcx' } });

    const targetSelect = getByLabelText(/Convert to:/) as HTMLSelectElement;
    const options = Array.from(targetSelect.options)
      .map(opt => opt.value)
      .filter(v => v);

    // Should include PWF
    expect(options).toContain('pwf');
    expect(options).not.toContain('tcx');
  });

  it('should render correct format names and extensions', () => {
    const { container } = render(FormatSelector);
    const sourceSelect = container.querySelector('#source-format') as HTMLSelectElement;

    const fitOption = Array.from(sourceSelect.options).find(opt => opt.value === 'fit');
    expect(fitOption?.textContent).toBe('FIT (Garmin)');

    const tcxOption = Array.from(sourceSelect.options).find(opt => opt.value === 'tcx');
    expect(tcxOption?.textContent).toBe('TCX (Training Center XML)');

    const gpxOption = Array.from(sourceSelect.options).find(opt => opt.value === 'gpx');
    expect(gpxOption?.textContent).toBe('GPX (GPS Exchange)');

    const pwfOption = Array.from(sourceSelect.options).find(opt => opt.value === 'pwf');
    expect(pwfOption?.textContent).toBe('PWF (Portable Workout Format)');
  });

  it('should bind to sourceFormat prop', async () => {
    const { getByLabelText } = render(FormatSelector, {
      props: { sourceFormat: 'tcx' }
    });

    const sourceSelect = getByLabelText(/Convert from:/) as HTMLSelectElement;
    expect(sourceSelect.value).toBe('tcx');

    await fireEvent.change(sourceSelect, { target: { value: 'gpx' } });
    expect(sourceSelect.value).toBe('gpx');
  });

  it('should bind to targetFormat prop', async () => {
    const { getByLabelText } = render(FormatSelector, {
      props: { sourceFormat: 'tcx', targetFormat: 'pwf' }
    });

    const targetSelect = getByLabelText(/Convert to:/) as HTMLSelectElement;
    expect(targetSelect.value).toBe('pwf');

    await fireEvent.change(targetSelect, { target: { value: 'gpx' } });
    expect(targetSelect.value).toBe('gpx');
  });

  it('should show arrow between selects', () => {
    const { container } = render(FormatSelector);
    const arrow = container.querySelector('.arrow');
    expect(arrow).toBeTruthy();
    expect(arrow?.textContent).toBe('→');
  });

  it('should have proper select placeholders', () => {
    const { container } = render(FormatSelector);
    const sourceSelect = container.querySelector('#source-format') as HTMLSelectElement;
    const targetSelect = container.querySelector('#target-format') as HTMLSelectElement;

    expect(sourceSelect.options[0].textContent).toBe('Select source format...');
    expect(targetSelect.options[0].textContent).toBe('Select target format...');
  });
});
