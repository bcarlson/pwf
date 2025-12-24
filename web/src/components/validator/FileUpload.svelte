<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let accept: string = '.yaml,.yml';
  export let label: string = 'Drop YAML file here or click to browse';

  const dispatch = createEventDispatcher();
  let isDragging = false;
  let fileInput: HTMLInputElement;

  function handleDrop(event: DragEvent) {
    event.preventDefault();
    isDragging = false;

    const files = event.dataTransfer?.files;
    if (files && files.length > 0) {
      handleFile(files[0]);
    }
  }

  function handleDragOver(event: DragEvent) {
    event.preventDefault();
    isDragging = true;
  }

  function handleDragLeave() {
    isDragging = false;
  }

  function handleFileSelect(event: Event) {
    const target = event.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      handleFile(target.files[0]);
    }
  }

  function handleFile(file: File) {
    // Validate file extension
    const validExtensions = accept.split(',').map(ext => ext.trim());
    const fileExtension = '.' + file.name.split('.').pop()?.toLowerCase();

    if (!validExtensions.includes(fileExtension)) {
      dispatch('error', { message: `Invalid file type. Expected: ${accept}` });
      return;
    }

    // Read file content
    const reader = new FileReader();
    reader.onload = (e) => {
      const content = e.target?.result as string;
      dispatch('fileLoaded', { file, content });
    };
    reader.onerror = () => {
      dispatch('error', { message: 'Failed to read file' });
    };
    reader.readAsText(file);
  }

  function openFilePicker() {
    fileInput.click();
  }
</script>

<div
  class="file-upload"
  class:dragging={isDragging}
  on:drop={handleDrop}
  on:dragover={handleDragOver}
  on:dragleave={handleDragLeave}
  on:click={openFilePicker}
  role="button"
  tabindex="0"
  on:keydown={(e) => e.key === 'Enter' && openFilePicker()}
>
  <input
    bind:this={fileInput}
    type="file"
    {accept}
    on:change={handleFileSelect}
    style="display: none"
  />

  <div class="upload-icon">
    {#if isDragging}
      📥
    {:else}
      📄
    {/if}
  </div>

  <p class="upload-label">{label}</p>
  <p class="upload-hint">Supports {accept.replace(/\./g, '').toUpperCase()} files</p>
</div>

<style>
  .file-upload {
    border: 2px dashed var(--border-color);
    border-radius: 12px;
    padding: 3rem 2rem;
    text-align: center;
    cursor: pointer;
    transition: all 0.3s;
    background: var(--bg-secondary);
    margin: 1rem 0;
  }

  .file-upload:hover {
    border-color: var(--accent-color);
    background: var(--bg-hover);
    transform: translateY(-2px);
  }

  .file-upload.dragging {
    border-color: var(--accent-color);
    background: var(--accent-color);
    opacity: 0.1;
    transform: scale(1.02);
  }

  .upload-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
    animation: bounce 2s infinite;
  }

  @keyframes bounce {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-10px); }
  }

  .upload-label {
    font-size: 1.1rem;
    font-weight: 500;
    margin-bottom: 0.5rem;
    color: var(--text-primary);
  }

  .upload-hint {
    font-size: 0.9rem;
    color: var(--text-secondary);
  }

  @media (max-width: 768px) {
    .file-upload {
      padding: 2rem 1rem;
    }

    .upload-icon {
      font-size: 2rem;
    }
  }
</style>
