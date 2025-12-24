<script lang="ts">
  export let plan: any;

  let expandedDays: Set<number> = new Set();
  let expandedExercises: Set<string> = new Set();

  function toggleDay(dayIndex: number) {
    if (expandedDays.has(dayIndex)) {
      expandedDays.delete(dayIndex);
    } else {
      expandedDays.add(dayIndex);
    }
    expandedDays = new Set(expandedDays);
  }

  function toggleExercise(exerciseId: string) {
    if (expandedExercises.has(exerciseId)) {
      expandedExercises.delete(exerciseId);
    } else {
      expandedExercises.add(exerciseId);
    }
    expandedExercises = new Set(expandedExercises);
  }

  function getModalityIcon(modality: string): string {
    const icons: Record<string, string> = {
      strength: '💪',
      countdown: '⏱️',
      stopwatch: '⏲️',
      interval: '🔄'
    };
    return icons[modality] || '📝';
  }

  function formatDuration(seconds: number): string {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  }
</script>

<div class="plan-tree">
  <div class="plan-header">
    <h3>
      {#if plan.meta?.name}
        {plan.meta.name}
      {:else}
        Workout Plan
      {/if}
    </h3>
    {#if plan.meta?.description}
      <p class="plan-description">{plan.meta.description}</p>
    {/if}
  </div>

  {#if plan.cycle}
    <div class="cycle-info">
      <div class="info-badge">
        <span class="badge-label">Cycle</span>
        <span class="badge-value">{plan.cycle.days.length} days</span>
      </div>
      {#if plan.cycle.target_reps}
        <div class="info-badge">
          <span class="badge-label">Target Reps</span>
          <span class="badge-value">{plan.cycle.target_reps}</span>
        </div>
      {/if}
    </div>

    <!-- Days -->
    <div class="days-list">
      {#each plan.cycle.days as day, dayIndex}
        <div class="day-item">
          <button
            class="day-header"
            on:click={() => toggleDay(dayIndex)}
            type="button"
          >
            <span class="expand-icon">
              {expandedDays.has(dayIndex) ? '▼' : '▶'}
            </span>
            <span class="day-title">
              Day {dayIndex + 1}
              {#if day.name}
                - {day.name}
              {/if}
            </span>
            <span class="exercise-count">
              {day.exercises.length} exercise{day.exercises.length !== 1 ? 's' : ''}
            </span>
          </button>

          {#if expandedDays.has(dayIndex)}
            <div class="exercises-list">
              {#each day.exercises as exercise, exerciseIndex}
                {@const exerciseId = `${dayIndex}-${exerciseIndex}`}
                <div class="exercise-item">
                  <button
                    class="exercise-header"
                    on:click={() => toggleExercise(exerciseId)}
                    type="button"
                  >
                    <span class="expand-icon small">
                      {expandedExercises.has(exerciseId) ? '▼' : '▶'}
                    </span>
                    <span class="modality-icon">
                      {getModalityIcon(exercise.modality)}
                    </span>
                    <span class="exercise-name">{exercise.name}</span>
                    <span class="modality-badge">{exercise.modality}</span>
                  </button>

                  {#if expandedExercises.has(exerciseId)}
                    <div class="exercise-details">
                      <!-- Modality-specific fields -->
                      {#if exercise.modality === 'strength'}
                        <div class="detail-row">
                          <span class="detail-label">Sets:</span>
                          <span class="detail-value">{exercise.target_sets || '-'}</span>
                        </div>
                        <div class="detail-row">
                          <span class="detail-label">Reps:</span>
                          <span class="detail-value">{exercise.target_reps || '-'}</span>
                        </div>
                        {#if exercise.target_load_kg}
                          <div class="detail-row">
                            <span class="detail-label">Load:</span>
                            <span class="detail-value">{exercise.target_load_kg} kg</span>
                          </div>
                        {/if}
                      {:else if exercise.modality === 'countdown' || exercise.modality === 'stopwatch'}
                        {#if exercise.target_duration_sec}
                          <div class="detail-row">
                            <span class="detail-label">Duration:</span>
                            <span class="detail-value">{formatDuration(exercise.target_duration_sec)}</span>
                          </div>
                        {/if}
                      {:else if exercise.modality === 'interval'}
                        <div class="detail-row">
                          <span class="detail-label">Sets:</span>
                          <span class="detail-value">{exercise.target_sets || '-'}</span>
                        </div>
                        <div class="detail-row">
                          <span class="detail-label">Duration:</span>
                          <span class="detail-value">{formatDuration(exercise.target_duration_sec || 0)}</span>
                        </div>
                      {/if}

                      <!-- Common fields -->
                      {#if exercise.equipment && exercise.equipment.length > 0}
                        <div class="detail-row">
                          <span class="detail-label">Equipment:</span>
                          <span class="detail-value">{exercise.equipment.join(', ')}</span>
                        </div>
                      {/if}

                      {#if exercise.notes}
                        <div class="detail-row">
                          <span class="detail-label">Notes:</span>
                          <span class="detail-value notes">{exercise.notes}</span>
                        </div>
                      {/if}
                    </div>
                  {/if}
                </div>
              {/each}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {:else}
    <p class="empty-state">No cycle data available</p>
  {/if}
</div>

<style>
  .plan-tree {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1.5rem;
  }

  .plan-header h3 {
    margin: 0 0 0.5rem 0;
    font-size: 1.4rem;
  }

  .plan-description {
    margin: 0 0 1rem 0;
    color: var(--text-secondary);
  }

  .cycle-info {
    display: flex;
    gap: 1rem;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
  }

  .info-badge {
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    padding: 0.5rem 1rem;
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .badge-label {
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .badge-value {
    font-weight: 600;
    color: var(--accent-color);
  }

  .days-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .day-item {
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    overflow: hidden;
  }

  .day-header {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    background: none;
    border: none;
    font: inherit;
    color: inherit;
    cursor: pointer;
    transition: background 0.2s;
    text-align: left;
  }

  .day-header:hover {
    background: var(--bg-hover);
  }

  .expand-icon {
    font-size: 0.85rem;
    color: var(--text-secondary);
    width: 16px;
  }

  .expand-icon.small {
    font-size: 0.75rem;
    width: 12px;
  }

  .day-title {
    flex: 1;
    font-weight: 600;
    font-size: 1.1rem;
  }

  .exercise-count {
    font-size: 0.9rem;
    color: var(--text-secondary);
  }

  .exercises-list {
    border-top: 1px solid var(--border-color);
    padding: 0.5rem;
  }

  .exercise-item {
    border: 1px solid var(--border-color);
    border-radius: 4px;
    margin-bottom: 0.5rem;
    overflow: hidden;
  }

  .exercise-item:last-child {
    margin-bottom: 0;
  }

  .exercise-header {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem;
    background: var(--bg-secondary);
    border: none;
    font: inherit;
    color: inherit;
    cursor: pointer;
    transition: background 0.2s;
    text-align: left;
  }

  .exercise-header:hover {
    background: var(--bg-hover);
  }

  .modality-icon {
    font-size: 1.2rem;
  }

  .exercise-name {
    flex: 1;
    font-weight: 500;
  }

  .modality-badge {
    font-size: 0.75rem;
    padding: 0.25rem 0.5rem;
    background: var(--accent-color);
    color: white;
    border-radius: 4px;
    text-transform: uppercase;
  }

  .exercise-details {
    padding: 0.75rem;
    background: var(--bg-primary);
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .detail-row {
    display: flex;
    gap: 0.5rem;
  }

  .detail-label {
    font-weight: 600;
    min-width: 100px;
    color: var(--text-secondary);
  }

  .detail-value {
    flex: 1;
  }

  .detail-value.notes {
    font-style: italic;
    opacity: 0.9;
  }

  .empty-state {
    text-align: center;
    padding: 2rem;
    color: var(--text-secondary);
  }

  @media (max-width: 768px) {
    .plan-tree {
      padding: 1rem;
    }

    .day-header,
    .exercise-header {
      padding: 0.75rem;
    }

    .detail-label {
      min-width: 80px;
    }
  }
</style>
