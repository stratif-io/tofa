import { useCallback, useEffect, useRef, useState } from 'react';

export interface Demo {
  src: string;
  poster: string;
  title: string;
  sub: string;
  body: string;
}

interface Props { demos: Demo[] }

export default function DemoGallery({ demos }: Props) {
  const [openIdx, setOpenIdx] = useState<number | null>(null);
  const triggerRefs = useRef<Array<HTMLButtonElement | null>>([]);

  const open = useCallback((idx: number) => setOpenIdx(idx), []);
  const close = useCallback(() => {
    setOpenIdx((current) => {
      if (current !== null) triggerRefs.current[current]?.focus();
      return null;
    });
  }, []);

  return (
    <>
      <div className="grid md:grid-cols-[1.4fr_1fr_1fr] gap-4">
        {demos.map((d, i) => (
          <button
            key={d.src}
            ref={(el) => { triggerRefs.current[i] = el; }}
            type="button"
            aria-haspopup="dialog"
            onClick={() => open(i)}
            data-umami-event={`demo-${d.src.split('/').pop()?.replace(/\.mp4$/, '')}`}
            className="group text-left rounded-tofa-lg bg-bg-sunken border border-border overflow-hidden hover:border-brand/40 transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-brand"
          >
            <div className="relative aspect-[16/10] bg-bg-elevated overflow-hidden">
              <img
                src={d.poster}
                alt=""
                loading="lazy"
                decoding="async"
                className="absolute inset-0 w-full h-full object-cover transition-transform group-hover:scale-[1.02]"
              />
              <div className="absolute inset-0 flex items-center justify-center bg-black/30 group-hover:bg-black/20 transition-colors">
                <div
                  aria-hidden="true"
                  className="w-14 h-14 rounded-full bg-brand text-on-brand flex items-center justify-center shadow-lg group-hover:scale-110 transition-transform"
                >
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M8 5v14l11-7z" />
                  </svg>
                </div>
              </div>
            </div>
            <div className="p-4">
              <h3 className="font-display font-bold text-text">{d.title}</h3>
              <div className="mt-1 font-mono text-xs text-text-subtle">{d.sub}</div>
              <p className="mt-2 text-sm text-text-muted leading-relaxed">{d.body}</p>
            </div>
          </button>
        ))}
      </div>

      {openIdx !== null && (
        <TheaterDialog demo={demos[openIdx]} onClose={close} />
      )}
    </>
  );
}

function TheaterDialog({ demo, onClose }: { demo: Demo; onClose: () => void }) {
  const dialogRef = useRef<HTMLDivElement | null>(null);
  const closeBtnRef = useRef<HTMLButtonElement | null>(null);
  const videoRef = useRef<HTMLVideoElement | null>(null);
  const titleId = `demo-${demo.src.replace(/[^a-z0-9]/gi, '-')}`;

  useEffect(() => {
    const prevOverflow = document.body.style.overflow;
    document.body.style.overflow = 'hidden';
    closeBtnRef.current?.focus();
    videoRef.current?.play().catch(() => {});

    function onKey(e: KeyboardEvent) {
      if (e.key === 'Escape') {
        e.preventDefault();
        onClose();
      }
    }
    document.addEventListener('keydown', onKey);
    return () => {
      document.removeEventListener('keydown', onKey);
      document.body.style.overflow = prevOverflow;
    };
  }, [onClose]);

  function onBackdropClick(e: React.MouseEvent<HTMLDivElement>) {
    if (e.target === e.currentTarget) onClose();
  }

  return (
    <div
      role="dialog"
      aria-modal="true"
      aria-labelledby={titleId}
      onClick={onBackdropClick}
      className="fixed inset-0 z-[100] flex items-center justify-center p-4 sm:p-8 bg-black/80 backdrop-blur-sm"
    >
      <div
        ref={dialogRef}
        className="relative w-full max-w-5xl rounded-tofa-lg bg-bg-elevated border border-border overflow-hidden shadow-2xl"
      >
        <button
          ref={closeBtnRef}
          type="button"
          onClick={onClose}
          aria-label="Close demo"
          className="absolute top-3 right-3 z-10 w-9 h-9 rounded-full bg-bg/80 backdrop-blur text-text hover:text-brand border border-border flex items-center justify-center transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-brand"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round">
            <path d="M18 6L6 18M6 6l12 12" />
          </svg>
        </button>
        <div className="aspect-video bg-black">
          <video
            ref={videoRef}
            data-testid="demo-video"
            src={demo.src}
            poster={demo.poster}
            controls
            autoPlay
            loop
            playsInline
            className="w-full h-full object-contain"
          />
        </div>
        <div className="p-5 border-t border-border">
          <h3 id={titleId} className="font-display font-bold text-text text-lg">{demo.title}</h3>
          <div className="mt-1 font-mono text-xs text-text-subtle">{demo.sub}</div>
          <p className="mt-2 text-sm text-text-muted leading-relaxed">{demo.body}</p>
        </div>
      </div>
    </div>
  );
}
