/**
 * Tests for BuilderPanel component
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, fireEvent, screen } from '@testing-library/svelte';
import BuilderPanel from '../BuilderPanel.svelte';
import { builderState } from '../../../lib/builderState';
import { get } from 'svelte/store';

describe('BuilderPanel', () => {
  beforeEach(() => {
    builderState.reset();
  });

  describe('Initial Render', () => {
    it('should render the component without crashing', () => {
      const { container } = render(BuilderPanel);
      expect(container).toBeTruthy();
      expect(container.firstChild).toBeTruthy();
    });

    it('should display template selection screen by default', () => {
      const { getByText } = render(BuilderPanel);
      expect(getByText('Create a New Workout Plan')).toBeTruthy();
      expect(getByText('Choose how you want to start building your plan')).toBeTruthy();
    });

    it('should show all three template options', () => {
      const { getByText } = render(BuilderPanel);
      expect(getByText('Use a Template')).toBeTruthy();
      expect(getByText('Start from Scratch')).toBeTruthy();
      expect(getByText('Import YAML')).toBeTruthy();
    });

    it('should not show "Coming Soon" badges anymore', () => {
      const { queryByText } = render(BuilderPanel);
      const comingSoonBadge = queryByText('Coming Soon');
      expect(comingSoonBadge).toBeNull();
    });
  });

  describe('Template Selection', () => {
    it('should select "templates" option when clicked', async () => {
      const { getByText, container } = render(BuilderPanel);
      const templatesButton = getByText('Use a Template').closest('button');

      await fireEvent.click(templatesButton);

      expect(templatesButton?.classList.contains('selected')).toBe(true);
    });

    it('should display template gallery when templates option is selected', async () => {
      const { getByText } = render(BuilderPanel);
      const templatesButton = getByText('Use a Template').closest('button');

      await fireEvent.click(templatesButton);

      expect(getByText('Choose a Template')).toBeTruthy();
      expect(getByText('5×5 Strength Program')).toBeTruthy();
    });

    it('should select "scratch" option and start wizard', async () => {
      const { getByText, queryByText } = render(BuilderPanel);
      const scratchButton = getByText('Start from Scratch').closest('button');

      await fireEvent.click(scratchButton);

      // Should show wizard instead of template selector
      expect(queryByText('Create a New Workout Plan')).toBeNull();

      // Verify builder state was updated
      const state = get(builderState);
      expect(state.currentStep).toBe(1);
    });

    it('should reset builderState when "scratch" is selected', async () => {
      // Set some state first
      builderState.updateMeta({ name: 'Test Plan' });
      builderState.nextStep();

      const { getByText } = render(BuilderPanel);
      const scratchButton = getByText('Start from Scratch').closest('button');

      await fireEvent.click(scratchButton);

      // State should be reset and then advanced to step 1
      const state = get(builderState);
      expect(state.currentStep).toBe(1);
      expect(state.plan.meta?.name).toBeUndefined();
    });

    it('should select "load-yaml" option and show import area', async () => {
      const { getByText } = render(BuilderPanel);
      const yamlButton = getByText('Import YAML').closest('button');

      await fireEvent.click(yamlButton);

      expect(yamlButton?.classList.contains('selected')).toBe(true);
      expect(getByText('Import YAML File')).toBeTruthy();
      expect(getByText('or paste YAML content')).toBeTruthy();
    });

    it('should hide template gallery when switching to another option', async () => {
      const { getByText, queryByText } = render(BuilderPanel);

      // Select templates to show gallery
      const templatesButton = getByText('Use a Template').closest('button');
      await fireEvent.click(templatesButton);
      expect(getByText('Choose a Template')).toBeTruthy();

      // Select different option
      const yamlButton = getByText('Import YAML').closest('button');
      await fireEvent.click(yamlButton);

      // Gallery should be hidden
      expect(queryByText('Choose a Template')).toBeNull();
      expect(getByText('Import YAML File')).toBeTruthy();
    });
  });

  describe('YAML Import', () => {
    it('should show textarea for YAML input when load-yaml is selected', async () => {
      const { getByText, getByPlaceholderText } = render(BuilderPanel);
      const yamlButton = getByText('Import YAML').closest('button');

      await fireEvent.click(yamlButton);

      const textarea = getByPlaceholderText('Paste your PWF YAML content here...');
      expect(textarea).toBeTruthy();
    });

    it('should show error when attempting to load empty YAML', async () => {
      const { getByText } = render(BuilderPanel);
      const yamlButton = getByText('Import YAML').closest('button');
      await fireEvent.click(yamlButton);

      const loadButton = getByText('Load YAML');
      await fireEvent.click(loadButton);

      expect(getByText('Please enter YAML content')).toBeTruthy();
    });

    it('should show error when attempting to load whitespace-only YAML', async () => {
      const { getByText, getByPlaceholderText } = render(BuilderPanel);
      const yamlButton = getByText('Import YAML').closest('button');
      await fireEvent.click(yamlButton);

      const textarea = getByPlaceholderText('Paste your PWF YAML content here...');
      await fireEvent.input(textarea, { target: { value: '   \n  \t  ' } });

      const loadButton = getByText('Load YAML');
      await fireEvent.click(loadButton);

      expect(getByText('Please enter YAML content')).toBeTruthy();
    });

    it('should load valid YAML and start wizard', async () => {
      const { getByText, getByPlaceholderText, queryByText } = render(BuilderPanel);
      const yamlButton = getByText('Import YAML').closest('button');
      await fireEvent.click(yamlButton);

      const textarea = getByPlaceholderText('Paste your PWF YAML content here...');
      const validYaml = 'plan_version: 1\nmeta:\n  name: Test Plan\ncycle:\n  days:\n    - exercises: []';
      await fireEvent.input(textarea, { target: { value: validYaml } });

      const loadButton = getByText('Load YAML');
      await fireEvent.click(loadButton);

      // Should switch to wizard mode
      expect(queryByText('Import YAML File')).toBeNull();

      // Verify state was loaded
      const state = get(builderState);
      expect(state.plan.meta?.name).toBe('Test Plan');
      expect(state.currentStep).toBe(1);
    });

    it('should show file upload button', async () => {
      const { getByText } = render(BuilderPanel);
      const yamlButton = getByText('Import YAML').closest('button');
      await fireEvent.click(yamlButton);

      expect(getByText('Choose File')).toBeTruthy();
    });

    it('should handle file upload with valid content', async () => {
      const { getByText, container, queryByText } = render(BuilderPanel);
      const yamlButton = getByText('Import YAML').closest('button');
      await fireEvent.click(yamlButton);

      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;
      expect(fileInput).toBeTruthy();

      const validYaml = 'plan_version: 1\nmeta:\n  name: Test Plan\ncycle:\n  days:\n    - exercises: []';
      const file = new File([validYaml], 'test.yaml', { type: 'text/yaml' });

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false,
      });

      await fireEvent.change(fileInput);

      // Wait for FileReader to complete
      await new Promise(resolve => setTimeout(resolve, 50));

      // Should switch to wizard mode
      expect(queryByText('Import YAML File')).toBeNull();

      // Verify state was loaded
      const state = get(builderState);
      expect(state.plan.meta?.name).toBe('Test Plan');
    });

    it('should handle file upload error', async () => {
      const { getByText, container } = render(BuilderPanel);
      const yamlButton = getByText('Import YAML').closest('button');
      await fireEvent.click(yamlButton);

      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      // Mock FileReader error
      const originalFileReader = window.FileReader;
      window.FileReader = class {
        readAsText() {
          setTimeout(() => {
            if (this.onerror) this.onerror(new Event('error'));
          }, 0);
        }
      } as any;

      const file = new File(['test'], 'test.yaml', { type: 'text/yaml' });
      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false,
      });

      await fireEvent.change(fileInput);
      await new Promise(resolve => setTimeout(resolve, 50));

      expect(getByText('Failed to read file')).toBeTruthy();

      // Restore FileReader
      window.FileReader = originalFileReader;
    });

    it('should not process file upload when no file is selected', async () => {
      const { getByText, container, queryByText } = render(BuilderPanel);
      const yamlButton = getByText('Import YAML').closest('button');
      await fireEvent.click(yamlButton);

      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      Object.defineProperty(fileInput, 'files', {
        value: [],
        writable: false,
      });

      await fireEvent.change(fileInput);
      await new Promise(resolve => setTimeout(resolve, 50));

      // Should not show any error and should stay on import screen
      expect(queryByText('Failed to read file')).toBeNull();
      expect(queryByText('Import YAML File')).toBeTruthy();
    });

    it('should show Cancel button in YAML import area', async () => {
      const { getByText } = render(BuilderPanel);
      const yamlButton = getByText('Import YAML').closest('button');
      await fireEvent.click(yamlButton);

      expect(getByText('Cancel')).toBeTruthy();
    });

    it('should reset selection when Cancel is clicked', async () => {
      const { getByText, queryByText, getAllByText } = render(BuilderPanel);
      const yamlButton = getByText('Import YAML').closest('button');
      await fireEvent.click(yamlButton);

      expect(yamlButton?.classList.contains('selected')).toBe(true);

      // Click cancel (need to be specific as there might be multiple cancel buttons)
      const cancelButtons = getAllByText('Cancel');
      const cancelButton = cancelButtons.find(btn =>
        btn.closest('.import-actions') !== null
      );

      if (cancelButton) {
        await fireEvent.click(cancelButton);
      }

      // Selection should be reset
      expect(yamlButton?.classList.contains('selected')).toBe(false);
      expect(queryByText('Import YAML File')).toBeNull();

      // State should be reset
      const state = get(builderState);
      expect(state.currentStep).toBe(0);
    });
  });

  describe('Wizard Display', () => {
    it('should show wizard when scratch option is selected and step > 0', async () => {
      const { getByText, container } = render(BuilderPanel);
      const scratchButton = getByText('Start from Scratch').closest('button');

      await fireEvent.click(scratchButton);

      // Should show wizard (check for ProgressIndicator or wizard-specific elements)
      const wizardElement = container.querySelector('.builder-wizard');
      expect(wizardElement).toBeTruthy();
    });

    it('should not show template selector when wizard is active', async () => {
      const { getByText, queryByText } = render(BuilderPanel);
      const scratchButton = getByText('Start from Scratch').closest('button');

      await fireEvent.click(scratchButton);

      expect(queryByText('Create a New Workout Plan')).toBeNull();
      expect(queryByText('Choose how you want to start building your plan')).toBeNull();
    });

    it('should return to template selector when wizard emits cancel event', async () => {
      const { getByText, container } = render(BuilderPanel);
      const scratchButton = getByText('Start from Scratch').closest('button');

      await fireEvent.click(scratchButton);

      // Find and click the Cancel button in the wizard
      const cancelButton = getByText('Cancel');

      // Mock window.confirm to return true
      const originalConfirm = window.confirm;
      window.confirm = vi.fn(() => true);

      await fireEvent.click(cancelButton);

      // Should show template selector again
      expect(getByText('Create a New Workout Plan')).toBeTruthy();

      // State should be reset
      const state = get(builderState);
      expect(state.currentStep).toBe(0);

      window.confirm = originalConfirm;
    });
  });

  describe('Error Display', () => {
    it('should show error banner when YAML import fails', async () => {
      const { getByText, container } = render(BuilderPanel);
      const yamlButton = getByText('Import YAML').closest('button');
      await fireEvent.click(yamlButton);

      const loadButton = getByText('Load YAML');
      await fireEvent.click(loadButton);

      const errorBanner = container.querySelector('.error-banner');
      expect(errorBanner).toBeTruthy();
      expect(errorBanner?.textContent).toContain('Please enter YAML content');
    });

    it('should show error icon in error banner', async () => {
      const { getByText, container } = render(BuilderPanel);
      const yamlButton = getByText('Import YAML').closest('button');
      await fireEvent.click(yamlButton);

      const loadButton = getByText('Load YAML');
      await fireEvent.click(loadButton);

      const errorIcon = container.querySelector('.error-icon');
      expect(errorIcon).toBeTruthy();
      expect(errorIcon?.textContent).toBe('⚠️');
    });

    it('should not show error banner when no error exists', () => {
      const { container } = render(BuilderPanel);
      const errorBanner = container.querySelector('.error-banner');
      expect(errorBanner).toBeNull();
    });
  });

  describe('Reactive Behavior', () => {
    it('should react to builderState changes', async () => {
      const { container, queryByText } = render(BuilderPanel);

      // Initially should show template selector
      expect(queryByText('Create a New Workout Plan')).toBeTruthy();

      // Manually update builder state
      builderState.nextStep();

      // Wait for reactivity
      await new Promise(resolve => setTimeout(resolve, 50));

      // Still shows template selector because selectedTemplate is null
      expect(queryByText('Create a New Workout Plan')).toBeTruthy();
    });

    it('should properly compute showWizard reactive statement', async () => {
      const { getByText } = render(BuilderPanel);
      const scratchButton = getByText('Start from Scratch').closest('button');

      await fireEvent.click(scratchButton);

      // showWizard should be true (selectedTemplate === 'scratch' && currentStep > 0)
      const state = get(builderState);
      expect(state.currentStep).toBeGreaterThan(0);
    });
  });
});
