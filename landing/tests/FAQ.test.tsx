import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import FAQ from '../src/components/FAQ';

const entries = [
  { q: 'Why?', a: 'Because.' },
  { q: 'How?', a: 'Like this.' },
];

describe('FAQ', () => {
  it('renders all questions', () => {
    render(<FAQ entries={entries} />);
    expect(screen.getByText('Why?')).toBeInTheDocument();
    expect(screen.getByText('How?')).toBeInTheDocument();
  });

  it('reveals an answer when its trigger is clicked', () => {
    render(<FAQ entries={entries} />);
    fireEvent.click(screen.getByText('Why?'));
    expect(screen.getByText('Because.')).toBeVisible();
  });
});
