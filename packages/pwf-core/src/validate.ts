import Ajv from 'ajv';
import addFormats from 'ajv-formats';
import type { ErrorObject } from 'ajv';
import { historySchema, planSchema } from './schema';
import type { ValidationIssue, ValidationOptions } from './types';

const ajv = new Ajv({
  allErrors: true,
  strict: false
});

addFormats(ajv);

const planValidator = ajv.compile(planSchema);
const historyValidator = ajv.compile(historySchema);

const identifierPattern = /^[A-Za-z_$][A-Za-z0-9_$]*$/;

function decodePointerSegment(segment: string): string {
  return segment.replace(/~1/g, '/').replace(/~0/g, '~');
}

function appendSegment(path: string, segment: string): string {
  if (/^\d+$/.test(segment)) {
    return `${path}[${segment}]`;
  }

  if (identifierPattern.test(segment)) {
    return path ? `${path}.${segment}` : segment;
  }

  const escaped = segment.replace(/\\/g, '\\\\').replace(/'/g, "\\'");
  return `${path}['${escaped}']`;
}

function formatPath(pointer: string): string {
  if (!pointer) {
    return '';
  }

  const segments = pointer
    .split('/')
    .slice(1)
    .map(decodePointerSegment);

  return segments.reduce((path, segment) => appendSegment(path, segment), '');
}

function formatErrorPath(error: ErrorObject): string {
  let path = formatPath(error.instancePath);

  if (error.keyword === 'required' && typeof error.params.missingProperty === 'string') {
    path = appendSegment(path, error.params.missingProperty);
  }

  if (
    error.keyword === 'additionalProperties' &&
    typeof error.params.additionalProperty === 'string'
  ) {
    path = appendSegment(path, error.params.additionalProperty);
  }

  return path;
}

function ajvErrorsToIssues(errors: ErrorObject[]): ValidationIssue[] {
  return errors.map((error) => ({
    path: formatErrorPath(error),
    message: error.message as string,
    severity: 'error'
  }));
}

export function validatePlan(plan: unknown, _options: ValidationOptions = {}): ValidationIssue[] {
  const valid = planValidator(plan);
  if (valid) {
    return [];
  }

  return ajvErrorsToIssues(planValidator.errors as ErrorObject[]);
}

export function validateHistory(history: unknown, _options: ValidationOptions = {}): ValidationIssue[] {
  const valid = historyValidator(history);
  if (valid) {
    return [];
  }

  return ajvErrorsToIssues(historyValidator.errors as ErrorObject[]);
}
