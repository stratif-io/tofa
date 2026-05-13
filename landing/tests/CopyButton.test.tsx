import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import CopyButton from '../src/components/CopyButton';

describe('CopyButton', () => {
  it('copies the value to the clipboard and shows feedback', async () => {
    const writeText = vi.fn().mockResolvedValue(undefined);
    Object.defineProperty(navigator, 'clipboard', {
      value: { writeText },
      configurable: true,
      writable: true,
    });

    render(<CopyButton value="brew install tofa" />);
    const btn = screen.getByRole('button', { name: /copy/i });
    fireEvent.click(btn);

    expect(writeText).toHaveBeenCalledWith('brew install tofa');
    await waitFor(() => expect(btn.textContent).toMatch(/copied/i));
  });
});
