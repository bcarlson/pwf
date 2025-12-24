import { parse, stringify } from 'yaml';
import type { ParseOptions, PwfHistory, PwfPlan, ValidationIssue } from './types';
import { validateHistory, validatePlan } from './validate';

function yamlParse(text: string): { value?: unknown; issue?: ValidationIssue } {
  try {
    return { value: parse(text) };
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Invalid YAML';
    return {
      issue: {
        path: '',
        message,
        severity: 'error'
      }
    };
  }
}

export function parsePlan(text: string, options: ParseOptions = {}): PwfPlan | ValidationIssue[] {
  const result = yamlParse(text);
  if (result.issue) {
    return [result.issue];
  }

  const issues = validatePlan(result.value, options);
  if (issues.length > 0) {
    return issues;
  }

  return result.value as PwfPlan;
}

export function parseHistory(text: string, options: ParseOptions = {}): PwfHistory | ValidationIssue[] {
  const result = yamlParse(text);
  if (result.issue) {
    return [result.issue];
  }

  const issues = validateHistory(result.value, options);
  if (issues.length > 0) {
    return issues;
  }

  return result.value as PwfHistory;
}

export function isValidationIssueList(value: unknown): value is ValidationIssue[] {
  return (
    Array.isArray(value) &&
    value.every(
      (item) =>
        item &&
        typeof item === 'object' &&
        'message' in item &&
        'severity' in item &&
        'path' in item
    )
  );
}

export function toYAML(value: unknown): string {
  return stringify(value);
}

export function fromYAML<T>(text: string): T {
  return parse(text) as T;
}
