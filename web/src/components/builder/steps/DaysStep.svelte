<script lang="ts">
  import DayForm from '../forms/DayForm.svelte';
  import { builderState } from '../../../lib/builderState';
  import { dndzone } from 'svelte-dnd-action';
  import type { DndEvent } from 'svelte-dnd-action';

  $: days = $builderState.plan.cycle.days.map((day, index) => ({ ...day, id: index }));
  $: hasValidDays = days.length >= 1;

  function addDay() {
    builderState.addDay();
  }

  function removeDay(index: number) {
    if (days.length <= 1) {
      return; // Prevent removing the last day
    }
    builderState.removeDay(index);
  }

  function handleDndConsider(e: CustomEvent<DndEvent>) {
    days = e.detail.items as any;
  }

  function handleDndFinalize(e: CustomEvent<DndEvent>) {
    const reorderedDays = e.detail.items.map(({ id, ...day }: any) => day);
    builderState.reorderDays(reorderedDays);
  }
</script>

<div class="days-step">
  <div class="step-header">
    <h2>Workout Days</h2>
    <p class="step-description">
      Define the days in your workout plan. Each day will contain exercises.
    </p>
  </div>

  {#if !hasValidDays}
    <div class="validation-warning">
      At least one day is required
    </div>
  {/if}

  <div class="days-list"
       use:dndzone={{ items: days, flipDurationMs: 200 }}
       on:consider={handleDndConsider}
       on:finalize={handleDndFinalize}
  >
    {#each days as day, index (day.id)}
      <div class="day-card">
        <div class="day-card-header">
          <div class="day-header-left">
            <span class="drag-handle" title="Drag to reorder">⋮⋮</span>
            <h3>Day {index + 1}</h3>
          </div>
          {#if days.length > 1}
            <button
              type="button"
              class="btn-remove-day"
              on:click={() => removeDay(index)}
              aria-label="Remove day"
            >
              Remove
            </button>
          {/if}
        </div>

        <DayForm dayIndex={index} />

        <div class="day-meta">
          <span class="day-info">
            {day.exercises.length} exercise{day.exercises.length !== 1 ? 's' : ''}
          </span>
          {#if day.focus}
            <span class="day-focus-badge">{day.focus}</span>
          {/if}
        </div>
      </div>
    {/each}
  </div>

  <button type="button" class="btn btn-add-day" on:click={addDay}>
    + Add Day
  </button>
</div>

<style>
  .days-step {
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

  .days-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .day-card {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    cursor: grab;
    transition: all 0.2s;
  }

  .day-card:active {
    cursor: grabbing;
  }

  .day-card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid var(--border-color);
  }

  .day-header-left {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .drag-handle {
    font-size: 1.25rem;
    color: var(--text-secondary);
    cursor: grab;
    user-select: none;
    line-height: 1;
  }

  .drag-handle:active {
    cursor: grabbing;
  }

  .day-card-header h3 {
    margin: 0;
    font-size: 1.25rem;
    color: var(--text-primary);
  }

  .btn-remove-day {
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

  .btn-remove-day:hover {
    background: #c82333;
    transform: translateY(-1px);
  }

  .day-meta {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding-top: 0.75rem;
    border-top: 1px solid var(--border-color);
  }

  .day-info {
    font-size: 0.9rem;
    color: var(--text-secondary);
  }

  .day-focus-badge {
    padding: 0.25rem 0.75rem;
    background: var(--accent-color);
    color: white;
    border-radius: 4px;
    font-size: 0.85rem;
    font-weight: 500;
  }

  .btn-add-day {
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

  .btn-add-day:hover {
    background: var(--bg-hover);
    border-color: var(--accent-color);
    color: var(--accent-color);
  }

  @media (max-width: 768px) {
    .step-header h2 {
      font-size: 1.5rem;
    }

    .day-card {
      padding: 1rem;
    }

    .day-card-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.75rem;
    }

    .btn-remove-day {
      width: 100%;
    }
  }
</style>
