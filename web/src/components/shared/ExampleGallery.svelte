<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  interface Example {
    name: string;
    description: string;
    filename: string;
    type: 'plan' | 'history';
  }

  const examples: Example[] = [
    {
      name: 'Beginner Strength',
      description: 'Simple 3-day strength training plan',
      filename: 'beginner-strength.yaml',
      type: 'plan'
    },
    {
      name: 'Minimal Plan',
      description: 'Bare minimum valid PWF plan',
      filename: 'minimal.yaml',
      type: 'plan'
    },
    {
      name: 'Mixed Modalities',
      description: 'Plan with strength, cardio, and intervals',
      filename: 'mixed-modalities.yaml',
      type: 'plan'
    },
    {
      name: 'History Export',
      description: 'Sample workout history with telemetry',
      filename: 'history-export.yaml',
      type: 'history'
    },
    {
      name: 'CrossFit Workout',
      description: 'High-intensity functional fitness',
      filename: 'history-crossfit.yaml',
      type: 'history'
    },
    {
      name: 'Hiking Activity',
      description: 'GPS-tracked hiking workout',
      filename: 'history-hiking.yaml',
      type: 'history'
    },
    {
      name: 'Rowing Session',
      description: 'Cardio rowing with metrics',
      filename: 'history-rowing.yaml',
      type: 'history'
    },
    {
      name: 'Strength Training',
      description: 'Weight lifting session',
      filename: 'history-strength-training.yaml',
      type: 'history'
    },
    {
      name: 'Yoga Practice',
      description: 'Flexibility and mindfulness',
      filename: 'history-yoga.yaml',
      type: 'history'
    }
  ];

  let loading = false;

  async function loadExample(example: Example) {
    loading = true;
    try {
      const response = await fetch(`${import.meta.env.BASE_URL}examples/${example.filename}`);
      if (!response.ok) {
        throw new Error(`Failed to load example: ${response.statusText}`);
      }
      const content = await response.text();
      dispatch('exampleLoaded', { example, content });
    } catch (error) {
      console.error('Failed to load example:', error);
      dispatch('error', { message: `Failed to load example: ${error}` });
    } finally {
      loading = false;
    }
  }
</script>

<div class="example-gallery">
  <h3>Load Example File</h3>
  <p class="description">Try out PWF validation with pre-made example files</p>

  <div class="example-grid">
    {#each examples as example}
      <button
        class="example-card"
        class:loading
        on:click={() => loadExample(example)}
        disabled={loading}
      >
        <div class="example-icon">
          {example.type === 'plan' ? '📋' : '📊'}
        </div>
        <div class="example-content">
          <h4>{example.name}</h4>
          <p>{example.description}</p>
          <span class="example-type">{example.type}</span>
        </div>
      </button>
    {/each}
  </div>
</div>

<style>
  .example-gallery {
    margin: 2rem 0;
  }

  h3 {
    font-size: 1.3rem;
    margin-bottom: 0.5rem;
  }

  .description {
    color: var(--text-secondary);
    margin-bottom: 1.5rem;
  }

  .example-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 1rem;
  }

  .example-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1rem;
    cursor: pointer;
    transition: all 0.2s;
    text-align: left;
  }

  .example-card:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    border-color: var(--accent-color);
  }

  .example-card:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .example-icon {
    font-size: 2rem;
    flex-shrink: 0;
  }

  .example-content {
    flex: 1;
  }

  .example-content h4 {
    margin: 0 0 0.25rem 0;
    font-size: 1rem;
    font-weight: 600;
  }

  .example-content p {
    margin: 0 0 0.5rem 0;
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .example-type {
    display: inline-block;
    padding: 0.2rem 0.5rem;
    background: var(--accent-color);
    color: white;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  @media (max-width: 768px) {
    .example-grid {
      grid-template-columns: 1fr;
    }

    .example-card {
      padding: 0.75rem;
    }

    .example-icon {
      font-size: 1.5rem;
    }
  }
</style>
