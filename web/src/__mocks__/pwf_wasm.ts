/**
 * Mock for pwf_wasm WASM module used in tests.
 * Provides a stub default export that simulates WASM initialization.
 */
export default function init() {
  return Promise.resolve();
}
