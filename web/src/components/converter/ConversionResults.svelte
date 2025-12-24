<script lang="ts">
  export let conversionResult: any;
  export let targetFormat: string;
  export let sourceFileName: string;

  let showFullContent = false;

  $: hasWarnings = conversionResult?.warnings && conversionResult.warnings.length > 0;
  $: hasError = conversionResult?.error;
  $: content = getContent();
  $: previewLines = content.split('\n').slice(0, 100);
  $: totalLines = content.split('\n').length;
  $: hasMoreLines = totalLines > 100;

  function getContent(): string {
    if (hasError) return '';

    if (targetFormat === 'tcx' && conversionResult?.tcx_xml) {
      return conversionResult.tcx_xml;
    } else if (targetFormat === 'gpx' && conversionResult?.gpx_xml) {
      return conversionResult.gpx_xml;
    } else if (targetFormat === 'csv' && conversionResult?.csv_data) {
      return conversionResult.csv_data;
    } else if (targetFormat === 'pwf' && conversionResult?.pwf_yaml) {
      return conversionResult.pwf_yaml;
    }
    return '';
  }

  function getFileExtension(): string {
    const extensions: Record<string, string> = {
      tcx: '.tcx',
      gpx: '.gpx',
      csv: '.csv',
      pwf: '.yaml'
    };
    return extensions[targetFormat] || '.txt';
  }

  function getFileName(): string {
    const baseName = sourceFileName.replace(/\.[^/.]+$/, '');
    return `${baseName}${getFileExtension()}`;
  }

  function downloadFile() {
    const blob = new Blob([content], {
      type: targetFormat === 'csv' ? 'text/csv' :
            targetFormat === 'pwf' ? 'text/yaml' :
            'application/xml'
    });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = getFileName();
    a.click();
    URL.revokeObjectURL(url);
  }

  function copyToClipboard() {
    navigator.clipboard.writeText(content).then(() => {
      alert('Copied to clipboard!');
    }).catch(err => {
      console.error('Failed to copy:', err);
      alert('Failed to copy to clipboard');
    });
  }
</script>

<div class="conversion-results">
  {#if hasError}
    <div class="error-section">
      <div class="error-header">
        <span class="error-icon">✗</span>
        <h3>Conversion Failed</h3>
      </div>
      <p class="error-message">{conversionResult.error}</p>
    </div>
  {:else}
    <div class="success-section">
      <div class="success-header">
        <span class="success-icon">✓</span>
        <h3>Conversion Successful</h3>
      </div>
      <p class="success-message">
        File converted to {targetFormat.toUpperCase()} format
      </p>
    </div>

    {#if hasWarnings}
      <div class="warnings-section">
        <h4 class="section-title">
          <span class="warning-icon">⚠️</span>
          Warnings ({conversionResult.warnings.length})
        </h4>
        <ul class="warnings-list">
          {#each conversionResult.warnings as warning}
            <li>{warning}</li>
          {/each}
        </ul>
      </div>
    {/if}

    <div class="actions">
      <button class="btn btn-primary" on:click={downloadFile}>
        ⬇️ Download {getFileName()}
      </button>
      <button class="btn btn-secondary" on:click={copyToClipboard}>
        📋 Copy to Clipboard
      </button>
    </div>

    <div class="preview-section">
      <div class="preview-header">
        <h4>Preview</h4>
        {#if hasMoreLines}
          <button class="btn-link" on:click={() => showFullContent = !showFullContent}>
            {showFullContent ? 'Show less' : `Show all ${totalLines} lines`}
          </button>
        {/if}
      </div>
      <pre class="content-preview">{showFullContent ? content : previewLines.join('\n')}{#if !showFullContent && hasMoreLines}

...
(showing first 100 of {totalLines} lines){/if}</pre>
    </div>
  {/if}
</div>

<style>
  .conversion-results {
    margin-top: 1.5rem;
  }

  .error-section,
  .success-section {
    padding: 1.5rem;
    border-radius: 8px;
    margin-bottom: 1.5rem;
  }

  .error-section {
    background: rgba(220, 53, 69, 0.1);
    border: 1px solid var(--error-color);
  }

  .success-section {
    background: rgba(25, 135, 84, 0.1);
    border: 1px solid var(--success-color);
  }

  .error-header,
  .success-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 0.5rem;
  }

  .error-icon {
    font-size: 2rem;
    color: var(--error-color);
  }

  .success-icon {
    font-size: 2rem;
    color: var(--success-color);
  }

  h3 {
    margin: 0;
    font-size: 1.3rem;
  }

  .error-message,
  .success-message {
    margin: 0;
    opacity: 0.9;
  }

  .warnings-section {
    background: rgba(255, 193, 7, 0.1);
    border: 1px solid var(--warning-color);
    border-radius: 8px;
    padding: 1rem;
    margin-bottom: 1.5rem;
  }

  .section-title {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin: 0 0 0.75rem 0;
    font-size: 1.1rem;
    font-weight: 600;
  }

  .warning-icon {
    font-size: 1.2rem;
  }

  .warnings-list {
    margin: 0;
    padding-left: 1.5rem;
  }

  .warnings-list li {
    margin-bottom: 0.5rem;
    color: var(--text-secondary);
  }

  .actions {
    display: flex;
    gap: 1rem;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
  }

  .preview-section {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1rem;
  }

  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.75rem;
  }

  .preview-header h4 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
  }

  .btn-link {
    background: none;
    border: none;
    color: var(--accent-color);
    cursor: pointer;
    font-size: 0.9rem;
    text-decoration: underline;
    padding: 0;
  }

  .btn-link:hover {
    color: var(--accent-color);
    opacity: 0.8;
  }

  .content-preview {
    margin: 0;
    padding: 1rem;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    overflow-x: auto;
    font-family: 'Courier New', monospace;
    font-size: 0.85rem;
    line-height: 1.5;
    max-height: 600px;
    overflow-y: auto;
  }

  @media (max-width: 768px) {
    .actions {
      flex-direction: column;
    }

    .preview-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.5rem;
    }
  }
</style>
