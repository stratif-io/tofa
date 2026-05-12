import React from "react";
import {
  AbsoluteFill,
  OffthreadVideo,
  Sequence,
  staticFile,
} from "remotion";
import { BrandCard, type CTAGroup } from "../components/BrandCard";
import { Callout } from "../components/Callout";
import { TitleCard } from "../components/TitleCard";
import { ZoomLayer, withEndpoints } from "../components/ZoomLayer";
import { tokens } from "../theme/tokens";

// =============================================================================
// EDIT SCRIPT — change directorial values here.
//
// Editorial cuts and speed changes live in `scripts/cut-rush.mjs` and are
// baked into `public/<scene>.mov`. After running `npm run cut-rush`, copy the
// printed clip durations into the `durationSec` field of each scene below.
//
// Times in this file are in **clip seconds** — positions in the pre-cut
// scene clip, not the raw rush.
// =============================================================================

const FPS = 30;

type CalloutPosition = "bottom-left" | "bottom-right" | "top-right";

type CalloutSpec = {
  /** Clip second the callout enters. */
  enter: number;
  /** Clip second it starts fading out. */
  exit: number;
  eyebrow?: string;
  body: string;
  position?: CalloutPosition;
};

type SceneSpec = {
  title: {
    eyebrow: string;
    command: string;
    subtitle: string;
    durationSec: number;
  };
  /** staticFile-relative path. Produce with `npm run cut-rush`. */
  src: string;
  /** Total length of the pre-cut clip in seconds. Read from cut-rush output. */
  durationSec: number;
  /** Static transform-origin if no pan keyframes. */
  origin?: string;
  /** Scale keyframes: [clipSec, scale]. */
  zoom?: ReadonlyArray<readonly [number, number]>;
  /** Origin keyframes (eased): [clipSec, [x%, y%]]. */
  pan?: ReadonlyArray<readonly [number, readonly [number, number]]>;
  callouts?: ReadonlyArray<CalloutSpec>;
};

type TourSpec = {
  /** Global playback rate applied to every video segment. */
  speed: number;
  intro: { title: string; subtitle: string; durationSec: number };
  scenes: readonly SceneSpec[];
  outro: {
    title: string;
    subtitle: string;
    cta?: string | CTAGroup[];
    footer?: string;
    durationSec: number;
  };
};

const SPEC: TourSpec = {
  speed: 1.5,
  intro: {
    title: "TOFA",
    subtitle: "Two ways to capture a QR with CLI",
    durationSec: 3,
  },
  scenes: [
    {
      title: {
        eyebrow: "Step 1 of 2",
        command: "tofa scan",
        subtitle: "Capture every QR on every screen at once",
        durationSec: 3,
      },
      src: "scan.mov",
      durationSec: 25.0,
      origin: "95% 0%",
      zoom: [
        [0, 1],
        [0.4, 1.8],
        [9.25, 1.88],
        [10.5, 1.0],
        [14, 1.88],
      ],
      callouts: [
        { enter: 0.4, exit: 5, eyebrow: "On screen", body: "Two QR codes on desktop." },
        {
          enter: 15,
          exit: 19.1,
          eyebrow: "One command",
          body: "` tofa scan` captures every display and decodes every QR.",
        },
        {
          enter: 19.1,
          exit: 23.4,
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
      src: "cam.mov",
      durationSec: 18.53,
      zoom: [[0, 1.88]],
      pan: [
        [0, [95, 0]],
        [2, [95, 0]],
        [3.5, [0, 0]],
        [9, [0, 0]],
        [11.5, [95, 0]],
      ],
      callouts: [
        {
          enter: 0.5,
          exit: 7,
          eyebrow: "Browser",
          body: "`tofa cam` opens a local URL that streams your webcam.",
        },
        {
          enter: 12.87,
          exit: 16.6,
          eyebrow: "Detected",
          body: "The third QR is decoded the moment it's centred.",
        },
        {
          enter: 16.6,
          exit: 25.6,
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
    cta: [
      {
        commands: [{ command: "cargo install tofa" }],
      },
      {
        commands: [
          { command: "brew tap stratif-io/tofa" },
          { command: "brew install tofa", note: "CLI + TUI" },
          { command: "brew install --cask tofa", note: "macOS app" },
        ],
      },
    ],
    footer: "docs.tofa.stratif.io",
    durationSec: 3.2,
  },
};

// =============================================================================
// Derivation. Trivial now that cuts and speedups live in the clip.
// =============================================================================

const secToFrame = (s: number) => Math.round((s * FPS) / SPEC.speed);

const INTRO_FRAMES = secToFrame(SPEC.intro.durationSec);
const OUTRO_FRAMES = secToFrame(SPEC.outro.durationSec);
const SCENES = SPEC.scenes.map((spec) => ({
  spec,
  totalFrames: secToFrame(spec.durationSec),
  titleFrames: secToFrame(spec.title.durationSec),
}));

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
// Scene renderer. One OffthreadVideo per scene; zoom and callouts overlay.
// =============================================================================

const SceneRenderer: React.FC<{ spec: SceneSpec; speed: number }> = ({
  spec,
  speed,
}) => {
  const total = secToFrame(spec.durationSec);
  const toFrame = (s: number) => Math.round((s * FPS) / speed);

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

  return (
    <>
      <ZoomLayer
        keyframes={zoomKfs}
        origin={spec.origin}
        originKeyframes={panKfs}
      >
        <OffthreadVideo
          src={staticFile(spec.src)}
          playbackRate={speed}
          style={{ width: "100%", height: "100%", objectFit: "contain" }}
        />
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
    <AbsoluteFill style={{ backgroundColor: tokens.color.bg }}>
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
