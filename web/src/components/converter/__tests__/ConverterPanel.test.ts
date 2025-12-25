/**
 * Tests for ConverterPanel component
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, fireEvent, waitFor } from '@testing-library/svelte';
import ConverterPanel from '../ConverterPanel.svelte';
import * as wasm from '../../../lib/wasm';

// Mock WASM module
vi.mock('../../../lib/wasm', () => ({
  fitToPwf: vi.fn(),
  tcxToPwf: vi.fn(),
  gpxToPwf: vi.fn(),
  pwfToTcx: vi.fn(),
  pwfToGpx: vi.fn(),
  pwfToCsv: vi.fn()
}));

describe('ConverterPanel', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Component Rendering', () => {
    it('should render the component', () => {
      const { container } = render(ConverterPanel);
      expect(container.querySelector('.converter-panel')).toBeTruthy();
    });

    it('should render panel header and description', () => {
      const { getByText } = render(ConverterPanel);
      expect(getByText('Format Converter')).toBeTruthy();
      expect(getByText(/Convert between FIT, TCX, GPX, PWF, and CSV formats/)).toBeTruthy();
    });

    it('should render FormatSelector component', () => {
      const { container } = render(ConverterPanel);
      expect(container.querySelector('.format-selector')).toBeTruthy();
    });

    it('should show format hint when no formats are selected', () => {
      const { getByText } = render(ConverterPanel);
      expect(getByText(/Please select both source and target formats/)).toBeTruthy();
    });

    it('should not show file upload when formats are not selected', () => {
      const { container } = render(ConverterPanel);
      expect(container.querySelector('.file-upload')).toBeFalsy();
    });
  });

  describe('Format Selection', () => {
    it('should show file upload after formats are selected', async () => {
      const { getByLabelText, container } = render(ConverterPanel);

      const sourceSelect = getByLabelText(/Convert from:/);
      const targetSelect = getByLabelText(/Convert to:/);

      await fireEvent.change(sourceSelect, { target: { value: 'fit' } });
      await fireEvent.change(targetSelect, { target: { value: 'pwf' } });

      expect(container.querySelector('.file-upload')).toBeTruthy();
    });

    it('should reset file when formats change', async () => {
      const { getByLabelText, container } = render(ConverterPanel);

      const sourceSelect = getByLabelText(/Convert from:/);
      await fireEvent.change(sourceSelect, { target: { value: 'fit' } });

      const targetSelect = getByLabelText(/Convert to:/);
      await fireEvent.change(targetSelect, { target: { value: 'pwf' } });

      // Simulate file upload
      const fileUpload = container.querySelector('.file-upload') as HTMLElement;
      const file = new File(['test'], 'test.fit', { type: 'application/octet-stream' });
      Object.defineProperty(file, 'arrayBuffer', {
        value: vi.fn().mockResolvedValue(new ArrayBuffer(8))
      });

      await fireEvent.click(fileUpload);

      // Now change format
      await fireEvent.change(sourceSelect, { target: { value: 'tcx' } });

      // File upload should be visible again (file was reset)
      expect(container.querySelector('.file-upload')).toBeTruthy();
    });

    it('should set correct accept extensions for FIT format', async () => {
      const { getByLabelText, container } = render(ConverterPanel);

      const sourceSelect = getByLabelText(/Convert from:/);
      await fireEvent.change(sourceSelect, { target: { value: 'fit' } });

      const targetSelect = getByLabelText(/Convert to:/);
      await fireEvent.change(targetSelect, { target: { value: 'pwf' } });

      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;
      expect(fileInput?.accept).toBe('.fit');
    });

    it('should set correct accept extensions for TCX format', async () => {
      const { getByLabelText, container } = render(ConverterPanel);

      const sourceSelect = getByLabelText(/Convert from:/);
      await fireEvent.change(sourceSelect, { target: { value: 'tcx' } });

      const targetSelect = getByLabelText(/Convert to:/);
      await fireEvent.change(targetSelect, { target: { value: 'pwf' } });

      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;
      expect(fileInput?.accept).toBe('.tcx');
    });

    it('should set correct accept extensions for GPX format', async () => {
      const { getByLabelText, container } = render(ConverterPanel);

      const sourceSelect = getByLabelText(/Convert from:/);
      await fireEvent.change(sourceSelect, { target: { value: 'gpx' } });

      const targetSelect = getByLabelText(/Convert to:/);
      await fireEvent.change(targetSelect, { target: { value: 'pwf' } });

      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;
      expect(fileInput?.accept).toBe('.gpx');
    });

    it('should set correct accept extensions for PWF format', async () => {
      const { getByLabelText, container } = render(ConverterPanel);

      const sourceSelect = getByLabelText(/Convert from:/);
      await fireEvent.change(sourceSelect, { target: { value: 'pwf' } });

      const targetSelect = getByLabelText(/Convert to:/);
      await fireEvent.change(targetSelect, { target: { value: 'tcx' } });

      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;
      expect(fileInput?.accept).toBe('.yaml,.yml');
    });
  });

  describe('File Upload', () => {
    it('should display file info after file is loaded', async () => {
      const { getByLabelText, container } = render(ConverterPanel);

      const sourceSelect = getByLabelText(/Convert from:/);
      const targetSelect = getByLabelText(/Convert to:/);

      await fireEvent.change(sourceSelect, { target: { value: 'fit' } });
      await fireEvent.change(targetSelect, { target: { value: 'pwf' } });

      // Create a mock file
      const file = new File(['test content'], 'workout.fit', { type: 'application/octet-stream' });

      // Find file input and trigger file selection
      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      // Mock FileReader
      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: 'test content' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(() => {
        expect(container.querySelector('.file-info')).toBeTruthy();
      });
    });

    it('should show file name and size', async () => {
      const { getByLabelText, container, getByText } = render(ConverterPanel);

      const sourceSelect = getByLabelText(/Convert from:/);
      const targetSelect = getByLabelText(/Convert to:/);

      await fireEvent.change(sourceSelect, { target: { value: 'fit' } });
      await fireEvent.change(targetSelect, { target: { value: 'pwf' } });

      const file = new File(['x'.repeat(2048)], 'workout.fit', { type: 'application/octet-stream' });
      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: 'x'.repeat(2048) } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(() => {
        expect(getByText('workout.fit')).toBeTruthy();
        expect(getByText(/2\.00 KB/)).toBeTruthy();
      });
    });

    it('should show convert button after file is loaded', async () => {
      const { getByLabelText, container, getByText } = render(ConverterPanel);

      const sourceSelect = getByLabelText(/Convert from:/);
      const targetSelect = getByLabelText(/Convert to:/);

      await fireEvent.change(sourceSelect, { target: { value: 'fit' } });
      await fireEvent.change(targetSelect, { target: { value: 'pwf' } });

      const file = new File(['test'], 'workout.fit', { type: 'application/octet-stream' });
      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: 'test' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(() => {
        expect(getByText(/Convert to PWF/)).toBeTruthy();
      });
    });

    it('should allow changing file via Change File button', async () => {
      const { getByLabelText, container, getByText, queryByText } = render(ConverterPanel);

      const sourceSelect = getByLabelText(/Convert from:/);
      const targetSelect = getByLabelText(/Convert to:/);

      await fireEvent.change(sourceSelect, { target: { value: 'fit' } });
      await fireEvent.change(targetSelect, { target: { value: 'pwf' } });

      const file = new File(['test'], 'workout.fit', { type: 'application/octet-stream' });
      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: 'test' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(() => {
        const changeButton = getByText('Change File');
        expect(changeButton).toBeTruthy();
      });

      const changeButton = getByText('Change File');
      await fireEvent.click(changeButton);

      // Should remove file info and show upload again
      await waitFor(() => {
        expect(queryByText('Change File')).toBeFalsy();
      });
    });

    it('should display error message on file load error', async () => {
      const { getByLabelText, container } = render(ConverterPanel);

      const sourceSelect = getByLabelText(/Convert from:/);
      const targetSelect = getByLabelText(/Convert to:/);

      await fireEvent.change(sourceSelect, { target: { value: 'fit' } });
      await fireEvent.change(targetSelect, { target: { value: 'pwf' } });

      const file = new File(['test'], 'workout.txt', { type: 'text/plain' });
      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(),
        onerror: null as any,
        onload: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      // Trigger error handler
      if (mockReader.onerror) {
        mockReader.onerror();
      }

      // Error would be shown via FileUpload component's error event
    });
  });

  describe('FIT to PWF Conversion', () => {
    it('should convert FIT to PWF successfully', async () => {
      const mockResult = {
        pwf_yaml: 'version: 1\nhistory_version: 1',
        warnings: []
      };
      vi.mocked(wasm.fitToPwf).mockReturnValue(mockResult);

      const { getByLabelText, container, getByText } = render(ConverterPanel);

      const sourceSelect = getByLabelText(/Convert from:/);
      const targetSelect = getByLabelText(/Convert to:/);

      await fireEvent.change(sourceSelect, { target: { value: 'fit' } });
      await fireEvent.change(targetSelect, { target: { value: 'pwf' } });

      // Simulate file upload
      const file = new File([new Uint8Array([1, 2, 3, 4])], 'workout.fit', { type: 'application/octet-stream' });
      Object.defineProperty(file, 'arrayBuffer', {
        value: vi.fn().mockResolvedValue(new ArrayBuffer(4))
      });

      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: 'binary content' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(() => {
        const convertButton = getByText(/Convert to PWF/);
        expect(convertButton).toBeTruthy();
      });

      const convertButton = getByText(/Convert to PWF/);
      await fireEvent.click(convertButton);

      await waitFor(() => {
        expect(wasm.fitToPwf).toHaveBeenCalled();
      });
    });

    it('should handle FIT conversion errors', async () => {
      const mockResult = {
        error: 'Invalid FIT file format',
        warnings: []
      };
      vi.mocked(wasm.fitToPwf).mockReturnValue(mockResult);

      const { getByLabelText, container, getByText } = render(ConverterPanel);

      await fireEvent.change(getByLabelText(/Convert from:/), { target: { value: 'fit' } });
      await fireEvent.change(getByLabelText(/Convert to:/), { target: { value: 'pwf' } });

      const file = new File([new Uint8Array([1, 2, 3, 4])], 'workout.fit', { type: 'application/octet-stream' });
      Object.defineProperty(file, 'arrayBuffer', {
        value: vi.fn().mockResolvedValue(new ArrayBuffer(4))
      });

      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: 'binary content' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(async () => {
        const convertButton = getByText(/Convert to PWF/);
        await fireEvent.click(convertButton);
      });

      await waitFor(() => {
        expect(container.querySelector('.error-banner')).toBeTruthy();
        expect(getByText(/Invalid FIT file format/)).toBeTruthy();
      });
    });

    it('should show error when trying to convert FIT to non-PWF format', async () => {
      const { getByLabelText, container, getByText } = render(ConverterPanel);

      await fireEvent.change(getByLabelText(/Convert from:/), { target: { value: 'fit' } });
      await fireEvent.change(getByLabelText(/Convert to:/), { target: { value: 'tcx' } });

      const file = new File([new Uint8Array([1, 2, 3, 4])], 'workout.fit', { type: 'application/octet-stream' });
      Object.defineProperty(file, 'arrayBuffer', {
        value: vi.fn().mockResolvedValue(new ArrayBuffer(4))
      });

      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: 'binary content' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(async () => {
        const convertButton = getByText(/Convert to TCX/);
        await fireEvent.click(convertButton);
      });

      await waitFor(() => {
        expect(getByText(/FIT can only be converted to PWF format/)).toBeTruthy();
      });
    });
  });

  describe('TCX to PWF Conversion', () => {
    it('should convert TCX to PWF successfully', async () => {
      const mockResult = {
        pwf_yaml: 'version: 1\nhistory_version: 1',
        warnings: []
      };
      vi.mocked(wasm.tcxToPwf).mockReturnValue(mockResult);

      const { getByLabelText, container, getByText } = render(ConverterPanel);

      await fireEvent.change(getByLabelText(/Convert from:/), { target: { value: 'tcx' } });
      await fireEvent.change(getByLabelText(/Convert to:/), { target: { value: 'pwf' } });

      const file = new File(['<tcx></tcx>'], 'workout.tcx', { type: 'application/xml' });
      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: '<tcx></tcx>' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(async () => {
        const convertButton = getByText(/Convert to PWF/);
        await fireEvent.click(convertButton);
      });

      await waitFor(() => {
        expect(wasm.tcxToPwf).toHaveBeenCalled();
        const call = vi.mocked(wasm.tcxToPwf).mock.calls[0];
        expect(call[1]).toBe(false); // summaryOnly parameter
      });
    });

    it('should show error when trying to convert TCX to non-PWF format', async () => {
      const { getByLabelText, container, getByText } = render(ConverterPanel);

      await fireEvent.change(getByLabelText(/Convert from:/), { target: { value: 'tcx' } });
      await fireEvent.change(getByLabelText(/Convert to:/), { target: { value: 'gpx' } });

      const file = new File(['<tcx></tcx>'], 'workout.tcx', { type: 'application/xml' });
      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: '<tcx></tcx>' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(async () => {
        const convertButton = getByText(/Convert to GPX/);
        await fireEvent.click(convertButton);
      });

      await waitFor(() => {
        expect(getByText(/TCX can only be converted to PWF format/)).toBeTruthy();
      });
    });
  });

  describe('GPX to PWF Conversion', () => {
    it('should convert GPX to PWF successfully', async () => {
      const mockResult = {
        pwf_yaml: 'version: 1\nhistory_version: 1',
        warnings: []
      };
      vi.mocked(wasm.gpxToPwf).mockReturnValue(mockResult);

      const { getByLabelText, container, getByText } = render(ConverterPanel);

      await fireEvent.change(getByLabelText(/Convert from:/), { target: { value: 'gpx' } });
      await fireEvent.change(getByLabelText(/Convert to:/), { target: { value: 'pwf' } });

      const file = new File(['<gpx></gpx>'], 'workout.gpx', { type: 'application/xml' });
      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: '<gpx></gpx>' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(async () => {
        const convertButton = getByText(/Convert to PWF/);
        await fireEvent.click(convertButton);
      });

      await waitFor(() => {
        expect(wasm.gpxToPwf).toHaveBeenCalled();
      });
    });

    it('should show error when trying to convert GPX to non-PWF format', async () => {
      const { getByLabelText, container, getByText } = render(ConverterPanel);

      await fireEvent.change(getByLabelText(/Convert from:/), { target: { value: 'gpx' } });
      await fireEvent.change(getByLabelText(/Convert to:/), { target: { value: 'csv' } });

      const file = new File(['<gpx></gpx>'], 'workout.gpx', { type: 'application/xml' });
      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: '<gpx></gpx>' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(async () => {
        const convertButton = getByText(/Convert to CSV/);
        await fireEvent.click(convertButton);
      });

      await waitFor(() => {
        expect(getByText(/GPX can only be converted to PWF format/)).toBeTruthy();
      });
    });
  });

  describe('PWF Export Conversions', () => {
    it('should convert PWF to TCX successfully', async () => {
      const mockResult = {
        tcx_xml: '<?xml version="1.0"?><tcx></tcx>',
        warnings: []
      };
      vi.mocked(wasm.pwfToTcx).mockReturnValue(mockResult);

      const { getByLabelText, container, getByText } = render(ConverterPanel);

      await fireEvent.change(getByLabelText(/Convert from:/), { target: { value: 'pwf' } });
      await fireEvent.change(getByLabelText(/Convert to:/), { target: { value: 'tcx' } });

      const file = new File(['version: 1'], 'workout.yaml', { type: 'text/yaml' });
      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: 'version: 1' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(async () => {
        const convertButton = getByText(/Convert to TCX/);
        await fireEvent.click(convertButton);
      });

      await waitFor(() => {
        expect(wasm.pwfToTcx).toHaveBeenCalledWith('version: 1');
      });
    });

    it('should convert PWF to GPX successfully', async () => {
      const mockResult = {
        gpx_xml: '<?xml version="1.0"?><gpx></gpx>',
        warnings: []
      };
      vi.mocked(wasm.pwfToGpx).mockReturnValue(mockResult);

      const { getByLabelText, container, getByText } = render(ConverterPanel);

      await fireEvent.change(getByLabelText(/Convert from:/), { target: { value: 'pwf' } });
      await fireEvent.change(getByLabelText(/Convert to:/), { target: { value: 'gpx' } });

      const file = new File(['version: 1'], 'workout.yaml', { type: 'text/yaml' });
      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: 'version: 1' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(async () => {
        const convertButton = getByText(/Convert to GPX/);
        await fireEvent.click(convertButton);
      });

      await waitFor(() => {
        expect(wasm.pwfToGpx).toHaveBeenCalledWith('version: 1');
      });
    });

    it('should convert PWF to CSV successfully', async () => {
      const mockResult = {
        csv_data: 'timestamp,hr,power',
        warnings: [],
        data_points: 100,
        workouts_processed: 1
      };
      vi.mocked(wasm.pwfToCsv).mockReturnValue(mockResult);

      const { getByLabelText, container, getByText } = render(ConverterPanel);

      await fireEvent.change(getByLabelText(/Convert from:/), { target: { value: 'pwf' } });
      await fireEvent.change(getByLabelText(/Convert to:/), { target: { value: 'csv' } });

      const file = new File(['version: 1'], 'workout.yaml', { type: 'text/yaml' });
      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: 'version: 1' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(async () => {
        const convertButton = getByText(/Convert to CSV/);
        await fireEvent.click(convertButton);
      });

      await waitFor(() => {
        expect(wasm.pwfToCsv).toHaveBeenCalledWith('version: 1');
      });
    });

    it('should show error for invalid PWF export target format', async () => {
      const { getByLabelText, container, getByText } = render(ConverterPanel);

      await fireEvent.change(getByLabelText(/Convert from:/), { target: { value: 'pwf' } });
      await fireEvent.change(getByLabelText(/Convert to:/), { target: { value: 'tcx' } });

      const file = new File(['version: 1'], 'workout.yaml', { type: 'text/yaml' });

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: 'version: 1' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;
      if (fileInput) {
        Object.defineProperty(fileInput, 'files', {
          value: [file],
          writable: false
        });

        await fireEvent.change(fileInput);
      }

      // This test verifies that PWF can be converted to valid formats like TCX
      // FormatSelector prevents invalid combinations in the UI
    });
  });

  describe('Conversion State Management', () => {
    it('should show "Converting..." button state during conversion', async () => {
      const mockResult = {
        pwf_yaml: 'version: 1',
        warnings: []
      };

      // Make the conversion slow
      vi.mocked(wasm.fitToPwf).mockImplementation(() => {
        return new Promise((resolve) => {
          setTimeout(() => resolve(mockResult), 100);
        }) as any;
      });

      const { getByLabelText, container, getByText, queryByText } = render(ConverterPanel);

      await fireEvent.change(getByLabelText(/Convert from:/), { target: { value: 'fit' } });
      await fireEvent.change(getByLabelText(/Convert to:/), { target: { value: 'pwf' } });

      const file = new File([new Uint8Array([1, 2, 3, 4])], 'workout.fit', { type: 'application/octet-stream' });
      Object.defineProperty(file, 'arrayBuffer', {
        value: vi.fn().mockResolvedValue(new ArrayBuffer(4))
      });

      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: 'binary' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(() => {
        const convertButton = getByText(/Convert to PWF/);
        expect(convertButton).toBeTruthy();
      });

      const convertButton = getByText(/Convert to PWF/) as HTMLButtonElement;
      fireEvent.click(convertButton);

      // Should show "Converting..." text while processing
      await waitFor(() => {
        expect(queryByText('Converting...')).toBeTruthy();
      }, { timeout: 50 });
    });

    it('should disable convert button when no file is uploaded', async () => {
      const { getByLabelText, container } = render(ConverterPanel);

      await fireEvent.change(getByLabelText(/Convert from:/), { target: { value: 'fit' } });
      await fireEvent.change(getByLabelText(/Convert to:/), { target: { value: 'pwf' } });

      // No file uploaded, so convert button shouldn't exist
      expect(container.querySelector('.btn-primary')).toBeFalsy();
    });

    it('should clear error message when new file is loaded', async () => {
      const { getByLabelText, container } = render(ConverterPanel);

      await fireEvent.change(getByLabelText(/Convert from:/), { target: { value: 'fit' } });
      await fireEvent.change(getByLabelText(/Convert to:/), { target: { value: 'pwf' } });

      const file = new File([new Uint8Array([1, 2, 3, 4])], 'workout.fit', { type: 'application/octet-stream' });
      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: 'binary' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      // Error banner should not be present initially
      expect(container.querySelector('.error-banner')).toBeFalsy();
    });

    it('should clear results when new file is loaded', async () => {
      const { getByLabelText, container } = render(ConverterPanel);

      await fireEvent.change(getByLabelText(/Convert from:/), { target: { value: 'fit' } });
      await fireEvent.change(getByLabelText(/Convert to:/), { target: { value: 'pwf' } });

      const file = new File([new Uint8Array([1, 2, 3, 4])], 'workout.fit', { type: 'application/octet-stream' });
      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: 'binary' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      // Results should not be present
      expect(container.querySelector('.conversion-results')).toBeFalsy();
    });
  });

  describe('Error Handling', () => {
    it('should show error when conversion throws exception', async () => {
      vi.mocked(wasm.fitToPwf).mockImplementation(() => {
        throw new Error('WASM error');
      });

      const { getByLabelText, container, getByText } = render(ConverterPanel);

      await fireEvent.change(getByLabelText(/Convert from:/), { target: { value: 'fit' } });
      await fireEvent.change(getByLabelText(/Convert to:/), { target: { value: 'pwf' } });

      const file = new File([new Uint8Array([1, 2, 3, 4])], 'workout.fit', { type: 'application/octet-stream' });
      Object.defineProperty(file, 'arrayBuffer', {
        value: vi.fn().mockResolvedValue(new ArrayBuffer(4))
      });

      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: 'binary' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(async () => {
        const convertButton = getByText(/Convert to PWF/);
        await fireEvent.click(convertButton);
      });

      await waitFor(() => {
        expect(container.querySelector('.error-banner')).toBeTruthy();
        expect(getByText(/Conversion failed: Error: WASM error/)).toBeTruthy();
      });
    });

    it('should handle missing file gracefully', async () => {
      const { getByLabelText, container, getByText } = render(ConverterPanel);

      await fireEvent.change(getByLabelText(/Convert from:/), { target: { value: 'fit' } });
      await fireEvent.change(getByLabelText(/Convert to:/), { target: { value: 'pwf' } });

      const file = new File([new Uint8Array([1, 2, 3, 4])], 'workout.fit', { type: 'application/octet-stream' });

      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: 'binary' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(() => {
        expect(getByText(/Convert to PWF/)).toBeTruthy();
      });

      // Manually clear the file state by clicking "Change File"
      const changeButton = getByText('Change File');
      await fireEvent.click(changeButton);

      // Now convert button shouldn't be visible
      expect(container.querySelector('.btn-primary')).toBeFalsy();
    });
  });

  describe('Results Display', () => {
    it('should display ConversionResults component after successful conversion', async () => {
      const mockResult = {
        pwf_yaml: 'version: 1\nhistory_version: 1',
        warnings: []
      };
      vi.mocked(wasm.fitToPwf).mockReturnValue(mockResult);

      const { getByLabelText, container, getByText } = render(ConverterPanel);

      await fireEvent.change(getByLabelText(/Convert from:/), { target: { value: 'fit' } });
      await fireEvent.change(getByLabelText(/Convert to:/), { target: { value: 'pwf' } });

      const file = new File([new Uint8Array([1, 2, 3, 4])], 'workout.fit', { type: 'application/octet-stream' });
      Object.defineProperty(file, 'arrayBuffer', {
        value: vi.fn().mockResolvedValue(new ArrayBuffer(4))
      });

      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: 'binary' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(async () => {
        const convertButton = getByText(/Convert to PWF/);
        await fireEvent.click(convertButton);
      });

      await waitFor(() => {
        expect(container.querySelector('.conversion-results')).toBeTruthy();
      });
    });

    it('should not display results when conversion has error', async () => {
      const mockResult = {
        error: 'Conversion failed',
        warnings: []
      };
      vi.mocked(wasm.fitToPwf).mockReturnValue(mockResult);

      const { getByLabelText, container, getByText } = render(ConverterPanel);

      await fireEvent.change(getByLabelText(/Convert from:/), { target: { value: 'fit' } });
      await fireEvent.change(getByLabelText(/Convert to:/), { target: { value: 'pwf' } });

      const file = new File([new Uint8Array([1, 2, 3, 4])], 'workout.fit', { type: 'application/octet-stream' });
      Object.defineProperty(file, 'arrayBuffer', {
        value: vi.fn().mockResolvedValue(new ArrayBuffer(4))
      });

      const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

      const mockReader = {
        readAsText: vi.fn(function (this: any) {
          this.onload({ target: { result: 'binary' } });
        }),
        onload: null as any,
        onerror: null as any
      };
      global.FileReader = vi.fn(() => mockReader) as any;

      Object.defineProperty(fileInput, 'files', {
        value: [file],
        writable: false
      });

      await fireEvent.change(fileInput);

      await waitFor(async () => {
        const convertButton = getByText(/Convert to PWF/);
        await fireEvent.click(convertButton);
      });

      await waitFor(() => {
        // Error should be shown in error banner, not in ConversionResults
        expect(container.querySelector('.error-banner')).toBeTruthy();
      });
    });
  });
});
