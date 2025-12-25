<script lang="ts">
  import type { Exercise, Modality } from '../../../lib/builderState';
  import ZoneBuilder from './ZoneBuilder.svelte';
  import IntervalBuilder from './IntervalBuilder.svelte';
  import RampBuilder from './RampBuilder.svelte';

  export let exercise: Exercise;
  export let onUpdate: (field: keyof Exercise, value: any) => void;

  function handleNumberInput(field: keyof Exercise, value: string) {
    const num = parseFloat(value);
    onUpdate(field, isNaN(num) || value === '' ? undefined : num);
  }

  function handleStringInput(field: keyof Exercise, value: string) {
    onUpdate(field, value === '' ? undefined : value);
  }

  // Determine which fields to show based on modality
  $: isEnduranceModality = ['cycling', 'running', 'rowing', 'swimming'].includes(exercise.modality);
  $: showDistanceField = ['interval', 'running', 'rowing', 'swimming'].includes(exercise.modality);
  $: showZones = isEnduranceModality;
  $: showRamp = exercise.modality === 'cycling' || exercise.modality === 'rowing';
  $: showIntervalPhases = isEnduranceModality;
</script>

<div class="modality-fields">
  {#if exercise.modality === 'strength'}
    <!-- Strength: target_sets, target_reps, target_load -->
    <div class="field-group">
      <label for="target-sets">
        Sets <span class="required">*</span>
      </label>
      <input
        id="target-sets"
        type="number"
        min="1"
        placeholder="e.g., 3"
        value={exercise.target_sets ?? ''}
        on:input={(e) => handleNumberInput('target_sets', e.currentTarget.value)}
      />
    </div>

    <div class="field-group">
      <label for="target-reps">
        Reps <span class="required">*</span>
      </label>
      <input
        id="target-reps"
        type="number"
        min="1"
        placeholder="e.g., 10"
        value={exercise.target_reps ?? ''}
        on:input={(e) => handleNumberInput('target_reps', e.currentTarget.value)}
      />
    </div>

    <div class="field-group">
      <label for="target-load">
        Load <span class="optional">optional (e.g., "185 lbs", "RPE 8")</span>
      </label>
      <input
        id="target-load"
        type="text"
        placeholder="e.g., 185 lbs, RPE 8"
        value={exercise.target_load ?? ''}
        on:input={(e) => handleStringInput('target_load', e.currentTarget.value)}
      />
    </div>

  {:else if exercise.modality === 'countdown'}
    <!-- Countdown: target_duration_sec, target_sets (optional) -->
    <div class="field-group">
      <label for="target-duration">
        Duration (seconds) <span class="required">*</span>
      </label>
      <input
        id="target-duration"
        type="number"
        min="1"
        placeholder="e.g., 60"
        value={exercise.target_duration_sec ?? ''}
        on:input={(e) => handleNumberInput('target_duration_sec', e.currentTarget.value)}
      />
    </div>

    <div class="field-group">
      <label for="countdown-sets">
        Sets <span class="optional">optional</span>
      </label>
      <input
        id="countdown-sets"
        type="number"
        min="1"
        placeholder="e.g., 3"
        value={exercise.target_sets ?? ''}
        on:input={(e) => handleNumberInput('target_sets', e.currentTarget.value)}
      />
    </div>

  {:else if exercise.modality === 'stopwatch'}
    <!-- Stopwatch: target_duration_sec (optional) -->
    <div class="field-group">
      <label for="target-duration">
        Duration (seconds) <span class="optional">optional</span>
      </label>
      <input
        id="target-duration"
        type="number"
        min="1"
        placeholder="e.g., 300"
        value={exercise.target_duration_sec ?? ''}
        on:input={(e) => handleNumberInput('target_duration_sec', e.currentTarget.value)}
      />
    </div>

  {:else if exercise.modality === 'interval'}
    <!-- Interval: target_sets, target_duration_sec, target_distance_meters -->
    <div class="field-group">
      <label for="interval-sets">
        Sets (intervals) <span class="required">*</span>
      </label>
      <input
        id="interval-sets"
        type="number"
        min="1"
        placeholder="e.g., 8"
        value={exercise.target_sets ?? ''}
        on:input={(e) => handleNumberInput('target_sets', e.currentTarget.value)}
      />
    </div>

    <div class="field-group">
      <label for="interval-duration">
        Duration per interval (seconds) <span class="optional">optional</span>
      </label>
      <input
        id="interval-duration"
        type="number"
        min="1"
        placeholder="e.g., 30"
        value={exercise.target_duration_sec ?? ''}
        on:input={(e) => handleNumberInput('target_duration_sec', e.currentTarget.value)}
      />
    </div>

    <div class="field-group">
      <label for="interval-distance">
        Distance (meters) <span class="optional">optional</span>
      </label>
      <input
        id="interval-distance"
        type="number"
        min="0"
        step="0.01"
        placeholder="e.g., 400"
        value={exercise.target_distance_meters ?? ''}
        on:input={(e) => handleNumberInput('target_distance_meters', e.currentTarget.value)}
      />
    </div>

  {:else if isEnduranceModality}
    <!-- Endurance modalities: cycling, running, rowing, swimming -->
    <div class="endurance-section">
      <h4>{exercise.modality.charAt(0).toUpperCase() + exercise.modality.slice(1)} Configuration</h4>

      {#if showDistanceField}
        <div class="field-group">
          <label for="endurance-distance">
            Target Distance (meters) <span class="optional">optional</span>
          </label>
          <input
            id="endurance-distance"
            type="number"
            min="0"
            step="0.01"
            placeholder="e.g., 5000"
            value={exercise.target_distance_meters ?? ''}
            on:input={(e) => handleNumberInput('target_distance_meters', e.currentTarget.value)}
          />
        </div>
      {/if}

      {#if showZones}
        <ZoneBuilder
          zones={exercise.zones || []}
          modality={exercise.modality}
          onChange={(zones) => onUpdate('zones', zones.length > 0 ? zones : undefined)}
        />
      {/if}

      {#if showRamp}
        <RampBuilder
          ramp={exercise.ramp}
          onChange={(ramp) => onUpdate('ramp', ramp)}
        />
      {/if}

      {#if showIntervalPhases}
        <IntervalBuilder
          interval_phases={exercise.interval_phases || []}
          modality={exercise.modality}
          onChange={(phases) => onUpdate('interval_phases', phases.length > 0 ? phases : undefined)}
        />
      {/if}
    </div>

  {:else}
    <p class="info-text">No additional fields required for this modality.</p>
  {/if}
</div>

<style>
  .modality-fields {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    padding: 1rem;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 6px;
  }

  .field-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  label {
    font-size: 0.9rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .required {
    color: var(--error-color);
    font-weight: bold;
  }

  .optional {
    font-size: 0.8rem;
    font-weight: 400;
    color: var(--text-secondary);
    font-style: italic;
  }

  input {
    font-size: 0.95rem;
    padding: 0.5rem;
    border: 1px solid var(--border-color);
    border-radius: 4px;
  }

  input:focus {
    outline: none;
    border-color: var(--primary-color, #3b82f6);
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
  }

  .endurance-section {
    grid-column: 1 / -1;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .endurance-section h4 {
    margin: 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--text-primary);
    border-bottom: 2px solid var(--border-color);
    padding-bottom: 0.5rem;
  }

  .info-text {
    grid-column: 1 / -1;
    color: var(--text-secondary);
    font-style: italic;
    margin: 0;
  }

  @media (max-width: 768px) {
    .modality-fields {
      grid-template-columns: 1fr;
    }
  }
</style>
