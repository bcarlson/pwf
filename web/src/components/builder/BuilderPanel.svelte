<script lang="ts">
  import { onMount } from 'svelte';
  import BuilderWizard from './BuilderWizard.svelte';
  import { builderState } from '../../lib/builderState';
  import { parse as parseYAML } from 'yaml';
  import { convertToDraft } from './utils/yamlImport';
  import { getAllTemplates, type WorkoutTemplate } from '../../lib/workoutTemplates';
  import { getPlanFromCurrentUrl } from '../../lib/shareUtils';
  import { getCustomTemplates, deleteCustomTemplate, type CustomTemplate } from '../../lib/customTemplates';

  let selectedTemplate: 'templates' | 'scratch' | 'load-yaml' | null = null;
  let yamlInput: string = '';
  let errorMessage: string = '';
  let templates = getAllTemplates();
  let customTemplates: CustomTemplate[] = [];

  // Load plan from URL if present and load custom templates
  onMount(() => {
    console.log('[BuilderPanel] Component mounted');
    customTemplates = getCustomTemplates();

    const sharedPlan = getPlanFromCurrentUrl();
    console.log('[BuilderPanel] Shared plan from URL:', sharedPlan ? 'FOUND ✓' : 'NOT FOUND');
    if (sharedPlan) {
      console.log('[BuilderPanel] Loading shared plan:', sharedPlan.meta?.name || 'Unnamed');
      builderState.loadPlan(sharedPlan);
      builderState.nextStep(); // Move to first step
      selectedTemplate = 'scratch'; // Show wizard
      console.log('[BuilderPanel] Wizard should now be visible');
    }
  });

  function selectTemplate(choice: 'templates' | 'scratch' | 'load-yaml') {
    selectedTemplate = choice;
    errorMessage = '';

    if (choice === 'scratch') {
      // Start with empty plan
      builderState.reset();
      builderState.nextStep();
    }
  }

  function loadTemplate(template: WorkoutTemplate) {
    builderState.loadPlan(template.plan);
    builderState.nextStep();
    selectedTemplate = 'scratch'; // Switch to wizard mode
  }

  function loadCustomTemplate(template: CustomTemplate) {
    builderState.loadPlan(template.plan);
    builderState.nextStep();
    selectedTemplate = 'scratch'; // Switch to wizard mode
  }

  function handleDeleteCustomTemplate(id: string) {
    if (confirm('Are you sure you want to delete this template?')) {
      try {
        deleteCustomTemplate(id);
        customTemplates = getCustomTemplates(); // Refresh list
      } catch (error) {
        console.error('Failed to delete template:', error);
        alert('Failed to delete template.');
      }
    }
  }

  function handleYamlLoad() {
    if (!yamlInput.trim()) {
      errorMessage = 'Please enter YAML content';
      return;
    }

    try {
      // Parse YAML and convert to draft format
      const parsed = parseYAML(yamlInput);
      const draft = convertToDraft(parsed);

      // Load into builder state
      builderState.loadPlan(draft);
      builderState.nextStep(); // Move to first step

      // Clear input and switch to wizard
      selectedTemplate = 'scratch';
      yamlInput = '';
      errorMessage = '';
    } catch (error) {
      errorMessage = `Failed to parse YAML: ${error instanceof Error ? error.message : String(error)}`;
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
    customTemplates = getCustomTemplates(); // Refresh custom templates
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
        </button>
      </div>

      <!-- Template Gallery -->
      {#if selectedTemplate === 'templates'}
        <div class="template-gallery">
          <!-- Custom Templates Section -->
          {#if customTemplates.length > 0}
            <div class="custom-templates-section">
              <h3>My Custom Templates</h3>
              <div class="template-grid">
                {#each customTemplates as template}
                  <div class="custom-template-wrapper">
                    <button
                      class="workout-template custom-template"
                      on:click={() => loadCustomTemplate(template)}
                    >
                      <div class="template-header">
                        <h4>{template.name}</h4>
                        <span class="badge badge-custom">Custom</span>
                      </div>
                      <p class="template-description">{template.description || 'No description'}</p>
                      <div class="template-meta">
                        <span>📅 {template.plan.meta?.days_per_week || 'N/A'} days/week</span>
                        <span>🏋️ {template.plan.cycle.days.length} workouts</span>
                      </div>
                    </button>
                    <button
                      class="btn-delete-template"
                      on:click|stopPropagation={() => handleDeleteCustomTemplate(template.id)}
                      title="Delete template"
                    >
                      🗑️
                    </button>
                  </div>
                {/each}
              </div>
            </div>
          {/if}

          <!-- Built-in Templates Section -->
          <h3>{customTemplates.length > 0 ? 'Built-in Templates' : 'Choose a Template'}</h3>
          <div class="template-grid">
            {#each templates as template}
              <button
                class="workout-template"
                on:click={() => loadTemplate(template)}
              >
                <div class="template-header">
                  <h4>{template.name}</h4>
                  <div class="template-badges">
                    <span class="badge badge-{template.category}">{template.category}</span>
                    <span class="badge badge-{template.difficulty}">{template.difficulty}</span>
                  </div>
                </div>
                <p class="template-description">{template.description}</p>
                <div class="template-meta">
                  <span>📅 {template.plan.meta?.days_per_week || 3} days/week</span>
                  <span>🏋️ {template.plan.cycle.days.length} workouts</span>
                </div>
              </button>
            {/each}
          </div>
          <div class="template-actions">
            <button class="btn-secondary" on:click={resetSelection}>
              Back
            </button>
          </div>
        </div>
      {/if}

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

  .template-gallery {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 12px;
    padding: 2rem;
    margin-top: 2rem;
  }

  .template-gallery h3 {
    margin: 0 0 1.5rem 0;
    font-size: 1.5rem;
    text-align: center;
  }

  .template-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1.5rem;
    margin-bottom: 2rem;
  }

  .workout-template {
    background: var(--bg-primary);
    border: 2px solid var(--border-color);
    border-radius: 10px;
    padding: 1.5rem;
    text-align: left;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .workout-template:hover {
    border-color: var(--accent-color);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  .template-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 1rem;
  }

  .template-header h4 {
    margin: 0;
    font-size: 1.2rem;
    color: var(--text-primary);
  }

  .template-badges {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    flex-shrink: 0;
  }

  .template-description {
    margin: 0;
    font-size: 0.95rem;
    line-height: 1.5;
    color: var(--text-secondary);
  }

  .template-meta {
    display: flex;
    gap: 1rem;
    font-size: 0.9rem;
    color: var(--text-secondary);
    padding-top: 0.5rem;
    border-top: 1px solid var(--border-color);
  }

  .badge-strength {
    background: rgba(220, 53, 69, 0.1);
    border: 1px solid var(--error-color);
    color: var(--error-color);
  }

  .badge-cardio {
    background: rgba(13, 110, 253, 0.1);
    border: 1px solid var(--accent-color);
    color: var(--accent-color);
  }

  .badge-hybrid {
    background: rgba(111, 66, 193, 0.1);
    border: 1px solid #6f42c1;
    color: #6f42c1;
  }

  .badge-beginner {
    background: rgba(25, 135, 84, 0.1);
    border: 1px solid var(--success-color);
    color: var(--success-color);
  }

  .badge-intermediate {
    background: rgba(255, 193, 7, 0.1);
    border: 1px solid var(--warning-color);
    color: #856404;
  }

  .badge-advanced {
    background: rgba(220, 53, 69, 0.1);
    border: 1px solid var(--error-color);
    color: var(--error-color);
  }

  .template-actions {
    display: flex;
    justify-content: center;
    padding-top: 1rem;
    border-top: 1px solid var(--border-color);
  }

  /* Custom Templates Styles */
  .custom-templates-section {
    margin-bottom: 3rem;
    padding-bottom: 2rem;
    border-bottom: 2px solid var(--border-color);
  }

  .custom-templates-section h3 {
    margin-bottom: 1.5rem;
  }

  .custom-template-wrapper {
    position: relative;
  }

  .custom-template {
    border: 2px solid var(--success-color);
  }

  .custom-template:hover {
    border-color: var(--success-color);
  }

  .btn-delete-template {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    background: var(--error-color);
    color: white;
    border: none;
    border-radius: 6px;
    width: 2.5rem;
    height: 2.5rem;
    font-size: 1.2rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    transition: all 0.2s;
    z-index: 10;
  }

  .custom-template-wrapper:hover .btn-delete-template {
    opacity: 1;
  }

  .btn-delete-template:hover {
    background: #c82333;
    transform: scale(1.1);
  }

  .badge-custom {
    background: rgba(25, 135, 84, 0.1);
    border: 1px solid var(--success-color);
    color: var(--success-color);
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

    .template-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
