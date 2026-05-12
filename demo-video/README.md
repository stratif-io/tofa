# demo-video/

Remotion v4 project that composes the README demo videos.

Outputs land in `../docs/`:

- `demo-scan-cam.mp4` — H.264, 1280×800, 30 fps
- `demo-scan-cam.gif` — palettised, 800×?, 12 fps (small enough for GitHub)

The source rush lives at `public/scan-cam.mov` and is **gitignored** — drop a
fresh screen recording at that path before re-rendering.

## Render

```
npm install
npm run build         # both MP4 and GIF
npm run build:mp4
npm run build:gif
```

## Studio (interactive preview)

```
npm run studio
```

## Conventions

- 30 fps, frame-based animations only (`useCurrentFrame()`, never CSS keyframes)
- Deterministic: no `Math.random()`, no `new Date()`
- All timings in `src/compositions/Tour.tsx`'s `RUSH` map
- Title card visuals match the TUI's dark theme (`#0e0c14` bg, `#b89eff` accent)
