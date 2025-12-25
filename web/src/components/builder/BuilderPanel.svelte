<script lang="ts">
  import BuilderWizard from './BuilderWizard.svelte';
  import { builderState } from '../../lib/builderState';

  let selectedTemplate: 'templates' | 'scratch' | 'load-yaml' | null = null;
  let yamlInput: string = '';
  let errorMessage: string = '';

  function selectTemplate(choice: 'templates' | 'scratch' | 'load-yaml') {
    selectedTemplate = choice;
    errorMessage = '';

    if (choice === 'scratch') {
      // Start with empty plan
      builderState.reset();
      builderState.nextStep();
    } else if (choice === 'templates') {
      // Will show template selector in next phase
      errorMessage = 'Template selection coming in Phase 2';
    }
  }

  function handleYamlLoad() {
    if (!yamlInput.trim()) {
      errorMessage = 'Please enter YAML content';
      return;
    }

    try {
      // TODO: Parse YAML and populate builder state
      errorMessage = 'YAML import coming in Phase 2';
    } catch (error) {
      errorMessage = `Failed to parse YAML: ${error}`;
    }
  }

  function handleFileUpload(event: Event) {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];

    if (!file) return;

    const reader = new FileReader();
    reader.onload = (e) => {
      yamlInput = e.target?.result as string;
      handleYamlLoad();
    };
    reader.onerror = () => {
      errorMessage = 'Failed to read file';
    };
    reader.readAsText(file);
  }

  function resetSelection() {
    selectedTemplate = null;
    yamlInput = '';
    errorMessage = '';
    builderState.reset();
  }

  $: showWizard = selectedTemplate === 'scratch' && $builderState.currentStep > 0;
</script>

<div class="builder-panel">
  {#if !showWizard}
    <!-- Template Selection Screen -->
    <div class="template-selector">
      <div class="panel-intro">
        <h2>Create a New Workout Plan</h2>
        <p>Choose how you want to start building your plan</p>
      </div>

      <div class="template-options">
        <!-- Option 1: Templates -->
        <button
          class="template-card"
          class:selected={selectedTemplate === 'templates'}
          on:click={() => selectTemplate('templates')}
        >
          <div class="card-icon">📋</div>
          <h3>Use a Template</h3>
          <p>Start from a pre-built workout plan template</p>
          <span class="badge coming-soon">Coming Soon</span>
        </button>

        <!-- Option 2: Start from Scratch -->
        <button
          class="template-card"
          class:selected={selectedTemplate === 'scratch'}
          on:click={() => selectTemplate('scratch')}
        >
          <div class="card-icon">✏️</div>
          <h3>Start from Scratch</h3>
          <p>Build your custom plan step by step</p>
        </button>

        <!-- Option 3: Load YAML -->
        <button
          class="template-card"
          class:selected={selectedTemplate === 'load-yaml'}
          on:click={() => selectTemplate('load-yaml')}
        >
          <div class="card-icon">📄</div>
          <h3>Import YAML</h3>
          <p>Load an existing PWF YAML file to edit</p>
          <span class="badge coming-soon">Coming Soon</span>
        </button>
      </div>

      <!-- YAML Import Area -->
      {#if selectedTemplate === 'load-yaml'}
        <div class="yaml-import">
          <h3>Import YAML File</h3>

          <div class="file-upload">
            <label for="yaml-file" class="btn btn-secondary">
              Choose File
            </label>
            <input
              id="yaml-file"
              type="file"
              accept=".yaml,.yml"
              on:change={handleFileUpload}
              style="display: none"
            />
          </div>

          <div class="divider">
            <span>or paste YAML content</span>
          </div>

          <textarea
            bind:value={yamlInput}
            placeholder="Paste your PWF YAML content here..."
            rows="10"
          ></textarea>

          <div class="import-actions">
            <button class="btn" on:click={handleYamlLoad}>
              Load YAML
            </button>
            <button class="btn-secondary" on:click={resetSelection}>
              Cancel
            </button>
          </div>
        </div>
      {/if}

      <!-- Error Message -->
      {#if errorMessage}
        <div class="error-banner">
          <span class="error-icon">⚠️</span>
          {errorMessage}
        </div>
      {/if}
    </div>
  {:else}
    <!-- Wizard Interface -->
    <BuilderWizard on:cancel={resetSelection} />
  {/if}
</div>

<style>
  .builder-panel {
    max-width: 1400px;
    margin: 0 auto;
    padding: 2rem;
  }

  .template-selector {
    max-width: 1000px;
    margin: 0 auto;
  }

  .panel-intro {
    text-align: center;
    margin-bottom: 3rem;
  }

  .panel-intro h2 {
    margin: 0 0 0.5rem 0;
    font-size: 2rem;
    color: var(--text-primary);
  }

  .panel-intro p {
    margin: 0;
    font-size: 1.1rem;
    color: var(--text-secondary);
  }

  .template-options {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 2rem;
    margin-bottom: 2rem;
  }

  .template-card {
    position: relative;
    background: var(--bg-secondary);
    border: 2px solid var(--border-color);
    border-radius: 12px;
    padding: 2rem;
    text-align: center;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  .template-card:hover {
    border-color: var(--accent-color);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .template-card.selected {
    border-color: var(--accent-color);
    background: rgba(13, 110, 253, 0.05);
  }

  .card-icon {
    font-size: 3rem;
    margin-bottom: 0.5rem;
  }

  .template-card h3 {
    margin: 0;
    font-size: 1.3rem;
    color: var(--text-primary);
  }

  .template-card p {
    margin: 0;
    font-size: 0.95rem;
    color: var(--text-secondary);
    line-height: 1.5;
  }

  .badge {
    position: absolute;
    top: 1rem;
    right: 1rem;
    padding: 0.25rem 0.75rem;
    font-size: 0.75rem;
    font-weight: 600;
    border-radius: 12px;
    text-transform: uppercase;
  }

  .badge.coming-soon {
    background: var(--warning-color);
    color: #000;
  }

  .yaml-import {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 12px;
    padding: 2rem;
    margin-top: 2rem;
  }

  .yaml-import h3 {
    margin: 0 0 1.5rem 0;
    font-size: 1.2rem;
  }

  .file-upload {
    display: flex;
    justify-content: center;
    margin-bottom: 1rem;
  }

  .divider {
    display: flex;
    align-items: center;
    margin: 1.5rem 0;
    color: var(--text-secondary);
    font-size: 0.9rem;
  }

  .divider::before,
  .divider::after {
    content: '';
    flex: 1;
    height: 1px;
    background: var(--border-color);
  }

  .divider span {
    padding: 0 1rem;
  }

  textarea {
    width: 100%;
    min-height: 200px;
    font-family: 'Monaco', 'Menlo', monospace;
    font-size: 0.9rem;
    resize: vertical;
  }

  .import-actions {
    display: flex;
    gap: 1rem;
    justify-content: center;
    margin-top: 1.5rem;
  }

  .error-banner {
    background: rgba(220, 53, 69, 0.1);
    border: 1px solid var(--error-color);
    border-radius: 8px;
    padding: 1rem;
    margin: 1.5rem 0;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    color: var(--error-color);
  }

  .error-icon {
    font-size: 1.5rem;
  }

  @media (max-width: 768px) {
    .builder-panel {
      padding: 1rem;
    }

    .template-options {
      grid-template-columns: 1fr;
    }

    .panel-intro h2 {
      font-size: 1.5rem;
    }
  }
</style>
