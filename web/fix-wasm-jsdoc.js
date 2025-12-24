#!/usr/bin/env node

/**
 * Post-processes the WASM-generated JavaScript file to fix JSDoc comments
 * that contain nested comments, which cause parsing errors in Vite.
 */

import { readFileSync, writeFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const wasmJsPath = join(__dirname, 'src', 'wasm', 'pwf_wasm.js');

try {
  let content = readFileSync(wasmJsPath, 'utf-8');

  // Fix nested comments in JSDoc by removing inline comments within JSDoc blocks
  // Replace patterns like: { /* Comment */ } within JSDoc with { ... }
  content = content.replace(/(\s*\*\s*.*?){([^}]*?)\/\*.*?\*\/([^}]*?)}/g, '$1{ ... }');

  writeFileSync(wasmJsPath, content, 'utf-8');
  console.log('✓ Fixed JSDoc comments in pwf_wasm.js');
} catch (error) {
  console.error('Failed to fix WASM JSDoc:', error.message);
  process.exit(1);
}
