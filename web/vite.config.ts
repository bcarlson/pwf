import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import wasm from 'vite-plugin-wasm';
import sveltePreprocess from 'svelte-preprocess';
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    svelte({
      preprocess: sveltePreprocess()
    }),
    wasm()
  ],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./src/test-setup.ts'],
    alias: {
      // Mock the WASM module in tests since .wasm files are not available
      '../wasm/pwf_wasm': path.resolve(__dirname, 'src/__mocks__/pwf_wasm.ts'),
    },
    coverage: {
      reporter: ['text', 'json', 'html'],
      include: ['src/**/*.ts', 'src/**/*.svelte'],
      exclude: ['src/**/*.test.ts', 'src/**/__tests__/**', 'src/test-setup.ts']
    }
  },
  base: '/pwf/',  // GitHub Pages deployment path (change to '/' for custom domain)
  build: {
    outDir: 'dist',
    assetsDir: 'assets',
    target: 'esnext',
    minify: 'esbuild',
    rollupOptions: {
      external: [],
      plugins: [
        {
          name: 'skip-wasm-import-analysis',
          transform(code, id) {
            // Skip import analysis for WASM-generated files
            if (id.includes('/wasm/pwf_wasm.js')) {
              return { code, moduleSideEffects: 'no-treeshake' };
            }
          }
        }
      ]
    },
    commonjsOptions: {
      exclude: [/wasm/]
    }
  },
  optimizeDeps: {
    exclude: ['pwf-wasm']  // Don't pre-bundle WASM module
  },
  server: {
    port: 5173,
    strictPort: false,
    open: false
  }
});
