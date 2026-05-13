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
const NEXT_LABEL: Record<Mode, string> = { system: 'dark', dark: 'light', light: 'system' };

function Icon({ mode }: { mode: Mode }) {
  if (mode === 'light') {
    return (
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
        <circle cx="12" cy="12" r="4" />
        <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M4.93 19.07l1.41-1.41M17.66 6.34l1.41-1.41" />
      </svg>
    );
  }
  if (mode === 'dark') {
    return (
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
      </svg>
    );
  }
  return (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
      <rect x="2" y="3" width="20" height="14" rx="2" />
      <path d="M8 21h8M12 17v4" />
    </svg>
  );
}

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
      aria-label={`Theme: ${mode}. Switch to ${NEXT_LABEL[mode]}`}
      title={`Theme: ${mode} (click for ${NEXT_LABEL[mode]})`}
      className="inline-flex items-center justify-center w-8 h-7 rounded-tofa-md border border-border bg-surface text-text-muted hover:text-text hover:border-border-strong transition-colors"
    >
      <Icon mode={mode} />
    </button>
  );
}
