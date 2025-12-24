<script lang="ts">
  import type { ValidationResult } from '../../lib/stores';

  export let result: ValidationResult;
  export let onErrorClick: ((path: string) => void) | null = null;

  function handleErrorClick(issue: any) {
    if (onErrorClick && issue.path) {
      onErrorClick(issue.path);
    }
  }

  function getSeverityColor(severity: 'error' | 'warning'): string {
    return severity === 'error' ? 'error' : 'warning';
  }

  function downloadYAML() {
    if (!result.plan && !result.history) return;

    const content = JSON.stringify(result.plan || result.history, null, 2);
    const blob = new Blob([content], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'validated-pwf.json';
    a.click();
    URL.revokeObjectURL(url);
  }
</script>

<div class="validation-results">
  <!-- Success/Error Header -->
  <div class="result-header" class:valid={result.valid} class:invalid={!result.valid}>
    <div class="status-icon">
      {result.valid ? '✓' : '✗'}
    </div>
    <div class="status-text">
      <h3>{result.valid ? 'Valid PWF File' : 'Validation Failed'}</h3>
      <p>
        {#if result.valid}
          No errors found. Your PWF file is valid!
        {:else}
          {result.errors.length} error{result.errors.length !== 1 ? 's' : ''} found
        {/if}
        {#if result.warnings.length > 0}
          · {result.warnings.length} warning{result.warnings.length !== 1 ? 's' : ''}
        {/if}
      </p>
    </div>
  </div>

  <!-- Errors List -->
  {#if result.errors.length > 0}
    <div class="issues-section">
      <h4 class="section-title text-error">Errors ({result.errors.length})</h4>
      <div class="issues-list">
        {#each result.errors as error}
          <button
            class="issue-item error"
            on:click={() => handleErrorClick(error)}
            type="button"
          >
            <div class="issue-severity">
              <span class="severity-badge error">ERROR</span>
              {#if error.code}
                <span class="error-code">{error.code}</span>
              {/if}
            </div>
            <div class="issue-content">
              <p class="issue-message">{error.message}</p>
              {#if error.path}
                <p class="issue-path">{error.path}</p>
              {/if}
            </div>
          </button>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Warnings List -->
  {#if result.warnings.length > 0}
    <div class="issues-section">
      <h4 class="section-title text-warning">Warnings ({result.warnings.length})</h4>
      <div class="issues-list">
        {#each result.warnings as warning}
          <button
            class="issue-item warning"
            on:click={() => handleErrorClick(warning)}
            type="button"
          >
            <div class="issue-severity">
              <span class="severity-badge warning">WARNING</span>
              {#if warning.code}
                <span class="error-code">{warning.code}</span>
              {/if}
            </div>
            <div class="issue-content">
              <p class="issue-message">{warning.message}</p>
              {#if warning.path}
                <p class="issue-path">{warning.path}</p>
              {/if}
            </div>
          </button>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Statistics -->
  {#if result.statistics}
    <div class="statistics">
      <h4 class="section-title">Statistics</h4>
      <pre>{JSON.stringify(result.statistics, null, 2)}</pre>
    </div>
  {/if}

  <!-- Actions -->
  {#if result.valid && (result.plan || result.history)}
    <div class="actions">
      <button class="btn" on:click={downloadYAML}>
        Download Validated File
      </button>
    </div>
  {/if}
</div>

<style>
  .validation-results {
    margin-top: 1rem;
  }

  .result-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1.5rem;
    border-radius: 8px;
    margin-bottom: 1.5rem;
  }

  .result-header.valid {
    background: rgba(25, 135, 84, 0.1);
    border: 1px solid var(--success-color);
  }

  .result-header.invalid {
    background: rgba(220, 53, 69, 0.1);
    border: 1px solid var(--error-color);
  }

  .status-icon {
    font-size: 2.5rem;
    flex-shrink: 0;
  }

  .result-header.valid .status-icon {
    color: var(--success-color);
  }

  .result-header.invalid .status-icon {
    color: var(--error-color);
  }

  .status-text h3 {
    margin: 0 0 0.25rem 0;
    font-size: 1.3rem;
  }

  .status-text p {
    margin: 0;
    opacity: 0.8;
  }

  .issues-section {
    margin-bottom: 1.5rem;
  }

  .section-title {
    font-size: 1.1rem;
    font-weight: 600;
    margin-bottom: 0.75rem;
  }

  .issues-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .issue-item {
    /* Reset button styles */
    width: 100%;
    text-align: left;
    font: inherit;
    color: inherit;

    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-left: 4px solid;
    border-radius: 6px;
    padding: 1rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .issue-item:hover {
    transform: translateX(4px);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .issue-item.error {
    border-left-color: var(--error-color);
  }

  .issue-item.warning {
    border-left-color: var(--warning-color);
  }

  .issue-severity {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .severity-badge {
    display: inline-block;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .severity-badge.error {
    background: var(--error-color);
    color: white;
  }

  .severity-badge.warning {
    background: var(--warning-color);
    color: black;
  }

  .error-code {
    font-family: monospace;
    font-size: 0.85rem;
    opacity: 0.7;
  }

  .issue-message {
    margin: 0;
    font-weight: 500;
  }

  .issue-path {
    margin: 0.25rem 0 0 0;
    font-family: monospace;
    font-size: 0.85rem;
    opacity: 0.7;
  }

  .statistics {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1rem;
    margin-bottom: 1.5rem;
  }

  .statistics pre {
    margin: 0.5rem 0 0 0;
    padding: 0.75rem;
    background: var(--bg-primary);
    border-radius: 4px;
    overflow-x: auto;
    font-size: 0.85rem;
  }

  .actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
  }

  @media (max-width: 768px) {
    .result-header {
      flex-direction: column;
      text-align: center;
    }

    .issue-item {
      padding: 0.75rem;
    }
  }
</style>
