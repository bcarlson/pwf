<script lang="ts">
  export let currentStep: number = 0;
  export let steps: Array<{ label: string; valid: boolean }> = [];

  function getStepStatus(index: number): 'completed' | 'current' | 'pending' | 'error' {
    if (index < currentStep) {
      return steps[index]?.valid ? 'completed' : 'error';
    } else if (index === currentStep) {
      return 'current';
    }
    return 'pending';
  }

  function getStepIcon(index: number): string {
    const status = getStepStatus(index);
    switch (status) {
      case 'completed':
        return '✓';
      case 'error':
        return '!';
      case 'current':
        return String(index + 1);
      default:
        return String(index + 1);
    }
  }
</script>

<div class="progress-indicator">
  <div class="steps">
    {#each steps as step, i}
      <div class="step-container">
        <div class="step" class:active={i === currentStep} class:completed={getStepStatus(i) === 'completed'} class:error={getStepStatus(i) === 'error'}>
          <div class="step-number">
            {getStepIcon(i)}
          </div>
          <div class="step-label">{step.label}</div>
        </div>
        {#if i < steps.length - 1}
          <div class="step-connector" class:completed={i < currentStep}></div>
        {/if}
      </div>
    {/each}
  </div>

  <div class="progress-bar">
    <div class="progress-fill" style="width: {(currentStep / (steps.length - 1)) * 100}%"></div>
  </div>
</div>

<style>
  .progress-indicator {
    width: 100%;
    padding: 2rem 0;
  }

  .steps {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 1rem;
    position: relative;
  }

  .step-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    flex: 1;
    position: relative;
  }

  .step {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    z-index: 1;
  }

  .step-number {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: var(--bg-secondary);
    border: 2px solid var(--border-color);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 1rem;
    color: var(--text-secondary);
    transition: all 0.3s;
  }

  .step.active .step-number {
    background: var(--accent-color);
    border-color: var(--accent-color);
    color: white;
    transform: scale(1.1);
  }

  .step.completed .step-number {
    background: var(--success-color);
    border-color: var(--success-color);
    color: white;
  }

  .step.error .step-number {
    background: var(--error-color);
    border-color: var(--error-color);
    color: white;
  }

  .step-label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text-secondary);
    text-align: center;
    max-width: 100px;
    transition: color 0.3s;
  }

  .step.active .step-label {
    color: var(--accent-color);
    font-weight: 600;
  }

  .step.completed .step-label,
  .step.error .step-label {
    color: var(--text-primary);
  }

  .step-connector {
    position: absolute;
    top: 20px;
    left: 50%;
    right: -50%;
    height: 2px;
    background: var(--border-color);
    z-index: 0;
    transition: background 0.3s;
  }

  .step-connector.completed {
    background: var(--success-color);
  }

  .step-container:last-child .step-connector {
    display: none;
  }

  .progress-bar {
    width: 100%;
    height: 4px;
    background: var(--bg-secondary);
    border-radius: 2px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, var(--accent-color), var(--success-color));
    transition: width 0.3s ease;
  }

  @media (max-width: 768px) {
    .step-label {
      font-size: 0.75rem;
      max-width: 70px;
    }

    .step-number {
      width: 32px;
      height: 32px;
      font-size: 0.875rem;
    }

    .step-connector {
      top: 16px;
    }
  }

  @media (max-width: 480px) {
    .steps {
      flex-direction: column;
      gap: 1.5rem;
    }

    .step-container {
      width: 100%;
    }

    .step {
      flex-direction: row;
      justify-content: flex-start;
      width: 100%;
    }

    .step-label {
      text-align: left;
      max-width: none;
    }

    .step-connector {
      display: none;
    }
  }
</style>
