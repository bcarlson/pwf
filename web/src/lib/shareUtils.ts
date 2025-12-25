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
    console.log('[PWF Share] Decoding plan from URL:', url);
    const urlObj = new URL(url);
    const planParam = urlObj.searchParams.get('plan');
    console.log('[PWF Share] Search param plan:', planParam ? 'found' : 'not found');

    if (!planParam) {
      // Try hash-based routing
      const hash = urlObj.hash;
      console.log('[PWF Share] Checking hash:', hash.substring(0, 50) + '...');
      const match = hash.match(/[?&]plan=([^&]+)/);
      if (match) {
        console.log('[PWF Share] Found plan in hash, compressed length:', match[1].length);
        const compressed = match[1];
        const json = decompressFromEncodedURIComponent(compressed);
        if (json) {
          console.log('[PWF Share] Decompressed JSON length:', json.length);
          const parsed = JSON.parse(json);
          const isValid = validatePlanStructure(parsed);
          console.log('[PWF Share] Plan validation:', isValid ? 'PASSED ✓' : 'FAILED ✗');
          if (isValid) {
            console.log('[PWF Share] Plan loaded:', parsed.meta?.name || 'Unnamed Plan');
          }
          return isValid ? parsed : null;
        } else {
          console.error('[PWF Share] Failed to decompress plan data');
        }
      } else {
        console.log('[PWF Share] No plan parameter found in hash');
      }
      return null;
    }

    const json = decompressFromEncodedURIComponent(planParam);
    if (!json) {
      console.error('[PWF Share] Failed to decompress plan from search param');
      return null;
    }

    const parsed = JSON.parse(json);
    const isValid = validatePlanStructure(parsed);
    console.log('[PWF Share] Plan validation:', isValid ? 'PASSED ✓' : 'FAILED ✗');
    return isValid ? parsed : null;
  } catch (error) {
    console.error('[PWF Share] Failed to decode plan from URL:', error);
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
