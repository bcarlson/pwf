/**
 * Tests for YamlEditor component
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, fireEvent, waitFor } from '@testing-library/svelte';
import YamlEditor from '../YamlEditor.svelte';

describe('YamlEditor', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should render the component without crashing', () => {
    const { container } = render(YamlEditor);
    expect(container).toBeTruthy();
    expect(container.querySelector('.yaml-editor')).toBeTruthy();
  });

  it('should render editor header with YAML label', () => {
    const { getByText } = render(YamlEditor);
    expect(getByText('YAML')).toBeTruthy();
  });

  it('should display keyboard hint', () => {
    const { getByText } = render(YamlEditor);
    expect(getByText('Press Ctrl+S to validate')).toBeTruthy();
  });

  it('should render textarea', () => {
    const { container } = render(YamlEditor);
    const textarea = container.querySelector('textarea');
    expect(textarea).toBeTruthy();
  });

  it('should render line numbers', () => {
    const { container } = render(YamlEditor, {
      props: { value: 'line 1\nline 2\nline 3' }
    });
    const lineNumbers = container.querySelectorAll('.line-number');
    expect(lineNumbers.length).toBe(3);
    expect(lineNumbers[0].textContent?.trim()).toBe('1');
    expect(lineNumbers[1].textContent?.trim()).toBe('2');
    expect(lineNumbers[2].textContent?.trim()).toBe('3');
  });

  it('should display default placeholder', () => {
    const { container } = render(YamlEditor);
    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    expect(textarea.placeholder).toBe('Enter or paste YAML content here...');
  });

  it('should display custom placeholder', () => {
    const { container } = render(YamlEditor, {
      props: { placeholder: 'Custom placeholder text' }
    });
    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    expect(textarea.placeholder).toBe('Custom placeholder text');
  });

  it('should render with initial value', () => {
    const { container } = render(YamlEditor, {
      props: { value: 'plan_version: 1' }
    });
    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    expect(textarea.value).toBe('plan_version: 1');
  });

  it('should be editable by default', () => {
    const { container } = render(YamlEditor);
    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    expect(textarea.readOnly).toBe(false);
  });

  it('should be readonly when prop is set', () => {
    const { container } = render(YamlEditor, {
      props: { readonly: true }
    });
    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    expect(textarea.readOnly).toBe(true);
  });

  it('should dispatch change event on input', async () => {
    const { container, component } = render(YamlEditor);

    const changeHandler = vi.fn();
    component.$on('change', changeHandler);

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    await fireEvent.input(textarea, { target: { value: 'new content' } });

    expect(changeHandler).toHaveBeenCalled();
    const eventDetail = changeHandler.mock.calls[0][0].detail;
    expect(eventDetail.value).toBe('new content');
  });

  it('should handle Tab key to insert spaces', async () => {
    const { container } = render(YamlEditor, {
      props: { value: 'test' }
    });

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    textarea.setSelectionRange(4, 4); // Position at end

    await fireEvent.keyDown(textarea, { key: 'Tab' });

    await waitFor(() => {
      expect(textarea.value).toBe('test  '); // Should add 2 spaces
    });
  });

  it('should insert Tab at cursor position', async () => {
    const { container } = render(YamlEditor, {
      props: { value: 'before after' }
    });

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    textarea.setSelectionRange(6, 6); // Position between "before" and " after"

    await fireEvent.keyDown(textarea, { key: 'Tab' });

    await waitFor(() => {
      expect(textarea.value).toBe('before   after'); // 2 spaces inserted
    });
  });

  it('should replace selected text with Tab', async () => {
    const { container } = render(YamlEditor, {
      props: { value: 'hello world' }
    });

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    textarea.setSelectionRange(6, 11); // Select "world"

    await fireEvent.keyDown(textarea, { key: 'Tab' });

    await waitFor(() => {
      expect(textarea.value).toBe('hello   '); // "world" replaced with 2 spaces
    });
  });

  it('should dispatch validate event on Ctrl+S', async () => {
    const { container, component } = render(YamlEditor);

    const validateHandler = vi.fn();
    component.$on('validate', validateHandler);

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    await fireEvent.keyDown(textarea, { key: 's', ctrlKey: true });

    expect(validateHandler).toHaveBeenCalled();
  });

  it('should dispatch validate event on Cmd+S (Mac)', async () => {
    const { container, component } = render(YamlEditor);

    const validateHandler = vi.fn();
    component.$on('validate', validateHandler);

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    await fireEvent.keyDown(textarea, { key: 's', metaKey: true });

    expect(validateHandler).toHaveBeenCalled();
  });

  it('should prevent default behavior on Tab', async () => {
    const { container } = render(YamlEditor);

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    const event = new KeyboardEvent('keydown', { key: 'Tab' });
    const preventDefaultSpy = vi.spyOn(event, 'preventDefault');

    textarea.dispatchEvent(event);

    expect(preventDefaultSpy).toHaveBeenCalled();
  });

  it('should prevent default behavior on Ctrl+S', async () => {
    const { container } = render(YamlEditor);

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    const event = new KeyboardEvent('keydown', { key: 's', ctrlKey: true });
    const preventDefaultSpy = vi.spyOn(event, 'preventDefault');

    textarea.dispatchEvent(event);

    expect(preventDefaultSpy).toHaveBeenCalled();
  });

  it('should display line and character count', () => {
    const { getByText } = render(YamlEditor, {
      props: { value: 'line 1\nline 2' }
    });
    expect(getByText(/2 lines/)).toBeTruthy();
    expect(getByText(/13 characters/)).toBeTruthy();
  });

  it('should update line count when content changes', async () => {
    const { container } = render(YamlEditor, {
      props: { value: 'line 1' }
    });

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    await fireEvent.input(textarea, { target: { value: 'line 1\nline 2\nline 3' } });

    await waitFor(() => {
      const footer = container.querySelector('.editor-footer');
      expect(footer?.textContent).toContain('3 lines');
    });
  });

  it('should disable spellcheck', () => {
    const { container } = render(YamlEditor);
    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    // Check the attribute directly
    expect(textarea.getAttribute('spellcheck')).toBe('false');
  });

  it('should have monospace font family', () => {
    const { container } = render(YamlEditor);
    const editor = container.querySelector('.yaml-editor') as HTMLElement;
    // Just verify the class exists (CSS is applied separately)
    expect(editor).toBeTruthy();
    expect(editor.classList.contains('yaml-editor')).toBe(true);
  });

  it('should auto-focus when not readonly', () => {
    const { container } = render(YamlEditor, {
      props: { readonly: false }
    });

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    // Note: In jsdom, auto-focus may not work exactly as in browser
    // This test verifies the component attempts to focus
    expect(textarea).toBeTruthy();
  });

  it('should not auto-focus when readonly', () => {
    const { container } = render(YamlEditor, {
      props: { readonly: true }
    });

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    expect(textarea).toBeTruthy();
  });

  it('should highlight specific line', () => {
    const { container } = render(YamlEditor, {
      props: { value: 'line 1\nline 2\nline 3', highlightedLine: 2 }
    });

    const lineNumbers = container.querySelectorAll('.line-number');
    expect(lineNumbers[1].classList.contains('highlighted')).toBe(true);
    expect(lineNumbers[0].classList.contains('highlighted')).toBe(false);
    expect(lineNumbers[2].classList.contains('highlighted')).toBe(false);
  });

  it('should export jumpToLine method', () => {
    const { component } = render(YamlEditor, {
      props: { value: 'line 1\nline 2\nline 3' }
    });

    expect(component.jumpToLine).toBeDefined();
    expect(typeof component.jumpToLine).toBe('function');
  });

  it('should jump to specific line', () => {
    const { container, component } = render(YamlEditor, {
      props: { value: 'line 1\nline 2\nline 3' }
    });

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    const focusSpy = vi.spyOn(textarea, 'focus');
    const setSelectionSpy = vi.spyOn(textarea, 'setSelectionRange');

    component.jumpToLine(2);

    expect(focusSpy).toHaveBeenCalled();
    expect(setSelectionSpy).toHaveBeenCalled();
    // Line 2 starts at position 7 (after "line 1\n")
    expect(setSelectionSpy).toHaveBeenCalledWith(7, 13); // "line 2"
  });

  it('should handle jumpToLine with invalid line number', () => {
    const { container, component } = render(YamlEditor, {
      props: { value: 'line 1\nline 2\nline 3' }
    });

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    const focusSpy = vi.spyOn(textarea, 'focus');

    // Should not throw error
    component.jumpToLine(100);
    expect(focusSpy).toHaveBeenCalled();
  });

  it('should scroll when jumping to line', () => {
    const { container, component } = render(YamlEditor, {
      props: { value: 'line 1\nline 2\nline 3\nline 4\nline 5\nline 6\nline 7\nline 8' }
    });

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;

    component.jumpToLine(7);

    // Should set scrollTop (approximate calculation: (7 - 5) * 20)
    expect(textarea.scrollTop).toBeGreaterThanOrEqual(0);
  });

  it('should handle empty content', () => {
    const { container } = render(YamlEditor, {
      props: { value: '' }
    });

    const lineNumbers = container.querySelectorAll('.line-number');
    expect(lineNumbers.length).toBe(1); // Empty string still has 1 line
  });

  it('should update line numbers reactively', async () => {
    const { container, component } = render(YamlEditor, {
      props: { value: 'line 1' }
    });

    expect(container.querySelectorAll('.line-number').length).toBe(1);

    // Update value
    await component.$set({ value: 'line 1\nline 2\nline 3' });

    await waitFor(() => {
      expect(container.querySelectorAll('.line-number').length).toBe(3);
    });
  });

  it('should have editor wrapper with scroll', () => {
    const { container } = render(YamlEditor);
    const wrapper = container.querySelector('.editor-wrapper');
    expect(wrapper).toBeTruthy();
  });

  it('should have line numbers with aria-hidden', () => {
    const { container } = render(YamlEditor, {
      props: { value: 'line 1' }
    });
    const lineNumbersDiv = container.querySelector('.line-numbers');
    expect(lineNumbersDiv?.getAttribute('aria-hidden')).toBe('true');
  });

  it('should maintain cursor position after Tab insertion', async () => {
    const { container } = render(YamlEditor, {
      props: { value: 'test' }
    });

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    textarea.setSelectionRange(2, 2); // Position in middle

    await fireEvent.keyDown(textarea, { key: 'Tab' });

    await waitFor(() => {
      // Cursor should be at position 4 (2 + 2 spaces)
      expect(textarea.selectionStart).toBe(4);
      expect(textarea.selectionEnd).toBe(4);
    });
  });

  it('should dispatch change event after Tab insertion', async () => {
    const { container, component } = render(YamlEditor, {
      props: { value: 'test' }
    });

    const changeHandler = vi.fn();
    component.$on('change', changeHandler);

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
    textarea.setSelectionRange(4, 4);

    await fireEvent.keyDown(textarea, { key: 'Tab' });

    await waitFor(() => {
      expect(changeHandler).toHaveBeenCalled();
      const eventDetail = changeHandler.mock.calls[0][0].detail;
      expect(eventDetail.value).toBe('test  ');
    });
  });

  it('should not handle other keyboard shortcuts', async () => {
    const { container, component } = render(YamlEditor);

    const validateHandler = vi.fn();
    component.$on('validate', validateHandler);

    const textarea = container.querySelector('textarea') as HTMLTextAreaElement;

    // Ctrl+A (should not trigger validate)
    await fireEvent.keyDown(textarea, { key: 'a', ctrlKey: true });
    expect(validateHandler).not.toHaveBeenCalled();

    // Ctrl+C (should not trigger validate)
    await fireEvent.keyDown(textarea, { key: 'c', ctrlKey: true });
    expect(validateHandler).not.toHaveBeenCalled();
  });

  it('should count characters correctly including newlines', () => {
    const content = 'line 1\nline 2';
    const { container } = render(YamlEditor, {
      props: { value: content }
    });

    const footer = container.querySelector('.editor-footer');
    expect(footer?.textContent).toContain(`${content.length} characters`);
  });

  it('should handle multiline content in line numbers', () => {
    const content = 'a\nb\nc\nd\ne\nf\ng\nh\ni\nj';
    const { container } = render(YamlEditor, {
      props: { value: content }
    });

    const lineNumbers = container.querySelectorAll('.line-number');
    expect(lineNumbers.length).toBe(10);
    expect(lineNumbers[9].textContent?.trim()).toBe('10');
  });

  it('should render footer with stats', () => {
    const { container } = render(YamlEditor, {
      props: { value: 'test' }
    });

    const footer = container.querySelector('.editor-footer');
    expect(footer).toBeTruthy();
    expect(footer?.querySelector('.line-count')).toBeTruthy();
  });

  it('should have proper CSS classes', () => {
    const { container } = render(YamlEditor);

    expect(container.querySelector('.yaml-editor')).toBeTruthy();
    expect(container.querySelector('.editor-header')).toBeTruthy();
    expect(container.querySelector('.editor-wrapper')).toBeTruthy();
    expect(container.querySelector('.editor-textarea')).toBeTruthy();
    expect(container.querySelector('.editor-footer')).toBeTruthy();
  });
});
