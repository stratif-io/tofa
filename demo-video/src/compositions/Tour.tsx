import React from "react";
import {
  AbsoluteFill,
  Easing,
  interpolate,
  OffthreadVideo,
  Sequence,
  staticFile,
  useCurrentFrame,
} from "remotion";
import { BrandCard } from "../components/BrandCard";
import { Callout } from "../components/Callout";
import { TitleCard } from "../components/TitleCard";

// =============================================================================
// EDIT SCRIPT — change directorial values here. Everything downstream derives
// from this object. All time values are in **scene-rush seconds**: the tempo
// of the source rush as if no speedups or cuts existed. The renderer maps
// them to composition frames automatically, accounting for cuts, speedups,
// and tail trims.
// =============================================================================

const FPS = 30;
const MAX_PLAYBACK_RATE = 16; // Chromium HTMLMediaElement hard cap.
const RUSH_SRC = staticFile("scan-cam.mov");

type CalloutPosition = "bottom-left" | "bottom-right" | "top-right";

type CalloutSpec = {
  /** Scene-rush second the callout enters. */
  enter: number;
  /** Scene-rush second it starts fading out. Clamped to scene end if needed. */
  exit: number;
  eyebrow?: string;
  body: string;
  position?: CalloutPosition;
};

type SpeedChange = {
  /** Scene-rush range to retime. */
  at: readonly [number, number];
  /** Multiplier on top of base speed. >1 fast-forward, <1 slow-mo. Capped. */
  factor: number;
};

type SceneSpec = {
  title: {
    eyebrow: string;
    command: string;
    subtitle: string;
    durationSec: number;
  };
  /** Absolute source-rush window (seconds in scan-cam.mov). */
  source: readonly [number, number];
  /** Scene-rush ranges to drop. */
  cuts?: ReadonlyArray<readonly [number, number]>;
  /** Scene-rush ranges to retime. */
  speedChanges?: ReadonlyArray<SpeedChange>;
  /** Drop this many seconds off the tail of the scene's source. */
  tailTrimSec?: number;
  /** Static transform-origin if no pan keyframes. */
  origin?: string;
  /** Scale keyframes: [sceneRushSec, scale]. */
  zoom?: ReadonlyArray<readonly [number, number]>;
  /** Origin keyframes (eased): [sceneRushSec, [x%, y%]]. */
  pan?: ReadonlyArray<readonly [number, readonly [number, number]]>;
  callouts?: ReadonlyArray<CalloutSpec>;
};

type TourSpec = {
  /** Base playback rate applied to every video segment. */
  speed: number;
  intro: { title: string; subtitle: string; durationSec: number };
  scenes: readonly SceneSpec[];
  outro: {
    title: string;
    subtitle: string;
    cta?: string;
    footer?: string;
    durationSec: number;
  };
};

const SPEC: TourSpec = {
  speed: 2,
  intro: {
    title: "TOFA",
    subtitle: "Two ways to capture a QR with CLI",
    durationSec: 4,
  },
  scenes: [
    {
      title: {
        eyebrow: "Step 1 of 2",
        command: "tofa scan",
        subtitle: "Capture every QR on every screen at once",
        durationSec: 4,
      },
      source: [1.0, 32.0],
      cuts: [[16.2, 19.1]], // skip the passphrase-prompt typing
      tailTrimSec: 4.7, // drop the trailing `tofa list` admin
      origin: "95% 0%",
      zoom: [
        [0, 1],
        [0.4, 1.8],
        [7, 1.88],
        [8, 1.0],
        [11, 1.0],
        [15, 1.88],
      ],
      callouts: [
        { enter: 0.4, exit: 5, eyebrow: "On screen", body: "Two QR codes on desktop." },
        {
          enter: 15,
          exit: 22,
          eyebrow: "One command",
          body: "`tofa scan` captures every display and decodes every QR.",
        },
        {
          enter: 22,
          exit: 30.5,
          eyebrow: "Result",
          body: "Imported 2 accounts from 1 screen.",
          position: "bottom-right",
        },
      ],
    },
    {
      title: {
        eyebrow: "Step 2 of 2",
        command: "tofa cam",
        subtitle: "Scan with your laptop webcam in the browser",
        durationSec: 3,
      },
      source: [32.0, 63.0],
      speedChanges: [{ at: [12.2667, 18.2667], factor: 10 }],
      zoom: [[0, 1.88]],
      pan: [
        [0, [95, 0]],
        [5, [95, 0]],
        [6.5, [20, 0]],
      ],
      callouts: [
        {
          enter: 0.5,
          exit: 7,
          eyebrow: "Browser",
          body: "`tofa cam` opens a local URL that streams your webcam.",
        },
        {
          enter: 15,
          exit: 22,
          eyebrow: "Detected",
          body: "The third QR is decoded the moment it's centred.",
        },
        {
          enter: 22,
          exit: 30.5,
          eyebrow: "Vault",
          body: "Three accounts now ticking down in your terminal.",
          position: "bottom-right",
        },
      ],
    },
  ],
  outro: {
    title: "Get TOFA",
    subtitle: "Open-source TOTP for your terminal, TUI, and menu bar",
    cta: "brew install --cask tofa",
    footer: "docs.tofa.stratif.io",
    durationSec: 3.2,
  },
};

// =============================================================================
// Derivation. Everything below is generated from SPEC — don't tweak by hand.
// =============================================================================

type PlaybackPiece = {
  /** Absolute source-rush seconds. */
  sourceStart: number;
  sourceEnd: number;
  /** playbackRate for OffthreadVideo. */
  rate: number;
  /** Composition frames this piece occupies. */
  compFrames: number;
};

/** Decompose a scene into back-to-back playback pieces. */
function piecesFor(scene: SceneSpec, speed: number): PlaybackPiece[] {
  const sceneStart = scene.source[0];
  const sceneEnd = scene.source[1] - (scene.tailTrimSec ?? 0);
  const toAbs = (s: number) => sceneStart + s;

  type Part = { src: readonly [number, number]; rate: number };
  let parts: Part[] = [{ src: [sceneStart, sceneEnd], rate: speed }];

  // Remove cut ranges.
  for (const [c0, c1] of (scene.cuts ?? []).map(
    ([a, b]) => [toAbs(a), toAbs(b)] as const,
  )) {
    parts = parts.flatMap((p) => {
      const [a, b] = p.src;
      if (c1 <= a || c0 >= b) return [p];
      const out: Part[] = [];
      if (c0 > a) out.push({ src: [a, c0], rate: p.rate });
      if (c1 < b) out.push({ src: [c1, b], rate: p.rate });
      return out;
    });
  }

  // Apply speed changes (clamping to the browser cap).
  for (const sc of scene.speedChanges ?? []) {
    const [s0, s1] = [toAbs(sc.at[0]), toAbs(sc.at[1])];
    const target = speed * sc.factor;
    const rate = Math.min(Math.max(target, 0.0625), MAX_PLAYBACK_RATE);
    parts = parts.flatMap((p) => {
      const [a, b] = p.src;
      if (s1 <= a || s0 >= b) return [p];
      const lo = Math.max(a, s0);
      const hi = Math.min(b, s1);
      const out: Part[] = [];
      if (lo > a) out.push({ src: [a, lo], rate: p.rate });
      out.push({ src: [lo, hi], rate });
      if (hi < b) out.push({ src: [hi, b], rate: p.rate });
      return out;
    });
  }

  return parts.map((p) => ({
    sourceStart: p.src[0],
    sourceEnd: p.src[1],
    rate: p.rate,
    compFrames: Math.max(1, Math.round(((p.src[1] - p.src[0]) * FPS) / p.rate)),
  }));
}

/** Map a scene-rush moment to a composition frame within the scene. */
function sceneSecToFrame(
  scene: SceneSpec,
  speed: number,
  sceneRushSec: number,
): number {
  const target = scene.source[0] + sceneRushSec;
  const ps = piecesFor(scene, speed);
  let compFrame = 0;
  for (const p of ps) {
    if (target <= p.sourceStart) return compFrame;
    if (target < p.sourceEnd) {
      return compFrame + Math.round(((target - p.sourceStart) * FPS) / p.rate);
    }
    compFrame += p.compFrames;
  }
  return compFrame;
}

const sceneTotalFrames = (scene: SceneSpec, speed: number) =>
  piecesFor(scene, speed).reduce((a, p) => a + p.compFrames, 0);

const secAtSpeed = (s: number, speed: number) =>
  Math.round((s * FPS) / speed);

const INTRO_FRAMES = secAtSpeed(SPEC.intro.durationSec, SPEC.speed);
const OUTRO_FRAMES = secAtSpeed(SPEC.outro.durationSec, SPEC.speed);
const SCENES = SPEC.scenes.map((spec) => ({
  spec,
  pieces: piecesFor(spec, SPEC.speed),
  totalFrames: sceneTotalFrames(spec, SPEC.speed),
  titleFrames: secAtSpeed(spec.title.durationSec, SPEC.speed),
}));

/** Frame offsets for each part of the tour. */
const OFFSETS = (() => {
  const sceneOffsets: Array<{ title: number; clip: number }> = [];
  let cursor = INTRO_FRAMES;
  for (const s of SCENES) {
    sceneOffsets.push({ title: cursor, clip: cursor + s.titleFrames });
    cursor += s.titleFrames + s.totalFrames;
  }
  return { intro: 0, scenes: sceneOffsets, outro: cursor };
})();

export const TOTAL_FRAMES = OFFSETS.outro + OUTRO_FRAMES;

// =============================================================================
// ZoomLayer: scales + pans the wrapped content using keyframes in comp-frame
// space. Pan keyframes use ease-in-out so glides feel cinematic.
// =============================================================================

const ZoomLayer: React.FC<
  React.PropsWithChildren<{
    keyframes: ReadonlyArray<readonly [number, number]>;
    origin?: string;
    originKeyframes?: ReadonlyArray<readonly [number, readonly [number, number]]>;
  }>
> = ({ keyframes, origin = "center", originKeyframes, children }) => {
  const frame = useCurrentFrame();
  const frames = keyframes.map(([f]) => f);
  const scales = keyframes.map(([, s]) => s);
  const scale = interpolate(frame, frames, scales, {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });

  let resolvedOrigin = origin;
  if (originKeyframes && originKeyframes.length > 0) {
    const oFrames = originKeyframes.map(([f]) => f);
    const xs = originKeyframes.map(([, [x]]) => x);
    const ys = originKeyframes.map(([, [, y]]) => y);
    const ease = Easing.bezier(0.4, 0, 0.2, 1);
    const x = interpolate(frame, oFrames, xs, {
      extrapolateLeft: "clamp",
      extrapolateRight: "clamp",
      easing: ease,
    });
    const y = interpolate(frame, oFrames, ys, {
      extrapolateLeft: "clamp",
      extrapolateRight: "clamp",
      easing: ease,
    });
    resolvedOrigin = `${x}% ${y}%`;
  }

  return (
    <AbsoluteFill
      style={{ transform: `scale(${scale})`, transformOrigin: resolvedOrigin }}
    >
      {children}
    </AbsoluteFill>
  );
};

// =============================================================================
// Scene renderer. Reads SceneSpec and emits the OffthreadVideo segments,
// ZoomLayer keyframes, and callouts.
// =============================================================================

/** Build a keyframe list that's guaranteed to satisfy `interpolate`. */
function withEndpoints<T>(
  kfs: ReadonlyArray<readonly [number, T]>,
  fallback: T,
  total: number,
): Array<readonly [number, T]> {
  if (kfs.length === 0) return [[0, fallback], [total, fallback]];
  const out: Array<readonly [number, T]> = [...kfs];
  if (out[0][0] > 0) out.unshift([0, out[0][1]]);
  if (out[out.length - 1][0] < total) out.push([total, out[out.length - 1][1]]);
  return out;
}

const SceneRenderer: React.FC<{ spec: SceneSpec; speed: number }> = ({
  spec,
  speed,
}) => {
  const ps = piecesFor(spec, speed);
  const total = ps.reduce((a, p) => a + p.compFrames, 0);
  const toFrame = (s: number) => sceneSecToFrame(spec, speed, s);

  const zoomKfs = withEndpoints(
    (spec.zoom ?? []).map(([s, scale]) => [toFrame(s), scale] as const),
    1,
    total,
  );
  const panKfs = spec.pan
    ? withEndpoints(
        spec.pan.map(([s, o]) => [toFrame(s), o] as const),
        [50, 50] as readonly [number, number],
        total,
      )
    : undefined;

  let offset = 0;
  return (
    <>
      <ZoomLayer
        keyframes={zoomKfs}
        origin={spec.origin}
        originKeyframes={panKfs}
      >
        {ps.map((p, i) => {
          const seq = (
            <Sequence key={i} from={offset} durationInFrames={p.compFrames}>
              <OffthreadVideo
                src={RUSH_SRC}
                startFrom={Math.round(p.sourceStart * FPS)}
                endAt={Math.round(p.sourceEnd * FPS)}
                playbackRate={p.rate}
                style={{ width: "100%", height: "100%", objectFit: "contain" }}
              />
            </Sequence>
          );
          offset += p.compFrames;
          return seq;
        })}
      </ZoomLayer>
      {(spec.callouts ?? []).map((c, i) => (
        <Callout
          key={i}
          enterAt={Math.max(0, toFrame(c.enter))}
          exitAt={Math.min(toFrame(c.exit), total - 10)}
          eyebrow={c.eyebrow}
          body={c.body}
          position={c.position}
        />
      ))}
    </>
  );
};

// =============================================================================
// Top-level composition.
// =============================================================================

export const ScanCamTour: React.FC = () => {
  return (
    <AbsoluteFill style={{ backgroundColor: "#0e0c14" }}>
      <Sequence from={OFFSETS.intro} durationInFrames={INTRO_FRAMES}>
        <BrandCard title={SPEC.intro.title} subtitle={SPEC.intro.subtitle} />
      </Sequence>

      {SCENES.map((s, i) => (
        <React.Fragment key={i}>
          <Sequence
            from={OFFSETS.scenes[i].title}
            durationInFrames={s.titleFrames}
          >
            <TitleCard
              eyebrow={s.spec.title.eyebrow}
              command={s.spec.title.command}
              subtitle={s.spec.title.subtitle}
            />
          </Sequence>
          <Sequence
            from={OFFSETS.scenes[i].clip}
            durationInFrames={s.totalFrames}
          >
            <SceneRenderer spec={s.spec} speed={SPEC.speed} />
          </Sequence>
        </React.Fragment>
      ))}

      <Sequence from={OFFSETS.outro} durationInFrames={OUTRO_FRAMES}>
        <BrandCard
          title={SPEC.outro.title}
          subtitle={SPEC.outro.subtitle}
          cta={SPEC.outro.cta}
          footer={SPEC.outro.footer}
        />
      </Sequence>
    </AbsoluteFill>
  );
};
