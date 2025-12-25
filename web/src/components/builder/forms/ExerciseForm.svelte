<script lang="ts">
  import { builderState, type Exercise } from '../../../lib/builderState';
  import { getSupportedModalities, getSupportedEquipment } from '../../../lib/wasm';
  import ModalityFields from './ModalityFields.svelte';

  export let dayIndex: number;
  export let exerciseIndex: number;

  let supportedModalities: string[] = [];
  let supportedEquipment: string[] = [];

  $: exercise = $builderState.plan.cycle.days[dayIndex]?.exercises[exerciseIndex];

  // Load supported options from WASM
  try {
    supportedModalities = getSupportedModalities();
    supportedEquipment = getSupportedEquipment();
  } catch (e) {
    console.warn('Could not load options from WASM:', e);
    supportedModalities = ['strength', 'countdown', 'stopwatch', 'interval', 'distance', 'continuous', 'pace', 'speed'];
    supportedEquipment = ['barbell', 'dumbbell', 'kettlebell', 'resistance_band', 'pull_up_bar'];
  }

  function updateExercise(field: keyof Exercise, value: any) {
    builderState.updateExercise(dayIndex, exerciseIndex, { [field]: value });
  }

  function toggleEquipment(equipment: string) {
    const currentEquipment = exercise.equipment || [];
    if (currentEquipment.includes(equipment)) {
      updateExercise('equipment', currentEquipment.filter(e => e !== equipment));
    } else {
      updateExercise('equipment', [...currentEquipment, equipment]);
    }
  }

  function deleteExercise() {
    builderState.removeExercise(dayIndex, exerciseIndex);
  }

  function handleModalityChange(modality: string) {
    // Clear modality-specific fields when modality changes
    updateExercise('modality', modality);
    updateExercise('target_sets', undefined);
    updateExercise('target_reps', undefined);
    updateExercise('target_duration_sec', undefined);
    updateExercise('target_load_kg', undefined);
    updateExercise('target_distance_meters', undefined);
    updateExercise('target_pace_min_per_km', undefined);
    updateExercise('target_speed_km_per_h', undefined);
    updateExercise('target_cadence_rpm', undefined);
    updateExercise('target_power_watts', undefined);
    updateExercise('target_heart_rate_bpm', undefined);
  }
</script>

<div class="exercise-form">
  <div class="form-header">
    <h4>Exercise {exerciseIndex + 1}</h4>
    <button
      type="button"
      class="btn-delete"
      on:click={deleteExercise}
      aria-label="Delete exercise"
    >
      Delete
    </button>
  </div>

  <div class="form-group">
    <label for="exercise-name-{dayIndex}-{exerciseIndex}">
      Exercise Name <span class="required">*</span>
    </label>
    <input
      id="exercise-name-{dayIndex}-{exerciseIndex}"
      type="text"
      placeholder="e.g., Barbell Squat"
      value={exercise?.name || ''}
      on:input={(e) => updateExercise('name', e.currentTarget.value)}
      required
    />
  </div>

  <div class="form-group">
    <label for="exercise-modality-{dayIndex}-{exerciseIndex}">
      Modality <span class="required">*</span>
    </label>
    <select
      id="exercise-modality-{dayIndex}-{exerciseIndex}"
      value={exercise?.modality || 'strength'}
      on:change={(e) => handleModalityChange(e.currentTarget.value)}
    >
      {#each supportedModalities as modality}
        <option value={modality}>
          {modality.charAt(0).toUpperCase() + modality.slice(1)}
        </option>
      {/each}
    </select>
  </div>

  {#if exercise}
    <ModalityFields
      {exercise}
      onUpdate={updateExercise}
    />
  {/if}

  <div class="form-group">
    <label>
      Equipment
      <span class="optional">optional</span>
    </label>
    <div class="equipment-grid">
      {#each supportedEquipment as equipment}
        <button
          type="button"
          class="equipment-chip"
          class:selected={exercise?.equipment?.includes(equipment)}
          on:click={() => toggleEquipment(equipment)}
        >
          {equipment.replace(/_/g, ' ')}
        </button>
      {/each}
    </div>
  </div>

  <div class="form-group">
    <label for="exercise-notes-{dayIndex}-{exerciseIndex}">
      Notes
      <span class="optional">optional</span>
    </label>
    <textarea
      id="exercise-notes-{dayIndex}-{exerciseIndex}"
      rows="2"
      placeholder="Add any notes for this exercise..."
      value={exercise?.notes || ''}
      on:input={(e) => updateExercise('notes', e.currentTarget.value || undefined)}
    />
  </div>
</div>

<style>
  .exercise-form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
  }

  .form-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid var(--border-color);
  }

  .form-header h4 {
    margin: 0;
    font-size: 1.1rem;
    color: var(--text-primary);
  }

  .btn-delete {
    padding: 0.5rem 1rem;
    background: var(--error-color);
    color: white;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.9rem;
    font-weight: 500;
    transition: all 0.2s;
  }

  .btn-delete:hover {
    background: #c82333;
    transform: translateY(-1px);
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  label {
    font-weight: 600;
    color: var(--text-primary);
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.9rem;
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

  .equipment-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 0.5rem;
  }

  .equipment-chip {
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    background: var(--bg-primary);
    color: var(--text-primary);
    cursor: pointer;
    transition: all 0.2s;
    text-transform: capitalize;
    font-size: 0.85rem;
  }

  .equipment-chip:hover {
    background: var(--bg-hover);
    border-color: var(--accent-color);
  }

  .equipment-chip.selected {
    background: var(--accent-color);
    color: white;
    border-color: var(--accent-color);
  }

  @media (max-width: 768px) {
    .exercise-form {
      padding: 0.75rem;
    }

    .equipment-grid {
      grid-template-columns: repeat(auto-fill, minmax(110px, 1fr));
    }

    .form-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.5rem;
    }

    .btn-delete {
      width: 100%;
    }
  }
</style>
