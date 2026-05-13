import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import DemoGallery from '../src/components/DemoGallery';

const demos = [
  { src: '/demos/a.mp4', poster: '/demos/a.png', title: 'A', sub: '~30s', body: 'About A' },
  { src: '/demos/b.mp4', poster: '/demos/b.png', title: 'B', sub: '~37s', body: 'About B' },
  { src: '/demos/c.mp4', poster: '/demos/c.png', title: 'C', sub: '~20s', body: 'About C' },
];

describe('DemoGallery', () => {
  beforeEach(() => {
    HTMLMediaElement.prototype.play = vi.fn().mockResolvedValue(undefined);
    HTMLMediaElement.prototype.pause = vi.fn();
  });

  function cardFor(title: string) {
    // Each card's accessible name is its visible text content (title + sub + body)
    return screen.getByRole('button', { name: new RegExp(`About ${title}`) });
  }

  it('renders three card thumbnails (poster only, no video) by default', () => {
    render(<DemoGallery demos={demos} />);
    expect(cardFor('A')).toBeInTheDocument();
    expect(cardFor('B')).toBeInTheDocument();
    expect(cardFor('C')).toBeInTheDocument();
    expect(screen.queryByRole('dialog')).toBeNull();
    expect(screen.queryByTestId('demo-video')).toBeNull();
  });

  it('clicking a card opens a theater dialog with the matching video', () => {
    render(<DemoGallery demos={demos} />);
    fireEvent.click(cardFor('B'));
    const dialog = screen.getByRole('dialog');
    expect(dialog).toBeInTheDocument();
    const video = screen.getByTestId('demo-video') as HTMLVideoElement;
    expect(video.getAttribute('src')).toBe('/demos/b.mp4');
    expect(video.play).toHaveBeenCalled();
  });

  it('pressing Escape closes the dialog', () => {
    render(<DemoGallery demos={demos} />);
    fireEvent.click(cardFor('A'));
    expect(screen.getByRole('dialog')).toBeInTheDocument();
    fireEvent.keyDown(document, { key: 'Escape' });
    expect(screen.queryByRole('dialog')).toBeNull();
  });

  it('clicking the close button closes the dialog', () => {
    render(<DemoGallery demos={demos} />);
    fireEvent.click(cardFor('C'));
    expect(screen.getByRole('dialog')).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: /close demo/i }));
    expect(screen.queryByRole('dialog')).toBeNull();
  });
});
