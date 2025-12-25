/**
 * Tests for FileUpload component
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, fireEvent, waitFor } from '@testing-library/svelte';
import FileUpload from '../FileUpload.svelte';

describe('FileUpload', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should render the component without crashing', () => {
    const { container } = render(FileUpload);
    expect(container).toBeTruthy();
    expect(container.querySelector('.file-upload')).toBeTruthy();
  });

  it('should display default label', () => {
    const { getByText } = render(FileUpload);
    expect(getByText('Drop YAML file here or click to browse')).toBeTruthy();
  });

  it('should display custom label when provided', () => {
    const { getByText } = render(FileUpload, {
      props: { label: 'Custom upload label' }
    });
    expect(getByText('Custom upload label')).toBeTruthy();
  });

  it('should display supported file types hint', () => {
    const { container } = render(FileUpload);
    const hint = container.querySelector('.upload-hint');
    expect(hint?.textContent).toContain('YAML');
    expect(hint?.textContent).toContain('YML');
  });

  it('should render file input with correct accept attribute', () => {
    const { container } = render(FileUpload);
    const input = container.querySelector('input[type="file"]') as HTMLInputElement;
    expect(input).toBeTruthy();
    expect(input.accept).toBe('.yaml,.yml');
  });

  it('should allow custom accept attribute', () => {
    const { container } = render(FileUpload, {
      props: { accept: '.txt,.md' }
    });
    const input = container.querySelector('input[type="file"]') as HTMLInputElement;
    expect(input.accept).toBe('.txt,.md');
  });

  it('should hide file input visually', () => {
    const { container } = render(FileUpload);
    const input = container.querySelector('input[type="file"]') as HTMLInputElement;
    expect(input.style.display).toBe('none');
  });

  it('should show document icon by default', () => {
    const { container } = render(FileUpload);
    const icon = container.querySelector('.upload-icon');
    expect(icon?.textContent).toBe('📄');
  });

  it('should show inbox icon when dragging', async () => {
    const { container } = render(FileUpload);
    const uploadDiv = container.querySelector('.file-upload') as HTMLElement;

    await fireEvent.dragOver(uploadDiv);

    const icon = container.querySelector('.upload-icon');
    expect(icon?.textContent).toBe('📥');
  });

  it('should add dragging class on dragover', async () => {
    const { container } = render(FileUpload);
    const uploadDiv = container.querySelector('.file-upload') as HTMLElement;

    await fireEvent.dragOver(uploadDiv);

    expect(uploadDiv.classList.contains('dragging')).toBe(true);
  });

  it('should remove dragging class on dragleave', async () => {
    const { container } = render(FileUpload);
    const uploadDiv = container.querySelector('.file-upload') as HTMLElement;

    await fireEvent.dragOver(uploadDiv);
    expect(uploadDiv.classList.contains('dragging')).toBe(true);

    await fireEvent.dragLeave(uploadDiv);
    expect(uploadDiv.classList.contains('dragging')).toBe(false);
  });

  it('should handle file drop', async () => {
    const { container, component } = render(FileUpload);
    const uploadDiv = container.querySelector('.file-upload') as HTMLElement;

    const fileLoadedHandler = vi.fn();
    component.$on('fileLoaded', fileLoadedHandler);

    const file = new File(['plan_version: 1'], 'test.yaml', { type: 'text/yaml' });

    const mockFileReader = {
      result: 'plan_version: 1',
      readAsText: vi.fn(function(this: any) {
        this.onload({ target: this });
      }),
    };

    vi.spyOn(global, 'FileReader').mockImplementation(() => mockFileReader as any);

    const dropEvent = {
      preventDefault: vi.fn(),
      dataTransfer: {
        files: [file]
      }
    };

    await fireEvent.drop(uploadDiv, dropEvent);

    await waitFor(() => {
      expect(fileLoadedHandler).toHaveBeenCalled();
      const eventDetail = fileLoadedHandler.mock.calls[0][0].detail;
      expect(eventDetail.file.name).toBe('test.yaml');
      expect(eventDetail.content).toBe('plan_version: 1');
    });
  });

  it('should handle file selection via input', async () => {
    const { container, component } = render(FileUpload);

    const fileLoadedHandler = vi.fn();
    component.$on('fileLoaded', fileLoadedHandler);

    const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;
    const file = new File(['plan_version: 1'], 'test.yaml', { type: 'text/yaml' });

    const mockFileReader = {
      result: 'plan_version: 1',
      readAsText: vi.fn(function(this: any) {
        this.onload({ target: this });
      }),
    };

    vi.spyOn(global, 'FileReader').mockImplementation(() => mockFileReader as any);

    Object.defineProperty(fileInput, 'files', {
      value: [file],
      writable: false,
    });

    await fireEvent.change(fileInput);

    await waitFor(() => {
      expect(fileLoadedHandler).toHaveBeenCalled();
    });
  });

  it('should open file picker on click', async () => {
    const { container } = render(FileUpload);
    const uploadDiv = container.querySelector('.file-upload') as HTMLElement;
    const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

    const clickSpy = vi.spyOn(fileInput, 'click');

    await fireEvent.click(uploadDiv);

    expect(clickSpy).toHaveBeenCalled();
  });

  it('should open file picker on Enter key', async () => {
    const { container } = render(FileUpload);
    const uploadDiv = container.querySelector('.file-upload') as HTMLElement;
    const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

    const clickSpy = vi.spyOn(fileInput, 'click');

    await fireEvent.keyDown(uploadDiv, { key: 'Enter' });

    expect(clickSpy).toHaveBeenCalled();
  });

  it('should not open file picker on other keys', async () => {
    const { container } = render(FileUpload);
    const uploadDiv = container.querySelector('.file-upload') as HTMLElement;
    const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

    const clickSpy = vi.spyOn(fileInput, 'click');

    await fireEvent.keyDown(uploadDiv, { key: 'Space' });

    expect(clickSpy).not.toHaveBeenCalled();
  });

  it('should validate file extension', async () => {
    const { container, component } = render(FileUpload);

    const errorHandler = vi.fn();
    component.$on('error', errorHandler);

    const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;
    const file = new File(['content'], 'test.txt', { type: 'text/plain' });

    Object.defineProperty(fileInput, 'files', {
      value: [file],
      writable: false,
    });

    await fireEvent.change(fileInput);

    await waitFor(() => {
      expect(errorHandler).toHaveBeenCalled();
      const eventDetail = errorHandler.mock.calls[0][0].detail;
      expect(eventDetail.message).toContain('Invalid file type');
    });
  });

  it('should accept .yaml files', async () => {
    const { container, component } = render(FileUpload);

    const fileLoadedHandler = vi.fn();
    const errorHandler = vi.fn();
    component.$on('fileLoaded', fileLoadedHandler);
    component.$on('error', errorHandler);

    const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;
    const file = new File(['content'], 'test.yaml', { type: 'text/yaml' });

    const mockFileReader = {
      result: 'content',
      readAsText: vi.fn(function(this: any) {
        this.onload({ target: this });
      }),
    };

    vi.spyOn(global, 'FileReader').mockImplementation(() => mockFileReader as any);

    Object.defineProperty(fileInput, 'files', {
      value: [file],
      writable: false,
    });

    await fireEvent.change(fileInput);

    await waitFor(() => {
      expect(fileLoadedHandler).toHaveBeenCalled();
      expect(errorHandler).not.toHaveBeenCalled();
    });
  });

  it('should accept .yml files', async () => {
    const { container, component } = render(FileUpload);

    const fileLoadedHandler = vi.fn();
    const errorHandler = vi.fn();
    component.$on('fileLoaded', fileLoadedHandler);
    component.$on('error', errorHandler);

    const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;
    const file = new File(['content'], 'test.yml', { type: 'text/yaml' });

    const mockFileReader = {
      result: 'content',
      readAsText: vi.fn(function(this: any) {
        this.onload({ target: this });
      }),
    };

    vi.spyOn(global, 'FileReader').mockImplementation(() => mockFileReader as any);

    Object.defineProperty(fileInput, 'files', {
      value: [file],
      writable: false,
    });

    await fireEvent.change(fileInput);

    await waitFor(() => {
      expect(fileLoadedHandler).toHaveBeenCalled();
      expect(errorHandler).not.toHaveBeenCalled();
    });
  });

  it('should handle FileReader errors', async () => {
    const { container, component } = render(FileUpload);

    const errorHandler = vi.fn();
    component.$on('error', errorHandler);

    const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;
    const file = new File(['content'], 'test.yaml', { type: 'text/yaml' });

    const mockFileReader = {
      readAsText: vi.fn(function(this: any) {
        this.onerror();
      }),
    };

    vi.spyOn(global, 'FileReader').mockImplementation(() => mockFileReader as any);

    Object.defineProperty(fileInput, 'files', {
      value: [file],
      writable: false,
    });

    await fireEvent.change(fileInput);

    await waitFor(() => {
      expect(errorHandler).toHaveBeenCalled();
      const eventDetail = errorHandler.mock.calls[0][0].detail;
      expect(eventDetail.message).toBe('Failed to read file');
    });
  });

  it('should handle empty file list', async () => {
    const { container, component } = render(FileUpload);

    const fileLoadedHandler = vi.fn();
    component.$on('fileLoaded', fileLoadedHandler);

    const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;

    Object.defineProperty(fileInput, 'files', {
      value: [],
      writable: false,
    });

    await fireEvent.change(fileInput);

    // Should not trigger fileLoaded event
    expect(fileLoadedHandler).not.toHaveBeenCalled();
  });

  it('should handle drop with no files', async () => {
    const { container, component } = render(FileUpload);
    const uploadDiv = container.querySelector('.file-upload') as HTMLElement;

    const fileLoadedHandler = vi.fn();
    component.$on('fileLoaded', fileLoadedHandler);

    const dropEvent = {
      preventDefault: vi.fn(),
      dataTransfer: {
        files: []
      }
    };

    await fireEvent.drop(uploadDiv, dropEvent);

    expect(fileLoadedHandler).not.toHaveBeenCalled();
  });

  it('should remove dragging class after drop', async () => {
    const { container } = render(FileUpload);
    const uploadDiv = container.querySelector('.file-upload') as HTMLElement;

    await fireEvent.dragOver(uploadDiv);
    expect(uploadDiv.classList.contains('dragging')).toBe(true);

    const dropEvent = {
      preventDefault: vi.fn(),
      dataTransfer: {
        files: []
      }
    };

    await fireEvent.drop(uploadDiv, dropEvent);
    expect(uploadDiv.classList.contains('dragging')).toBe(false);
  });

  it('should read file content as text', async () => {
    const { container, component } = render(FileUpload);

    const fileLoadedHandler = vi.fn();
    component.$on('fileLoaded', fileLoadedHandler);

    const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;
    const file = new File(['YAML content here'], 'test.yaml', { type: 'text/yaml' });

    const mockFileReader = {
      result: 'YAML content here',
      readAsText: vi.fn(function(this: any) {
        this.onload({ target: this });
      }),
    };

    vi.spyOn(global, 'FileReader').mockImplementation(() => mockFileReader as any);

    Object.defineProperty(fileInput, 'files', {
      value: [file],
      writable: false,
    });

    await fireEvent.change(fileInput);

    await waitFor(() => {
      expect(fileLoadedHandler).toHaveBeenCalled();
      const eventDetail = fileLoadedHandler.mock.calls[0][0].detail;
      expect(eventDetail.content).toBe('YAML content here');
    });
  });

  it('should have accessibility attributes', () => {
    const { container } = render(FileUpload);
    const uploadDiv = container.querySelector('.file-upload') as HTMLElement;

    expect(uploadDiv.getAttribute('role')).toBe('button');
    expect(uploadDiv.getAttribute('tabindex')).toBe('0');
  });

  it('should handle case-insensitive file extensions', async () => {
    const { container, component } = render(FileUpload);

    const fileLoadedHandler = vi.fn();
    const errorHandler = vi.fn();
    component.$on('fileLoaded', fileLoadedHandler);
    component.$on('error', errorHandler);

    const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;
    const file = new File(['content'], 'test.YAML', { type: 'text/yaml' });

    const mockFileReader = {
      result: 'content',
      readAsText: vi.fn(function(this: any) {
        this.onload({ target: this });
      }),
    };

    vi.spyOn(global, 'FileReader').mockImplementation(() => mockFileReader as any);

    Object.defineProperty(fileInput, 'files', {
      value: [file],
      writable: false,
    });

    await fireEvent.change(fileInput);

    await waitFor(() => {
      expect(fileLoadedHandler).toHaveBeenCalled();
      expect(errorHandler).not.toHaveBeenCalled();
    });
  });

  it('should dispatch file object and content', async () => {
    const { container, component } = render(FileUpload);

    const fileLoadedHandler = vi.fn();
    component.$on('fileLoaded', fileLoadedHandler);

    const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement;
    const file = new File(['test content'], 'myfile.yaml', { type: 'text/yaml' });

    const mockFileReader = {
      result: 'test content',
      readAsText: vi.fn(function(this: any) {
        this.onload({ target: this });
      }),
    };

    vi.spyOn(global, 'FileReader').mockImplementation(() => mockFileReader as any);

    Object.defineProperty(fileInput, 'files', {
      value: [file],
      writable: false,
    });

    await fireEvent.change(fileInput);

    await waitFor(() => {
      expect(fileLoadedHandler).toHaveBeenCalled();
      const eventDetail = fileLoadedHandler.mock.calls[0][0].detail;
      expect(eventDetail.file).toBeTruthy();
      expect(eventDetail.file.name).toBe('myfile.yaml');
      expect(eventDetail.content).toBe('test content');
    });
  });
});
