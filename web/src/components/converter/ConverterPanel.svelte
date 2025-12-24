<script lang="ts">
  import FormatSelector from './FormatSelector.svelte';
  import ConversionResults from './ConversionResults.svelte';
  import FileUpload from '../validator/FileUpload.svelte';
  import {
    fitToPwf,
    tcxToPwf,
    gpxToPwf,
    pwfToTcx,
    pwfToGpx,
    pwfToCsv
  } from '../../lib/wasm';

  let sourceFormat: string = '';
  let targetFormat: string = '';
  let sourceFile: File | null = null;
  let sourceContent: string = '';
  let conversionResult: any = null;
  let isConverting: boolean = false;
  let errorMessage: string = '';

  $: canConvert = sourceFormat && targetFormat && sourceFile;
  $: acceptedExtensions = getAcceptedExtensions(sourceFormat);

  function getAcceptedExtensions(format: string): string {
    const extensions: Record<string, string> = {
      fit: '.fit',
      tcx: '.tcx',
      gpx: '.gpx',
      pwf: '.yaml,.yml'
    };
    return extensions[format] || '';
  }

  function handleFormatChange(event: CustomEvent) {
    const { sourceFormat: src, targetFormat: tgt } = event.detail;
    sourceFormat = src;
    targetFormat = tgt;

    // Reset file and results when formats change
    sourceFile = null;
    sourceContent = '';
    conversionResult = null;
    errorMessage = '';
  }

  function handleFileLoaded(event: CustomEvent) {
    const { file, content } = event.detail;
    sourceFile = file;
    sourceContent = content;
    errorMessage = '';
    conversionResult = null;
  }

  function handleError(event: CustomEvent) {
    errorMessage = event.detail.message;
  }

  async function performConversion() {
    if (!sourceFile || !sourceContent) {
      errorMessage = 'Please upload a file first';
      return;
    }

    isConverting = true;
    errorMessage = '';
    conversionResult = null;

    try {
      // For binary files (FIT), we need to read as ArrayBuffer
      if (sourceFormat === 'fit') {
        const arrayBuffer = await sourceFile.arrayBuffer();
        const bytes = new Uint8Array(arrayBuffer);

        if (targetFormat === 'pwf') {
          conversionResult = fitToPwf(bytes, false);
        } else {
          errorMessage = 'FIT can only be converted to PWF format';
          return;
        }
      }
      // For text-based imports (TCX, GPX)
      else if (sourceFormat === 'tcx' || sourceFormat === 'gpx') {
        const encoder = new TextEncoder();
        const bytes = encoder.encode(sourceContent);

        if (targetFormat === 'pwf') {
          if (sourceFormat === 'tcx') {
            conversionResult = tcxToPwf(bytes, false);
          } else {
            conversionResult = gpxToPwf(bytes, false);
          }
        } else {
          errorMessage = `${sourceFormat.toUpperCase()} can only be converted to PWF format`;
          return;
        }
      }
      // For PWF exports
      else if (sourceFormat === 'pwf') {
        if (targetFormat === 'tcx') {
          conversionResult = pwfToTcx(sourceContent);
        } else if (targetFormat === 'gpx') {
          conversionResult = pwfToGpx(sourceContent);
        } else if (targetFormat === 'csv') {
          conversionResult = pwfToCsv(sourceContent);
        } else {
          errorMessage = 'Invalid target format for PWF conversion';
          return;
        }
      }

      // Check if conversion returned an error
      if (conversionResult?.error) {
        errorMessage = conversionResult.error;
      }
    } catch (error) {
      console.error('Conversion error:', error);
      errorMessage = `Conversion failed: ${error}`;
      conversionResult = null;
    } finally {
      isConverting = false;
    }
  }

  function clearAll() {
    sourceFormat = '';
    targetFormat = '';
    sourceFile = null;
    sourceContent = '';
    conversionResult = null;
    errorMessage = '';
  }
</script>

<div class="converter-panel">
  <div class="panel-intro">
    <h2>Format Converter</h2>
    <p>Convert between FIT, TCX, GPX, PWF, and CSV formats</p>
  </div>

  <!-- Format Selection -->
  <FormatSelector
    bind:sourceFormat
    bind:targetFormat
    on:change={handleFormatChange}
  />

  {#if sourceFormat && targetFormat}
    <!-- File Upload -->
    {#if !sourceFile}
      <FileUpload
        on:fileLoaded={handleFileLoaded}
        on:error={handleError}
        accept={acceptedExtensions}
      />
    {:else}
      <!-- File Info and Convert Button -->
      <div class="file-info">
        <div class="file-details">
          <span class="file-icon">📄</span>
          <div>
            <p class="file-name">{sourceFile.name}</p>
            <p class="file-size">{(sourceFile.size / 1024).toFixed(2)} KB</p>
          </div>
        </div>
        <button class="btn-secondary" on:click={clearAll}>
          Change File
        </button>
      </div>

      <div class="convert-actions">
        <button
          class="btn btn-primary"
          on:click={performConversion}
          disabled={isConverting || !canConvert}
        >
          {#if isConverting}
            Converting...
          {:else}
            Convert to {targetFormat.toUpperCase()}
          {/if}
        </button>
      </div>
    {/if}

    <!-- Error Message -->
    {#if errorMessage}
      <div class="error-banner">
        <span class="error-icon">⚠️</span>
        {errorMessage}
      </div>
    {/if}

    <!-- Conversion Results -->
    {#if conversionResult && !conversionResult.error}
      <ConversionResults
        {conversionResult}
        {targetFormat}
        sourceFileName={sourceFile?.name || 'converted'}
      />
    {/if}
  {:else}
    <div class="format-hint">
      <p>Please select both source and target formats to begin conversion</p>
    </div>
  {/if}
</div>

<style>
  .converter-panel {
    max-width: 1000px;
    margin: 0 auto;
  }

  .panel-intro {
    margin-bottom: 2rem;
  }

  .panel-intro h2 {
    margin: 0 0 0.5rem 0;
    font-size: 1.5rem;
  }

  .panel-intro p {
    margin: 0;
    color: var(--text-secondary);
  }

  .file-info {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1.5rem;
    margin-bottom: 1.5rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 1rem;
  }

  .file-details {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .file-icon {
    font-size: 2rem;
  }

  .file-name {
    margin: 0;
    font-weight: 600;
    font-size: 1.1rem;
  }

  .file-size {
    margin: 0.25rem 0 0 0;
    font-size: 0.9rem;
    color: var(--text-secondary);
  }

  .convert-actions {
    display: flex;
    gap: 1rem;
    justify-content: center;
    margin: 1.5rem 0;
  }

  .error-banner {
    background: rgba(220, 53, 69, 0.1);
    border: 1px solid var(--error-color);
    border-radius: 8px;
    padding: 1rem;
    margin: 1rem 0;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    color: var(--error-color);
  }

  .error-icon {
    font-size: 1.5rem;
  }

  .format-hint {
    text-align: center;
    padding: 3rem 2rem;
    color: var(--text-secondary);
  }

  .format-hint p {
    margin: 0;
    font-size: 1.1rem;
  }

  @media (max-width: 768px) {
    .file-info {
      flex-direction: column;
      align-items: flex-start;
    }

    .convert-actions {
      flex-direction: column;
    }
  }
</style>
