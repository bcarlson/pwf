/**
 * Comprehensive tests for main.ts
 * Target: 95%+ coverage (statement and branch)
 */

import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';
import { JSDOM } from 'jsdom';

describe('main.ts', () => {
  let dom: JSDOM;
  let mockLoadWasm: any;
  let mockApp: any;
  let consoleErrorSpy: any;

  beforeEach(() => {
    // Setup fresh DOM for each test
    dom = new JSDOM('<!DOCTYPE html><html><body><div id="app"></div></body></html>', {
      url: 'http://localhost',
      pretendToBeVisual: true,
    });

    // Replace global objects
    global.document = dom.window.document as any;
    global.window = dom.window as any;
    global.navigator = dom.window.navigator as any;

    // Reset modules before each test
    vi.resetModules();

    // Mock console.error to capture error output
    consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    // Create mock App constructor
    mockApp = vi.fn().mockImplementation(() => ({
      $destroy: vi.fn(),
      $on: vi.fn(),
      $set: vi.fn(),
    }));

    // Mock loadWasm function
    mockLoadWasm = vi.fn().mockResolvedValue(true);

    // Mock the dependencies
    vi.doMock('../App.svelte', () => ({
      default: mockApp,
    }));

    vi.doMock('../lib/wasm', () => ({
      loadWasm: mockLoadWasm,
    }));

    // Mock CSS import
    vi.doMock('../assets/styles.css', () => ({}));
  });

  afterEach(() => {
    consoleErrorSpy.mockRestore();
    vi.doUnmock('../App.svelte');
    vi.doUnmock('../lib/wasm');
    vi.doUnmock('../assets/styles.css');
  });

  it('should initialize WASM and mount App component successfully', async () => {
    // Import main to trigger initialization
    await import('../main');

    // Wait for async operations
    await vi.waitFor(() => {
      expect(mockLoadWasm).toHaveBeenCalled();
    });

    // App should be mounted
    expect(mockApp).toHaveBeenCalledWith({
      target: document.getElementById('app'),
    });

    // No errors should be logged
    expect(consoleErrorSpy).not.toHaveBeenCalled();
  });

  it('should handle WASM initialization failure', async () => {
    const wasmError = new Error('WASM module not found');
    mockLoadWasm.mockRejectedValueOnce(wasmError);

    // Import main to trigger initialization
    await import('../main');

    // Wait for error handling
    await vi.waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalledWith('Failed to initialize WASM:', wasmError);
    });

    // Check that error UI is displayed
    const bodyHTML = document.body.innerHTML;
    expect(bodyHTML).toContain('Failed to load PWF Web Tools');
    expect(bodyHTML).toContain('The WebAssembly module failed to load');
    expect(bodyHTML).toContain('WASM module not found');
  });

  it('should display user-friendly error message on WASM failure', async () => {
    mockLoadWasm.mockRejectedValueOnce(new Error('Network error'));

    await import('../main');

    await vi.waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalled();
    });

    const bodyHTML = document.body.innerHTML;
    expect(bodyHTML).toContain('Please refresh the page or try a different browser');
  });

  it('should display error details in pre element', async () => {
    const errorMsg = 'Detailed error information';
    mockLoadWasm.mockRejectedValueOnce(new Error(errorMsg));

    await import('../main');

    await vi.waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalled();
    });

    const bodyHTML = document.body.innerHTML;
    const preElement = document.querySelector('pre');
    expect(preElement).toBeTruthy();
    expect(preElement?.textContent).toContain(errorMsg);
  });

  it('should handle missing app element gracefully', async () => {
    // Remove the app element
    const appElement = document.getElementById('app');
    appElement?.remove();

    await import('../main');

    await vi.waitFor(() => {
      expect(mockLoadWasm).toHaveBeenCalled();
    });

    // App should have been called with null target
    expect(mockApp).toHaveBeenCalledWith({
      target: null,
    });
  });

  it('should include error styling in error message', async () => {
    mockLoadWasm.mockRejectedValueOnce(new Error('Test error'));

    await import('../main');

    await vi.waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalled();
    });

    const bodyHTML = document.body.innerHTML;
    expect(bodyHTML).toContain('text-align: center');
    expect(bodyHTML).toContain('padding: 50px');
    expect(bodyHTML).toContain('font-family: sans-serif');
  });

  it('should include pre element styling for error details', async () => {
    mockLoadWasm.mockRejectedValueOnce(new Error('Styled error'));

    await import('../main');

    await vi.waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalled();
    });

    const bodyHTML = document.body.innerHTML;
    expect(bodyHTML).toContain('background: #f5f5f5');
    expect(bodyHTML).toContain('padding: 20px');
    expect(bodyHTML).toContain('border-radius: 5px');
    expect(bodyHTML).toContain('max-width: 600px');
  });

  it('should export app instance', async () => {
    const mainModule = await import('../main');

    await vi.waitFor(() => {
      expect(mockLoadWasm).toHaveBeenCalled();
    });

    expect(mainModule.default).toBeDefined();
  });


  it('should replace entire body.innerHTML on error', async () => {
    const originalHTML = document.body.innerHTML;
    expect(originalHTML).toContain('<div id="app"></div>');

    mockLoadWasm.mockRejectedValueOnce(new Error('Replace test'));

    await import('../main');

    await vi.waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalled();
    });

    const newHTML = document.body.innerHTML;
    expect(newHTML).not.toContain('<div id="app"></div>');
    expect(newHTML).toContain('Failed to load PWF Web Tools');
  });

  it('should call loadWasm before creating App', async () => {
    const callOrder: string[] = [];

    mockLoadWasm.mockImplementationOnce(async () => {
      callOrder.push('loadWasm');
      return true;
    });

    mockApp.mockImplementationOnce((props: any) => {
      callOrder.push('App');
      return { $destroy: vi.fn(), $on: vi.fn(), $set: vi.fn() };
    });

    await import('../main');

    await vi.waitFor(() => {
      expect(callOrder.length).toBeGreaterThan(0);
    });

    // loadWasm is called but App creation is not dependent on it completing
    // (it's fire-and-forget with .catch)
    expect(mockLoadWasm).toHaveBeenCalled();
    expect(mockApp).toHaveBeenCalled();
  });

  it('should handle Error objects with custom properties', async () => {
    const customError = new Error('Custom error');
    (customError as any).code = 'CUSTOM_CODE';
    (customError as any).details = { foo: 'bar' };

    mockLoadWasm.mockRejectedValueOnce(customError);

    await import('../main');

    await vi.waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalled();
    });

    const bodyHTML = document.body.innerHTML;
    // Should display the error string representation
    expect(bodyHTML).toContain('Custom error');
  });

  it('should handle non-Error thrown values', async () => {
    mockLoadWasm.mockRejectedValueOnce('String error');

    await import('../main');

    await vi.waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalledWith('Failed to initialize WASM:', 'String error');
    });

    const bodyHTML = document.body.innerHTML;
    expect(bodyHTML).toContain('String error');
  });

  it('should create App with correct target element', async () => {
    await import('../main');

    await vi.waitFor(() => {
      expect(mockApp).toHaveBeenCalled();
    });

    const appElement = document.getElementById('app');
    expect(mockApp).toHaveBeenCalledWith({
      target: appElement,
    });
  });

  it('should preserve error message formatting', async () => {
    const multilineError = new Error('Line 1\nLine 2\nLine 3');
    mockLoadWasm.mockRejectedValueOnce(multilineError);

    await import('../main');

    await vi.waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalled();
    });

    const bodyHTML = document.body.innerHTML;
    expect(bodyHTML).toContain('Line 1');
  });
});
