<script lang="ts">
  import { builderState } from '../../../lib/builderState';
  import PlanTreeView from '../../visualizer/PlanTreeView.svelte';
  import LivePreview from '../shared/LivePreview.svelte';
  import { validatePlan } from '../../../lib/wasm';
  import type { ValidationResult } from '../../../lib/stores';
  import { generateYAML } from '../utils/yamlGenerator';

  $: draft = $builderState.plan;
  let yamlContent = '';
  let validationResult: ValidationResult | null = null;
  let copySuccess = false;

  $: {
    // Generate YAML when draft changes
    try {
      yamlContent = generateYAML(draft);
      validationResult = validatePlan(yamlContent);
    } catch (error) {
      console.error('Failed to generate or validate YAML:', error);
      yamlContent = '';
      validationResult = null;
    }
  }

  function downloadYAML() {
    if (!yamlContent) return;

    const blob = new Blob([yamlContent], { type: 'text/yaml' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    const filename = draft.meta?.name?.replace(/\s+/g, '-').toLowerCase() || 'workout-plan';
    a.download = `${filename}.yaml`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }

  function copyToClipboard() {
    if (!yamlContent) return;

    navigator.clipboard.writeText(yamlContent).then(() => {
      copySuccess = true;
      setTimeout(() => {
        copySuccess = false;
      }, 2000);
    }).catch(err => {
      console.error('Failed to copy to clipboard:', err);
    });
  }

  // Parse plan for tree view
  $: plan = validationResult?.plan || null;
</script>

<div class="review-step">
  <div class="step-header">
    <h2>Review & Export</h2>
    <p class="step-description">
      Review your workout plan, validate it, and export as YAML.
    </p>
  </div>

  <!-- Validation Status -->
  <div class="validation-status">
    {#if validationResult === null}
      <div class="status-badge neutral">
        <span class="icon">—</span>
        <span>Not validated</span>
      </div>
    {:else if validationResult.valid}
      <div class="status-badge success">
        <span class="icon">✓</span>
        <span>Valid PWF Plan</span>
      </div>
    {:else}
      <div class="status-badge error">
        <span class="icon">!</span>
        <span>{validationResult.errors.length} Error{validationResult.errors.length !== 1 ? 's' : ''}</span>
      </div>
    {/if}

    {#if validationResult && validationResult.warnings.length > 0}
      <div class="status-badge warning">
        <span class="icon">⚠</span>
        <span>{validationResult.warnings.length} Warning{validationResult.warnings.length !== 1 ? 's' : ''}</span>
      </div>
    {/if}
  </div>

  <!-- Validation Errors/Warnings -->
  {#if validationResult && !validationResult.valid}
    <div class="validation-messages">
      <h3>Validation Errors</h3>
      <ul class="error-list">
        {#each validationResult.errors as error}
          <li>
            <strong>{error.path}:</strong> {error.message}
            {#if error.code}
              <code class="error-code">{error.code}</code>
            {/if}
          </li>
        {/each}
      </ul>
    </div>
  {/if}

  {#if validationResult && validationResult.warnings.length > 0}
    <div class="validation-messages warnings">
      <h3>Warnings</h3>
      <ul class="warning-list">
        {#each validationResult.warnings as warning}
          <li>
            <strong>{warning.path}:</strong> {warning.message}
            {#if warning.code}
              <code class="warning-code">{warning.code}</code>
            {/if}
          </li>
        {/each}
      </ul>
    </div>
  {/if}

  <!-- Plan Tree View -->
  {#if plan}
    <div class="tree-view-section">
      <h3>Plan Structure</h3>
      <PlanTreeView {plan} />
    </div>
  {/if}

  <!-- YAML Preview -->
  <div class="yaml-preview-section">
    <h3>YAML Preview</h3>
    <div class="yaml-preview">
      {#if yamlContent}
        <pre><code>{yamlContent}</code></pre>
      {:else}
        <div class="empty-state">
          <p>No YAML content available</p>
        </div>
      {/if}
    </div>
  </div>

  <!-- Actions -->
  <div class="review-actions">
    <button
      type="button"
      class="btn btn-secondary"
      on:click={copyToClipboard}
      disabled={!yamlContent}
    >
      {copySuccess ? 'Copied!' : 'Copy to Clipboard'}
    </button>
    <button
      type="button"
      class="btn"
      on:click={downloadYAML}
      disabled={!yamlContent}
    >
      Download YAML
    </button>
  </div>
</div>

<style>
  .review-step {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .step-header {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .step-header h2 {
    margin: 0;
    font-size: 1.75rem;
    color: var(--text-primary);
  }

  .step-description {
    margin: 0;
    color: var(--text-secondary);
    font-size: 1rem;
  }

  .validation-status {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .status-badge {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.25rem;
    border-radius: 8px;
    font-weight: 600;
    font-size: 1rem;
  }

  .status-badge.success {
    background: rgba(25, 135, 84, 0.1);
    border: 1px solid var(--success-color);
    color: var(--success-color);
  }

  .status-badge.error {
    background: rgba(220, 53, 69, 0.1);
    border: 1px solid var(--error-color);
    color: var(--error-color);
  }

  .status-badge.warning {
    background: rgba(255, 193, 7, 0.1);
    border: 1px solid var(--warning-color);
    color: var(--warning-color);
  }

  .status-badge.neutral {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    color: var(--text-secondary);
  }

  .status-badge .icon {
    font-size: 1.2rem;
  }

  .validation-messages {
    padding: 1.5rem;
    background: rgba(220, 53, 69, 0.05);
    border: 1px solid var(--error-color);
    border-radius: 8px;
  }

  .validation-messages.warnings {
    background: rgba(255, 193, 7, 0.05);
    border-color: var(--warning-color);
  }

  .validation-messages h3 {
    margin: 0 0 1rem 0;
    font-size: 1.1rem;
    color: var(--error-color);
  }

  .validation-messages.warnings h3 {
    color: var(--warning-color);
  }

  .error-list,
  .warning-list {
    margin: 0;
    padding: 0 0 0 1.5rem;
  }

  .error-list li,
  .warning-list li {
    margin-bottom: 0.75rem;
    font-size: 0.95rem;
    line-height: 1.5;
  }

  .error-list li:last-child,
  .warning-list li:last-child {
    margin-bottom: 0;
  }

  .error-code,
  .warning-code {
    display: inline-block;
    margin-left: 0.5rem;
    padding: 0.15rem 0.5rem;
    background: var(--bg-secondary);
    border-radius: 4px;
    font-size: 0.8rem;
    font-family: monospace;
  }

  .tree-view-section,
  .yaml-preview-section {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .tree-view-section h3,
  .yaml-preview-section h3 {
    margin: 0;
    font-size: 1.25rem;
    color: var(--text-primary);
  }

  .yaml-preview {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1.5rem;
    max-height: 500px;
    overflow: auto;
  }

  .yaml-preview pre {
    margin: 0;
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
    font-size: 0.9rem;
    line-height: 1.6;
    color: var(--text-primary);
  }

  .yaml-preview code {
    display: block;
    white-space: pre;
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 3rem;
  }

  .empty-state p {
    margin: 0;
    color: var(--text-secondary);
    font-style: italic;
  }

  .review-actions {
    display: flex;
    gap: 1rem;
    padding-top: 1rem;
    border-top: 1px solid var(--border-color);
  }

  .review-actions button {
    flex: 1;
  }

  .review-actions button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  @media (max-width: 768px) {
    .step-header h2 {
      font-size: 1.5rem;
    }

    .review-actions {
      flex-direction: column;
    }

    .yaml-preview {
      padding: 1rem;
      max-height: 400px;
    }
  }
</style>
