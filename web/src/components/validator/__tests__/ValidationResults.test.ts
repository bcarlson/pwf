/**
 * Tests for ValidationResults component
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import ValidationResults from '../ValidationResults.svelte';
import type { ValidationResult } from '../../../lib/stores';

describe('ValidationResults', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should render the component without crashing', () => {
    const result: ValidationResult = {
      valid: true,
      errors: [],
      warnings: [],
    };
    const { container } = render(ValidationResults, { props: { result } });
    expect(container).toBeTruthy();
    expect(container.querySelector('.validation-results')).toBeTruthy();
  });

  it('should display success header for valid files', () => {
    const result: ValidationResult = {
      valid: true,
      errors: [],
      warnings: [],
    };
    const { getByText } = render(ValidationResults, { props: { result } });
    expect(getByText('Valid PWF File')).toBeTruthy();
    expect(getByText('No errors found. Your PWF file is valid!')).toBeTruthy();
  });

  it('should display error header for invalid files', () => {
    const result: ValidationResult = {
      valid: false,
      errors: [
        { path: 'cycle', message: 'Missing required field', severity: 'error', code: 'PWF-P001' }
      ],
      warnings: [],
    };
    const { getByText } = render(ValidationResults, { props: { result } });
    expect(getByText('Validation Failed')).toBeTruthy();
    expect(getByText('1 error found')).toBeTruthy();
  });

  it('should show checkmark icon for valid files', () => {
    const result: ValidationResult = {
      valid: true,
      errors: [],
      warnings: [],
    };
    const { container } = render(ValidationResults, { props: { result } });
    const icon = container.querySelector('.status-icon');
    expect(icon?.textContent).toBe('✓');
  });

  it('should show X icon for invalid files', () => {
    const result: ValidationResult = {
      valid: false,
      errors: [{ path: 'root', message: 'Error', severity: 'error' }],
      warnings: [],
    };
    const { container } = render(ValidationResults, { props: { result } });
    const icon = container.querySelector('.status-icon');
    expect(icon?.textContent).toBe('✗');
  });

  it('should apply valid CSS class for valid results', () => {
    const result: ValidationResult = {
      valid: true,
      errors: [],
      warnings: [],
    };
    const { container } = render(ValidationResults, { props: { result } });
    const header = container.querySelector('.result-header');
    expect(header?.classList.contains('valid')).toBe(true);
    expect(header?.classList.contains('invalid')).toBe(false);
  });

  it('should apply invalid CSS class for invalid results', () => {
    const result: ValidationResult = {
      valid: false,
      errors: [{ path: 'root', message: 'Error', severity: 'error' }],
      warnings: [],
    };
    const { container } = render(ValidationResults, { props: { result } });
    const header = container.querySelector('.result-header');
    expect(header?.classList.contains('invalid')).toBe(true);
    expect(header?.classList.contains('valid')).toBe(false);
  });

  it('should display errors list', () => {
    const result: ValidationResult = {
      valid: false,
      errors: [
        { path: 'cycle.days[0]', message: 'Missing exercises', severity: 'error', code: 'PWF-P002' },
        { path: 'meta', message: 'Invalid name', severity: 'error', code: 'PWF-P003' },
      ],
      warnings: [],
    };
    const { getByText } = render(ValidationResults, { props: { result } });
    expect(getByText('Errors (2)')).toBeTruthy();
    expect(getByText('Missing exercises')).toBeTruthy();
    expect(getByText('Invalid name')).toBeTruthy();
  });

  it('should display warnings list', () => {
    const result: ValidationResult = {
      valid: true,
      errors: [],
      warnings: [
        { path: 'meta', message: 'Missing optional field', severity: 'warning' },
      ],
    };
    const { getByText } = render(ValidationResults, { props: { result } });
    expect(getByText('Warnings (1)')).toBeTruthy();
    expect(getByText('Missing optional field')).toBeTruthy();
  });

  it('should display error codes', () => {
    const result: ValidationResult = {
      valid: false,
      errors: [
        { path: 'cycle', message: 'Error message', severity: 'error', code: 'PWF-P001' },
      ],
      warnings: [],
    };
    const { getByText } = render(ValidationResults, { props: { result } });
    expect(getByText('PWF-P001')).toBeTruthy();
  });

  it('should display error paths', () => {
    const result: ValidationResult = {
      valid: false,
      errors: [
        { path: 'cycle.days[0].exercises[1]', message: 'Invalid modality', severity: 'error', code: 'PWF-P004' },
      ],
      warnings: [],
    };
    const { getByText } = render(ValidationResults, { props: { result } });
    expect(getByText('cycle.days[0].exercises[1]')).toBeTruthy();
  });

  it('should handle errors without codes', () => {
    const result: ValidationResult = {
      valid: false,
      errors: [
        { path: 'root', message: 'Generic error', severity: 'error' },
      ],
      warnings: [],
    };
    const { getByText, queryByText } = render(ValidationResults, { props: { result } });
    expect(getByText('Generic error')).toBeTruthy();
    // Should still show ERROR badge
    expect(getByText('ERROR')).toBeTruthy();
  });

  it('should handle errors without paths', () => {
    const result: ValidationResult = {
      valid: false,
      errors: [
        { path: '', message: 'Root level error', severity: 'error', code: 'PWF-P005' },
      ],
      warnings: [],
    };
    const { getByText } = render(ValidationResults, { props: { result } });
    expect(getByText('Root level error')).toBeTruthy();
  });

  it('should show error count with correct plural', () => {
    const result1: ValidationResult = {
      valid: false,
      errors: [{ path: 'root', message: 'Error', severity: 'error' }],
      warnings: [],
    };
    const { getByText: getText1 } = render(ValidationResults, { props: { result: result1 } });
    expect(getText1('1 error found')).toBeTruthy();

    const result2: ValidationResult = {
      valid: false,
      errors: [
        { path: 'root', message: 'Error 1', severity: 'error' },
        { path: 'root', message: 'Error 2', severity: 'error' },
      ],
      warnings: [],
    };
    const { getByText: getText2 } = render(ValidationResults, { props: { result: result2 } });
    expect(getText2('2 errors found')).toBeTruthy();
  });

  it('should show warning count with correct plural', () => {
    const result1: ValidationResult = {
      valid: true,
      errors: [],
      warnings: [{ path: 'meta', message: 'Warning', severity: 'warning' }],
    };
    const { container: container1 } = render(ValidationResults, { props: { result: result1 } });
    const text1 = container1.textContent || '';
    expect(text1).toContain('1');
    expect(text1).toContain('warning');

    const result2: ValidationResult = {
      valid: true,
      errors: [],
      warnings: [
        { path: 'meta', message: 'Warning 1', severity: 'warning' },
        { path: 'meta', message: 'Warning 2', severity: 'warning' },
      ],
    };
    const { container: container2 } = render(ValidationResults, { props: { result: result2 } });
    const text2 = container2.textContent || '';
    expect(text2).toContain('2');
    expect(text2).toContain('warnings');
  });

  it('should display both errors and warnings count', () => {
    const result: ValidationResult = {
      valid: false,
      errors: [
        { path: 'root', message: 'Error', severity: 'error' },
      ],
      warnings: [
        { path: 'meta', message: 'Warning', severity: 'warning' },
      ],
    };
    const { container } = render(ValidationResults, { props: { result } });
    const text = container.textContent || '';
    expect(text).toContain('1 error');
    expect(text).toContain('1 warning');
  });

  it('should call onErrorClick when error is clicked', async () => {
    const onErrorClick = vi.fn();
    const result: ValidationResult = {
      valid: false,
      errors: [
        { path: 'cycle.days[0]', message: 'Error message', severity: 'error', code: 'PWF-P001' },
      ],
      warnings: [],
    };
    const { container } = render(ValidationResults, {
      props: { result, onErrorClick }
    });

    const errorItem = container.querySelector('.issue-item.error') as HTMLElement;
    await fireEvent.click(errorItem);

    expect(onErrorClick).toHaveBeenCalledWith('cycle.days[0]');
  });

  it('should call onErrorClick when warning is clicked', async () => {
    const onErrorClick = vi.fn();
    const result: ValidationResult = {
      valid: true,
      errors: [],
      warnings: [
        { path: 'meta.name', message: 'Warning message', severity: 'warning' },
      ],
    };
    const { container } = render(ValidationResults, {
      props: { result, onErrorClick }
    });

    const warningItem = container.querySelector('.issue-item.warning') as HTMLElement;
    await fireEvent.click(warningItem);

    expect(onErrorClick).toHaveBeenCalledWith('meta.name');
  });

  it('should not call onErrorClick when null', async () => {
    const result: ValidationResult = {
      valid: false,
      errors: [
        { path: 'cycle', message: 'Error', severity: 'error' },
      ],
      warnings: [],
    };
    const { container } = render(ValidationResults, {
      props: { result, onErrorClick: null }
    });

    const errorItem = container.querySelector('.issue-item') as HTMLElement;
    // Should not throw error
    await fireEvent.click(errorItem);
  });

  it('should not call onErrorClick when issue has no path', async () => {
    const onErrorClick = vi.fn();
    const result: ValidationResult = {
      valid: false,
      errors: [
        { path: '', message: 'Error', severity: 'error' },
      ],
      warnings: [],
    };
    const { container } = render(ValidationResults, {
      props: { result, onErrorClick }
    });

    const errorItem = container.querySelector('.issue-item') as HTMLElement;
    await fireEvent.click(errorItem);

    expect(onErrorClick).not.toHaveBeenCalled();
  });

  it('should display statistics when provided', () => {
    const result: ValidationResult = {
      valid: true,
      errors: [],
      warnings: [],
      statistics: {
        total_days: 7,
        total_exercises: 21,
        modalities: ['strength', 'endurance'],
      },
    };
    const { getByText, container } = render(ValidationResults, { props: { result } });
    expect(getByText('Statistics')).toBeTruthy();
    const statsText = container.textContent || '';
    expect(statsText).toContain('total_days');
    expect(statsText).toContain('7');
  });

  it('should not display statistics section when not provided', () => {
    const result: ValidationResult = {
      valid: true,
      errors: [],
      warnings: [],
    };
    const { queryByText } = render(ValidationResults, { props: { result } });
    expect(queryByText('Statistics')).toBeFalsy();
  });

  it('should display download button for valid plan', () => {
    const result: ValidationResult = {
      valid: true,
      errors: [],
      warnings: [],
      plan: {
        plan_version: 1,
        cycle: { days: [] },
      },
    };
    const { getByText } = render(ValidationResults, { props: { result } });
    expect(getByText('Download Validated File')).toBeTruthy();
  });

  it('should display download button for valid history', () => {
    const result: ValidationResult = {
      valid: true,
      errors: [],
      warnings: [],
      history: {
        history_version: 1,
        workouts: [],
      },
    };
    const { getByText } = render(ValidationResults, { props: { result } });
    expect(getByText('Download Validated File')).toBeTruthy();
  });

  it('should not display download button for invalid files', () => {
    const result: ValidationResult = {
      valid: false,
      errors: [{ path: 'root', message: 'Error', severity: 'error' }],
      warnings: [],
      plan: {
        plan_version: 1,
        cycle: { days: [] },
      },
    };
    const { queryByText } = render(ValidationResults, { props: { result } });
    expect(queryByText('Download Validated File')).toBeFalsy();
  });

  it('should download JSON when download button is clicked', async () => {
    const result: ValidationResult = {
      valid: true,
      errors: [],
      warnings: [],
      plan: {
        plan_version: 1,
        cycle: { days: [] },
      },
    };

    // Mock URL.createObjectURL and revokeObjectURL
    const mockURL = 'blob:mock-url';
    const createObjectURLSpy = vi.spyOn(URL, 'createObjectURL').mockReturnValue(mockURL);
    const revokeObjectURLSpy = vi.spyOn(URL, 'revokeObjectURL').mockImplementation(() => {});

    const { getByText } = render(ValidationResults, { props: { result } });
    const downloadBtn = getByText('Download Validated File');

    await fireEvent.click(downloadBtn);

    expect(createObjectURLSpy).toHaveBeenCalled();
    expect(revokeObjectURLSpy).toHaveBeenCalledWith(mockURL);

    createObjectURLSpy.mockRestore();
    revokeObjectURLSpy.mockRestore();
  });

  it('should apply error border color to error items', () => {
    const result: ValidationResult = {
      valid: false,
      errors: [
        { path: 'root', message: 'Error', severity: 'error' },
      ],
      warnings: [],
    };
    const { container } = render(ValidationResults, { props: { result } });
    const errorItem = container.querySelector('.issue-item');
    expect(errorItem).toBeTruthy();
    expect(errorItem?.classList.contains('error')).toBe(true);
  });

  it('should apply warning border color to warning items', () => {
    const result: ValidationResult = {
      valid: true,
      errors: [],
      warnings: [
        { path: 'meta', message: 'Warning', severity: 'warning' },
      ],
    };
    const { container } = render(ValidationResults, { props: { result } });
    const warningItem = container.querySelector('.issue-item');
    expect(warningItem).toBeTruthy();
    expect(warningItem?.classList.contains('warning')).toBe(true);
  });

  it('should render error badge with correct text', () => {
    const result: ValidationResult = {
      valid: false,
      errors: [
        { path: 'root', message: 'Error', severity: 'error' },
      ],
      warnings: [],
    };
    const { container } = render(ValidationResults, { props: { result } });
    const badge = container.querySelector('.severity-badge');
    expect(badge).toBeTruthy();
    expect(badge?.textContent).toBe('ERROR');
    expect(badge?.classList.contains('error')).toBe(true);
  });

  it('should render warning badge with correct text', () => {
    const result: ValidationResult = {
      valid: true,
      errors: [],
      warnings: [
        { path: 'meta', message: 'Warning', severity: 'warning' },
      ],
    };
    const { container } = render(ValidationResults, { props: { result } });
    const badge = container.querySelector('.severity-badge');
    expect(badge).toBeTruthy();
    expect(badge?.textContent).toBe('WARNING');
    expect(badge?.classList.contains('warning')).toBe(true);
  });

  it('should render multiple errors', () => {
    const result: ValidationResult = {
      valid: false,
      errors: [
        { path: 'cycle', message: 'Error 1', severity: 'error', code: 'PWF-P001' },
        { path: 'meta', message: 'Error 2', severity: 'error', code: 'PWF-P002' },
        { path: 'days[0]', message: 'Error 3', severity: 'error', code: 'PWF-P003' },
      ],
      warnings: [],
    };
    const { container } = render(ValidationResults, { props: { result } });
    const errorItems = container.querySelectorAll('.issue-item');
    expect(errorItems.length).toBe(3);
  });

  it('should render multiple warnings', () => {
    const result: ValidationResult = {
      valid: true,
      errors: [],
      warnings: [
        { path: 'meta.name', message: 'Warning 1', severity: 'warning' },
        { path: 'meta.author', message: 'Warning 2', severity: 'warning' },
      ],
    };
    const { container } = render(ValidationResults, { props: { result } });
    const warningItems = container.querySelectorAll('.issue-item');
    expect(warningItems.length).toBe(2);
  });

  it('should format statistics as JSON', () => {
    const result: ValidationResult = {
      valid: true,
      errors: [],
      warnings: [],
      statistics: {
        total_days: 3,
        exercises_count: 10,
      },
    };
    const { container } = render(ValidationResults, { props: { result } });
    const pre = container.querySelector('pre');
    expect(pre).toBeTruthy();
    expect(pre?.textContent).toContain('total_days');
    expect(pre?.textContent).toContain('3');
  });
});
