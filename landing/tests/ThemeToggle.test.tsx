import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import ThemeToggle from '../src/components/ThemeToggle';

describe('ThemeToggle', () => {
  it('cycles system → dark → light → system', () => {
    render(<ThemeToggle />);
    const btn = screen.getByRole('button', { name: /toggle theme/i });

    expect(localStorage.getItem('tofa-theme')).toBeNull();
    fireEvent.click(btn);
    expect(localStorage.getItem('tofa-theme')).toBe('dark');
    expect(document.documentElement.getAttribute('data-theme')).toBeNull();
    fireEvent.click(btn);
    expect(localStorage.getItem('tofa-theme')).toBe('light');
    expect(document.documentElement.getAttribute('data-theme')).toBe('light');
    fireEvent.click(btn);
    expect(localStorage.getItem('tofa-theme')).toBeNull();
  });

  it('reads stored preference on mount', () => {
    localStorage.setItem('tofa-theme', 'light');
    render(<ThemeToggle />);
    expect(screen.getByRole('button')).toHaveAttribute('aria-label', expect.stringContaining('light'));
  });
});
