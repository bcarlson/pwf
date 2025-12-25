/**
 * Tests for MetaStep component
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import { get } from 'svelte/store';
import MetaStep from '../steps/MetaStep.svelte';
import { builderState } from '../../../lib/builderState';

describe('MetaStep', () => {
  beforeEach(() => {
    builderState.reset();
  });

  it('should render the component', () => {
    const { container } = render(MetaStep);
    expect(container.querySelector('.meta-step')).toBeTruthy();
  });

  it('should render step header with title and description', () => {
    const { getByText } = render(MetaStep);
    expect(getByText('Plan Metadata')).toBeTruthy();
    expect(getByText(/Add optional information/)).toBeTruthy();
  });

  it('should render MetaForm component', () => {
    const { container } = render(MetaStep);
    expect(container.querySelector('.meta-form')).toBeTruthy();
  });

  it('should render "Skip to Days" button', () => {
    const { getByText } = render(MetaStep);
    const skipButton = getByText('Skip to Days');
    expect(skipButton).toBeTruthy();
  });

  it('should navigate to step 1 when "Skip to Days" is clicked', async () => {
    const { getByText } = render(MetaStep);
    const skipButton = getByText('Skip to Days');

    await fireEvent.click(skipButton);

    const state = get(builderState);
    expect(state.currentStep).toBe(1);
  });

  it('should have correct CSS classes', () => {
    const { container } = render(MetaStep);
    const stepHeader = container.querySelector('.step-header');
    const stepActions = container.querySelector('.step-actions');

    expect(stepHeader).toBeTruthy();
    expect(stepActions).toBeTruthy();
  });
});
