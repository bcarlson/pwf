<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { builderState } from '../../lib/builderState';
  import ProgressIndicator from './shared/ProgressIndicator.svelte';
  import LivePreview from './shared/LivePreview.svelte';
  import MetaStep from './steps/MetaStep.svelte';
  import DaysStep from './steps/DaysStep.svelte';
  import ExercisesStep from './steps/ExercisesStep.svelte';
  import ReviewStep from './steps/ReviewStep.svelte';

  const dispatch = createEventDispatcher();

  // Define wizard steps
  const steps = [
    { label: 'Plan Info', valid: false },
    { label: 'Days', valid: false },
    { label: 'Exercises', valid: false },
    { label: 'Review', valid: false },
  ];

  $: currentStep = $builderState.currentStep;

  // Update step validation status based on builder state
  $: {
    steps[0].valid = !!($builderState.plan.meta?.name && $builderState.plan.meta.name.length > 0);
    steps[1].valid = $builderState.plan.cycle.days.length > 0;
    steps[2].valid = $builderState.plan.cycle.days.some(day => day.exercises.length > 0);
    steps[3].valid = steps[0].valid && steps[1].valid && steps[2].valid;
  }

  function handleNext() {
    if (currentStep < steps.length - 1) {
      builderState.nextStep();
    }
  }

  function handleBack() {
    if (currentStep > 0) {
      builderState.prevStep();
    }
  }

  function handleCancel() {
    if (confirm('Are you sure you want to cancel? All progress will be lost.')) {
      builderState.reset();
      dispatch('cancel');
    }
  }

  function canProceed(): boolean {
    // Can always go back or stay on current step
    // Check if current step is valid before allowing next
    switch (currentStep) {
      case 0: // Meta step
        return steps[0].valid;
      case 1: // Days step
        return steps[1].valid;
      case 2: // Exercises step
        return steps[2].valid;
      default:
        return true;
    }
  }
</script>

<div class="builder-wizard">
  <!-- Progress Indicator -->
  <div class="wizard-progress">
    <ProgressIndicator {currentStep} {steps} />
  </div>

  <!-- Main Content Area: Split Layout -->
  <div class="wizard-content">
    <!-- Left Side: Step Content (60%) -->
    <div class="wizard-step">
      <div class="step-container">
        {#if currentStep === 0}
          <MetaStep />
        {:else if currentStep === 1}
          <DaysStep />
        {:else if currentStep === 2}
          <ExercisesStep />
        {:else if currentStep === 3}
          <ReviewStep />
        {/if}
      </div>

      <!-- Navigation Buttons -->
      <div class="wizard-navigation">
        <div class="nav-left">
          <button
            type="button"
            class="btn-secondary"
            on:click={handleCancel}
          >
            Cancel
          </button>
        </div>

        <div class="nav-right">
          {#if currentStep > 0}
            <button
              type="button"
              class="btn-secondary"
              on:click={handleBack}
            >
              Back
            </button>
          {/if}

          {#if currentStep < steps.length - 1}
            <button
              type="button"
              class="btn"
              on:click={handleNext}
              disabled={!canProceed()}
            >
              Next
            </button>
          {/if}
        </div>
      </div>
    </div>

    <!-- Right Side: Live Preview (40%) -->
    <div class="wizard-preview">
      <LivePreview draft={$builderState.plan} />
    </div>
  </div>
</div>

<style>
  .builder-wizard {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .wizard-progress {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1.5rem;
  }

  .wizard-content {
    display: grid;
    grid-template-columns: 60% 40%;
    gap: 2rem;
    flex: 1;
    min-height: 0;
  }

  .wizard-step {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    overflow: hidden;
  }

  .step-container {
    flex: 1;
    overflow-y: auto;
    padding: 2rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
  }

  .wizard-navigation {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
  }

  .nav-left,
  .nav-right {
    display: flex;
    gap: 1rem;
  }

  .wizard-preview {
    position: sticky;
    top: 0;
    height: fit-content;
    max-height: calc(100vh - 12rem);
    overflow: hidden;
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  button:disabled:hover {
    transform: none;
    box-shadow: none;
  }

  @media (max-width: 1200px) {
    .wizard-content {
      grid-template-columns: 1fr;
    }

    .wizard-preview {
      position: static;
      max-height: 500px;
    }
  }

  @media (max-width: 768px) {
    .builder-wizard {
      gap: 1rem;
    }

    .wizard-progress {
      padding: 1rem;
    }

    .wizard-content {
      gap: 1rem;
    }

    .step-container {
      padding: 1rem;
    }

    .wizard-navigation {
      flex-direction: column;
      gap: 1rem;
      padding: 1rem;
    }

    .nav-left,
    .nav-right {
      width: 100%;
      flex-direction: column;
    }

    .nav-left button,
    .nav-right button {
      width: 100%;
    }
  }
</style>
