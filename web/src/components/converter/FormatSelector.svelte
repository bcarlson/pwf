<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  export let sourceFormat: string = '';
  export let targetFormat: string = '';

  type Format = {
    id: string;
    name: string;
    extension: string;
    canImport: boolean;
    canExport: boolean;
  };

  const formats: Format[] = [
    { id: 'fit', name: 'FIT', extension: '.fit', canImport: true, canExport: false },
    { id: 'tcx', name: 'TCX (Training Center XML)', extension: '.tcx', canImport: true, canExport: true },
    { id: 'gpx', name: 'GPX (GPS Exchange)', extension: '.gpx', canImport: true, canExport: true },
    { id: 'pwf', name: 'PWF (Portable Workout Format)', extension: '.yaml', canImport: true, canExport: true },
    { id: 'csv', name: 'CSV (Telemetry Data)', extension: '.csv', canImport: false, canExport: true }
  ];

  $: sourceFormats = formats.filter(f => f.canImport);
  $: targetFormats = formats.filter(f => f.canExport && f.id !== sourceFormat);
  $: selectedSourceFormat = formats.find(f => f.id === sourceFormat);
  $: selectedTargetFormat = formats.find(f => f.id === targetFormat);

  function handleSourceChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    sourceFormat = target.value;
    // Reset target if it's the same as source
    if (targetFormat === sourceFormat) {
      targetFormat = '';
    }
    dispatch('change', { sourceFormat, targetFormat });
  }

  function handleTargetChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    targetFormat = target.value;
    dispatch('change', { sourceFormat, targetFormat });
  }
</script>

<div class="format-selector">
  <div class="selector-group">
    <label for="source-format">
      <span class="label-text">Convert from:</span>
      <select
        id="source-format"
        bind:value={sourceFormat}
        on:change={handleSourceChange}
        class="format-select"
      >
        <option value="">Select source format...</option>
        {#each sourceFormats as format}
          <option value={format.id}>{format.name}</option>
        {/each}
      </select>
    </label>
  </div>

  <div class="arrow">→</div>

  <div class="selector-group">
    <label for="target-format">
      <span class="label-text">Convert to:</span>
      <select
        id="target-format"
        bind:value={targetFormat}
        on:change={handleTargetChange}
        class="format-select"
        disabled={!sourceFormat}
      >
        <option value="">Select target format...</option>
        {#each targetFormats as format}
          <option value={format.id}>{format.name}</option>
        {/each}
      </select>
    </label>
  </div>
</div>

{#if selectedSourceFormat && selectedTargetFormat}
  <div class="format-info">
    <p>
      Converting from <strong>{selectedSourceFormat.name}</strong> ({selectedSourceFormat.extension})
      to <strong>{selectedTargetFormat.name}</strong> ({selectedTargetFormat.extension})
    </p>
  </div>
{/if}

<style>
  .format-selector {
    display: flex;
    align-items: flex-end;
    gap: 1.5rem;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
  }

  .selector-group {
    flex: 1;
    min-width: 250px;
  }

  .label-text {
    display: block;
    font-weight: 600;
    margin-bottom: 0.5rem;
    color: var(--text-primary);
  }

  .format-select {
    width: 100%;
    padding: 0.75rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    font-size: 1rem;
    color: var(--text-primary);
    cursor: pointer;
    transition: all 0.2s;
  }

  .format-select:hover:not(:disabled) {
    border-color: var(--accent-color);
  }

  .format-select:focus {
    outline: none;
    border-color: var(--accent-color);
    box-shadow: 0 0 0 3px rgba(13, 110, 253, 0.1);
  }

  .format-select:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .arrow {
    font-size: 2rem;
    color: var(--accent-color);
    font-weight: bold;
    margin-bottom: 0.5rem;
  }

  .format-info {
    background: rgba(13, 110, 253, 0.1);
    border: 1px solid var(--accent-color);
    border-radius: 6px;
    padding: 1rem;
    margin-bottom: 1.5rem;
  }

  .format-info p {
    margin: 0;
    font-size: 0.95rem;
  }

  @media (max-width: 768px) {
    .format-selector {
      flex-direction: column;
      align-items: stretch;
    }

    .arrow {
      transform: rotate(90deg);
      align-self: center;
      margin: 0.5rem 0;
    }

    .selector-group {
      min-width: unset;
    }
  }
</style>
