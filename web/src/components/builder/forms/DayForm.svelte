<script lang="ts">
  import { builderState, type Day } from '../../../lib/builderState';

  export let dayIndex: number;

  $: day = $builderState.plan.cycle.days[dayIndex];

  function updateDay(field: keyof Day, value: any) {
    builderState.updateDay(dayIndex, { [field]: value });
  }
</script>

<div class="day-form">
  <div class="form-group">
    <label for="day-focus-{dayIndex}">
      Focus
      <span class="optional">optional</span>
    </label>
    <input
      id="day-focus-{dayIndex}"
      type="text"
      placeholder="e.g., Upper Body, Cardio, Recovery"
      value={day?.focus || ''}
      on:input={(e) => updateDay('focus', e.currentTarget.value || undefined)}
    />
  </div>

  <div class="form-group">
    <label for="day-notes-{dayIndex}">
      Notes
      <span class="optional">optional</span>
    </label>
    <textarea
      id="day-notes-{dayIndex}"
      rows="2"
      placeholder="Add any notes for this day..."
      value={day?.notes || ''}
      on:input={(e) => updateDay('notes', e.currentTarget.value || undefined)}
    />
  </div>
</div>

<style>
  .day-form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
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
    font-size: 0.9rem;
  }

  .optional {
    font-size: 0.8rem;
    font-weight: 400;
    color: var(--text-secondary);
    font-style: italic;
  }

  input,
  textarea {
    font-size: 0.95rem;
  }
</style>
