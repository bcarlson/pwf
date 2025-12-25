/**
 * Tests for MetaForm component
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import { get } from 'svelte/store';
import MetaForm from '../forms/MetaForm.svelte';
import { builderState } from '../../../lib/builderState';
import * as wasm from '../../../lib/wasm';

// Mock WASM module
vi.mock('../../../lib/wasm', () => ({
  getSupportedEquipment: vi.fn(() => ['barbell', 'dumbbell', 'kettlebell', 'resistance_band'])
}));

describe('MetaForm', () => {
  beforeEach(() => {
    builderState.reset();
    vi.clearAllMocks();
  });

  it('should render the form', () => {
    const { container } = render(MetaForm);
    expect(container.querySelector('.meta-form')).toBeTruthy();
  });

  it('should render all form fields', () => {
    const { getByLabelText, getByText } = render(MetaForm);

    expect(getByLabelText(/Plan Name/)).toBeTruthy();
    expect(getByLabelText(/Description/)).toBeTruthy();
    expect(getByLabelText(/Author/)).toBeTruthy();
    expect(getByLabelText(/Days Per Week/)).toBeTruthy();
    expect(getByText(/Equipment/)).toBeTruthy(); // Equipment is a label for a group, not a form control
    expect(getByLabelText(/Tags/)).toBeTruthy();
  });

  it('should update plan name when input changes', async () => {
    const { getByLabelText } = render(MetaForm);
    const nameInput = getByLabelText(/Plan Name/) as HTMLInputElement;

    await fireEvent.input(nameInput, { target: { value: 'My Workout Plan' } });

    const state = get(builderState);
    expect(state.plan.meta?.name).toBe('My Workout Plan');
  });

  it('should update description when textarea changes', async () => {
    const { getByLabelText } = render(MetaForm);
    const descInput = getByLabelText(/Description/) as HTMLTextAreaElement;

    await fireEvent.input(descInput, { target: { value: 'Test description' } });

    const state = get(builderState);
    expect(state.plan.meta?.description).toBe('Test description');
  });

  it('should update author when input changes', async () => {
    const { getByLabelText } = render(MetaForm);
    const authorInput = getByLabelText(/Author/) as HTMLInputElement;

    await fireEvent.input(authorInput, { target: { value: 'John Doe' } });

    const state = get(builderState);
    expect(state.plan.meta?.author).toBe('John Doe');
  });

  it('should update days per week when input changes', async () => {
    const { getByLabelText } = render(MetaForm);
    const daysInput = getByLabelText(/Days Per Week/) as HTMLInputElement;

    await fireEvent.input(daysInput, { target: { value: '3' } });

    const state = get(builderState);
    expect(state.plan.meta?.days_per_week).toBe(3);
  });

  it('should toggle equipment when chip is clicked', async () => {
    const { getByText } = render(MetaForm);
    const barbellChip = getByText('barbell');

    await fireEvent.click(barbellChip);

    let state = get(builderState);
    expect(state.plan.meta?.equipment).toContain('barbell');

    // Click again to deselect
    await fireEvent.click(barbellChip);

    state = get(builderState);
    expect(state.plan.meta?.equipment).not.toContain('barbell');
  });

  it('should add tag when Add button is clicked', async () => {
    const { getByLabelText, getByText } = render(MetaForm);
    const tagInput = getByLabelText(/Tags/) as HTMLInputElement;
    const addButton = getByText('Add');

    await fireEvent.input(tagInput, { target: { value: 'strength' } });
    await fireEvent.click(addButton);

    const state = get(builderState);
    expect(state.plan.meta?.tags).toContain('strength');
    expect(tagInput.value).toBe('');
  });

  it('should add tag when Enter key is pressed', async () => {
    const { getByLabelText } = render(MetaForm);
    const tagInput = getByLabelText(/Tags/) as HTMLInputElement;

    await fireEvent.input(tagInput, { target: { value: 'cardio' } });
    await fireEvent.keyDown(tagInput, { key: 'Enter' });

    const state = get(builderState);
    expect(state.plan.meta?.tags).toContain('cardio');
  });

  it('should remove tag when remove button is clicked', async () => {
    // First add a tag
    builderState.updateMeta({ tags: ['strength'] });

    const { container } = render(MetaForm);
    const removeButton = container.querySelector('.remove-tag') as HTMLButtonElement;

    await fireEvent.click(removeButton);

    const state = get(builderState);
    expect(state.plan.meta?.tags).not.toContain('strength');
  });

  it('should not add empty tags', async () => {
    const { getByLabelText, getByText } = render(MetaForm);
    const tagInput = getByLabelText(/Tags/) as HTMLInputElement;
    const addButton = getByText('Add');

    await fireEvent.input(tagInput, { target: { value: '   ' } });
    await fireEvent.click(addButton);

    const state = get(builderState);
    expect(state.plan.meta?.tags || []).toHaveLength(0);
  });

  it('should not add duplicate tags', async () => {
    builderState.updateMeta({ tags: ['strength'] });

    const { getByLabelText, getByText } = render(MetaForm);
    const tagInput = getByLabelText(/Tags/) as HTMLInputElement;
    const addButton = getByText('Add');

    await fireEvent.input(tagInput, { target: { value: 'strength' } });
    await fireEvent.click(addButton);

    const state = get(builderState);
    expect(state.plan.meta?.tags?.filter(t => t === 'strength')).toHaveLength(1);
  });

  it('should render equipment from WASM', () => {
    const { getByText } = render(MetaForm);

    expect(getByText('barbell')).toBeTruthy();
    expect(getByText('dumbbell')).toBeTruthy();
    expect(getByText('kettlebell')).toBeTruthy();
    expect(getByText('resistance band')).toBeTruthy(); // underscores replaced
  });

  it('should handle WASM errors gracefully', () => {
    vi.mocked(wasm.getSupportedEquipment).mockImplementation(() => {
      throw new Error('WASM not ready');
    });

    // Should not throw
    expect(() => render(MetaForm)).not.toThrow();
  });
});
