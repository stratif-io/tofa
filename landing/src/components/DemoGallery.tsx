import { useEffect, useRef, useState } from 'react';

export interface Demo {
  src: string;
  poster: string;
  title: string;
  sub: string;
  body: string;
}

interface Props { demos: Demo[] }

function useReducedMotion() {
  const [v, setV] = useState(false);
  useEffect(() => {
    const mq = window.matchMedia('(prefers-reduced-motion: reduce)');
    setV(mq.matches);
    const onChange = () => setV(mq.matches);
    mq.addEventListener?.('change', onChange);
    return () => mq.removeEventListener?.('change', onChange);
  }, []);
  return v;
}

export default function DemoGallery({ demos }: Props) {
  const refs = useRef<Array<HTMLVideoElement | null>>([]);
  const [active, setActive] = useState<number | null>(null);
  const reducedMotion = useReducedMotion();

  useEffect(() => {
    if (reducedMotion || demos.length === 0) return;
    const lead = refs.current[0];
    if (!lead || typeof IntersectionObserver === 'undefined') return;
    const io = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            lead.play().catch(() => {});
            setActive(0);
            io.disconnect();
          }
        }
      },
      { threshold: 0.4 },
    );
    io.observe(lead);
    return () => io.disconnect();
  }, [reducedMotion, demos.length]);

  function playOne(idx: number) {
    refs.current.forEach((v, i) => {
      if (!v) return;
      if (i === idx) v.play().catch(() => {});
      else v.pause();
    });
    setActive(idx);
  }

  return (
    <div className="grid md:grid-cols-[1.4fr_1fr_1fr] gap-4">
      {demos.map((d, i) => (
        <button
          key={d.src}
          type="button"
          aria-label={`Play demo: ${d.title}`}
          onClick={() => playOne(i)}
          className="group text-left rounded-tofa-lg bg-bg-sunken border border-border overflow-hidden hover:border-brand/40 transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-brand"
        >
          <div className="relative aspect-[16/10] bg-bg-elevated">
            <video
              ref={(el) => { refs.current[i] = el; }}
              data-testid="demo-video"
              src={d.src}
              poster={d.poster}
              muted
              loop
              playsInline
              preload="metadata"
              className="absolute inset-0 w-full h-full object-cover"
            />
            {active !== i && (
              <div className="absolute inset-0 flex items-center justify-center bg-black/30 group-hover:bg-black/20 transition-colors">
                <div className="w-12 h-12 rounded-full bg-brand/90 text-on-brand flex items-center justify-center font-mono">▶</div>
              </div>
            )}
          </div>
          <div className="p-4">
            <h3 className="font-display font-bold text-text">{d.title}</h3>
            <div className="mt-1 font-mono text-xs text-text-subtle">{d.sub}</div>
            <p className="mt-2 text-sm text-text-muted leading-relaxed">{d.body}</p>
          </div>
        </button>
      ))}
    </div>
  );
}
