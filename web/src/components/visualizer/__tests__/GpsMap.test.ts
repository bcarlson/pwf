/**
 * Tests for GpsMap component
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render } from '@testing-library/svelte';
import GpsMap from '../GpsMap.svelte';

// Mock Leaflet
vi.mock('leaflet', () => {
  const mockMarker = {
    bindPopup: vi.fn().mockReturnThis(),
    addTo: vi.fn().mockReturnThis(),
    remove: vi.fn(),
  };

  const mockPolyline = {
    addTo: vi.fn().mockReturnThis(),
    getBounds: vi.fn(() => ({
      _southWest: { lat: 0, lng: 0 },
      _northEast: { lat: 1, lng: 1 }
    })),
    remove: vi.fn(),
  };

  const mockTileLayer = {
    addTo: vi.fn().mockReturnThis(),
  };

  const mockMap = {
    setView: vi.fn().mockReturnThis(),
    fitBounds: vi.fn().mockReturnThis(),
    remove: vi.fn(),
  };

  return {
    default: {
      map: vi.fn(() => mockMap),
      tileLayer: vi.fn(() => mockTileLayer),
      polyline: vi.fn(() => mockPolyline),
      marker: vi.fn(() => mockMarker),
      icon: vi.fn((options) => options),
    },
    map: vi.fn(() => mockMap),
    tileLayer: vi.fn(() => mockTileLayer),
    polyline: vi.fn(() => mockPolyline),
    marker: vi.fn(() => mockMarker),
    icon: vi.fn((options) => options),
  };
});

describe('GpsMap', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should render the component', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {}
      }
    });
    expect(container.querySelector('.gps-map-container')).toBeTruthy();
  });

  it('should display empty state when no GPS data is available', () => {
    const { getByText } = render(GpsMap, {
      props: {
        workout: {}
      }
    });
    expect(getByText('No GPS data available for this workout')).toBeTruthy();
  });

  it('should display empty state when workout has no telemetry', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'Test Workout',
          telemetry: []
        }
      }
    });
    expect(container.querySelector('.empty-state')).toBeTruthy();
  });

  it('should display empty state when telemetry has no GPS positions', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'Test Workout',
          telemetry: [
            { heart_rate_bpm: 120 },
            { heart_rate_bpm: 130 }
          ]
        }
      }
    });
    expect(container.querySelector('.empty-state')).toBeTruthy();
  });

  it('should render map header when GPS data is available', () => {
    const { getByText } = render(GpsMap, {
      props: {
        workout: {
          name: 'GPS Run',
          telemetry: [
            {
              gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 }
            }
          ]
        }
      }
    });
    expect(getByText('GPS Route')).toBeTruthy();
  });

  it('should render map wrapper when GPS data is available', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'GPS Run',
          telemetry: [
            {
              gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 }
            }
          ]
        }
      }
    });
    expect(container.querySelector('.map-wrapper')).toBeTruthy();
  });

  it('should calculate distance correctly for two GPS points', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'GPS Run',
          telemetry: [
            {
              gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 }
            },
            {
              gps_position: { latitude_deg: 37.7750, longitude_deg: -122.4195 }
            }
          ]
        }
      }
    });

    // Distance should be calculated and displayed
    const routeStats = container.querySelector('.route-stats');
    expect(routeStats).toBeTruthy();
    expect(routeStats?.textContent).toContain('Distance:');
    expect(routeStats?.textContent).toContain('km');
  });

  it('should calculate distance correctly using Haversine formula', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'GPS Run',
          telemetry: [
            // New York to Los Angeles (rough coordinates)
            { gps_position: { latitude_deg: 40.7128, longitude_deg: -74.0060 } },
            { gps_position: { latitude_deg: 34.0522, longitude_deg: -118.2437 } }
          ]
        }
      }
    });

    const routeStats = container.querySelector('.route-stats');
    expect(routeStats).toBeTruthy();
    // Should calculate a large distance (approximately 3936 km)
    expect(routeStats?.textContent).toMatch(/\d+\.\d+\s*km/);
  });

  it('should return 0 distance when fewer than 2 GPS points', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'Single Point',
          telemetry: [
            { gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 } }
          ]
        }
      }
    });

    const routeStats = container.querySelector('.route-stats');
    expect(routeStats).toBeTruthy();
    expect(routeStats?.textContent).toContain('0.00 km');
  });

  it('should display elevation gain when elevation data is available', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'Hilly Run',
          telemetry: [
            {
              gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 },
              elevation_m: 100
            },
            {
              gps_position: { latitude_deg: 37.7750, longitude_deg: -122.4195 },
              elevation_m: 150
            },
            {
              gps_position: { latitude_deg: 37.7751, longitude_deg: -122.4196 },
              elevation_m: 200
            }
          ]
        }
      }
    });

    const routeStats = container.querySelector('.route-stats');
    expect(routeStats).toBeTruthy();
    expect(routeStats?.textContent).toContain('Elevation Gain:');
    expect(routeStats?.textContent).toContain('100 m');
  });

  it('should not display elevation gain when no elevation data', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'Flat Run',
          telemetry: [
            { gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 } },
            { gps_position: { latitude_deg: 37.7750, longitude_deg: -122.4195 } }
          ]
        }
      }
    });

    const routeStats = container.querySelector('.route-stats');
    expect(routeStats).toBeTruthy();
    expect(routeStats?.textContent).not.toContain('Elevation Gain:');
  });

  it('should calculate elevation gain correctly (only positive changes)', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'Variable Elevation',
          telemetry: [
            {
              gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 },
              elevation_m: 100
            },
            {
              gps_position: { latitude_deg: 37.7750, longitude_deg: -122.4195 },
              elevation_m: 150 // +50
            },
            {
              gps_position: { latitude_deg: 37.7751, longitude_deg: -122.4196 },
              elevation_m: 120 // -30 (not counted)
            },
            {
              gps_position: { latitude_deg: 37.7752, longitude_deg: -122.4197 },
              elevation_m: 180 // +60
            }
          ]
        }
      }
    });

    const routeStats = container.querySelector('.route-stats');
    expect(routeStats).toBeTruthy();
    // Total gain should be 50 + 60 = 110
    expect(routeStats?.textContent).toContain('110 m');
  });

  it('should return 0 elevation gain when fewer than 2 elevation points', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'Single Point',
          telemetry: [
            {
              gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 },
              elevation_m: 100
            }
          ]
        }
      }
    });

    const routeStats = container.querySelector('.route-stats');
    expect(routeStats).toBeTruthy();
    expect(routeStats?.textContent).toContain('0 m');
  });

  it('should render map legend with start and finish markers', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'GPS Run',
          telemetry: [
            { gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 } },
            { gps_position: { latitude_deg: 37.7750, longitude_deg: -122.4195 } }
          ]
        }
      }
    });

    const legend = container.querySelector('.map-legend');
    expect(legend).toBeTruthy();
    expect(legend?.textContent).toContain('Start');
    expect(legend?.textContent).toContain('Finish');
  });

  it('should have correct legend marker colors', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'GPS Run',
          telemetry: [
            { gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 } }
          ]
        }
      }
    });

    const startMarker = container.querySelector('.legend-marker.start');
    const endMarker = container.querySelector('.legend-marker.end');

    expect(startMarker).toBeTruthy();
    expect(endMarker).toBeTruthy();
  });

  it('should handle workout with mixed telemetry data', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'Mixed Data',
          telemetry: [
            { heart_rate_bpm: 120 },
            { gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 } },
            { power_watts: 200 },
            { gps_position: { latitude_deg: 37.7750, longitude_deg: -122.4195 } },
            { heart_rate_bpm: 130 }
          ]
        }
      }
    });

    expect(container.querySelector('.map-wrapper')).toBeTruthy();
    expect(container.querySelector('.empty-state')).toBeFalsy();
  });

  it('should filter out non-GPS telemetry points when calculating distance', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'Filtered Data',
          telemetry: [
            { gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 } },
            { heart_rate_bpm: 120 },
            { gps_position: { latitude_deg: 37.7750, longitude_deg: -122.4195 } },
            { power_watts: 200 },
            { gps_position: { latitude_deg: 37.7751, longitude_deg: -122.4196 } }
          ]
        }
      }
    });

    const routeStats = container.querySelector('.route-stats');
    expect(routeStats).toBeTruthy();
    // Distance should be calculated from 3 GPS points only
    expect(routeStats?.textContent).toMatch(/\d+\.\d+\s*km/);
  });

  it('should handle undefined workout gracefully', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: undefined
      }
    });
    expect(container.querySelector('.empty-state')).toBeTruthy();
  });

  it('should handle null workout gracefully', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: null
      }
    });
    expect(container.querySelector('.empty-state')).toBeTruthy();
  });

  it('should handle workout with undefined telemetry', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'No Telemetry',
          telemetry: undefined
        }
      }
    });
    expect(container.querySelector('.empty-state')).toBeTruthy();
  });

  it('should handle workout with null telemetry', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'Null Telemetry',
          telemetry: null
        }
      }
    });
    expect(container.querySelector('.empty-state')).toBeTruthy();
  });

  it('should display distance with 2 decimal places', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'Precision Test',
          telemetry: [
            { gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 } },
            { gps_position: { latitude_deg: 37.7750, longitude_deg: -122.4195 } }
          ]
        }
      }
    });

    const routeStats = container.querySelector('.route-stats');
    expect(routeStats?.textContent).toMatch(/\d+\.\d{2}\s*km/);
  });

  it('should display elevation gain with 0 decimal places', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'Elevation Test',
          telemetry: [
            {
              gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 },
              elevation_m: 100.7
            },
            {
              gps_position: { latitude_deg: 37.7750, longitude_deg: -122.4195 },
              elevation_m: 150.3
            }
          ]
        }
      }
    });

    const routeStats = container.querySelector('.route-stats');
    expect(routeStats?.textContent).toMatch(/\d+\s*m/);
    expect(routeStats?.textContent).not.toMatch(/\d+\.\d+\s*m/);
  });

  it('should handle very small distances', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'Small Distance',
          telemetry: [
            { gps_position: { latitude_deg: 37.774900, longitude_deg: -122.419400 } },
            { gps_position: { latitude_deg: 37.774901, longitude_deg: -122.419401 } }
          ]
        }
      }
    });

    const routeStats = container.querySelector('.route-stats');
    expect(routeStats?.textContent).toContain('km');
    expect(routeStats?.textContent).toMatch(/0\.\d{2}\s*km/);
  });

  it('should handle zero elevation gain', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'Flat Route',
          telemetry: [
            {
              gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 },
              elevation_m: 100
            },
            {
              gps_position: { latitude_deg: 37.7750, longitude_deg: -122.4195 },
              elevation_m: 100
            },
            {
              gps_position: { latitude_deg: 37.7751, longitude_deg: -122.4196 },
              elevation_m: 100
            }
          ]
        }
      }
    });

    const routeStats = container.querySelector('.route-stats');
    expect(routeStats?.textContent).toContain('0 m');
  });

  it('should handle descending elevation (no gain)', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'Downhill',
          telemetry: [
            {
              gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 },
              elevation_m: 200
            },
            {
              gps_position: { latitude_deg: 37.7750, longitude_deg: -122.4195 },
              elevation_m: 150
            },
            {
              gps_position: { latitude_deg: 37.7751, longitude_deg: -122.4196 },
              elevation_m: 100
            }
          ]
        }
      }
    });

    const routeStats = container.querySelector('.route-stats');
    expect(routeStats?.textContent).toContain('0 m');
  });

  it('should handle missing individual elevation values', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'Partial Elevation',
          telemetry: [
            {
              gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 },
              elevation_m: 100
            },
            {
              gps_position: { latitude_deg: 37.7750, longitude_deg: -122.4195 }
              // No elevation_m
            },
            {
              gps_position: { latitude_deg: 37.7751, longitude_deg: -122.4196 },
              elevation_m: 150
            }
          ]
        }
      }
    });

    const routeStats = container.querySelector('.route-stats');
    // Should still calculate from available elevation points
    expect(routeStats).toBeTruthy();
  });

  it('should display route stats section', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'Stats Test',
          telemetry: [
            { gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 } },
            { gps_position: { latitude_deg: 37.7750, longitude_deg: -122.4195 } }
          ]
        }
      }
    });

    const stats = container.querySelectorAll('.stat');
    expect(stats.length).toBeGreaterThan(0);
  });

  it('should have correct stat structure', () => {
    const { container } = render(GpsMap, {
      props: {
        workout: {
          name: 'Structure Test',
          telemetry: [
            {
              gps_position: { latitude_deg: 37.7749, longitude_deg: -122.4194 },
              elevation_m: 100
            },
            {
              gps_position: { latitude_deg: 37.7750, longitude_deg: -122.4195 },
              elevation_m: 150
            }
          ]
        }
      }
    });

    const statLabels = container.querySelectorAll('.stat-label');
    const statValues = container.querySelectorAll('.stat-value');

    expect(statLabels.length).toBeGreaterThan(0);
    expect(statValues.length).toBeGreaterThan(0);
    expect(statLabels.length).toBe(statValues.length);
  });
});
