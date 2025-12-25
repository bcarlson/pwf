<script lang="ts">
  import { builderState, type Meta } from '../../../lib/builderState';
  import { getSupportedEquipment } from '../../../lib/wasm';

  let supportedEquipment: string[] = [];
  let tagInput = '';

  $: meta = $builderState.plan.meta || {};

  // Load supported equipment from WASM
  try {
    supportedEquipment = getSupportedEquipment();
  } catch (e) {
    console.warn('Could not load equipment list:', e);
    supportedEquipment = ['barbell', 'dumbbell', 'kettlebell', 'resistance_band', 'pull_up_bar'];
  }

  function updateMeta(field: keyof Meta, value: any) {
    builderState.updateMeta({ [field]: value });
  }

  function addTag() {
    if (!tagInput.trim()) return;

    const currentTags = meta.tags || [];
    if (!currentTags.includes(tagInput.trim())) {
      updateMeta('tags', [...currentTags, tagInput.trim()]);
    }
    tagInput = '';
  }

  function removeTag(tag: string) {
    const currentTags = meta.tags || [];
    updateMeta('tags', currentTags.filter(t => t !== tag));
  }

  function toggleEquipment(equipment: string) {
    const currentEquipment = meta.equipment || [];
    if (currentEquipment.includes(equipment)) {
      updateMeta('equipment', currentEquipment.filter(e => e !== equipment));
    } else {
      updateMeta('equipment', [...currentEquipment, equipment]);
    }
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      event.preventDefault();
      addTag();
    }
  }
</script>

<div class="meta-form">
  <div class="form-group">
    <label for="plan-name">
      Plan Name
      <span class="optional">optional</span>
    </label>
    <input
      id="plan-name"
      type="text"
      placeholder="e.g., Beginner Strength Training"
      value={meta.name || ''}
      on:input={(e) => updateMeta('name', e.currentTarget.value)}
    />
  </div>

  <div class="form-group">
    <label for="plan-description">
      Description
      <span class="optional">optional</span>
    </label>
    <textarea
      id="plan-description"
      rows="3"
      placeholder="Describe your workout plan..."
      value={meta.description || ''}
      on:input={(e) => updateMeta('description', e.currentTarget.value)}
    />
  </div>

  <div class="form-group">
    <label for="plan-author">
      Author
      <span class="optional">optional</span>
    </label>
    <input
      id="plan-author"
      type="text"
      placeholder="Your name"
      value={meta.author || ''}
      on:input={(e) => updateMeta('author', e.currentTarget.value)}
    />
  </div>

  <div class="form-group">
    <label for="days-per-week">
      Days Per Week
      <span class="optional">optional</span>
    </label>
    <input
      id="days-per-week"
      type="number"
      min="1"
      max="7"
      placeholder="e.g., 3"
      value={meta.days_per_week || ''}
      on:input={(e) => {
        const val = parseInt(e.currentTarget.value);
        updateMeta('days_per_week', isNaN(val) ? undefined : val);
      }}
    />
  </div>

  <div class="form-group">
    <label>
      Equipment
      <span class="optional">optional</span>
    </label>
    <div class="equipment-grid">
      {#each supportedEquipment as equipment}
        <button
          type="button"
          class="equipment-chip"
          class:selected={meta.equipment?.includes(equipment)}
          on:click={() => toggleEquipment(equipment)}
        >
          {equipment.replace(/_/g, ' ')}
        </button>
      {/each}
    </div>
  </div>

  <div class="form-group">
    <label for="tag-input">
      Tags
      <span class="optional">optional</span>
    </label>
    <div class="tag-input-wrapper">
      <input
        id="tag-input"
        type="text"
        placeholder="Add tag and press Enter"
        bind:value={tagInput}
        on:keydown={handleKeyDown}
      />
      <button type="button" class="btn-add-tag" on:click={addTag}>
        Add
      </button>
    </div>
    {#if meta.tags && meta.tags.length > 0}
      <div class="tags-list">
        {#each meta.tags as tag}
          <div class="tag-chip">
            <span>{tag}</span>
            <button
              type="button"
              class="remove-tag"
              on:click={() => removeTag(tag)}
              aria-label="Remove tag"
            >
              ×
            </button>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .meta-form {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  label {
    font-weight: 600;
    color: var(--text-primary);
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .optional {
    font-size: 0.85rem;
    font-weight: 400;
    color: var(--text-secondary);
    font-style: italic;
  }

  .equipment-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
    gap: 0.5rem;
  }

  .equipment-chip {
    padding: 0.5rem 1rem;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    background: var(--bg-primary);
    color: var(--text-primary);
    cursor: pointer;
    transition: all 0.2s;
    text-transform: capitalize;
    font-size: 0.9rem;
  }

  .equipment-chip:hover {
    background: var(--bg-hover);
    border-color: var(--accent-color);
  }

  .equipment-chip.selected {
    background: var(--accent-color);
    color: white;
    border-color: var(--accent-color);
  }

  .tag-input-wrapper {
    display: flex;
    gap: 0.5rem;
  }

  .tag-input-wrapper input {
    flex: 1;
  }

  .btn-add-tag {
    padding: 0.75rem 1.5rem;
    background: var(--accent-color);
    color: white;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 500;
    transition: background 0.2s;
  }

  .btn-add-tag:hover {
    background: var(--accent-hover);
  }

  .tags-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-top: 0.5rem;
  }

  .tag-chip {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    font-size: 0.9rem;
  }

  .remove-tag {
    background: none;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 1.5rem;
    line-height: 1;
    padding: 0;
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    transition: all 0.2s;
  }

  .remove-tag:hover {
    background: var(--error-color);
    color: white;
  }

  @media (max-width: 768px) {
    .equipment-grid {
      grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    }

    .tag-input-wrapper {
      flex-direction: column;
    }
  }
</style>
