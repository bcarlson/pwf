/**
 * Tests for ReviewStep component
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import ReviewStep from '../steps/ReviewStep.svelte';
import { builderState } from '../../../lib/builderState';
import * as wasm from '../../../lib/wasm';
import * as yamlGenerator from '../utils/yamlGenerator';

// Mock WASM module
vi.mock('../../../lib/wasm', () => ({
  validatePlan: vi.fn(() => ({
    valid: true,
    errors: [],
    warnings: [],
    plan: {
      plan_version: 1,
      cycle: {
        days: []
      }
    }
  }))
}));

// Mock YAML generator
vi.mock('../utils/yamlGenerator', () => ({
  generateYAML: vi.fn(() => 'plan_version: 1\ncycle:\n  days: []')
}));

describe('ReviewStep', () => {
  beforeEach(() => {
    builderState.reset();
    vi.clearAllMocks();

    // Setup default mocks
    vi.mocked(wasm.validatePlan).mockReturnValue({
      valid: true,
      errors: [],
      warnings: [],
      plan: {
        plan_version: 1,
        cycle: {
          days: [{
            exercises: []
          }]
        }
      }
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('should render the component', () => {
    const { container } = render(ReviewStep);
    expect(container.querySelector('.review-step')).toBeTruthy();
  });

  it('should render step header', () => {
    const { getByText } = render(ReviewStep);
    expect(getByText('Review & Export')).toBeTruthy();
    expect(getByText(/Review your workout plan/)).toBeTruthy();
  });

  it('should show valid status badge when plan is valid', () => {
    const { getByText } = render(ReviewStep);
    expect(getByText('Valid PWF Plan')).toBeTruthy();
  });

  it('should show error status badge when plan has errors', () => {
    vi.mocked(wasm.validatePlan).mockReturnValue({
      valid: false,
      errors: [
        { path: 'cycle.days[0]', message: 'Missing exercises', severity: 'error' as const }
      ],
      warnings: [],
      plan: null
    });

    const { getByText } = render(ReviewStep);
    expect(getByText('1 Error')).toBeTruthy();
  });

  it('should show plural errors when multiple errors exist', () => {
    vi.mocked(wasm.validatePlan).mockReturnValue({
      valid: false,
      errors: [
        { path: 'cycle.days[0]', message: 'Error 1', severity: 'error' as const },
        { path: 'cycle.days[1]', message: 'Error 2', severity: 'error' as const }
      ],
      warnings: [],
      plan: null
    });

    const { getByText } = render(ReviewStep);
    expect(getByText('2 Errors')).toBeTruthy();
  });

  it('should show warning badge when plan has warnings', () => {
    vi.mocked(wasm.validatePlan).mockReturnValue({
      valid: true,
      errors: [],
      warnings: [
        { path: 'meta', message: 'No description', severity: 'warning' as const }
      ],
      plan: {
        plan_version: 1,
        cycle: { days: [] }
      }
    });

    const { getByText } = render(ReviewStep);
    expect(getByText('1 Warning')).toBeTruthy();
  });

  it('should display validation errors', () => {
    vi.mocked(wasm.validatePlan).mockReturnValue({
      valid: false,
      errors: [
        {
          path: 'cycle.days[0].exercises',
          message: 'At least one exercise required',
          severity: 'error' as const,
          code: 'PWF-P001'
        }
      ],
      warnings: [],
      plan: null
    });

    const { getByText } = render(ReviewStep);
    expect(getByText('Validation Errors')).toBeTruthy();
    expect(getByText(/At least one exercise required/)).toBeTruthy();
    expect(getByText('PWF-P001')).toBeTruthy();
  });

  it('should display validation warnings', () => {
    vi.mocked(wasm.validatePlan).mockReturnValue({
      valid: true,
      errors: [],
      warnings: [
        {
          path: 'meta.description',
          message: 'Description recommended',
          severity: 'warning' as const
        }
      ],
      plan: {
        plan_version: 1,
        cycle: { days: [] }
      }
    });

    const { getByText } = render(ReviewStep);
    expect(getByText('Warnings')).toBeTruthy();
    expect(getByText(/Description recommended/)).toBeTruthy();
  });

  it('should render YAML preview', () => {
    const { getByText } = render(ReviewStep);
    expect(getByText('YAML Preview')).toBeTruthy();
    expect(getByText(/plan_version: 1/)).toBeTruthy();
  });

  it('should render Copy to Clipboard button', () => {
    const { getByText } = render(ReviewStep);
    expect(getByText('Copy to Clipboard')).toBeTruthy();
  });

  it('should render Download YAML button', () => {
    const { getByText } = render(ReviewStep);
    expect(getByText('Download YAML')).toBeTruthy();
  });

  it('should copy YAML to clipboard when button is clicked', async () => {
    const clipboardSpy = vi.spyOn(navigator.clipboard, 'writeText').mockResolvedValue();

    const { getByText } = render(ReviewStep);
    const copyButton = getByText('Copy to Clipboard');

    await fireEvent.click(copyButton);

    expect(clipboardSpy).toHaveBeenCalledWith('plan_version: 1\ncycle:\n  days: []');
  });

  it('should show "Copied!" feedback after copying', async () => {
    vi.spyOn(navigator.clipboard, 'writeText').mockResolvedValue();

    const { getByText, queryByText } = render(ReviewStep);
    const copyButton = getByText('Copy to Clipboard');

    await fireEvent.click(copyButton);

    expect(getByText('Copied!')).toBeTruthy();

    // Wait for the timeout to complete
    await new Promise(resolve => setTimeout(resolve, 2100));

    expect(queryByText('Copied!')).toBeNull();
  });

  it('should download YAML when Download button is clicked', async () => {
    builderState.updateMeta({ name: 'Test Plan' });

    const { getByText } = render(ReviewStep);

    const originalCreateElement = document.createElement.bind(document);
    const originalAppendChild = document.body.appendChild.bind(document.body);
    const originalRemoveChild = document.body.removeChild.bind(document.body);

    const createElementSpy = vi.spyOn(document, 'createElement');
    const appendChildSpy = vi.spyOn(document.body, 'appendChild');
    const removeChildSpy = vi.spyOn(document.body, 'removeChild');

    const clickSpy = vi.fn();
    const mockAnchor = {
      href: '',
      download: '',
      click: clickSpy,
      remove: vi.fn(),
      style: {}
    };

    // Only mock anchor elements, let other elements be created normally
    createElementSpy.mockImplementation((tagName: string) => {
      if (tagName === 'a') {
        return mockAnchor as any;
      }
      return originalCreateElement(tagName);
    });

    // Only mock appendChild/removeChild for the mock anchor
    appendChildSpy.mockImplementation((node: any) => {
      if (node === mockAnchor) {
        return null as any;
      }
      return originalAppendChild(node);
    });

    removeChildSpy.mockImplementation((node: any) => {
      if (node === mockAnchor) {
        return null as any;
      }
      return originalRemoveChild(node);
    });

    const downloadButton = getByText('Download YAML');
    await fireEvent.click(downloadButton);

    expect(mockAnchor.download).toBe('test-plan.yaml');
    expect(clickSpy).toHaveBeenCalled();

    createElementSpy.mockRestore();
    appendChildSpy.mockRestore();
    removeChildSpy.mockRestore();
  });

  it('should use default filename when plan has no name', async () => {
    const { getByText } = render(ReviewStep);

    const originalCreateElement = document.createElement.bind(document);
    const originalAppendChild = document.body.appendChild.bind(document.body);
    const originalRemoveChild = document.body.removeChild.bind(document.body);

    const createElementSpy = vi.spyOn(document, 'createElement');
    const appendChildSpy = vi.spyOn(document.body, 'appendChild');
    const removeChildSpy = vi.spyOn(document.body, 'removeChild');

    const mockAnchor = {
      href: '',
      download: '',
      click: vi.fn(),
      remove: vi.fn(),
      style: {}
    };

    // Only mock anchor elements, let other elements be created normally
    createElementSpy.mockImplementation((tagName: string) => {
      if (tagName === 'a') {
        return mockAnchor as any;
      }
      return originalCreateElement(tagName);
    });

    // Only mock appendChild/removeChild for the mock anchor
    appendChildSpy.mockImplementation((node: any) => {
      if (node === mockAnchor) {
        return null as any;
      }
      return originalAppendChild(node);
    });

    removeChildSpy.mockImplementation((node: any) => {
      if (node === mockAnchor) {
        return null as any;
      }
      return originalRemoveChild(node);
    });

    const downloadButton = getByText('Download YAML');
    await fireEvent.click(downloadButton);

    expect(mockAnchor.download).toBe('workout-plan.yaml');

    createElementSpy.mockRestore();
    appendChildSpy.mockRestore();
    removeChildSpy.mockRestore();
  });

  it('should disable buttons when no YAML content', () => {
    vi.mocked(yamlGenerator.generateYAML).mockReturnValue('');

    const { getByText } = render(ReviewStep);
    const copyButton = getByText('Copy to Clipboard') as HTMLButtonElement;
    const downloadButton = getByText('Download YAML') as HTMLButtonElement;

    expect(copyButton.disabled).toBe(true);
    expect(downloadButton.disabled).toBe(true);
  });

  it('should render PlanTreeView when plan is valid', () => {
    const { container } = render(ReviewStep);
    expect(container.querySelector('.plan-tree')).toBeTruthy();
  });

  it('should handle YAML generation errors gracefully', () => {
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    vi.mocked(yamlGenerator.generateYAML).mockImplementation(() => {
      throw new Error('Generation failed');
    });

    // Should not throw
    expect(() => render(ReviewStep)).not.toThrow();

    expect(consoleSpy).toHaveBeenCalled();
    consoleSpy.mockRestore();
  });
});
