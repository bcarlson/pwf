<script lang="ts">
  import { onMount } from 'svelte';
  import { wasmReady, currentTab, darkMode, type TabType } from './lib/stores';
  import { loadWasm } from './lib/wasm';
  import ValidatorPanel from './components/validator/ValidatorPanel.svelte';
  import ConverterPanel from './components/converter/ConverterPanel.svelte';
  import VisualizerPanel from './components/visualizer/VisualizerPanel.svelte';
  import BuilderPanel from './components/builder/BuilderPanel.svelte';

  // Load WASM on mount and handle routing
  onMount(async () => {
    await loadWasm();

    // Handle hash-based routing for builder
    const hash = window.location.hash;
    if (hash.startsWith('#/builder')) {
      currentTab.set('builder');
    }

    // Listen for hash changes
    window.addEventListener('hashchange', () => {
      const newHash = window.location.hash;
      if (newHash.startsWith('#/builder')) {
        currentTab.set('builder');
      }
    });
  });

  function switchTab(tab: TabType) {
    currentTab.set(tab);
    // Update URL hash for builder tab
    if (tab === 'builder') {
      window.location.hash = '#/builder';
    } else {
      window.location.hash = '';
    }
  }

  function toggleDarkMode() {
    darkMode.update(v => !v);
  }
</script>

<div class="app" class:dark={$darkMode}>
  <!-- Header -->
  <header>
    <div class="header-content">
      <h1>PWF Web Tools</h1>
      <p class="subtitle">Portable Workout Format - Validator & Converter</p>

      <div class="header-actions">
        <button
          class="dark-mode-toggle"
          on:click={toggleDarkMode}
          aria-label="Toggle dark mode"
        >
          {$darkMode ? '☀️' : '🌙'}
        </button>
      </div>
    </div>
  </header>

  <!-- Tab Navigation -->
  <nav class="tabs">
    <button
      class:active={$currentTab === 'validate'}
      on:click={() => switchTab('validate')}
    >
      ✓ Validate
    </button>
    <button
      class:active={$currentTab === 'convert'}
      on:click={() => switchTab('convert')}
    >
      🔄 Convert
    </button>
    <button
      class:active={$currentTab === 'visualize'}
      on:click={() => switchTab('visualize')}
    >
      📊 Visualize
    </button>
    <button
      class:active={$currentTab === 'builder'}
      on:click={() => switchTab('builder')}
    >
      ✏️ Plan Builder
    </button>
  </nav>

  <!-- Main Content -->
  <main>
    {#if !$wasmReady}
      <div class="loading">
        <div class="spinner"></div>
        <p>Loading PWF Web Tools...</p>
      </div>
    {:else if $currentTab === 'validate'}
      <div class="tab-content">
        <ValidatorPanel />
      </div>
    {:else if $currentTab === 'convert'}
      <div class="tab-content">
        <ConverterPanel />
      </div>
    {:else if $currentTab === 'visualize'}
      <div class="tab-content">
        <VisualizerPanel />
      </div>
    {:else if $currentTab === 'builder'}
      <div class="tab-content">
        <BuilderPanel />
      </div>
    {/if}
  </main>

  <!-- Footer -->
  <footer>
    <p>
      PWF v{#if $wasmReady}1.3.0{:else}...{/if} |
      <a href="https://github.com/bcarlson/pwf" target="_blank" rel="noopener">GitHub</a> |
      <a href="https://pwf.dev" target="_blank" rel="noopener">Documentation</a>
    </p>
  </footer>
</div>

<style>
  .app {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    background: var(--bg-primary);
    color: var(--text-primary);
    transition: background 0.3s, color 0.3s;
  }

  header {
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    padding: 1.5rem 2rem;
  }

  .header-content {
    max-width: 1200px;
    margin: 0 auto;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  h1 {
    margin: 0;
    font-size: 2rem;
    font-weight: 700;
  }

  .subtitle {
    margin: 0.25rem 0 0 0;
    font-size: 0.9rem;
    opacity: 0.7;
  }

  .header-actions {
    display: flex;
    gap: 1rem;
  }

  .dark-mode-toggle {
    background: none;
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 0.5rem 1rem;
    cursor: pointer;
    font-size: 1.2rem;
    transition: all 0.2s;
  }

  .dark-mode-toggle:hover {
    background: var(--bg-hover);
    transform: scale(1.05);
  }

  .tabs {
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    display: flex;
    gap: 0;
    padding: 0 2rem;
    max-width: 1200px;
    margin: 0 auto;
    width: 100%;
  }

  .tabs button {
    background: none;
    border: none;
    padding: 1rem 2rem;
    cursor: pointer;
    font-size: 1rem;
    font-weight: 500;
    color: var(--text-secondary);
    border-bottom: 3px solid transparent;
    transition: all 0.2s;
  }

  .tabs button:hover {
    color: var(--text-primary);
    background: var(--bg-hover);
  }

  .tabs button.active {
    color: var(--accent-color);
    border-bottom-color: var(--accent-color);
  }

  main {
    flex: 1;
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
    width: 100%;
  }

  .loading {
    text-align: center;
    padding: 4rem 2rem;
  }

  .spinner {
    width: 50px;
    height: 50px;
    border: 4px solid var(--border-color);
    border-top-color: var(--accent-color);
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin: 0 auto 1rem;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .tab-content {
    animation: fadeIn 0.3s;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  footer {
    background: var(--bg-secondary);
    border-top: 1px solid var(--border-color);
    padding: 1.5rem 2rem;
    text-align: center;
    font-size: 0.9rem;
    opacity: 0.7;
  }

  footer a {
    color: var(--accent-color);
    text-decoration: none;
  }

  footer a:hover {
    text-decoration: underline;
  }

  @media (max-width: 768px) {
    header {
      padding: 1rem;
    }

    .header-content {
      flex-direction: column;
      align-items: flex-start;
      gap: 1rem;
    }

    h1 {
      font-size: 1.5rem;
    }

    .tabs {
      padding: 0 1rem;
    }

    .tabs button {
      padding: 0.75rem 1rem;
      font-size: 0.9rem;
    }

    main {
      padding: 1rem;
    }
  }
</style>
