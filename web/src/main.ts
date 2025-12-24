import './assets/styles.css';
import App from './App.svelte';
import { loadWasm } from './lib/wasm';

// Initialize WASM module
loadWasm().catch(error => {
  console.error('Failed to initialize WASM:', error);
  // Show error to user
  document.body.innerHTML = `
    <div style="text-align: center; padding: 50px; font-family: sans-serif;">
      <h1>Failed to load PWF Web Tools</h1>
      <p>The WebAssembly module failed to load. Please refresh the page or try a different browser.</p>
      <pre style="background: #f5f5f5; padding: 20px; border-radius: 5px; text-align: left; max-width: 600px; margin: 20px auto;">${error}</pre>
    </div>
  `;
});

const app = new App({
  target: document.getElementById('app')!,
});

export default app;
