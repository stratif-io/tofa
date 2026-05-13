import * as matchers from '@testing-library/jest-dom/matchers';
import { beforeEach, expect } from 'vitest';

expect.extend(matchers);

// happy-dom@20 ships an empty `{}` as localStorage instead of a real Storage.
// Replace it with a tiny in-memory shim so component code that touches
// localStorage works in tests.
class MemStorage implements Storage {
  private data = new Map<string, string>();
  get length() { return this.data.size; }
  clear() { this.data.clear(); }
  getItem(k: string) { return this.data.get(k) ?? null; }
  key(i: number) { return [...this.data.keys()][i] ?? null; }
  removeItem(k: string) { this.data.delete(k); }
  setItem(k: string, v: string) { this.data.set(k, String(v)); }
}

const storage = new MemStorage();
Object.defineProperty(globalThis, 'localStorage', { value: storage, writable: true, configurable: true });
if (typeof window !== 'undefined') {
  Object.defineProperty(window, 'localStorage', { value: storage, writable: true, configurable: true });
}

// matchMedia is also commonly missing in headless DOM shims
if (typeof window !== 'undefined' && !window.matchMedia) {
  window.matchMedia = (query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: () => {},
    removeListener: () => {},
    addEventListener: () => {},
    removeEventListener: () => {},
    dispatchEvent: () => false,
  });
}

beforeEach(() => {
  storage.clear();
  document.documentElement.removeAttribute('data-theme');
});
