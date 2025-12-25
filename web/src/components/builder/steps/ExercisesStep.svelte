<script lang="ts">
  import ExerciseForm from '../forms/ExerciseForm.svelte';
  import { builderState } from '../../../lib/builderState';
  import { dndzone } from 'svelte-dnd-action';
  import type { DndEvent } from 'svelte-dnd-action';

  $: days = $builderState.plan.cycle.days;
  $: currentDayIndex = $builderState.currentDayIndex;
  $: currentDay = days[currentDayIndex];
  $: exercises = currentDay?.exercises.map((ex, i) => ({ ...ex, id: i })) || [];
  $: hasExercises = exercises.length > 0;

  function setDay(index: number) {
    builderState.setCurrentDay(index);
  }

  function addExercise() {
    builderState.addExercise(currentDayIndex);
  }

  function handleDndConsider(e: CustomEvent<DndEvent>) {
    // Only update local array for visual feedback during drag
    exercises = e.detail.items as any;
  }

  function handleDndFinalize(e: CustomEvent<DndEvent>) {
    // Update both local and global state when drag completes
    const reorderedExercises = e.detail.items.map(({ id, ...ex }: any) => ex);
    builderState.reorderExercises(currentDayIndex, reorderedExercises);
    // Re-sync local array with updated state to ensure consistency
    exercises = currentDay?.exercises.map((ex, i) => ({ ...ex, id: i })) || [];
  }

  $: hasValidationErrors = days.some(day => day.exercises.length === 0);
</script>

<div class="exercises-step">
  <div class="step-header">
    <h2>Exercises</h2>
    <p class="step-description">
      Add exercises to each day. Each day must have at least one exercise.
    </p>
  </div>

  {#if hasValidationErrors}
    <div class="validation-warning">
      All days must have at least one exercise
    </div>
  {/if}

  <!-- Day selector -->
  <div class="day-selector">
    <label>Select Day:</label>
    <div class="day-tabs">
      {#each days as day, index}
        <button
          type="button"
          class="day-tab"
          class:active={index === currentDayIndex}
          on:click={() => setDay(index)}
        >
          <span class="day-label">Day {index + 1}</span>
          {#if day.focus}
            <span class="day-focus">{day.focus}</span>
          {/if}
          <span class="exercise-count" class:error={day.exercises.length === 0}>
            {day.exercises.length}
          </span>
        </button>
      {/each}
    </div>
  </div>

  <!-- Current day info -->
  <div class="current-day-info">
    <h3>
      Day {currentDayIndex + 1}
      {#if currentDay?.focus}
        - {currentDay.focus}
      {/if}
    </h3>
    {#if currentDay?.notes}
      <p class="day-notes">{currentDay.notes}</p>
    {/if}
  </div>

  <!-- Exercises list -->
  {#if hasExercises}
    <div class="exercises-list"
         use:dndzone={{
           items: exercises,
           flipDurationMs: 200,
           dragDisabled: false,
           dropFromOthersDisabled: true
         }}
         on:consider={handleDndConsider}
         on:finalize={handleDndFinalize}
    >
      {#each exercises as exercise, index (exercise.id)}
        <div class="exercise-wrapper">
          <span class="drag-handle" title="Drag to reorder">⋮⋮</span>
          <ExerciseForm
            dayIndex={currentDayIndex}
            exerciseIndex={index}
          />
        </div>
      {/each}
    </div>
  {:else}
    <div class="empty-state">
      <p>No exercises added yet</p>
      <p class="hint">Click "Add Exercise" below to get started</p>
    </div>
  {/if}

  <button type="button" class="btn btn-add-exercise" on:click={addExercise}>
    + Add Exercise
  </button>
</div>

<style>
  .exercises-step {
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

  .validation-warning {
    padding: 1rem;
    background: rgba(220, 53, 69, 0.1);
    border: 1px solid var(--error-color);
    border-radius: 6px;
    color: var(--error-color);
    font-weight: 500;
  }

  .day-selector {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .day-selector label {
    font-weight: 600;
    color: var(--text-primary);
  }

  .day-tabs {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .day-tab {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
    padding: 0.75rem 1rem;
    background: var(--bg-secondary);
    border: 2px solid var(--border-color);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
    min-width: 100px;
  }

  .day-tab:hover {
    background: var(--bg-hover);
    border-color: var(--accent-color);
  }

  .day-tab.active {
    background: var(--accent-color);
    color: white;
    border-color: var(--accent-color);
  }

  .day-label {
    font-weight: 600;
    font-size: 0.9rem;
  }

  .day-focus {
    font-size: 0.8rem;
    opacity: 0.9;
  }

  .day-tab.active .day-focus {
    opacity: 1;
  }

  .exercise-count {
    font-size: 0.85rem;
    padding: 0.25rem 0.5rem;
    background: var(--bg-primary);
    border-radius: 4px;
    font-weight: 600;
  }

  .day-tab.active .exercise-count {
    background: rgba(255, 255, 255, 0.2);
    color: white;
  }

  .exercise-count.error {
    background: var(--error-color);
    color: white;
  }

  .current-day-info {
    padding: 1rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
  }

  .current-day-info h3 {
    margin: 0 0 0.5rem 0;
    font-size: 1.25rem;
    color: var(--text-primary);
  }

  .day-notes {
    margin: 0;
    color: var(--text-secondary);
    font-style: italic;
  }

  .exercises-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .exercise-wrapper {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
    cursor: grab;
  }

  .exercise-wrapper:active {
    cursor: grabbing;
  }

  .exercise-wrapper .drag-handle {
    font-size: 1.25rem;
    color: var(--text-secondary);
    cursor: grab;
    user-select: none;
    line-height: 1;
    margin-top: 1.5rem; /* Align with exercise form header */
  }

  .exercise-wrapper .drag-handle:active {
    cursor: grabbing;
  }

  .exercise-wrapper > :global(.exercise-form) {
    flex: 1;
  }

  .empty-state {
    padding: 3rem 2rem;
    text-align: center;
    background: var(--bg-secondary);
    border: 2px dashed var(--border-color);
    border-radius: 8px;
  }

  .empty-state p {
    margin: 0;
    color: var(--text-secondary);
  }

  .empty-state .hint {
    margin-top: 0.5rem;
    font-size: 0.9rem;
  }

  .btn-add-exercise {
    width: 100%;
    padding: 1rem;
    background: var(--bg-secondary);
    color: var(--text-primary);
    border: 2px dashed var(--border-color);
    border-radius: 8px;
    cursor: pointer;
    font-size: 1rem;
    font-weight: 600;
    transition: all 0.2s;
  }

  .btn-add-exercise:hover {
    background: var(--bg-hover);
    border-color: var(--accent-color);
    color: var(--accent-color);
  }

  @media (max-width: 768px) {
    .step-header h2 {
      font-size: 1.5rem;
    }

    .day-tabs {
      overflow-x: auto;
      flex-wrap: nowrap;
      padding-bottom: 0.5rem;
    }

    .day-tab {
      min-width: 90px;
      padding: 0.6rem 0.75rem;
    }
  }
</style>
