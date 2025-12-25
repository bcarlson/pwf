import { describe, it, expect, beforeEach, vi } from 'vitest';
import {
  getCustomTemplates,
  saveCustomTemplate,
  deleteCustomTemplate,
  updateCustomTemplate,
  type CustomTemplate
} from '../customTemplates';
import type { PlanDraft } from '../builderState';

describe('customTemplates', () => {
  const mockPlan: PlanDraft = {
    plan_version: 1,
    meta: {
      name: 'Test Plan',
      description: 'A test workout plan'
    },
    cycle: {
      days: [
        {
          focus: 'Upper Body',
          exercises: [
            {
              name: 'Bench Press',
              modality: 'strength',
              target_sets: 3,
              target_reps: 10
            }
          ]
        }
      ]
    }
  };

  let mockStorage: { [key: string]: string } = {};

  beforeEach(() => {
    // Reset mock storage
    mockStorage = {};

    // Mock localStorage
    global.localStorage = {
      getItem: vi.fn((key: string) => mockStorage[key] || null),
      setItem: vi.fn((key: string, value: string) => {
        mockStorage[key] = value;
      }),
      removeItem: vi.fn((key: string) => {
        delete mockStorage[key];
      }),
      clear: vi.fn(() => {
        mockStorage = {};
      }),
      length: 0,
      key: vi.fn(() => null)
    } as any;
  });

  describe('getCustomTemplates', () => {
    it('should return empty array when no templates exist', () => {
      const templates = getCustomTemplates();

      expect(templates).toEqual([]);
    });

    it('should return stored templates', () => {
      const mockTemplate: CustomTemplate = {
        id: 'test-123',
        name: 'My Template',
        description: 'Test description',
        plan: mockPlan,
        createdAt: Date.now()
      };

      mockStorage['pwf_custom_templates'] = JSON.stringify([mockTemplate]);

      const templates = getCustomTemplates();

      expect(templates).toEqual([mockTemplate]);
    });

    it('should handle corrupted storage data', () => {
      mockStorage['pwf_custom_templates'] = 'invalid json';

      const templates = getCustomTemplates();

      expect(templates).toEqual([]);
    });
  });

  describe('saveCustomTemplate', () => {
    it('should save a new custom template', () => {
      const template = saveCustomTemplate('My Plan', 'A great plan', mockPlan);

      expect(template.name).toBe('My Plan');
      expect(template.description).toBe('A great plan');
      expect(template.plan).toEqual(mockPlan);
      expect(template.id).toMatch(/^custom-\d+-[a-z0-9]+$/);
      expect(template.createdAt).toBeGreaterThan(0);
    });

    it('should store template in localStorage', () => {
      const template = saveCustomTemplate('My Plan', 'Description', mockPlan);

      const stored = JSON.parse(mockStorage['pwf_custom_templates']);
      expect(stored).toHaveLength(1);
      expect(stored[0]).toEqual(template);
    });

    it('should append to existing templates', () => {
      saveCustomTemplate('Plan 1', 'First', mockPlan);
      saveCustomTemplate('Plan 2', 'Second', mockPlan);

      const templates = getCustomTemplates();
      expect(templates).toHaveLength(2);
      expect(templates[0].name).toBe('Plan 1');
      expect(templates[1].name).toBe('Plan 2');
    });

    it('should throw error if storage fails', () => {
      const setItemMock = vi.fn(() => {
        throw new Error('Storage full');
      });
      global.localStorage.setItem = setItemMock;

      expect(() => saveCustomTemplate('Plan', 'Desc', mockPlan)).toThrow(
        'Failed to save template. Storage may be full.'
      );
    });

    it('should generate unique IDs for each template', () => {
      const template1 = saveCustomTemplate('Plan 1', '', mockPlan);
      const template2 = saveCustomTemplate('Plan 2', '', mockPlan);

      expect(template1.id).not.toBe(template2.id);
    });
  });

  describe('deleteCustomTemplate', () => {
    it('should delete a template by ID', () => {
      const template1 = saveCustomTemplate('Plan 1', '', mockPlan);
      const template2 = saveCustomTemplate('Plan 2', '', mockPlan);

      deleteCustomTemplate(template1.id);

      const templates = getCustomTemplates();
      expect(templates).toHaveLength(1);
      expect(templates[0].id).toBe(template2.id);
    });

    it('should handle deleting non-existent template', () => {
      saveCustomTemplate('Plan 1', '', mockPlan);

      deleteCustomTemplate('non-existent-id');

      const templates = getCustomTemplates();
      expect(templates).toHaveLength(1);
    });

    it('should handle empty template list', () => {
      expect(() => deleteCustomTemplate('any-id')).not.toThrow();
    });

    it('should throw error if storage fails', () => {
      const template = saveCustomTemplate('Plan', '', mockPlan);

      const setItemMock = vi.fn(() => {
        throw new Error('Storage error');
      });
      global.localStorage.setItem = setItemMock;

      expect(() => deleteCustomTemplate(template.id)).toThrow('Failed to delete template.');
    });
  });

  describe('updateCustomTemplate', () => {
    it('should update template name', () => {
      const template = saveCustomTemplate('Old Name', 'Description', mockPlan);

      updateCustomTemplate(template.id, { name: 'New Name' });

      const templates = getCustomTemplates();
      expect(templates[0].name).toBe('New Name');
      expect(templates[0].description).toBe('Description');
    });

    it('should update template description', () => {
      const template = saveCustomTemplate('Name', 'Old Desc', mockPlan);

      updateCustomTemplate(template.id, { description: 'New Desc' });

      const templates = getCustomTemplates();
      expect(templates[0].description).toBe('New Desc');
    });

    it('should update template plan', () => {
      const template = saveCustomTemplate('Name', 'Desc', mockPlan);

      const newPlan: PlanDraft = {
        plan_version: 1,
        cycle: { days: [] }
      };

      updateCustomTemplate(template.id, { plan: newPlan });

      const templates = getCustomTemplates();
      expect(templates[0].plan).toEqual(newPlan);
    });

    it('should update multiple fields at once', () => {
      const template = saveCustomTemplate('Old', 'Old Desc', mockPlan);

      const newPlan: PlanDraft = {
        plan_version: 1,
        cycle: { days: [] }
      };

      updateCustomTemplate(template.id, {
        name: 'New',
        description: 'New Desc',
        plan: newPlan
      });

      const templates = getCustomTemplates();
      expect(templates[0].name).toBe('New');
      expect(templates[0].description).toBe('New Desc');
      expect(templates[0].plan).toEqual(newPlan);
    });

    it('should preserve id and createdAt', () => {
      const template = saveCustomTemplate('Name', 'Desc', mockPlan);
      const originalId = template.id;
      const originalCreatedAt = template.createdAt;

      updateCustomTemplate(template.id, { name: 'Updated' });

      const templates = getCustomTemplates();
      expect(templates[0].id).toBe(originalId);
      expect(templates[0].createdAt).toBe(originalCreatedAt);
    });

    it('should throw error for non-existent template', () => {
      expect(() => updateCustomTemplate('non-existent', { name: 'Test' })).toThrow(
        'Template not found'
      );
    });

    it('should throw error if storage fails', () => {
      const template = saveCustomTemplate('Plan', '', mockPlan);

      const setItemMock = vi.fn(() => {
        throw new Error('Storage error');
      });
      global.localStorage.setItem = setItemMock;

      expect(() => updateCustomTemplate(template.id, { name: 'New' })).toThrow(
        'Failed to update template.'
      );
    });
  });

  describe('storage key', () => {
    it('should use consistent storage key', () => {
      saveCustomTemplate('Plan', 'Desc', mockPlan);

      expect(localStorage.setItem).toHaveBeenCalledWith(
        'pwf_custom_templates',
        expect.any(String)
      );
    });
  });

  describe('integration scenarios', () => {
    it('should handle save, update, delete workflow', () => {
      // Save
      const template = saveCustomTemplate('Original', 'Original Desc', mockPlan);
      expect(getCustomTemplates()).toHaveLength(1);

      // Update
      updateCustomTemplate(template.id, { name: 'Updated' });
      expect(getCustomTemplates()[0].name).toBe('Updated');

      // Delete
      deleteCustomTemplate(template.id);
      expect(getCustomTemplates()).toHaveLength(0);
    });

    it('should handle multiple templates', () => {
      const t1 = saveCustomTemplate('Plan 1', 'Desc 1', mockPlan);
      const t2 = saveCustomTemplate('Plan 2', 'Desc 2', mockPlan);
      const t3 = saveCustomTemplate('Plan 3', 'Desc 3', mockPlan);

      expect(getCustomTemplates()).toHaveLength(3);

      updateCustomTemplate(t2.id, { name: 'Updated Plan 2' });
      expect(getCustomTemplates()[1].name).toBe('Updated Plan 2');

      deleteCustomTemplate(t1.id);
      expect(getCustomTemplates()).toHaveLength(2);
      expect(getCustomTemplates()[0].id).toBe(t2.id);
      expect(getCustomTemplates()[1].id).toBe(t3.id);
    });
  });
});
