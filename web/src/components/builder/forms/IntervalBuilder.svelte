<script lang="ts">
  import type { IntervalPhase, Modality } from '../../../lib/builderState';

  export let interval_phases: IntervalPhase[] = [];
  export let modality: Modality;
  export let onChange: (phases: IntervalPhase[]) => void;

  // Determine which fields to show based on modality
  $: showPower = modality === 'cycling' || modality === 'rowing';
  $: showPace = modality === 'running';
  $: showCadence = modality === 'cycling';

  function addPhase() {
    const newPhase: IntervalPhase = {
      name: interval_phases.length === 0 ? 'work' : 'recovery',
      duration_sec: 60,
    };

    if (showPower) {
      newPhase.target_power_watts = 200;
    }
    if (showPace) {
      newPhase.target_pace_sec_per_km = 300;
    }
    if (showCadence) {
      newPhase.cadence_rpm = 90;
    }

    interval_phases = [...interval_phases, newPhase];
    onChange(interval_phases);
  }

  function removePhase(index: number) {
    interval_phases = interval_phases.filter((_, i) => i !== index);
    onChange(interval_phases);
  }

  function updatePhase(index: number, field: keyof IntervalPhase, value: string | number | undefined) {
    interval_phases = interval_phases.map((phase, i) => {
      if (i === index) {
        return { ...phase, [field]: value };
      }
      return phase;
    });
    onChange(interval_phases);
  }
</script>

<div class="interval-builder">
  <div class="header">
    <h4>Interval Phases</h4>
    <button type="button" on:click={addPhase} class="btn-add">Add Phase</button>
  </div>

  {#if interval_phases.length === 0}
    <div class="empty-state">
      <p>No interval phases defined. Click "Add Phase" to create one.</p>
    </div>
  {:else}
    <div class="phase-list">
      {#each interval_phases as phase, index}
        <div class="phase-card">
          <div class="phase-header">
            <span class="phase-title">Phase {index + 1}: {phase.name}</span>
            <button
              type="button"
              on:click={() => removePhase(index)}
              class="btn-remove-phase"
              aria-label="Remove phase"
            >
              ×
            </button>
          </div>

          <div class="phase-fields">
            <div class="field">
              <label for="phase-name-{index}">
                Phase Name
                <span class="required">*</span>
              </label>
              <input
                id="phase-name-{index}"
                type="text"
                value={phase.name}
                on:input={(e) => updatePhase(index, 'name', e.currentTarget.value)}
                placeholder="e.g., work, recovery, warmup"
                required
              />
            </div>

            <div class="field">
              <label for="phase-duration-{index}">
                Duration (seconds)
                <span class="required">*</span>
              </label>
              <input
                id="phase-duration-{index}"
                type="number"
                min="1"
                value={phase.duration_sec}
                on:input={(e) => updatePhase(index, 'duration_sec', parseInt(e.currentTarget.value) || 1)}
                required
              />
            </div>

            {#if showPower}
              <div class="field">
                <label for="phase-power-{index}">
                  Target Power (watts)
                </label>
                <input
                  id="phase-power-{index}"
                  type="number"
                  min="0"
                  value={phase.target_power_watts || ''}
                  on:input={(e) => {
                    const val = e.currentTarget.value;
                    updatePhase(index, 'target_power_watts', val ? parseInt(val) : undefined);
                  }}
                />
              </div>
            {/if}

            <div class="field">
              <label for="phase-hr-{index}">
                Target HR (bpm)
              </label>
              <input
                id="phase-hr-{index}"
                type="number"
                min="0"
                max="250"
                value={phase.target_hr_bpm || ''}
                on:input={(e) => {
                  const val = e.currentTarget.value;
                  updatePhase(index, 'target_hr_bpm', val ? parseInt(val) : undefined);
                }}
              />
            </div>

            {#if showPace}
              <div class="field">
                <label for="phase-pace-{index}">
                  Target Pace (sec/km)
                </label>
                <input
                  id="phase-pace-{index}"
                  type="number"
                  min="0"
                  value={phase.target_pace_sec_per_km || ''}
                  on:input={(e) => {
                    const val = e.currentTarget.value;
                    updatePhase(index, 'target_pace_sec_per_km', val ? parseInt(val) : undefined);
                  }}
                />
              </div>
            {/if}

            {#if showCadence}
              <div class="field">
                <label for="phase-cadence-{index}">
                  Cadence (rpm)
                </label>
                <input
                  id="phase-cadence-{index}"
                  type="number"
                  min="0"
                  value={phase.cadence_rpm || ''}
                  on:input={(e) => {
                    const val = e.currentTarget.value;
                    updatePhase(index, 'cadence_rpm', val ? parseInt(val) : undefined);
                  }}
                />
              </div>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .interval-builder {
    margin: 1rem 0;
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

  .empty-state {
    text-align: center;
    padding: 2rem;
    border: 2px dashed #d1d5db;
    border-radius: 8px;
    color: #6b7280;
  }

  .empty-state p {
    margin: 0;
  }

  .phase-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .phase-card {
    border: 1px solid #e5e7eb;
    border-radius: 8px;
    padding: 1rem;
    background: #f9fafb;
  }

  .phase-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .phase-title {
    font-weight: 600;
    color: #111827;
  }

  .btn-remove-phase {
    background: transparent;
    border: none;
    color: #6b7280;
    font-size: 1.5rem;
    line-height: 1;
    cursor: pointer;
    padding: 0;
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    transition: all 0.2s;
  }

  .btn-remove-phase:hover {
    background: #fee2e2;
    color: #dc2626;
  }

  .phase-fields {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
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

  /* Dark mode support */
  :global(.dark) h4 {
    color: #f9fafb;
  }

  :global(.dark) .phase-card {
    background: #1f2937;
    border-color: #374151;
  }

  :global(.dark) .phase-title {
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

  :global(.dark) .empty-state {
    border-color: #4b5563;
    color: #9ca3af;
  }

  :global(.dark) .btn-remove-phase {
    color: #9ca3af;
  }

  :global(.dark) .btn-remove-phase:hover {
    background: #7f1d1d;
    color: #fca5a5;
  }
</style>
