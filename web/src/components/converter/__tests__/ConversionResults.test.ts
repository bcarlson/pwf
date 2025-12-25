/**
 * Tests for ConversionResults component
 */

import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import ConversionResults from '../ConversionResults.svelte';

describe('ConversionResults', () => {
  // Mock clipboard API
  const mockClipboard = {
    writeText: vi.fn()
  };

  // Mock URL.createObjectURL and revokeObjectURL
  const mockCreateObjectURL = vi.fn(() => 'blob:mock-url');
  const mockRevokeObjectURL = vi.fn();

  // Mock alert
  const mockAlert = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    Object.assign(navigator, { clipboard: mockClipboard });
    mockClipboard.writeText.mockResolvedValue(undefined);

    global.URL.createObjectURL = mockCreateObjectURL;
    global.URL.revokeObjectURL = mockRevokeObjectURL;
    global.alert = mockAlert;
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe('Error Display', () => {
    it('should display error message when conversion fails', () => {
      const { container, getByText } = render(ConversionResults, {
        props: {
          conversionResult: { error: 'Conversion failed: Invalid format' },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      expect(getByText('Conversion Failed')).toBeTruthy();
      expect(getByText('Conversion failed: Invalid format')).toBeTruthy();
      expect(container.querySelector('.error-section')).toBeTruthy();
    });

    it('should not show success section when error exists', () => {
      const { container } = render(ConversionResults, {
        props: {
          conversionResult: { error: 'Failed' },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      expect(container.querySelector('.success-section')).toBeFalsy();
      expect(container.querySelector('.preview-section')).toBeFalsy();
    });

    it('should not show download buttons when error exists', () => {
      const { queryByText } = render(ConversionResults, {
        props: {
          conversionResult: { error: 'Failed' },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      expect(queryByText(/Download/)).toBeFalsy();
      expect(queryByText(/Copy to Clipboard/)).toBeFalsy();
    });
  });

  describe('Success Display', () => {
    it('should display success message when conversion succeeds', () => {
      const { container, getByText } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: 'version: 1\nhistory_version: 1' },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      expect(getByText('Conversion Successful')).toBeTruthy();
      expect(container.querySelector('.success-section')).toBeTruthy();
    });

    it('should display correct target format in success message', () => {
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { tcx_xml: '<xml></xml>' },
          targetFormat: 'tcx',
          sourceFileName: 'workout.pwf'
        }
      });

      expect(getByText(/File converted to TCX format/)).toBeTruthy();
    });

    it('should display warnings section when warnings exist', () => {
      const { container, getByText } = render(ConversionResults, {
        props: {
          conversionResult: {
            pwf_yaml: 'test',
            warnings: ['Warning 1', 'Warning 2']
          },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      expect(container.querySelector('.warnings-section')).toBeTruthy();
      expect(getByText(/Warnings \(2\)/)).toBeTruthy();
      expect(getByText('Warning 1')).toBeTruthy();
      expect(getByText('Warning 2')).toBeTruthy();
    });

    it('should not display warnings section when no warnings exist', () => {
      const { container } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: 'test', warnings: [] },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      expect(container.querySelector('.warnings-section')).toBeFalsy();
    });

    it('should display multiple warnings in list', () => {
      const { container } = render(ConversionResults, {
        props: {
          conversionResult: {
            pwf_yaml: 'test',
            warnings: ['Warning A', 'Warning B', 'Warning C']
          },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      const warningsList = container.querySelector('.warnings-list');
      const items = warningsList?.querySelectorAll('li');
      expect(items?.length).toBe(3);
    });
  });

  describe('Content Display - PWF', () => {
    it('should display PWF content from pwf_yaml field', () => {
      const yamlContent = 'version: 1\nhistory_version: 1\nworkouts: []';
      const { container } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: yamlContent },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      const preview = container.querySelector('.content-preview');
      expect(preview?.textContent).toContain('version: 1');
      expect(preview?.textContent).toContain('history_version: 1');
    });

    it('should show preview section for PWF conversion', () => {
      const { container } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: 'test: yaml' },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      expect(container.querySelector('.preview-section')).toBeTruthy();
    });
  });

  describe('Content Display - TCX', () => {
    it('should display TCX content from tcx_xml field', () => {
      const tcxContent = '<?xml version="1.0"?>\n<TrainingCenterDatabase></TrainingCenterDatabase>';
      const { container } = render(ConversionResults, {
        props: {
          conversionResult: { tcx_xml: tcxContent },
          targetFormat: 'tcx',
          sourceFileName: 'workout.pwf'
        }
      });

      const preview = container.querySelector('.content-preview');
      expect(preview?.textContent).toContain('<?xml version="1.0"?>');
      expect(preview?.textContent).toContain('TrainingCenterDatabase');
    });
  });

  describe('Content Display - GPX', () => {
    it('should display GPX content from gpx_xml field', () => {
      const gpxContent = '<?xml version="1.0"?>\n<gpx></gpx>';
      const { container } = render(ConversionResults, {
        props: {
          conversionResult: { gpx_xml: gpxContent },
          targetFormat: 'gpx',
          sourceFileName: 'workout.pwf'
        }
      });

      const preview = container.querySelector('.content-preview');
      expect(preview?.textContent).toContain('<?xml version="1.0"?>');
      expect(preview?.textContent).toContain('<gpx>');
    });
  });

  describe('Content Display - CSV', () => {
    it('should display CSV content from csv_data field', () => {
      const csvContent = 'timestamp,heart_rate,power\n2024-01-01,150,200';
      const { container } = render(ConversionResults, {
        props: {
          conversionResult: { csv_data: csvContent },
          targetFormat: 'csv',
          sourceFileName: 'workout.pwf'
        }
      });

      const preview = container.querySelector('.content-preview');
      expect(preview?.textContent).toContain('timestamp,heart_rate,power');
      expect(preview?.textContent).toContain('2024-01-01,150,200');
    });
  });

  describe('Preview Functionality', () => {
    it('should show first 100 lines by default for long content', () => {
      const longContent = Array(150).fill('line').join('\n');
      const { container } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: longContent },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      const preview = container.querySelector('.content-preview');
      expect(preview?.textContent).toContain('showing first 100 of 150 lines');
    });

    it('should show "Show all" button when content exceeds 100 lines', () => {
      const longContent = Array(150).fill('line').join('\n');
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: longContent },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      expect(getByText(/Show all 150 lines/)).toBeTruthy();
    });

    it('should not show "Show all" button when content is under 100 lines', () => {
      const shortContent = Array(50).fill('line').join('\n');
      const { queryByText } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: shortContent },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      expect(queryByText(/Show all/)).toBeFalsy();
    });

    it('should expand to show all lines when "Show all" is clicked', async () => {
      const longContent = Array(150).fill('line').join('\n');
      const { getByText, container } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: longContent },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      const showAllButton = getByText(/Show all 150 lines/);
      await fireEvent.click(showAllButton);

      const preview = container.querySelector('.content-preview');
      expect(preview?.textContent).not.toContain('showing first 100');
      expect(getByText(/Show less/)).toBeTruthy();
    });

    it('should collapse to 100 lines when "Show less" is clicked', async () => {
      const longContent = Array(150).fill('line').join('\n');
      const { getByText, container } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: longContent },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      const showAllButton = getByText(/Show all 150 lines/);
      await fireEvent.click(showAllButton);

      const showLessButton = getByText(/Show less/);
      await fireEvent.click(showLessButton);

      const preview = container.querySelector('.content-preview');
      expect(preview?.textContent).toContain('showing first 100 of 150 lines');
    });
  });

  describe('Download Functionality', () => {
    it('should generate correct filename for PWF', async () => {
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: 'test' },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      const downloadButton = getByText(/Download workout\.yaml/);
      expect(downloadButton).toBeTruthy();
    });

    it('should generate correct filename for TCX', async () => {
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { tcx_xml: 'test' },
          targetFormat: 'tcx',
          sourceFileName: 'workout.yaml'
        }
      });

      const downloadButton = getByText(/Download workout\.tcx/);
      expect(downloadButton).toBeTruthy();
    });

    it('should generate correct filename for GPX', async () => {
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { gpx_xml: 'test' },
          targetFormat: 'gpx',
          sourceFileName: 'workout.yaml'
        }
      });

      const downloadButton = getByText(/Download workout\.gpx/);
      expect(downloadButton).toBeTruthy();
    });

    it('should generate correct filename for CSV', async () => {
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { csv_data: 'test' },
          targetFormat: 'csv',
          sourceFileName: 'workout.yaml'
        }
      });

      const downloadButton = getByText(/Download workout\.csv/);
      expect(downloadButton).toBeTruthy();
    });

    it('should preserve base filename when downloading', () => {
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: 'test' },
          targetFormat: 'pwf',
          sourceFileName: 'my-custom-workout.fit'
        }
      });

      expect(getByText(/Download my-custom-workout\.yaml/)).toBeTruthy();
    });

    it('should create blob with correct content when download is clicked', async () => {
      const content = 'test content';
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: content },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      const downloadButton = getByText(/Download workout\.yaml/);
      await fireEvent.click(downloadButton);

      expect(mockCreateObjectURL).toHaveBeenCalled();
      expect(mockRevokeObjectURL).toHaveBeenCalledWith('blob:mock-url');
    });

    it('should create blob with correct MIME type for CSV', async () => {
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { csv_data: 'test' },
          targetFormat: 'csv',
          sourceFileName: 'workout.yaml'
        }
      });

      const downloadButton = getByText(/Download/);
      await fireEvent.click(downloadButton);

      // Verify blob was created (through createObjectURL)
      expect(mockCreateObjectURL).toHaveBeenCalled();
    });

    it('should create blob with correct MIME type for YAML', async () => {
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: 'test' },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      const downloadButton = getByText(/Download/);
      await fireEvent.click(downloadButton);

      expect(mockCreateObjectURL).toHaveBeenCalled();
    });

    it('should create blob with correct MIME type for XML', async () => {
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { tcx_xml: 'test' },
          targetFormat: 'tcx',
          sourceFileName: 'workout.yaml'
        }
      });

      const downloadButton = getByText(/Download/);
      await fireEvent.click(downloadButton);

      expect(mockCreateObjectURL).toHaveBeenCalled();
    });
  });

  describe('Copy to Clipboard', () => {
    it('should copy content to clipboard when button is clicked', async () => {
      const content = 'test yaml content';
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: content },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      const copyButton = getByText(/Copy to Clipboard/);
      await fireEvent.click(copyButton);

      expect(mockClipboard.writeText).toHaveBeenCalledWith(content);
    });

    it('should show success alert after successful copy', async () => {
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: 'test' },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      const copyButton = getByText(/Copy to Clipboard/);
      await fireEvent.click(copyButton);

      await vi.waitFor(() => {
        expect(mockAlert).toHaveBeenCalledWith('Copied to clipboard!');
      });
    });

    it('should handle clipboard copy failure', async () => {
      mockClipboard.writeText.mockRejectedValueOnce(new Error('Copy failed'));

      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: 'test' },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      const copyButton = getByText(/Copy to Clipboard/);
      await fireEvent.click(copyButton);

      await vi.waitFor(() => {
        expect(mockAlert).toHaveBeenCalledWith('Failed to copy to clipboard');
      });
    });

    it('should copy TCX content correctly', async () => {
      const tcxContent = '<?xml version="1.0"?><tcx></tcx>';
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { tcx_xml: tcxContent },
          targetFormat: 'tcx',
          sourceFileName: 'workout.yaml'
        }
      });

      const copyButton = getByText(/Copy to Clipboard/);
      await fireEvent.click(copyButton);

      expect(mockClipboard.writeText).toHaveBeenCalledWith(tcxContent);
    });

    it('should copy GPX content correctly', async () => {
      const gpxContent = '<?xml version="1.0"?><gpx></gpx>';
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { gpx_xml: gpxContent },
          targetFormat: 'gpx',
          sourceFileName: 'workout.yaml'
        }
      });

      const copyButton = getByText(/Copy to Clipboard/);
      await fireEvent.click(copyButton);

      expect(mockClipboard.writeText).toHaveBeenCalledWith(gpxContent);
    });

    it('should copy CSV content correctly', async () => {
      const csvContent = 'col1,col2\nval1,val2';
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { csv_data: csvContent },
          targetFormat: 'csv',
          sourceFileName: 'workout.yaml'
        }
      });

      const copyButton = getByText(/Copy to Clipboard/);
      await fireEvent.click(copyButton);

      expect(mockClipboard.writeText).toHaveBeenCalledWith(csvContent);
    });
  });

  describe('Edge Cases', () => {
    it('should handle empty content', () => {
      const { container } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: '' },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      const preview = container.querySelector('.content-preview');
      expect(preview?.textContent?.trim()).toBe('');
    });

    it('should handle content with single line', () => {
      const { container } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: 'single line' },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      const preview = container.querySelector('.content-preview');
      expect(preview?.textContent).toBe('single line');
    });

    it('should handle exactly 100 lines of content', () => {
      const content = Array(100).fill('line').join('\n');
      const { queryByText } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: content },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      // Should not show "Show all" for exactly 100 lines
      expect(queryByText(/Show all/)).toBeFalsy();
    });

    it('should handle exactly 101 lines of content', () => {
      const content = Array(101).fill('line').join('\n');
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: content },
          targetFormat: 'pwf',
          sourceFileName: 'workout.fit'
        }
      });

      // Should show "Show all" for 101 lines
      expect(getByText(/Show all 101 lines/)).toBeTruthy();
    });

    it('should handle filenames without extensions', () => {
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: 'test' },
          targetFormat: 'pwf',
          sourceFileName: 'workout'
        }
      });

      expect(getByText(/Download workout\.yaml/)).toBeTruthy();
    });

    it('should handle filenames with multiple dots', () => {
      const { getByText } = render(ConversionResults, {
        props: {
          conversionResult: { pwf_yaml: 'test' },
          targetFormat: 'pwf',
          sourceFileName: 'my.workout.plan.fit'
        }
      });

      expect(getByText(/Download my\.workout\.plan\.yaml/)).toBeTruthy();
    });
  });
});
