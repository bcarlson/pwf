<script lang="ts">
  import { onMount } from 'svelte';
  import { Line, Bar } from 'svelte-chartjs';
  import {
    Chart as ChartJS,
    Title,
    Tooltip,
    Legend,
    LineElement,
    BarElement,
    CategoryScale,
    LinearScale,
    PointElement,
    Filler
  } from 'chart.js';

  export let workout: any;

  // Register Chart.js components
  ChartJS.register(
    Title,
    Tooltip,
    Legend,
    LineElement,
    BarElement,
    CategoryScale,
    LinearScale,
    PointElement,
    Filler
  );

  let hasTelemetry = false;
  let hasHeartRate = false;
  let hasPower = false;
  let hasPace = false;
  let hasSpeed = false;
  let hasElevation = false;

  $: {
    if (workout?.telemetry && workout.telemetry.length > 0) {
      hasTelemetry = true;
      const firstPoint = workout.telemetry[0];
      hasHeartRate = firstPoint.heart_rate_bpm !== undefined;
      hasPower = firstPoint.power_watts !== undefined;
      hasPace = firstPoint.pace_min_per_km !== undefined;
      hasSpeed = firstPoint.speed_m_per_s !== undefined;
      hasElevation = firstPoint.elevation_m !== undefined;
    } else {
      hasTelemetry = false;
    }
  }

  function getHeartRateData() {
    if (!hasHeartRate) return null;

    const labels = workout.telemetry.map((_: any, i: number) => i);
    const data = workout.telemetry.map((point: any) => point.heart_rate_bpm);

    return {
      labels,
      datasets: [
        {
          label: 'Heart Rate (BPM)',
          data,
          borderColor: 'rgb(220, 53, 69)',
          backgroundColor: 'rgba(220, 53, 69, 0.1)',
          fill: true,
          tension: 0.4
        }
      ]
    };
  }

  function getPowerData() {
    if (!hasPower) return null;

    const labels = workout.telemetry.map((_: any, i: number) => i);
    const data = workout.telemetry.map((point: any) => point.power_watts);

    return {
      labels,
      datasets: [
        {
          label: 'Power (Watts)',
          data,
          borderColor: 'rgb(255, 193, 7)',
          backgroundColor: 'rgba(255, 193, 7, 0.1)',
          fill: true,
          tension: 0.4
        }
      ]
    };
  }

  function getSpeedData() {
    if (!hasSpeed && !hasPace) return null;

    const labels = workout.telemetry.map((_: any, i: number) => i);

    if (hasSpeed) {
      const data = workout.telemetry.map((point: any) => point.speed_m_per_s * 3.6); // Convert to km/h

      return {
        labels,
        datasets: [
          {
            label: 'Speed (km/h)',
            data,
            borderColor: 'rgb(13, 110, 253)',
            backgroundColor: 'rgba(13, 110, 253, 0.1)',
            fill: true,
            tension: 0.4
          }
        ]
      };
    } else {
      const data = workout.telemetry.map((point: any) => point.pace_min_per_km);

      return {
        labels,
        datasets: [
          {
            label: 'Pace (min/km)',
            data,
            borderColor: 'rgb(13, 110, 253)',
            backgroundColor: 'rgba(13, 110, 253, 0.1)',
            fill: true,
            tension: 0.4
          }
        ]
      };
    }
  }

  function getElevationData() {
    if (!hasElevation) return null;

    const labels = workout.telemetry.map((_: any, i: number) => i);
    const data = workout.telemetry.map((point: any) => point.elevation_m);

    return {
      labels,
      datasets: [
        {
          label: 'Elevation (m)',
          data,
          borderColor: 'rgb(25, 135, 84)',
          backgroundColor: 'rgba(25, 135, 84, 0.3)',
          fill: true,
          tension: 0.4
        }
      ]
    };
  }

  const chartOptions = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        position: 'top' as const
      },
      tooltip: {
        mode: 'index' as const,
        intersect: false
      }
    },
    scales: {
      x: {
        title: {
          display: true,
          text: 'Data Point'
        },
        grid: {
          color: 'rgba(0, 0, 0, 0.05)'
        }
      },
      y: {
        beginAtZero: false,
        grid: {
          color: 'rgba(0, 0, 0, 0.05)'
        }
      }
    },
    interaction: {
      mode: 'nearest' as const,
      axis: 'x' as const,
      intersect: false
    }
  };
</script>

<div class="telemetry-charts">
  {#if hasTelemetry}
    <div class="charts-header">
      <h3>Telemetry Data</h3>
      <p class="data-points">{workout.telemetry.length} data points</p>
    </div>

    <div class="charts-grid">
      {#if hasHeartRate}
        <div class="chart-container">
          <div class="chart-wrapper">
            <Line data={getHeartRateData()} options={chartOptions} />
          </div>
        </div>
      {/if}

      {#if hasPower}
        <div class="chart-container">
          <div class="chart-wrapper">
            <Line data={getPowerData()} options={chartOptions} />
          </div>
        </div>
      {/if}

      {#if hasSpeed || hasPace}
        <div class="chart-container">
          <div class="chart-wrapper">
            <Line data={getSpeedData()} options={chartOptions} />
          </div>
        </div>
      {/if}

      {#if hasElevation}
        <div class="chart-container">
          <div class="chart-wrapper">
            <Line data={getElevationData()} options={chartOptions} />
          </div>
        </div>
      {/if}
    </div>

    <!-- Summary Stats -->
    <div class="stats-section">
      <h4>Summary Statistics</h4>
      <div class="stats-grid">
        {#if hasHeartRate}
          {@const hrData = workout.telemetry.map(p => p.heart_rate_bpm).filter(v => v !== undefined)}
          {@const avgHr = (hrData.reduce((a, b) => a + b, 0) / hrData.length).toFixed(0)}
          {@const maxHr = Math.max(...hrData)}
          <div class="stat-card">
            <div class="stat-label">Avg Heart Rate</div>
            <div class="stat-value">{avgHr} <span class="stat-unit">BPM</span></div>
          </div>
          <div class="stat-card">
            <div class="stat-label">Max Heart Rate</div>
            <div class="stat-value">{maxHr} <span class="stat-unit">BPM</span></div>
          </div>
        {/if}

        {#if hasPower}
          {@const powerData = workout.telemetry.map(p => p.power_watts).filter(v => v !== undefined)}
          {@const avgPower = (powerData.reduce((a, b) => a + b, 0) / powerData.length).toFixed(0)}
          {@const maxPower = Math.max(...powerData)}
          <div class="stat-card">
            <div class="stat-label">Avg Power</div>
            <div class="stat-value">{avgPower} <span class="stat-unit">W</span></div>
          </div>
          <div class="stat-card">
            <div class="stat-label">Max Power</div>
            <div class="stat-value">{maxPower} <span class="stat-unit">W</span></div>
          </div>
        {/if}
      </div>
    </div>
  {:else}
    <div class="empty-state">
      <p>No telemetry data available for this workout</p>
    </div>
  {/if}
</div>

<style>
  .telemetry-charts {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1.5rem;
  }

  .charts-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  .charts-header h3 {
    margin: 0;
    font-size: 1.3rem;
  }

  .data-points {
    margin: 0;
    color: var(--text-secondary);
    font-size: 0.9rem;
  }

  .charts-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
    gap: 1.5rem;
    margin-bottom: 1.5rem;
  }

  .chart-container {
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    padding: 1rem;
  }

  .chart-wrapper {
    height: 300px;
  }

  .stats-section {
    margin-top: 1.5rem;
    padding-top: 1.5rem;
    border-top: 1px solid var(--border-color);
  }

  .stats-section h4 {
    margin: 0 0 1rem 0;
    font-size: 1.1rem;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 1rem;
  }

  .stat-card {
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    padding: 1rem;
    text-align: center;
  }

  .stat-label {
    font-size: 0.85rem;
    color: var(--text-secondary);
    margin-bottom: 0.5rem;
  }

  .stat-value {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--accent-color);
  }

  .stat-unit {
    font-size: 0.9rem;
    font-weight: 400;
    color: var(--text-secondary);
  }

  .empty-state {
    text-align: center;
    padding: 3rem 2rem;
    color: var(--text-secondary);
  }

  @media (max-width: 768px) {
    .charts-grid {
      grid-template-columns: 1fr;
    }

    .chart-wrapper {
      height: 250px;
    }

    .charts-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.5rem;
    }
  }
</style>
