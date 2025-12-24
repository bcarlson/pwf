import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import wasm from 'vite-plugin-wasm';
import sveltePreprocess from 'svelte-preprocess';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    svelte({
      preprocess: sveltePreprocess()
    }),
    wasm()
  ],
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
