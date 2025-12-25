/**
 * Utilities for managing user-created custom templates
 */

import type { PlanDraft } from './builderState';

export interface CustomTemplate {
  id: string;
  name: string;
  description: string;
  plan: PlanDraft;
  createdAt: number;
}

const STORAGE_KEY = 'pwf_custom_templates';

/**
 * Get all custom templates from localStorage
 */
export function getCustomTemplates(): CustomTemplate[] {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (!stored) return [];
    return JSON.parse(stored) as CustomTemplate[];
  } catch (error) {
    console.error('Failed to load custom templates:', error);
    return [];
  }
}

/**
 * Save a new custom template
 */
export function saveCustomTemplate(
  name: string,
  description: string,
  plan: PlanDraft
): CustomTemplate {
  const templates = getCustomTemplates();

  const newTemplate: CustomTemplate = {
    id: `custom-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
    name,
    description,
    plan,
    createdAt: Date.now()
  };

  templates.push(newTemplate);

  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(templates));
    return newTemplate;
  } catch (error) {
    console.error('Failed to save custom template:', error);
    throw new Error('Failed to save template. Storage may be full.');
  }
}

/**
 * Delete a custom template by ID
 */
export function deleteCustomTemplate(id: string): void {
  const templates = getCustomTemplates();
  const filtered = templates.filter(t => t.id !== id);

  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(filtered));
  } catch (error) {
    console.error('Failed to delete custom template:', error);
    throw new Error('Failed to delete template.');
  }
}

/**
 * Update an existing custom template
 */
export function updateCustomTemplate(
  id: string,
  updates: Partial<Omit<CustomTemplate, 'id' | 'createdAt'>>
): void {
  const templates = getCustomTemplates();
  const index = templates.findIndex(t => t.id === id);

  if (index === -1) {
    throw new Error('Template not found');
  }

  templates[index] = {
    ...templates[index],
    ...updates
  };

  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(templates));
  } catch (error) {
    console.error('Failed to update custom template:', error);
    throw new Error('Failed to update template.');
  }
}
