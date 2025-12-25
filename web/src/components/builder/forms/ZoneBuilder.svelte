<script lang="ts">
  import type { TrainingZone, Modality } from '../../../lib/builderState';

  export let zones: TrainingZone[] = [];
  export let modality: Modality;
  export let onChange: (zones: TrainingZone[]) => void;

  // Determine which fields to show based on modality
  $: showPower = modality === 'cycling' || modality === 'rowing';
  $: showPace = modality === 'running';
  $: showCadence = modality === 'cycling';

  function addZone() {
    const newZone: TrainingZone = {
      zone: 2,
      duration_sec: 300,
    };

    if (showPower) {
      newZone.target_power_watts = 200;
    }
    if (showPace) {
      newZone.target_pace_sec_per_km = 300;
    }
    if (showCadence) {
      newZone.cadence_rpm = 90;
    }

    zones = [...zones, newZone];
    onChange(zones);
  }

  function removeZone(index: number) {
    zones = zones.filter((_, i) => i !== index);
    onChange(zones);
  }

  function updateZone(index: number, field: keyof TrainingZone, value: number | undefined) {
    zones = zones.map((zone, i) => {
      if (i === index) {
        return { ...zone, [field]: value };
      }
      return zone;
    });
    onChange(zones);
  }
</script>

<div class="zone-builder">
  <div class="header">
    <h4>Training Zones</h4>
    <button type="button" on:click={addZone} class="btn-add">Add Zone</button>
  </div>

  {#if zones.length === 0}
    <div class="empty-state">
      <p>No training zones defined. Click "Add Zone" to create one.</p>
    </div>
  {:else}
    <div class="zone-list">
      {#each zones as zone, index}
        <div class="zone-card">
          <div class="zone-header">
            <span class="zone-title">Zone {zone.zone}</span>
            <button
              type="button"
              on:click={() => removeZone(index)}
              class="btn-remove-zone"
              aria-label="Remove zone"
            >
              ×
            </button>
          </div>

          <div class="zone-fields">
            <div class="field">
              <label for="zone-num-{index}">
                Zone Number (1-7)
                <span class="required">*</span>
              </label>
              <input
                id="zone-num-{index}"
                type="number"
                min="1"
                max="7"
                value={zone.zone}
                on:input={(e) => updateZone(index, 'zone', parseInt(e.currentTarget.value) || 1)}
                required
              />
            </div>

            <div class="field">
              <label for="duration-{index}">
                Duration (seconds)
              </label>
              <input
                id="duration-{index}"
                type="number"
                min="0"
                value={zone.duration_sec || ''}
                on:input={(e) => {
                  const val = e.currentTarget.value;
                  updateZone(index, 'duration_sec', val ? parseInt(val) : undefined);
                }}
              />
            </div>

            {#if showPower}
              <div class="field">
                <label for="power-{index}">
                  Target Power (watts)
                </label>
                <input
                  id="power-{index}"
                  type="number"
                  min="0"
                  value={zone.target_power_watts || ''}
                  on:input={(e) => {
                    const val = e.currentTarget.value;
                    updateZone(index, 'target_power_watts', val ? parseInt(val) : undefined);
                  }}
                />
              </div>
            {/if}

            <div class="field">
              <label for="hr-{index}">
                Target HR (bpm)
              </label>
              <input
                id="hr-{index}"
                type="number"
                min="0"
                max="250"
                value={zone.target_hr_bpm || ''}
                on:input={(e) => {
                  const val = e.currentTarget.value;
                  updateZone(index, 'target_hr_bpm', val ? parseInt(val) : undefined);
                }}
              />
            </div>

            {#if showPace}
              <div class="field">
                <label for="pace-{index}">
                  Target Pace (sec/km)
                </label>
                <input
                  id="pace-{index}"
                  type="number"
                  min="0"
                  value={zone.target_pace_sec_per_km || ''}
                  on:input={(e) => {
                    const val = e.currentTarget.value;
                    updateZone(index, 'target_pace_sec_per_km', val ? parseInt(val) : undefined);
                  }}
                />
              </div>
            {/if}

            {#if showCadence}
              <div class="field">
                <label for="cadence-{index}">
                  Cadence (rpm)
                </label>
                <input
                  id="cadence-{index}"
                  type="number"
                  min="0"
                  value={zone.cadence_rpm || ''}
                  on:input={(e) => {
                    const val = e.currentTarget.value;
                    updateZone(index, 'cadence_rpm', val ? parseInt(val) : undefined);
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
  .zone-builder {
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

  .zone-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .zone-card {
    border: 1px solid #e5e7eb;
    border-radius: 8px;
    padding: 1rem;
    background: #f9fafb;
  }

  .zone-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .zone-title {
    font-weight: 600;
    color: #111827;
  }

  .btn-remove-zone {
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

  .btn-remove-zone:hover {
    background: #fee2e2;
    color: #dc2626;
  }

  .zone-fields {
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

  :global(.dark) .zone-card {
    background: #1f2937;
    border-color: #374151;
  }

  :global(.dark) .zone-title {
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

  :global(.dark) .btn-remove-zone {
    color: #9ca3af;
  }

  :global(.dark) .btn-remove-zone:hover {
    background: #7f1d1d;
    color: #fca5a5;
  }
</style>
