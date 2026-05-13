# demo-video/

Remotion v4 project that composes the README's demo videos.

Two compositions, two output families in `../docs/`:

| Composition    | Source rush              | Output                                       |
|----------------|--------------------------|----------------------------------------------|
| `ScanCamTour`  | `public/scan-cam.mov`    | `../docs/demo-scan-cam.{mp4,gif}` (1280×800) |
| `MacAppDemo`   | `public/mac-app-raw.mov` | `../docs/demo-app.{mp4,gif}` (1280×800)      |

Source rushes live on the maintainer's desktop and are **gitignored**
(`public/*.mov`). Drop a fresh recording at the matching path before
re-cutting.

## Pipeline

```
scripts/cut-rush.mjs        src/compositions/*.tsx
        │                            │
        ▼                            ▼
public/<scene>.mov   →   OffthreadVideo + ZoomLayer + Callouts   →   .mp4   →   .gif
   (ffmpeg pre-cut)             (Remotion render)                 (h.264)   (ffmpeg palette)
```

Editorial decisions (cuts, speed changes, crop region) live at the top of
`scripts/cut-rush.mjs`. Composition timing (titles, callouts, zoom, pan)
lives in `src/compositions/*.tsx`. Editing one doesn't usually require
re-running the other.

## Commands

```bash
npm install

# Interactive preview
npm run studio

# Full scan-cam pipeline (cut → mp4 → gif)
npm run build

# Mac-app only
npm run build:mac

# Individual stages
npm run cut-rush [name]    # name optional; defaults to every EDIT entry
npm run build:mp4          # ScanCamTour  → ../docs/demo-scan-cam.mp4
npm run build:gif          # ffmpeg palette pass → ../docs/demo-scan-cam.gif
npm run build:mac-mp4      # MacAppDemo   → ../docs/demo-app.mp4
npm run build:mac-gif      # ffmpeg palette pass → ../docs/demo-app.gif
```

## Conventions

- 30 fps; frame-based animations only (`useCurrentFrame()`, never CSS
  keyframes or `setInterval`).
- Deterministic: no `Math.random()`, no `new Date()`. Same input ⇒ same
  output.
- Colors, spacing, radius, typography, easing, and spring config are
  centralised in `src/theme/tokens.ts`, which mirrors
  `../docs/design/assets/css/tokens.css`. Components import from there
  instead of hard-coding values.
- Fonts (JetBrains Mono, Inter, Fraunces) load via `@remotion/google-fonts`
  with weight/subset trimming so renders don't issue 28 requests per font.
- Title cards and brand cards use `tokens.color.bg` (ink-900) for the
  background and `tokens.color.brand` (purple-300) as the accent —
  matches the design-system dark theme exactly.

## Adding a new demo

1. Drop the raw rush at `public/<name>-raw.mov` (auto-gitignored).
2. Add an EDIT entry to `scripts/cut-rush.mjs` with `source`, `output`,
   optional `crop: { x, y, w, h }`, and one or more `segments` (each
   `{ in, out, speed? }`). Run `npm run cut-rush <name>` and note the
   printed actual duration — that goes into the SPEC below.
3. Create `src/compositions/<Name>.tsx`. Mirror `MacApp.tsx` for a
   single-scene demo or `Tour.tsx` for multi-scene. Wire `BrandCard`
   (intro/outro), optional `TitleCard`, `ZoomLayer`, and `Callout`.
4. Register the composition in `src/Root.tsx` with its dimensions and
   duration.
5. Add `build:<name>-mp4` and `build:<name>-gif` scripts in
   `package.json`, mirroring the existing pair.
