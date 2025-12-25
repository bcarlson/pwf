/**
 * Tests for ExampleGallery component
 */

import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { render, fireEvent, waitFor } from '@testing-library/svelte';
import ExampleGallery from '../ExampleGallery.svelte';

// Mock fetch
global.fetch = vi.fn();

// Get the actual BASE_URL from import.meta.env or default to '/pwf/'
const BASE_URL = import.meta.env.BASE_URL || '/pwf/';

describe('ExampleGallery', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe('rendering', () => {
    it('should render the gallery', () => {
      const { container } = render(ExampleGallery);
      expect(container.querySelector('.example-gallery')).toBeTruthy();
    });

    it('should render the title', () => {
      const { getByText } = render(ExampleGallery);
      expect(getByText('Load Example File')).toBeTruthy();
    });

    it('should render the description', () => {
      const { getByText } = render(ExampleGallery);
      expect(getByText('Try out PWF validation with pre-made example files')).toBeTruthy();
    });

    it('should render the example grid', () => {
      const { container } = render(ExampleGallery);
      expect(container.querySelector('.example-grid')).toBeTruthy();
    });

    it('should render all example cards', () => {
      const { container } = render(ExampleGallery);
      const cards = container.querySelectorAll('.example-card');
      expect(cards.length).toBe(9); // 9 examples total
    });
  });

  describe('example cards', () => {
    it('should render Beginner Strength example', () => {
      const { getByText } = render(ExampleGallery);
      expect(getByText('Beginner Strength')).toBeTruthy();
      expect(getByText('Simple 3-day strength training plan')).toBeTruthy();
    });

    it('should render Minimal Plan example', () => {
      const { getByText } = render(ExampleGallery);
      expect(getByText('Minimal Plan')).toBeTruthy();
      expect(getByText('Bare minimum valid PWF plan')).toBeTruthy();
    });

    it('should render Mixed Modalities example', () => {
      const { getByText } = render(ExampleGallery);
      expect(getByText('Mixed Modalities')).toBeTruthy();
      expect(getByText('Plan with strength, cardio, and intervals')).toBeTruthy();
    });

    it('should render History Export example', () => {
      const { getByText } = render(ExampleGallery);
      expect(getByText('History Export')).toBeTruthy();
      expect(getByText('Sample workout history with telemetry')).toBeTruthy();
    });

    it('should render CrossFit Workout example', () => {
      const { getByText } = render(ExampleGallery);
      expect(getByText('CrossFit Workout')).toBeTruthy();
      expect(getByText('High-intensity functional fitness')).toBeTruthy();
    });

    it('should render Hiking Activity example', () => {
      const { getByText } = render(ExampleGallery);
      expect(getByText('Hiking Activity')).toBeTruthy();
      expect(getByText('GPS-tracked hiking workout')).toBeTruthy();
    });

    it('should render Rowing Session example', () => {
      const { getByText } = render(ExampleGallery);
      expect(getByText('Rowing Session')).toBeTruthy();
      expect(getByText('Cardio rowing with metrics')).toBeTruthy();
    });

    it('should render Strength Training example', () => {
      const { getByText } = render(ExampleGallery);
      expect(getByText('Strength Training')).toBeTruthy();
      expect(getByText('Weight lifting session')).toBeTruthy();
    });

    it('should render Yoga Practice example', () => {
      const { getByText } = render(ExampleGallery);
      expect(getByText('Yoga Practice')).toBeTruthy();
      expect(getByText('Flexibility and mindfulness')).toBeTruthy();
    });

    it('should display plan icon for plan examples', () => {
      const { getByText } = render(ExampleGallery);
      const beginnerStrength = getByText('Beginner Strength').closest('.example-card');
      const icon = beginnerStrength?.querySelector('.example-icon');
      expect(icon?.textContent).toBe('📋');
    });

    it('should display history icon for history examples', () => {
      const { getByText } = render(ExampleGallery);
      const historyExport = getByText('History Export').closest('.example-card');
      const icon = historyExport?.querySelector('.example-icon');
      expect(icon?.textContent).toBe('📊');
    });

    it('should display correct type badge for plan examples', () => {
      const { getByText } = render(ExampleGallery);
      const beginnerStrength = getByText('Beginner Strength').closest('.example-card');
      const typeBadge = beginnerStrength?.querySelector('.example-type');
      expect(typeBadge?.textContent).toBe('plan');
    });

    it('should display correct type badge for history examples', () => {
      const { getByText } = render(ExampleGallery);
      const historyExport = getByText('History Export').closest('.example-card');
      const typeBadge = historyExport?.querySelector('.example-type');
      expect(typeBadge?.textContent).toBe('history');
    });
  });

  describe('example loading', () => {
    it('should fetch example file when card is clicked', async () => {
      const mockResponse = 'plan_version: 1\ncycle:\n  days: []';
      vi.mocked(fetch).mockResolvedValueOnce({
        ok: true,
        text: async () => mockResponse
      } as Response);

      const { getByText } = render(ExampleGallery);
      const beginnerButton = getByText('Beginner Strength').closest('button');

      if (beginnerButton) {
        await fireEvent.click(beginnerButton);
      }

      await waitFor(() => {
        expect(fetch).toHaveBeenCalledWith(`${BASE_URL}examples/beginner-strength.yaml`);
      });
    });

    it('should use correct BASE_URL when fetching', async () => {
      vi.mocked(fetch).mockResolvedValueOnce({
        ok: true,
        text: async () => 'test content'
      } as Response);

      const { getByText } = render(ExampleGallery);
      const minimalButton = getByText('Minimal Plan').closest('button');

      if (minimalButton) {
        await fireEvent.click(minimalButton);
      }

      await waitFor(() => {
        expect(fetch).toHaveBeenCalledWith(`${BASE_URL}examples/minimal.yaml`);
      });
    });

    it('should dispatch exampleLoaded event on successful fetch', async () => {
      const mockContent = 'plan_version: 1\ncycle:\n  days: []';
      vi.mocked(fetch).mockResolvedValueOnce({
        ok: true,
        text: async () => mockContent
      } as Response);

      const component = render(ExampleGallery);
      let eventFired = false;
      let eventDetail: any = null;

      component.component.$on('exampleLoaded', (event: CustomEvent) => {
        eventFired = true;
        eventDetail = event.detail;
      });

      const beginnerButton = component.getByText('Beginner Strength').closest('button');
      if (beginnerButton) {
        await fireEvent.click(beginnerButton);
      }

      await waitFor(() => {
        expect(eventFired).toBe(true);
      });

      expect(eventDetail).toBeTruthy();
      expect(eventDetail.content).toBe(mockContent);
      expect(eventDetail.example.name).toBe('Beginner Strength');
      expect(eventDetail.example.filename).toBe('beginner-strength.yaml');
    });

    it('should disable all buttons while loading', async () => {
      // Create a promise that we can control
      let resolvePromise: (value: any) => void;
      const fetchPromise = new Promise((resolve) => {
        resolvePromise = resolve;
      });

      vi.mocked(fetch).mockReturnValueOnce(fetchPromise as any);

      const { container, getByText } = render(ExampleGallery);
      const beginnerButton = getByText('Beginner Strength').closest('button');

      if (beginnerButton) {
        await fireEvent.click(beginnerButton);
      }

      // Check that buttons are disabled during loading
      const buttons = container.querySelectorAll('.example-card');
      buttons.forEach(button => {
        expect((button as HTMLButtonElement).disabled).toBe(true);
      });

      // Resolve the promise
      resolvePromise!({
        ok: true,
        text: async () => 'content'
      });

      // Wait for loading to complete
      await waitFor(() => {
        buttons.forEach(button => {
          expect((button as HTMLButtonElement).disabled).toBe(false);
        });
      });
    });

    it('should apply loading class during fetch', async () => {
      let resolvePromise: (value: any) => void;
      const fetchPromise = new Promise((resolve) => {
        resolvePromise = resolve;
      });

      vi.mocked(fetch).mockReturnValueOnce(fetchPromise as any);

      const { container, getByText } = render(ExampleGallery);
      const beginnerButton = getByText('Beginner Strength').closest('button');

      if (beginnerButton) {
        await fireEvent.click(beginnerButton);
      }

      // Check for loading class
      const buttons = container.querySelectorAll('.example-card.loading');
      expect(buttons.length).toBeGreaterThan(0);

      // Resolve the promise
      resolvePromise!({
        ok: true,
        text: async () => 'content'
      });

      // Wait for loading to complete
      await waitFor(() => {
        const loadingButtons = container.querySelectorAll('.example-card.loading');
        expect(loadingButtons.length).toBe(0);
      });
    });
  });

  describe('error handling', () => {
    it('should handle fetch errors', async () => {
      vi.mocked(fetch).mockRejectedValueOnce(new Error('Network error'));

      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      const { getByText } = render(ExampleGallery);
      const beginnerButton = getByText('Beginner Strength').closest('button');

      if (beginnerButton) {
        await fireEvent.click(beginnerButton);
      }

      await waitFor(() => {
        expect(consoleErrorSpy).toHaveBeenCalledWith(
          'Failed to load example:',
          expect.any(Error)
        );
      });

      consoleErrorSpy.mockRestore();
    });

    it('should dispatch error event on fetch failure', async () => {
      vi.mocked(fetch).mockRejectedValueOnce(new Error('Network error'));

      const component = render(ExampleGallery);
      let errorFired = false;
      let errorDetail: any = null;

      component.component.$on('error', (event: CustomEvent) => {
        errorFired = true;
        errorDetail = event.detail;
      });

      const beginnerButton = component.getByText('Beginner Strength').closest('button');
      if (beginnerButton) {
        await fireEvent.click(beginnerButton);
      }

      await waitFor(() => {
        expect(errorFired).toBe(true);
      });

      expect(errorDetail).toBeTruthy();
      expect(errorDetail.message).toContain('Failed to load example');
    });

    it('should handle HTTP errors', async () => {
      vi.mocked(fetch).mockResolvedValueOnce({
        ok: false,
        statusText: 'Not Found'
      } as Response);

      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      const { getByText } = render(ExampleGallery);
      const beginnerButton = getByText('Beginner Strength').closest('button');

      if (beginnerButton) {
        await fireEvent.click(beginnerButton);
      }

      await waitFor(() => {
        expect(consoleErrorSpy).toHaveBeenCalled();
      });

      consoleErrorSpy.mockRestore();
    });

    it('should dispatch error event for HTTP errors', async () => {
      vi.mocked(fetch).mockResolvedValueOnce({
        ok: false,
        statusText: 'Not Found'
      } as Response);

      const component = render(ExampleGallery);
      let errorFired = false;

      component.component.$on('error', () => {
        errorFired = true;
      });

      const beginnerButton = component.getByText('Beginner Strength').closest('button');
      if (beginnerButton) {
        await fireEvent.click(beginnerButton);
      }

      await waitFor(() => {
        expect(errorFired).toBe(true);
      });
    });

    it('should reset loading state after error', async () => {
      vi.mocked(fetch).mockRejectedValueOnce(new Error('Network error'));

      const { container, getByText } = render(ExampleGallery);
      const beginnerButton = getByText('Beginner Strength').closest('button');

      if (beginnerButton) {
        await fireEvent.click(beginnerButton);
      }

      await waitFor(() => {
        const buttons = container.querySelectorAll('.example-card');
        buttons.forEach(button => {
          expect((button as HTMLButtonElement).disabled).toBe(false);
        });
      });
    });
  });

  describe('all examples loading', () => {
    it('should load Mixed Modalities example', async () => {
      vi.mocked(fetch).mockResolvedValueOnce({
        ok: true,
        text: async () => 'mixed content'
      } as Response);

      const { getByText } = render(ExampleGallery);
      const button = getByText('Mixed Modalities').closest('button');

      if (button) {
        await fireEvent.click(button);
      }

      await waitFor(() => {
        expect(fetch).toHaveBeenCalledWith(`${BASE_URL}examples/mixed-modalities.yaml`);
      });
    });

    it('should load History Export example', async () => {
      vi.mocked(fetch).mockResolvedValueOnce({
        ok: true,
        text: async () => 'history content'
      } as Response);

      const { getByText } = render(ExampleGallery);
      const button = getByText('History Export').closest('button');

      if (button) {
        await fireEvent.click(button);
      }

      await waitFor(() => {
        expect(fetch).toHaveBeenCalledWith(`${BASE_URL}examples/history-export.yaml`);
      });
    });

    it('should load CrossFit Workout example', async () => {
      vi.mocked(fetch).mockResolvedValueOnce({
        ok: true,
        text: async () => 'crossfit content'
      } as Response);

      const { getByText } = render(ExampleGallery);
      const button = getByText('CrossFit Workout').closest('button');

      if (button) {
        await fireEvent.click(button);
      }

      await waitFor(() => {
        expect(fetch).toHaveBeenCalledWith(`${BASE_URL}examples/history-crossfit.yaml`);
      });
    });

    it('should load Hiking Activity example', async () => {
      vi.mocked(fetch).mockResolvedValueOnce({
        ok: true,
        text: async () => 'hiking content'
      } as Response);

      const { getByText } = render(ExampleGallery);
      const button = getByText('Hiking Activity').closest('button');

      if (button) {
        await fireEvent.click(button);
      }

      await waitFor(() => {
        expect(fetch).toHaveBeenCalledWith(`${BASE_URL}examples/history-hiking.yaml`);
      });
    });

    it('should load Rowing Session example', async () => {
      vi.mocked(fetch).mockResolvedValueOnce({
        ok: true,
        text: async () => 'rowing content'
      } as Response);

      const { getByText } = render(ExampleGallery);
      const button = getByText('Rowing Session').closest('button');

      if (button) {
        await fireEvent.click(button);
      }

      await waitFor(() => {
        expect(fetch).toHaveBeenCalledWith(`${BASE_URL}examples/history-rowing.yaml`);
      });
    });

    it('should load Strength Training example', async () => {
      vi.mocked(fetch).mockResolvedValueOnce({
        ok: true,
        text: async () => 'strength content'
      } as Response);

      const { getByText } = render(ExampleGallery);
      const button = getByText('Strength Training').closest('button');

      if (button) {
        await fireEvent.click(button);
      }

      await waitFor(() => {
        expect(fetch).toHaveBeenCalledWith(`${BASE_URL}examples/history-strength-training.yaml`);
      });
    });

    it('should load Yoga Practice example', async () => {
      vi.mocked(fetch).mockResolvedValueOnce({
        ok: true,
        text: async () => 'yoga content'
      } as Response);

      const { getByText } = render(ExampleGallery);
      const button = getByText('Yoga Practice').closest('button');

      if (button) {
        await fireEvent.click(button);
      }

      await waitFor(() => {
        expect(fetch).toHaveBeenCalledWith(`${BASE_URL}examples/history-yoga.yaml`);
      });
    });
  });

  describe('event detail structure', () => {
    it('should include all example properties in event detail', async () => {
      vi.mocked(fetch).mockResolvedValueOnce({
        ok: true,
        text: async () => 'test content'
      } as Response);

      const component = render(ExampleGallery);
      let eventDetail: any = null;

      component.component.$on('exampleLoaded', (event: CustomEvent) => {
        eventDetail = event.detail;
      });

      const button = component.getByText('Beginner Strength').closest('button');
      if (button) {
        await fireEvent.click(button);
      }

      await waitFor(() => {
        expect(eventDetail).toBeTruthy();
      });

      expect(eventDetail.example).toHaveProperty('name');
      expect(eventDetail.example).toHaveProperty('description');
      expect(eventDetail.example).toHaveProperty('filename');
      expect(eventDetail.example).toHaveProperty('type');
      expect(eventDetail).toHaveProperty('content');
    });

    it('should include correct type in event detail for plan', async () => {
      vi.mocked(fetch).mockResolvedValueOnce({
        ok: true,
        text: async () => 'plan content'
      } as Response);

      const component = render(ExampleGallery);
      let eventDetail: any = null;

      component.component.$on('exampleLoaded', (event: CustomEvent) => {
        eventDetail = event.detail;
      });

      const button = component.getByText('Minimal Plan').closest('button');
      if (button) {
        await fireEvent.click(button);
      }

      await waitFor(() => {
        expect(eventDetail?.example.type).toBe('plan');
      });
    });

    it('should include correct type in event detail for history', async () => {
      vi.mocked(fetch).mockResolvedValueOnce({
        ok: true,
        text: async () => 'history content'
      } as Response);

      const component = render(ExampleGallery);
      let eventDetail: any = null;

      component.component.$on('exampleLoaded', (event: CustomEvent) => {
        eventDetail = event.detail;
      });

      const button = component.getByText('CrossFit Workout').closest('button');
      if (button) {
        await fireEvent.click(button);
      }

      await waitFor(() => {
        expect(eventDetail?.example.type).toBe('history');
      });
    });
  });

  describe('accessibility', () => {
    it('should render buttons as button elements', () => {
      const { container } = render(ExampleGallery);
      const cards = container.querySelectorAll('.example-card');
      cards.forEach(card => {
        expect(card.tagName).toBe('BUTTON');
      });
    });

    it('should have disabled attribute when loading', async () => {
      let resolvePromise: (value: any) => void;
      const fetchPromise = new Promise((resolve) => {
        resolvePromise = resolve;
      });

      vi.mocked(fetch).mockReturnValueOnce(fetchPromise as any);

      const { getByText } = render(ExampleGallery);
      const button = getByText('Beginner Strength').closest('button') as HTMLButtonElement;

      await fireEvent.click(button);

      expect(button.disabled).toBe(true);

      resolvePromise!({
        ok: true,
        text: async () => 'content'
      });

      await waitFor(() => {
        expect(button.disabled).toBe(false);
      });
    });
  });
});
