<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import L from 'leaflet';
  import 'leaflet/dist/leaflet.css';

  export let workout: any;

  let mapContainer: HTMLDivElement;
  let map: L.Map | null = null;
  let route: L.Polyline | null = null;
  let markers: L.Marker[] = [];
  let hasGps = false;
  let hasElevation = false;

  $: {
    if (workout?.telemetry && workout.telemetry.length > 0) {
      const firstPoint = workout.telemetry.find((p: any) => p.gps_position);
      hasGps = !!firstPoint;
      hasElevation = workout.telemetry.some((p: any) => p.elevation_m !== undefined);

      if (hasGps && map) {
        updateMap();
      }
    } else {
      hasGps = false;
      hasElevation = false;
    }
  }

  onMount(() => {
    if (hasGps) {
      initMap();
    }
  });

  onDestroy(() => {
    if (map) {
      map.remove();
      map = null;
    }
  });

  function initMap() {
    if (!mapContainer || !hasGps) return;

    // Get first GPS point for initial view
    const firstPoint = workout.telemetry.find((p: any) => p.gps_position);
    if (!firstPoint) return;

    const { latitude_deg, longitude_deg } = firstPoint.gps_position;

    // Create map
    map = L.map(mapContainer).setView([latitude_deg, longitude_deg], 13);

    // Add OpenStreetMap tile layer
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
      maxZoom: 19
    }).addTo(map);

    updateMap();
  }

  function updateMap() {
    if (!map) return;

    // Clear existing route and markers
    if (route) {
      route.remove();
    }
    markers.forEach(m => m.remove());
    markers = [];

    // Extract GPS points
    const gpsPoints: [number, number][] = workout.telemetry
      .filter((p: any) => p.gps_position)
      .map((p: any) => [
        p.gps_position.latitude_deg,
        p.gps_position.longitude_deg
      ]);

    if (gpsPoints.length === 0) return;

    // Draw route
    route = L.polyline(gpsPoints, {
      color: '#0d6efd',
      weight: 4,
      opacity: 0.7
    }).addTo(map);

    // Add start marker
    const startIcon = L.icon({
      iconUrl: 'https://raw.githubusercontent.com/pointhi/leaflet-color-markers/master/img/marker-icon-2x-green.png',
      shadowUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/0.7.7/images/marker-shadow.png',
      iconSize: [25, 41],
      iconAnchor: [12, 41],
      popupAnchor: [1, -34],
      shadowSize: [41, 41]
    });

    const startMarker = L.marker(gpsPoints[0], { icon: startIcon })
      .bindPopup('Start')
      .addTo(map);
    markers.push(startMarker);

    // Add end marker
    const endIcon = L.icon({
      iconUrl: 'https://raw.githubusercontent.com/pointhi/leaflet-color-markers/master/img/marker-icon-2x-red.png',
      shadowUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/0.7.7/images/marker-shadow.png',
      iconSize: [25, 41],
      iconAnchor: [12, 41],
      popupAnchor: [1, -34],
      shadowSize: [41, 41]
    });

    const endMarker = L.marker(gpsPoints[gpsPoints.length - 1], { icon: endIcon })
      .bindPopup('Finish')
      .addTo(map);
    markers.push(endMarker);

    // Fit map to route bounds
    map.fitBounds(route.getBounds(), { padding: [50, 50] });
  }

  function calculateDistance(): number {
    if (!workout?.telemetry) return 0;

    const gpsPoints = workout.telemetry
      .filter((p: any) => p.gps_position)
      .map((p: any) => p.gps_position);

    if (gpsPoints.length < 2) return 0;

    let totalDistance = 0;

    for (let i = 1; i < gpsPoints.length; i++) {
      const p1 = gpsPoints[i - 1];
      const p2 = gpsPoints[i];

      // Haversine formula for distance between two GPS points
      const R = 6371e3; // Earth radius in meters
      const φ1 = (p1.latitude_deg * Math.PI) / 180;
      const φ2 = (p2.latitude_deg * Math.PI) / 180;
      const Δφ = ((p2.latitude_deg - p1.latitude_deg) * Math.PI) / 180;
      const Δλ = ((p2.longitude_deg - p1.longitude_deg) * Math.PI) / 180;

      const a =
        Math.sin(Δφ / 2) * Math.sin(Δφ / 2) +
        Math.cos(φ1) * Math.cos(φ2) * Math.sin(Δλ / 2) * Math.sin(Δλ / 2);
      const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));

      totalDistance += R * c;
    }

    return totalDistance / 1000; // Convert to kilometers
  }

  function getElevationGain(): number {
    if (!workout?.telemetry) return 0;

    const elevations = workout.telemetry
      .filter((p: any) => p.elevation_m !== undefined)
      .map((p: any) => p.elevation_m);

    if (elevations.length < 2) return 0;

    let gain = 0;

    for (let i = 1; i < elevations.length; i++) {
      const diff = elevations[i] - elevations[i - 1];
      if (diff > 0) {
        gain += diff;
      }
    }

    return gain;
  }
</script>

<div class="gps-map-container">
  {#if hasGps}
    <div class="map-header">
      <h3>GPS Route</h3>
      <div class="route-stats">
        <div class="stat">
          <span class="stat-label">Distance:</span>
          <span class="stat-value">{calculateDistance().toFixed(2)} km</span>
        </div>
        {#if hasElevation}
          <div class="stat">
            <span class="stat-label">Elevation Gain:</span>
            <span class="stat-value">{getElevationGain().toFixed(0)} m</span>
          </div>
        {/if}
      </div>
    </div>

    <div class="map-wrapper" bind:this={mapContainer}></div>

    <div class="map-legend">
      <div class="legend-item">
        <span class="legend-marker start">●</span>
        <span>Start</span>
      </div>
      <div class="legend-item">
        <span class="legend-marker end">●</span>
        <span>Finish</span>
      </div>
    </div>
  {:else}
    <div class="empty-state">
      <p>No GPS data available for this workout</p>
    </div>
  {/if}
</div>

<style>
  .gps-map-container {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1.5rem;
  }

  .map-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
    flex-wrap: wrap;
    gap: 1rem;
  }

  .map-header h3 {
    margin: 0;
    font-size: 1.3rem;
  }

  .route-stats {
    display: flex;
    gap: 1.5rem;
  }

  .stat {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .stat-label {
    color: var(--text-secondary);
    font-size: 0.9rem;
  }

  .stat-value {
    font-weight: 600;
    color: var(--accent-color);
  }

  .map-wrapper {
    height: 500px;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    overflow: hidden;
  }

  .map-legend {
    display: flex;
    gap: 1.5rem;
    margin-top: 1rem;
    justify-content: center;
  }

  .legend-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .legend-marker {
    font-size: 1.5rem;
  }

  .legend-marker.start {
    color: #28a745;
  }

  .legend-marker.end {
    color: #dc3545;
  }

  .empty-state {
    text-align: center;
    padding: 3rem 2rem;
    color: var(--text-secondary);
  }

  @media (max-width: 768px) {
    .map-wrapper {
      height: 350px;
    }

    .map-header {
      flex-direction: column;
      align-items: flex-start;
    }

    .route-stats {
      width: 100%;
      flex-direction: column;
      gap: 0.5rem;
    }
  }

  /* Leaflet CSS overrides for dark mode */
  :global(.dark) :global(.leaflet-tile-pane) {
    filter: brightness(0.6) invert(1) contrast(3) hue-rotate(200deg) saturate(0.3) brightness(0.7);
  }
</style>
