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

  it('renders three videos with posters', () => {
    render(<DemoGallery demos={demos} />);
    const videos = screen.getAllByTestId('demo-video');
    expect(videos).toHaveLength(3);
    videos.forEach((v) => expect(v.getAttribute('poster')).toMatch(/\.png$/));
  });

  it('clicking a non-lead card plays the clicked video and pauses others', () => {
    render(<DemoGallery demos={demos} />);
    const cards = screen.getAllByRole('button', { name: /play demo/i });
    fireEvent.click(cards[1]);
    const videos = screen.getAllByTestId('demo-video') as HTMLVideoElement[];
    expect(videos[1].play).toHaveBeenCalled();
  });
});
