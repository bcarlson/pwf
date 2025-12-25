/**
 * Utilities for sharing workout plans via URL
 */

import { compressToEncodedURIComponent, decompressFromEncodedURIComponent } from 'lz-string';
import type { PlanDraft } from './builderState';

/**
 * Encode a workout plan into a shareable URL
 */
export function encodePlanToUrl(plan: PlanDraft, baseUrl: string = window.location.origin): string {
  try {
    const json = JSON.stringify(plan);
    const compressed = compressToEncodedURIComponent(json);
    return `${baseUrl}/#/builder?plan=${compressed}`;
  } catch (error) {
    console.error('Failed to encode plan:', error);
    throw new Error('Failed to create share link');
  }
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
          return JSON.parse(json) as PlanDraft;
        }
      }
      return null;
    }

    const json = decompressFromEncodedURIComponent(planParam);
    if (!json) {
      return null;
    }

    return JSON.parse(json) as PlanDraft;
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
