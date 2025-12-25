/**
 * Comprehensive tests for App.svelte
 * Target: 95%+ coverage (statement and branch)
 */

import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';
import { render, fireEvent, waitFor } from '@testing-library/svelte';
import { get } from 'svelte/store';
import { tick } from 'svelte';
import App from '../App.svelte';
import { wasmReady, currentTab, darkMode } from '../lib/stores';

// Mock loadWasm
vi.mock('../lib/wasm', () => ({
  loadWasm: vi.fn().mockResolvedValue(true),
}));

// Import the mocked function
import { loadWasm } from '../lib/wasm';

describe('App.svelte', () => {
  beforeEach(() => {
    // Reset stores before each test
    wasmReady.set(false);
    currentTab.set('validate');
    darkMode.set(false);

    // Clear all mocks
    vi.clearAllMocks();

    // Clear localStorage
    localStorage.clear();

    // Remove any existing dark class
    document.documentElement.classList.remove('dark');
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  describe('Component Rendering', () => {
    it('should render without crashing', () => {
      const { container } = render(App);
      expect(container).toBeTruthy();
      expect(container.firstChild).toBeTruthy();
    });

    it('should render header with title and subtitle', () => {
      const { getByText } = render(App);

      expect(getByText('PWF Web Tools')).toBeTruthy();
      expect(getByText('Portable Workout Format - Validator & Converter')).toBeTruthy();
    });

    it('should render all tab buttons', () => {
      const { container } = render(App);

      const buttons = container.querySelectorAll('nav.tabs button');
      expect(buttons.length).toBe(3);
      expect(buttons[0].textContent).toContain('Validate');
      expect(buttons[1].textContent).toContain('Convert');
      expect(buttons[2].textContent).toContain('Visualize');
    });

    it('should render footer with version and links', () => {
      const { getByText } = render(App);

      expect(getByText(/PWF v/)).toBeTruthy();
      expect(getByText('GitHub')).toBeTruthy();
      expect(getByText('Documentation')).toBeTruthy();
    });

    it('should render dark mode toggle button', () => {
      const { container } = render(App);

      const toggleButton = container.querySelector('[aria-label="Toggle dark mode"]');
      expect(toggleButton).toBeTruthy();
    });
  });

  describe('WASM Loading', () => {
    it('should show loading spinner when WASM is not ready', () => {
      wasmReady.set(false);
      const { getByText, container } = render(App);

      expect(getByText('Loading PWF Web Tools...')).toBeTruthy();
      const spinner = container.querySelector('.spinner');
      expect(spinner).toBeTruthy();
    });

    it('should hide loading spinner when WASM is ready', async () => {
      wasmReady.set(true);
      const { queryByText } = render(App);

      expect(queryByText('Loading PWF Web Tools...')).toBeFalsy();
    });

    it('should show version in footer when WASM is ready', () => {
      wasmReady.set(true);
      const { getByText } = render(App);

      expect(getByText(/PWF v1\.3\.0/)).toBeTruthy();
    });

    it('should show ellipsis in footer when WASM is not ready', () => {
      wasmReady.set(false);
      const { getByText } = render(App);

      expect(getByText(/PWF v\.\.\./)).toBeTruthy();
    });
  });

  describe('Tab Switching', () => {
    beforeEach(() => {
      wasmReady.set(true);
    });

    it('should show validator panel by default', () => {
      currentTab.set('validate');
      const { container } = render(App);

      const tabContent = container.querySelector('.tab-content');
      expect(tabContent).toBeTruthy();
    });

    it('should switch to convert tab when clicked', async () => {
      const { container } = render(App);

      const buttons = container.querySelectorAll('nav.tabs button');
      const convertButton = buttons[1] as HTMLButtonElement;
      await fireEvent.click(convertButton);

      expect(get(currentTab)).toBe('convert');
    });

    it('should switch to visualize tab when clicked', async () => {
      const { container } = render(App);

      const buttons = container.querySelectorAll('nav.tabs button');
      const visualizeButton = buttons[2] as HTMLButtonElement;
      await fireEvent.click(visualizeButton);

      expect(get(currentTab)).toBe('visualize');
    });

    it('should switch back to validate tab when clicked', async () => {
      currentTab.set('convert');
      const { container } = render(App);

      const buttons = container.querySelectorAll('nav.tabs button');
      const validateButton = buttons[0] as HTMLButtonElement;
      await fireEvent.click(validateButton);

      expect(get(currentTab)).toBe('validate');
    });

    it('should apply active class to current tab', () => {
      currentTab.set('validate');
      const { container } = render(App);

      const buttons = container.querySelectorAll('nav.tabs button');
      const validateButton = buttons[0] as HTMLButtonElement;
      expect(validateButton.className).toContain('active');
    });

    it('should not apply active class to inactive tabs', () => {
      currentTab.set('validate');
      const { container } = render(App);

      const buttons = container.querySelectorAll('nav.tabs button');
      const convertButton = buttons[1] as HTMLButtonElement;
      const visualizeButton = buttons[2] as HTMLButtonElement;

      expect(convertButton.className).not.toContain('active');
      expect(visualizeButton.className).not.toContain('active');
    });

    it('should render different panels for different tabs', async () => {
      const { container } = render(App);

      // Start with validate
      currentTab.set('validate');
      await waitFor(() => {
        expect(container.querySelector('.tab-content')).toBeTruthy();
      });

      // Switch to convert
      currentTab.set('convert');
      await waitFor(() => {
        expect(container.querySelector('.tab-content')).toBeTruthy();
      });

      // Switch to visualize
      currentTab.set('visualize');
      await waitFor(() => {
        expect(container.querySelector('.tab-content')).toBeTruthy();
      });
    });
  });

  describe('Dark Mode', () => {
    it('should show moon icon when dark mode is off', () => {
      darkMode.set(false);
      const { container } = render(App);

      const toggleButton = container.querySelector('[aria-label="Toggle dark mode"]');
      expect(toggleButton?.textContent).toContain('🌙');
    });

    it('should show sun icon when dark mode is on', () => {
      darkMode.set(true);
      const { container } = render(App);

      const toggleButton = container.querySelector('[aria-label="Toggle dark mode"]');
      expect(toggleButton?.textContent).toContain('☀️');
    });

    it('should toggle dark mode when button is clicked', async () => {
      darkMode.set(false);
      const { container } = render(App);

      const toggleButton = container.querySelector('[aria-label="Toggle dark mode"]') as HTMLButtonElement;
      await fireEvent.click(toggleButton);

      expect(get(darkMode)).toBe(true);
    });

    it('should toggle dark mode off when clicked again', async () => {
      darkMode.set(true);
      const { container } = render(App);

      const toggleButton = container.querySelector('[aria-label="Toggle dark mode"]') as HTMLButtonElement;
      await fireEvent.click(toggleButton);

      expect(get(darkMode)).toBe(false);
    });

    it('should apply dark class to app element when dark mode is on', () => {
      darkMode.set(true);
      const { container } = render(App);

      const appElement = container.querySelector('.app');
      expect(appElement?.className).toContain('dark');
    });

    it('should not apply dark class when dark mode is off', () => {
      darkMode.set(false);
      const { container } = render(App);

      const appElement = container.querySelector('.app');
      expect(appElement?.className).not.toContain('dark');
    });

    it('should update dark class when toggling', async () => {
      darkMode.set(false);
      const { container } = render(App);

      const appElement = container.querySelector('.app');
      expect(appElement?.className).not.toContain('dark');

      const toggleButton = container.querySelector('[aria-label="Toggle dark mode"]') as HTMLButtonElement;
      await fireEvent.click(toggleButton);

      await waitFor(() => {
        expect(appElement?.className).toContain('dark');
      });
    });
  });

  describe('Footer Links', () => {
    it('should have correct GitHub link', () => {
      const { getByText } = render(App);

      const githubLink = getByText('GitHub') as HTMLAnchorElement;
      expect(githubLink.getAttribute('href')).toBe('https://github.com/bcarlson/pwf');
      expect(githubLink.getAttribute('target')).toBe('_blank');
      expect(githubLink.getAttribute('rel')).toBe('noopener');
    });

    it('should have correct documentation link', () => {
      const { getByText } = render(App);

      const docsLink = getByText('Documentation') as HTMLAnchorElement;
      expect(docsLink.getAttribute('href')).toBe('https://pwf.dev');
      expect(docsLink.getAttribute('target')).toBe('_blank');
      expect(docsLink.getAttribute('rel')).toBe('noopener');
    });
  });

  describe('Loading State Transitions', () => {
    it('should transition from loading to content when WASM loads', async () => {
      wasmReady.set(false);
      const { getByText, queryByText, container } = render(App);

      expect(getByText('Loading PWF Web Tools...')).toBeTruthy();

      wasmReady.set(true);

      await waitFor(() => {
        expect(queryByText('Loading PWF Web Tools...')).toBeFalsy();
        expect(container.querySelector('.tab-content')).toBeTruthy();
      });
    });

    it('should show correct panel after WASM loads', async () => {
      wasmReady.set(false);
      currentTab.set('convert');
      const { container } = render(App);

      wasmReady.set(true);

      await waitFor(() => {
        expect(container.querySelector('.tab-content')).toBeTruthy();
      });
    });
  });

  describe('Component Structure', () => {
    it('should have header, nav, main, and footer sections', () => {
      const { container } = render(App);

      expect(container.querySelector('header')).toBeTruthy();
      expect(container.querySelector('nav.tabs')).toBeTruthy();
      expect(container.querySelector('main')).toBeTruthy();
      expect(container.querySelector('footer')).toBeTruthy();
    });

    it('should have proper semantic HTML structure', () => {
      const { container } = render(App);

      const appDiv = container.querySelector('.app');
      expect(appDiv).toBeTruthy();

      const header = appDiv?.querySelector('header');
      expect(header).toBeTruthy();

      const nav = appDiv?.querySelector('nav');
      expect(nav).toBeTruthy();

      const main = appDiv?.querySelector('main');
      expect(main).toBeTruthy();

      const footer = appDiv?.querySelector('footer');
      expect(footer).toBeTruthy();
    });
  });

  describe('Tab Icons', () => {
    it('should display checkmark icon for validate tab', () => {
      const { container } = render(App);

      const buttons = container.querySelectorAll('nav.tabs button');
      const validateButton = buttons[0] as HTMLButtonElement;
      expect(validateButton.textContent).toContain('✓');
    });

    it('should display arrows icon for convert tab', () => {
      const { container } = render(App);

      const buttons = container.querySelectorAll('nav.tabs button');
      const convertButton = buttons[1] as HTMLButtonElement;
      expect(convertButton.textContent).toContain('🔄');
    });

    it('should display chart icon for visualize tab', () => {
      const { container } = render(App);

      const buttons = container.querySelectorAll('nav.tabs button');
      const visualizeButton = buttons[2] as HTMLButtonElement;
      expect(visualizeButton.textContent).toContain('📊');
    });
  });

  describe('Accessibility', () => {
    it('should have accessible dark mode toggle button', () => {
      const { container } = render(App);

      const toggleButton = container.querySelector('[aria-label="Toggle dark mode"]');
      expect(toggleButton?.getAttribute('aria-label')).toBe('Toggle dark mode');
    });

    it('should have clickable tab buttons', () => {
      const { container } = render(App);

      const buttons = container.querySelectorAll('nav.tabs button');
      const validateButton = buttons[0] as HTMLButtonElement;
      const convertButton = buttons[1] as HTMLButtonElement;
      const visualizeButton = buttons[2] as HTMLButtonElement;

      expect(validateButton.disabled).toBe(false);
      expect(convertButton.disabled).toBe(false);
      expect(visualizeButton.disabled).toBe(false);
    });
  });

  describe('Multiple Tab Interactions', () => {
    beforeEach(() => {
      wasmReady.set(true);
    });

    it('should handle rapid tab switching', async () => {
      const { container } = render(App);

      const buttons = container.querySelectorAll('nav.tabs button');
      const validateButton = buttons[0] as HTMLButtonElement;
      const convertButton = buttons[1] as HTMLButtonElement;
      const visualizeButton = buttons[2] as HTMLButtonElement;

      await fireEvent.click(convertButton);
      expect(get(currentTab)).toBe('convert');

      await fireEvent.click(visualizeButton);
      expect(get(currentTab)).toBe('visualize');

      await fireEvent.click(validateButton);
      expect(get(currentTab)).toBe('validate');
    });

    it('should update active class during tab switching', async () => {
      const { container } = render(App);

      const buttons = container.querySelectorAll('nav.tabs button');
      const validateButton = buttons[0] as HTMLButtonElement;
      const convertButton = buttons[1] as HTMLButtonElement;

      expect(validateButton.className).toContain('active');
      expect(convertButton.className).not.toContain('active');

      await fireEvent.click(convertButton);

      await waitFor(() => {
        expect(convertButton.className).toContain('active');
        expect(validateButton.className).not.toContain('active');
      });
    });
  });

  describe('Edge Cases', () => {
    it('should handle missing WASM gracefully', () => {
      vi.mocked(loadWasm).mockRejectedValueOnce(new Error('WASM failed'));
      const { container, getByText } = render(App);

      expect(container).toBeTruthy();
      expect(getByText('Loading PWF Web Tools...')).toBeTruthy();
    });

    it('should render even when stores have unexpected values', () => {
      (currentTab as any).set('invalid-tab' as any);
      wasmReady.set(true);

      const { container } = render(App);
      expect(container).toBeTruthy();
    });

    it('should handle multiple dark mode toggles', async () => {
      const { container } = render(App);

      const toggleButton = container.querySelector('[aria-label="Toggle dark mode"]') as HTMLButtonElement;

      await fireEvent.click(toggleButton);
      expect(get(darkMode)).toBe(true);

      await fireEvent.click(toggleButton);
      expect(get(darkMode)).toBe(false);

      await fireEvent.click(toggleButton);
      expect(get(darkMode)).toBe(true);
    });
  });

  describe('Conditional Rendering', () => {
    it('should only show one panel at a time', async () => {
      wasmReady.set(true);
      const { container } = render(App);

      // Should only have one .tab-content element
      const tabContents = container.querySelectorAll('.tab-content');
      expect(tabContents.length).toBe(1);
    });

    it('should not show panels when WASM is loading', () => {
      wasmReady.set(false);
      const { container } = render(App);

      const tabContents = container.querySelectorAll('.tab-content');
      expect(tabContents.length).toBe(0);
    });

    it('should show loading state and hide tabs content', () => {
      wasmReady.set(false);
      const { container } = render(App);

      expect(container.querySelector('.loading')).toBeTruthy();
      expect(container.querySelector('.tab-content')).toBeFalsy();
    });
  });
});
