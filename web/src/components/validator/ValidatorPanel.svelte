<script lang="ts">
  import FileUpload from './FileUpload.svelte';
  import YamlEditor from './YamlEditor.svelte';
  import ValidationResults from './ValidationResults.svelte';
  import ExampleGallery from '../shared/ExampleGallery.svelte';
  import { validatePlan, validateHistory } from '../../lib/wasm';
  import type { ValidationResult } from '../../lib/stores';

  let yamlContent: string = '';
  let currentFile: File | null = null;
  let validationResult: ValidationResult | null = null;
  let validationType: 'plan' | 'history' = 'plan';
  let errorMessage: string = '';
  let isValidating: boolean = false;
  let editorComponent: any;

  function handleFileLoaded(event: CustomEvent) {
    const { file, content } = event.detail;
    currentFile = file;
    yamlContent = content;
    errorMessage = '';

    // Auto-detect type from filename
    if (file.name.includes('history')) {
      validationType = 'history';
    } else {
      validationType = 'plan';
    }

    // Auto-validate
    performValidation();
  }

  function handleEditorChange(event: CustomEvent) {
    yamlContent = event.detail.value;
    validationResult = null; // Clear previous results
  }

  function handleExampleLoaded(event: CustomEvent) {
    const { example, content } = event.detail;
    yamlContent = content;
    validationType = example.type;
    currentFile = null;
    errorMessage = '';

    // Auto-validate
    performValidation();
  }

  function handleError(event: CustomEvent) {
    errorMessage = event.detail.message;
  }

  function performValidation() {
    if (!yamlContent.trim()) {
      errorMessage = 'Please enter or upload YAML content';
      return;
    }

    isValidating = true;
    errorMessage = '';

    try {
      if (validationType === 'plan') {
        validationResult = validatePlan(yamlContent);
      } else {
        validationResult = validateHistory(yamlContent);
      }
    } catch (error) {
      console.error('Validation error:', error);
      errorMessage = `Validation failed: ${error}`;
      validationResult = null;
    } finally {
      isValidating = false;
    }
  }

  function handleErrorClick(path: string) {
    // Extract line number from path if possible
    // For now, just log it
    console.log('Jump to:', path);
    // TODO: Parse path and jump to line in editor
  }

  function clearAll() {
    yamlContent = '';
    validationResult = null;
    currentFile = null;
    errorMessage = '';
  }
</script>

<div class="validator-panel">
  <!-- Type Selection -->
  <div class="type-selector">
    <label>
      <input
        type="radio"
        value="plan"
        bind:group={validationType}
        on:change={() => validationResult = null}
      />
      Plan
    </label>
    <label>
      <input
        type="radio"
        value="history"
        bind:group={validationType}
        on:change={() => validationResult = null}
      />
      History
    </label>
  </div>

  <!-- File Upload -->
  {#if !yamlContent}
    <FileUpload
      on:fileLoaded={handleFileLoaded}
      on:error={handleError}
    />

    <ExampleGallery
      on:exampleLoaded={handleExampleLoaded}
      on:error={handleError}
    />
  {:else}
    <!-- Editor -->
    <div class="editor-section">
      <div class="editor-header">
        <h3>
          {#if currentFile}
            {currentFile.name}
          {:else}
            Example File
          {/if}
        </h3>
        <button class="btn-secondary" on:click={clearAll}>
          Clear & Upload New
        </button>
      </div>

      <YamlEditor
        bind:this={editorComponent}
        value={yamlContent}
        on:change={handleEditorChange}
        on:validate={performValidation}
      />

      <div class="editor-actions">
        <button
          class="btn"
          on:click={performValidation}
          disabled={isValidating || !yamlContent.trim()}
        >
          {#if isValidating}
            Validating...
          {:else}
            Validate YAML
          {/if}
        </button>
      </div>
    </div>

    <!-- Error Message -->
    {#if errorMessage}
      <div class="error-banner">
        <span class="error-icon">⚠️</span>
        {errorMessage}
      </div>
    {/if}

    <!-- Validation Results -->
    {#if validationResult}
      <ValidationResults
        result={validationResult}
        onErrorClick={handleErrorClick}
      />
    {/if}
  {/if}
</div>

<style>
  .validator-panel {
    max-width: 1000px;
    margin: 0 auto;
  }

  .type-selector {
    display: flex;
    gap: 2rem;
    margin-bottom: 2rem;
    padding: 1rem;
    background: var(--bg-secondary);
    border-radius: 8px;
  }

  .type-selector label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    font-weight: 500;
  }

  .type-selector input[type="radio"] {
    width: auto;
    cursor: pointer;
  }

  .editor-section {
    margin: 2rem 0;
  }

  .editor-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .editor-header h3 {
    margin: 0;
    font-size: 1.2rem;
  }

  .editor-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
    margin-top: 1rem;
  }

  .error-banner {
    background: rgba(220, 53, 69, 0.1);
    border: 1px solid var(--error-color);
    border-radius: 8px;
    padding: 1rem;
    margin: 1rem 0;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    color: var(--error-color);
  }

  .error-icon {
    font-size: 1.5rem;
  }

  @media (max-width: 768px) {
    .editor-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 1rem;
    }

    .type-selector {
      flex-direction: column;
      gap: 1rem;
    }
  }
</style>
