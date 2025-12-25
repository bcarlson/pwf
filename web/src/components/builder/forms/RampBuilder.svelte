<script lang="ts">
  import type { RampConfig } from '../../../lib/builderState';

  export let ramp: RampConfig | undefined = undefined;
  export let onChange: (ramp: RampConfig | undefined) => void;

  function updateRamp(field: keyof RampConfig, value: number | undefined) {
    if (!ramp) return;

    ramp = {
      ...ramp,
      [field]: value,
    };
    onChange(ramp);
  }

  function removeRamp() {
    ramp = undefined;
    onChange(undefined);
  }

  // Validation
  $: isValid = ramp ?
    ramp.start_power_watts < ramp.end_power_watts &&
    ramp.duration_sec > 0 :
    true;
</script>

{#if ramp}
  <div class="ramp-builder">
    <div class="header">
      <h4>Power Ramp Configuration</h4>
      <button type="button" on:click={removeRamp} class="btn-remove">Remove Ramp</button>
    </div>

    <div class="ramp-fields">
      <div class="field">
        <label for="start-power">
          Start Power (watts)
          <span class="required">*</span>
        </label>
        <input
          id="start-power"
          type="number"
          min="0"
          value={ramp.start_power_watts}
          on:input={(e) => updateRamp('start_power_watts', parseInt(e.currentTarget.value) || 0)}
          required
        />
      </div>

      <div class="field">
        <label for="end-power">
          End Power (watts)
          <span class="required">*</span>
        </label>
        <input
          id="end-power"
          type="number"
          min="0"
          value={ramp.end_power_watts}
          on:input={(e) => updateRamp('end_power_watts', parseInt(e.currentTarget.value) || 0)}
          required
        />
      </div>

      <div class="field">
        <label for="duration">
          Duration (seconds)
          <span class="required">*</span>
        </label>
        <input
          id="duration"
          type="number"
          min="1"
          value={ramp.duration_sec}
          on:input={(e) => updateRamp('duration_sec', parseInt(e.currentTarget.value) || 0)}
          required
        />
      </div>

      <div class="field">
        <label for="step-duration">
          Step Duration (seconds)
          <span class="optional">optional</span>
        </label>
        <input
          id="step-duration"
          type="number"
          min="1"
          value={ramp.step_duration_sec || ''}
          on:input={(e) => {
            const val = e.currentTarget.value;
            updateRamp('step_duration_sec', val ? parseInt(val) : undefined);
          }}
        />
      </div>
    </div>

    {#if !isValid}
      <div class="validation-error">
        Start power must be less than end power, and duration must be greater than 0
      </div>
    {/if}
  </div>
{:else}
  <div class="ramp-builder-empty">
    <p>No ramp configuration</p>
    <button
      type="button"
      on:click={() => {
        ramp = {
          start_power_watts: 100,
          end_power_watts: 300,
          duration_sec: 600,
        };
        onChange(ramp);
      }}
      class="btn-add"
    >
      Add Power Ramp
    </button>
  </div>
{/if}

<style>
  .ramp-builder {
    border: 1px solid #e5e7eb;
    border-radius: 8px;
    padding: 1rem;
    margin: 1rem 0;
    background: #f9fafb;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  h4 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: #111827;
  }

  .ramp-fields {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }

  .field {
    display: flex;
    flex-direction: column;
  }

  label {
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
    margin-bottom: 0.25rem;
  }

  .required {
    color: #dc2626;
    margin-left: 0.25rem;
  }

  .optional {
    color: #6b7280;
    margin-left: 0.25rem;
    font-weight: 400;
  }

  input {
    padding: 0.5rem;
    border: 1px solid #d1d5db;
    border-radius: 4px;
    font-size: 0.875rem;
  }

  input:focus {
    outline: none;
    border-color: #3b82f6;
    ring: 2px;
    ring-color: #3b82f6;
  }

  .btn-remove {
    padding: 0.5rem 1rem;
    background: #ef4444;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 0.875rem;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-remove:hover {
    background: #dc2626;
  }

  .btn-add {
    padding: 0.5rem 1rem;
    background: #3b82f6;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 0.875rem;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-add:hover {
    background: #2563eb;
  }

  .ramp-builder-empty {
    text-align: center;
    padding: 2rem;
    border: 2px dashed #d1d5db;
    border-radius: 8px;
    margin: 1rem 0;
  }

  .ramp-builder-empty p {
    color: #6b7280;
    margin-bottom: 1rem;
  }

  .validation-error {
    margin-top: 1rem;
    padding: 0.75rem;
    background: #fef2f2;
    border: 1px solid #fecaca;
    border-radius: 4px;
    color: #dc2626;
    font-size: 0.875rem;
  }

  /* Dark mode support */
  :global(.dark) .ramp-builder {
    background: #1f2937;
    border-color: #374151;
  }

  :global(.dark) h4 {
    color: #f9fafb;
  }

  :global(.dark) label {
    color: #d1d5db;
  }

  :global(.dark) input {
    background: #374151;
    border-color: #4b5563;
    color: #f9fafb;
  }

  :global(.dark) .ramp-builder-empty {
    border-color: #4b5563;
  }

  :global(.dark) .ramp-builder-empty p {
    color: #9ca3af;
  }

  :global(.dark) .validation-error {
    background: #7f1d1d;
    border-color: #991b1b;
    color: #fca5a5;
  }
</style>
