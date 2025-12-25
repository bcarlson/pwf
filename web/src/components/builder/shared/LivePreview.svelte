<script lang="ts">
  import { onMount } from 'svelte';
  import { validatePlan } from '../../../lib/wasm';
  import type { ValidationResult } from '../../../lib/stores';
  import type { PlanDraft } from '../../../lib/builderState';
  import { generateYAML } from '../utils/yamlGenerator';

  export let draft: PlanDraft;

  let yamlContent: string = '';
  let validationResult: ValidationResult | null = null;
  let isValidating: boolean = false;
  let errorMessage: string = '';

  $: {
    // Reactively generate YAML when draft changes
    if (draft) {
      updatePreview();
    }
  }

  function updatePreview() {
    try {
      yamlContent = generateYAML(draft);
      validateYAML();
    } catch (error) {
      errorMessage = `Failed to generate YAML: ${error}`;
      yamlContent = '';
      validationResult = null;
    }
  }

  function validateYAML() {
    if (!yamlContent) {
      validationResult = null;
      return;
    }

    isValidating = true;
    errorMessage = '';

    try {
      validationResult = validatePlan(yamlContent);
    } catch (error) {
      errorMessage = `Validation error: ${error}`;
      validationResult = null;
    } finally {
      isValidating = false;
    }
  }

  function downloadYAML() {
    if (!yamlContent) return;

    const blob = new Blob([yamlContent], { type: 'text/yaml' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = draft.meta?.name?.replace(/\s+/g, '-').toLowerCase() || 'plan.yaml';
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }

  function copyToClipboard() {
    if (!yamlContent) return;

    navigator.clipboard.writeText(yamlContent).then(() => {
      // Show success feedback (could be enhanced with a toast)
      console.log('YAML copied to clipboard');
    }).catch(err => {
      errorMessage = `Failed to copy: ${err}`;
    });
  }

  onMount(() => {
    updatePreview();
  });
</script>

<div class="live-preview">
  <div class="preview-header">
    <h3>Live Preview</h3>
    <div class="validation-badge" class:valid={validationResult?.valid} class:invalid={validationResult && !validationResult.valid}>
      {#if isValidating}
        <span class="spinner">⏳</span> Validating...
      {:else if validationResult?.valid}
        <span class="icon">✓</span> Valid
      {:else if validationResult}
        <span class="icon">!</span> Errors
      {:else}
        <span class="icon">—</span> Not Validated
      {/if}
    </div>
  </div>

  {#if errorMessage}
    <div class="error-message">
      <span class="error-icon">⚠️</span>
      {errorMessage}
    </div>
  {/if}

  <div class="preview-content">
    {#if yamlContent}
      <pre><code>{yamlContent}</code></pre>
    {:else}
      <div class="empty-state">
        <p>Start building your plan to see the YAML preview</p>
      </div>
    {/if}
  </div>

  {#if validationResult && !validationResult.valid}
    <div class="validation-errors">
      <h4>Validation Errors ({validationResult.errors.length})</h4>
      <ul>
        {#each validationResult.errors.slice(0, 5) as error}
          <li>
            <strong>{error.path}:</strong> {error.message}
            {#if error.code}
              <code class="error-code">{error.code}</code>
            {/if}
          </li>
        {/each}
        {#if validationResult.errors.length > 5}
          <li class="more-errors">
            ...and {validationResult.errors.length - 5} more error(s)
          </li>
        {/if}
      </ul>
    </div>
  {/if}

  <div class="preview-actions">
    <button
      class="btn-secondary"
      on:click={copyToClipboard}
      disabled={!yamlContent}
      title="Copy YAML to clipboard"
    >
      Copy
    </button>
    <button
      class="btn-secondary"
      on:click={downloadYAML}
      disabled={!yamlContent}
      title="Download YAML file"
    >
      Download
    </button>
  </div>
</div>

<style>
  .live-preview {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    overflow: hidden;
  }

  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.5rem;
    border-bottom: 1px solid var(--border-color);
    background: var(--bg-primary);
  }

  .preview-header h3 {
    margin: 0;
    font-size: 1.1rem;
    color: var(--text-primary);
  }

  .validation-badge {
    padding: 0.4rem 0.9rem;
    border-radius: 20px;
    font-size: 0.85rem;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 0.4rem;
    background: var(--bg-secondary);
    color: var(--text-secondary);
  }

  .validation-badge.valid {
    background: rgba(25, 135, 84, 0.1);
    color: var(--success-color);
  }

  .validation-badge.invalid {
    background: rgba(220, 53, 69, 0.1);
    color: var(--error-color);
  }

  .validation-badge .icon {
    font-size: 1rem;
  }

  .spinner {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .error-message {
    background: rgba(220, 53, 69, 0.1);
    border-bottom: 1px solid var(--error-color);
    padding: 0.75rem 1.5rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--error-color);
    font-size: 0.9rem;
  }

  .preview-content {
    flex: 1;
    overflow: auto;
    padding: 1rem;
  }

  pre {
    margin: 0;
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
    font-size: 0.85rem;
    line-height: 1.5;
    color: var(--text-primary);
  }

  code {
    display: block;
    white-space: pre;
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    min-height: 200px;
  }

  .empty-state p {
    color: var(--text-secondary);
    font-size: 0.95rem;
    text-align: center;
  }

  .validation-errors {
    border-top: 1px solid var(--border-color);
    padding: 1rem 1.5rem;
    background: rgba(220, 53, 69, 0.05);
    max-height: 200px;
    overflow-y: auto;
  }

  .validation-errors h4 {
    margin: 0 0 0.75rem 0;
    font-size: 0.95rem;
    color: var(--error-color);
  }

  .validation-errors ul {
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .validation-errors li {
    padding: 0.5rem 0;
    font-size: 0.85rem;
    color: var(--text-primary);
    border-bottom: 1px solid var(--border-color);
  }

  .validation-errors li:last-child {
    border-bottom: none;
  }

  .validation-errors strong {
    color: var(--error-color);
    font-weight: 600;
  }

  .error-code {
    display: inline-block;
    margin-left: 0.5rem;
    padding: 0.1rem 0.4rem;
    background: var(--bg-secondary);
    border-radius: 3px;
    font-size: 0.75rem;
    font-family: monospace;
  }

  .more-errors {
    font-style: italic;
    color: var(--text-secondary);
  }

  .preview-actions {
    display: flex;
    gap: 0.75rem;
    padding: 1rem 1.5rem;
    border-top: 1px solid var(--border-color);
    background: var(--bg-primary);
  }

  .preview-actions button {
    flex: 1;
  }

  @media (max-width: 768px) {
    .preview-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.75rem;
    }

    .preview-actions {
      flex-direction: column;
    }
  }
</style>
