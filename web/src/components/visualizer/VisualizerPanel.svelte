<script lang="ts">
  import FileUpload from '../validator/FileUpload.svelte';
  import ExampleGallery from '../shared/ExampleGallery.svelte';
  import PlanTreeView from './PlanTreeView.svelte';
  import TelemetryCharts from './TelemetryCharts.svelte';
  import GpsMap from './GpsMap.svelte';
  import * as yaml from 'yaml';

  let currentFile: File | null = null;
  let yamlContent: string = '';
  let parsedData: any = null;
  let dataType: 'plan' | 'history' | null = null;
  let errorMessage: string = '';
  let selectedWorkout: any = null;

  function handleFileLoaded(event: CustomEvent) {
    const { file, content } = event.detail;
    currentFile = file;
    yamlContent = content;
    errorMessage = '';

    parseYaml();
  }

  function handleExampleLoaded(event: CustomEvent) {
    const { example, content } = event.detail;
    yamlContent = content;
    currentFile = null;
    errorMessage = '';

    parseYaml();
  }

  function handleError(event: CustomEvent) {
    errorMessage = event.detail.message;
  }

  function parseYaml() {
    try {
      parsedData = yaml.parse(yamlContent);

      // Detect type
      if (parsedData.plan_version !== undefined) {
        dataType = 'plan';
      } else if (parsedData.history_version !== undefined) {
        dataType = 'history';
        // Select first workout by default
        if (parsedData.workouts && parsedData.workouts.length > 0) {
          selectedWorkout = parsedData.workouts[0];
        }
      } else {
        errorMessage = 'Unknown file type. Expected PWF plan or history file.';
        parsedData = null;
        dataType = null;
      }
    } catch (error) {
      console.error('YAML parsing error:', error);
      errorMessage = `Failed to parse YAML: ${error}`;
      parsedData = null;
      dataType = null;
    }
  }

  function selectWorkout(index: number) {
    if (parsedData?.workouts && parsedData.workouts[index]) {
      selectedWorkout = parsedData.workouts[index];
    }
  }

  function clearAll() {
    currentFile = null;
    yamlContent = '';
    parsedData = null;
    dataType = null;
    errorMessage = '';
    selectedWorkout = null;
  }
</script>

<div class="visualizer-panel">
  <div class="panel-intro">
    <h2>Workout Visualizer</h2>
    <p>Visualize PWF plans and workout history with interactive charts and maps</p>
  </div>

  {#if !parsedData}
    <!-- File Upload -->
    <FileUpload
      on:fileLoaded={handleFileLoaded}
      on:error={handleError}
      label="Drop PWF file here or click to browse"
    />

    <ExampleGallery
      on:exampleLoaded={handleExampleLoaded}
      on:error={handleError}
    />

    {#if errorMessage}
      <div class="error-banner">
        <span class="error-icon">⚠️</span>
        {errorMessage}
      </div>
    {/if}
  {:else}
    <!-- File Info -->
    <div class="file-info">
      <div class="file-details">
        <span class="file-icon">📊</span>
        <div>
          <p class="file-name">
            {#if currentFile}
              {currentFile.name}
            {:else}
              Example File
            {/if}
          </p>
          <p class="file-type">{dataType === 'plan' ? 'PWF Plan' : 'PWF History'}</p>
        </div>
      </div>
      <button class="btn-secondary" on:click={clearAll}>
        Load Different File
      </button>
    </div>

    <!-- Plan Visualization -->
    {#if dataType === 'plan'}
      <PlanTreeView plan={parsedData} />
    {/if}

    <!-- History Visualization -->
    {#if dataType === 'history' && parsedData.workouts}
      <div class="history-visualization">
        <!-- Workout Selector -->
        {#if parsedData.workouts.length > 1}
          <div class="workout-selector">
            <h3>Select Workout</h3>
            <div class="workout-list">
              {#each parsedData.workouts as workout, index}
                <button
                  class="workout-item"
                  class:active={selectedWorkout === workout}
                  on:click={() => selectWorkout(index)}
                  type="button"
                >
                  <div class="workout-info">
                    <span class="workout-name">
                      {workout.name || `Workout ${index + 1}`}
                    </span>
                    {#if workout.sport}
                      <span class="workout-sport">{workout.sport}</span>
                    {/if}
                  </div>
                  {#if workout.start_time}
                    <span class="workout-date">
                      {new Date(workout.start_time).toLocaleDateString()}
                    </span>
                  {/if}
                </button>
              {/each}
            </div>
          </div>
        {/if}

        <!-- Selected Workout Details -->
        {#if selectedWorkout}
          <div class="workout-details">
            <div class="workout-header">
              <h3>{selectedWorkout.name || 'Workout'}</h3>
              {#if selectedWorkout.start_time}
                <p class="workout-time">
                  {new Date(selectedWorkout.start_time).toLocaleString()}
                </p>
              {/if}
            </div>

            <!-- Summary -->
            <div class="workout-summary">
              {#if selectedWorkout.sport}
                <div class="summary-item">
                  <span class="summary-label">Sport:</span>
                  <span class="summary-value">{selectedWorkout.sport}</span>
                </div>
              {/if}
              {#if selectedWorkout.total_duration_sec}
                <div class="summary-item">
                  <span class="summary-label">Duration:</span>
                  <span class="summary-value">
                    {Math.floor(selectedWorkout.total_duration_sec / 60)} min
                  </span>
                </div>
              {/if}
              {#if selectedWorkout.total_distance_m}
                <div class="summary-item">
                  <span class="summary-label">Distance:</span>
                  <span class="summary-value">
                    {(selectedWorkout.total_distance_m / 1000).toFixed(2)} km
                  </span>
                </div>
              {/if}
            </div>

            <!-- GPS Map -->
            <GpsMap workout={selectedWorkout} />

            <!-- Telemetry Charts -->
            <div class="charts-section">
              <TelemetryCharts workout={selectedWorkout} />
            </div>
          </div>
        {/if}
      </div>
    {/if}
  {/if}
</div>

<style>
  .visualizer-panel {
    max-width: 1200px;
    margin: 0 auto;
  }

  .panel-intro {
    margin-bottom: 2rem;
  }

  .panel-intro h2 {
    margin: 0 0 0.5rem 0;
    font-size: 1.5rem;
  }

  .panel-intro p {
    margin: 0;
    color: var(--text-secondary);
  }

  .file-info {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1.5rem;
    margin-bottom: 1.5rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 1rem;
  }

  .file-details {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .file-icon {
    font-size: 2rem;
  }

  .file-name {
    margin: 0;
    font-weight: 600;
    font-size: 1.1rem;
  }

  .file-type {
    margin: 0.25rem 0 0 0;
    font-size: 0.9rem;
    color: var(--text-secondary);
  }

  .error-banner {
    background: rgba(220, 53, 69, 0.1);
    border: 1px solid var(--error-color);
    border-radius: 8px;
    padding: 1rem;
    margin: 1rem 0;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    color: var(--error-color);
  }

  .error-icon {
    font-size: 1.5rem;
  }

  .history-visualization {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .workout-selector {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1.5rem;
  }

  .workout-selector h3 {
    margin: 0 0 1rem 0;
    font-size: 1.2rem;
  }

  .workout-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .workout-item {
    width: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
    text-align: left;
    font: inherit;
    color: inherit;
  }

  .workout-item:hover {
    border-color: var(--accent-color);
    transform: translateX(4px);
  }

  .workout-item.active {
    background: var(--accent-color);
    color: white;
    border-color: var(--accent-color);
  }

  .workout-info {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .workout-name {
    font-weight: 600;
  }

  .workout-sport {
    font-size: 0.85rem;
    padding: 0.25rem 0.5rem;
    background: rgba(255, 255, 255, 0.2);
    border-radius: 4px;
  }

  .workout-item.active .workout-sport {
    background: rgba(255, 255, 255, 0.3);
  }

  .workout-date {
    font-size: 0.9rem;
    opacity: 0.8;
  }

  .workout-details {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .workout-header {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1.5rem;
  }

  .workout-header h3 {
    margin: 0 0 0.5rem 0;
    font-size: 1.4rem;
  }

  .workout-time {
    margin: 0;
    color: var(--text-secondary);
  }

  .workout-summary {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1.5rem;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 1rem;
  }

  .summary-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .summary-label {
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .summary-value {
    font-size: 1.2rem;
    font-weight: 600;
    color: var(--accent-color);
  }

  .charts-section {
    margin-top: 0;
  }

  @media (max-width: 768px) {
    .file-info {
      flex-direction: column;
      align-items: flex-start;
    }

    .workout-summary {
      grid-template-columns: 1fr;
    }
  }
</style>
