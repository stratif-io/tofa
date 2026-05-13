import { useEffect, useState } from 'react';

type Mode = 'system' | 'dark' | 'light';

function readMode(): Mode {
  if (typeof localStorage === 'undefined') return 'system';
  const v = localStorage.getItem('tofa-theme');
  return v === 'dark' || v === 'light' ? v : 'system';
}

function applyMode(mode: Mode) {
  const root = document.documentElement;
  if (mode === 'light') {
    root.setAttribute('data-theme', 'light');
    localStorage.setItem('tofa-theme', 'light');
  } else if (mode === 'dark') {
    root.removeAttribute('data-theme');
    localStorage.setItem('tofa-theme', 'dark');
  } else {
    root.removeAttribute('data-theme');
    localStorage.removeItem('tofa-theme');
    if (window.matchMedia('(prefers-color-scheme: light)').matches) {
      root.setAttribute('data-theme', 'light');
    }
  }
}

const NEXT: Record<Mode, Mode> = { system: 'dark', dark: 'light', light: 'system' };
const LABEL: Record<Mode, string> = { system: '◐ system', dark: '◑ dark', light: '◐ light' };

export default function ThemeToggle() {
  const [mode, setMode] = useState<Mode>('system');

  useEffect(() => { setMode(readMode()); }, []);

  function cycle() {
    const next = NEXT[mode];
    setMode(next);
    applyMode(next);
  }

  return (
    <button
      type="button"
      onClick={cycle}
      aria-label={`Toggle theme (current: ${mode})`}
      className="font-mono text-xs px-3 py-1.5 rounded-tofa-md border border-border bg-surface text-text-muted hover:text-text hover:border-border-strong transition-colors"
    >
      {LABEL[mode]}
    </button>
  );
}
