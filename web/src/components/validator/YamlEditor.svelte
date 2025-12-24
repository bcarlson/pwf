<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let value: string = '';
  export let readonly: boolean = false;
  export let placeholder: string = 'Enter or paste YAML content here...';
  export let highlightedLine: number | null = null;

  const dispatch = createEventDispatcher();
  let textarea: HTMLTextAreaElement;

  function handleInput(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    dispatch('change', { value: target.value });
  }

  function handleKeyDown(event: KeyboardEvent) {
    // Tab key inserts 2 spaces
    if (event.key === 'Tab') {
      event.preventDefault();
      const start = textarea.selectionStart;
      const end = textarea.selectionEnd;
      const newValue = value.substring(0, start) + '  ' + value.substring(end);
      value = newValue;
      setTimeout(() => {
        textarea.selectionStart = textarea.selectionEnd = start + 2;
      }, 0);
      dispatch('change', { value });
    }

    // Ctrl+S to validate
    if ((event.ctrlKey || event.metaKey) && event.key === 's') {
      event.preventDefault();
      dispatch('validate');
    }
  }

  export function jumpToLine(lineNumber: number) {
    if (!textarea) return;

    const lines = value.split('\n');
    let position = 0;
    for (let i = 0; i < lineNumber - 1 && i < lines.length; i++) {
      position += lines[i].length + 1; // +1 for newline
    }

    textarea.focus();
    textarea.setSelectionRange(position, position + (lines[lineNumber - 1]?.length || 0));
    textarea.scrollTop = (lineNumber - 5) * 20; // Approximate scroll
  }

  // Auto-focus on mount
  import { onMount } from 'svelte';
  onMount(() => {
    if (!readonly && textarea) {
      textarea.focus();
    }
  });
</script>

<div class="yaml-editor">
  <div class="editor-header">
    <span class="file-type">YAML</span>
    <span class="editor-hint">Press Ctrl+S to validate</span>
  </div>

  <div class="editor-wrapper">
    <div class="line-numbers" aria-hidden="true">
      {#each value.split('\n') as _, index}
        <div
          class="line-number"
          class:highlighted={highlightedLine === index + 1}
        >
          {index + 1}
        </div>
      {/each}
    </div>

    <textarea
      bind:this={textarea}
      bind:value
      {readonly}
      {placeholder}
      spellcheck="false"
      on:input={handleInput}
      on:keydown={handleKeyDown}
      class="editor-textarea"
    ></textarea>
  </div>

  <div class="editor-footer">
    <span class="line-count">
      {value.split('\n').length} lines · {value.length} characters
    </span>
  </div>
</div>

<style>
  .yaml-editor {
    border: 1px solid var(--border-color);
    border-radius: 8px;
    overflow: hidden;
    background: var(--bg-secondary);
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
  }

  .editor-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 1rem;
    background: var(--bg-hover);
    border-bottom: 1px solid var(--border-color);
  }

  .file-type {
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--accent-color);
    text-transform: uppercase;
  }

  .editor-hint {
    font-size: 0.75rem;
    opacity: 0.6;
  }

  .editor-wrapper {
    display: flex;
    background: var(--bg-primary);
    min-height: 400px;
    max-height: 600px;
    overflow: auto;
  }

  .line-numbers {
    flex-shrink: 0;
    padding: 1rem 0.5rem;
    background: var(--bg-hover);
    border-right: 1px solid var(--border-color);
    user-select: none;
    text-align: right;
    color: var(--text-secondary);
    font-size: 0.9rem;
    line-height: 1.5;
  }

  .line-number {
    padding: 0 0.5rem;
    min-width: 2.5rem;
  }

  .line-number.highlighted {
    background: rgba(255, 193, 7, 0.2);
    color: var(--warning-color);
    font-weight: bold;
  }

  .editor-textarea {
    flex: 1;
    padding: 1rem;
    border: none;
    outline: none;
    background: transparent;
    color: var(--text-primary);
    font-family: inherit;
    font-size: 0.9rem;
    line-height: 1.5;
    resize: none;
    overflow-y: auto;
  }

  .editor-textarea::placeholder {
    color: var(--text-secondary);
    opacity: 0.5;
  }

  .editor-footer {
    padding: 0.5rem 1rem;
    background: var(--bg-hover);
    border-top: 1px solid var(--border-color);
    font-size: 0.75rem;
    color: var(--text-secondary);
    text-align: right;
  }

  @media (max-width: 768px) {
    .editor-wrapper {
      min-height: 300px;
    }

    .editor-hint {
      display: none;
    }

    .line-numbers {
      font-size: 0.75rem;
    }

    .editor-textarea {
      font-size: 0.85rem;
    }
  }
</style>
