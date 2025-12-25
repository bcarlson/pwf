<script lang="ts">
  import MetaForm from '../forms/MetaForm.svelte';
  import { builderState } from '../../../lib/builderState';

  function skipToDays() {
    // Go to step 1 (Days step)
    builderState.goToStep(1);
  }
</script>

<div class="meta-step">
  <div class="step-header">
    <h2>Plan Metadata</h2>
    <p class="step-description">
      Add optional information about your workout plan. You can skip this and jump straight to defining your workout days.
    </p>
  </div>

  <MetaForm />

  <div class="step-actions">
    <button type="button" class="btn btn-secondary" on:click={skipToDays}>
      Skip to Days
    </button>
  </div>
</div>

<style>
  .meta-step {
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

  .step-actions {
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
    padding-top: 1rem;
    border-top: 1px solid var(--border-color);
  }

  @media (max-width: 768px) {
    .step-header h2 {
      font-size: 1.5rem;
    }

    .step-actions {
      flex-direction: column;
    }
  }
</style>
