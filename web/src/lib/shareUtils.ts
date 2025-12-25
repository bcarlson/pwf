/**
 * Utilities for sharing workout plans via URL
 */

import { compressToEncodedURIComponent, decompressFromEncodedURIComponent } from 'lz-string';
import type { PlanDraft } from './builderState';

/**
 * Encode a workout plan into a shareable URL
 */
export function encodePlanToUrl(plan: PlanDraft, baseUrl?: string): string {
  try {
    const json = JSON.stringify(plan);
    const compressed = compressToEncodedURIComponent(json);

    // Use provided baseUrl or construct from current location preserving subpath
    if (!baseUrl) {
      const url = new URL(window.location.href);
      baseUrl = url.origin + url.pathname.replace(/\/[^/]*$/, '');
    }

    return `${baseUrl}/#/builder?plan=${compressed}`;
  } catch (error) {
    console.error('Failed to encode plan:', error);
    throw new Error('Failed to create share link');
  }
}

/**
 * Validate that a decoded plan has the minimum required structure
 */
function validatePlanStructure(plan: any): plan is PlanDraft {
  if (!plan || typeof plan !== 'object') return false;
  if (typeof plan.plan_version !== 'number') return false;
  if (!plan.cycle || typeof plan.cycle !== 'object') return false;
  if (!Array.isArray(plan.cycle.days)) return false;

  // Validate each day has exercises array
  for (const day of plan.cycle.days) {
    if (!day || typeof day !== 'object') return false;
    if (!Array.isArray(day.exercises)) return false;
  }

  return true;
}

/**
 * Decode a workout plan from URL parameters
 */
export function decodePlanFromUrl(url: string = window.location.href): PlanDraft | null {
  try {
    const urlObj = new URL(url);
    const planParam = urlObj.searchParams.get('plan');

    if (!planParam) {
      // Try hash-based routing
      const hash = urlObj.hash;
      const match = hash.match(/[?&]plan=([^&]+)/);
      if (match) {
        const compressed = match[1];
        const json = decompressFromEncodedURIComponent(compressed);
        if (json) {
          const parsed = JSON.parse(json);
          return validatePlanStructure(parsed) ? parsed : null;
        }
      }
      return null;
    }

    const json = decompressFromEncodedURIComponent(planParam);
    if (!json) {
      return null;
    }

    const parsed = JSON.parse(json);
    return validatePlanStructure(parsed) ? parsed : null;
  } catch (error) {
    console.error('Failed to decode plan from URL:', error);
    return null;
  }
}

/**
 * Get current plan from URL if present
 */
export function getPlanFromCurrentUrl(): PlanDraft | null {
  return decodePlanFromUrl(window.location.href);
}

/**
 * Copy share link to clipboard
 */
export async function copyShareLink(plan: PlanDraft): Promise<void> {
  const url = encodePlanToUrl(plan);
  await navigator.clipboard.writeText(url);
}
