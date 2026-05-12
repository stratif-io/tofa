import { AbsoluteFill, interpolate, OffthreadVideo, Sequence, staticFile, useCurrentFrame } from "remotion";
import { BrandCard } from "../components/BrandCard";
import { Callout } from "../components/Callout";
import { TitleCard } from "../components/TitleCard";

const FPS = 30;
const sec = (s: number) => Math.round(s * FPS);

// Trim points on the source rush. The user types `tofa cam` around 0:32,
// which is the natural seam between the two scenes.
const RUSH = {
  scanStart: 1.0,
  scanEnd: 32.0,
  camStart: 32.0,
  camEnd: 63.0,
} as const;

const INTRO_FRAMES = sec(2.4);
const SCAN_INTRO_FRAMES = sec(1.6);
const SCAN_CLIP_FRAMES = sec(RUSH.scanEnd - RUSH.scanStart);
const MID_CARD_FRAMES = sec(1.6);
const CAM_CLIP_FRAMES = sec(RUSH.camEnd - RUSH.camStart);
const OUTRO_FRAMES = sec(3.2);

export const TOTAL_FRAMES =
  INTRO_FRAMES +
  SCAN_INTRO_FRAMES +
  SCAN_CLIP_FRAMES +
  MID_CARD_FRAMES +
  CAM_CLIP_FRAMES +
  OUTRO_FRAMES;

const SCAN_INTRO_START = INTRO_FRAMES;
const SCENE1_START = SCAN_INTRO_START + SCAN_INTRO_FRAMES;
const MID_CARD_START = SCENE1_START + SCAN_CLIP_FRAMES;
const SCENE2_START = MID_CARD_START + MID_CARD_FRAMES;
const OUTRO_START = SCENE2_START + CAM_CLIP_FRAMES;

const RUSH_SRC = staticFile("scan-cam.mov");

/**
 * Scale wrapper that interpolates between an ordered list of `[frame, scale]`
 * keyframes. Used for brief punch-in zooms that hold then settle back — keeps
 * most of the scene at scale 1.0 so the GIF palette compression stays cheap.
 */
const ZoomLayer: React.FC<
  React.PropsWithChildren<{
    keyframes: ReadonlyArray<readonly [number, number]>;
    origin?: string;
  }>
> = ({ keyframes, origin = "center", children }) => {
  const frame = useCurrentFrame();
  const frames = keyframes.map(([f]) => f);
  const scales = keyframes.map(([, s]) => s);
  const scale = interpolate(frame, frames, scales, {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });
  return (
    <AbsoluteFill style={{ transform: `scale(${scale})`, transformOrigin: origin }}>
      {children}
    </AbsoluteFill>
  );
};

export const ScanCamTour: React.FC = () => {
  return (
    <AbsoluteFill style={{ backgroundColor: "#0e0c14" }}>
      {/* Intro */}
      <Sequence from={0} durationInFrames={INTRO_FRAMES}>
        <BrandCard
          title="TOFA"
          subtitle="Two ways to capture a QR — without reaching for your phone"
        />
      </Sequence>

      {/* Scene 1 title card */}
      <Sequence from={SCAN_INTRO_START} durationInFrames={SCAN_INTRO_FRAMES}>
        <TitleCard
          eyebrow="Step 1 of 2"
          command="tofa scan"
          subtitle="Capture every QR on every screen at once"
        />
      </Sequence>

      {/* Scene 1 footage — terminal-focused zoom: in at start, out at scene-10s,
          back in at scene-12s, settle at scene-end so the mid card cuts cleanly. */}
      <Sequence from={SCENE1_START} durationInFrames={SCAN_CLIP_FRAMES}>
        <ZoomLayer
          origin="75% 18%"
          keyframes={[
            [0, 1.0],
            [sec(0.4), 1.15],
            [sec(10), 1.15],
            [sec(10.5), 1.0],
            [sec(12), 1.0],
            [sec(12.5), 1.15],
            [sec(29.5), 1.15],
            [sec(30.5), 1.0],
            [SCAN_CLIP_FRAMES, 1.0],
          ]}
        >
          <OffthreadVideo
            src={RUSH_SRC}
            startFrom={sec(RUSH.scanStart)}
            endAt={sec(RUSH.scanEnd)}
            style={{ width: "100%", height: "100%", objectFit: "contain" }}
          />
        </ZoomLayer>
        <Callout
          enterAt={sec(0.4)}
          exitAt={sec(5)}
          eyebrow="On screen"
          body="Two QR codes on desktop."
        />
        <Callout
          enterAt={sec(15)}
          exitAt={sec(22)}
          eyebrow="One command"
          body="`tofa scan` captures every display and decodes every QR."
        />
        <Callout
          enterAt={sec(22)}
          exitAt={sec(30.5)}
          eyebrow="Result"
          body="Imported 2 accounts from 1 screen."
          position="bottom-right"
        />
      </Sequence>

      {/* Scene 2 title card */}
      <Sequence from={MID_CARD_START} durationInFrames={MID_CARD_FRAMES}>
        <TitleCard
          eyebrow="Step 2 of 2"
          command="tofa cam"
          subtitle="Scan with your laptop webcam in the browser"
        />
      </Sequence>

      {/* Scene 2 footage — static (matches scan: zoom only at the top of the scene if added later) */}
      <Sequence from={SCENE2_START} durationInFrames={CAM_CLIP_FRAMES}>
        <ZoomLayer
          keyframes={[
            [0, 1.0],
            [CAM_CLIP_FRAMES, 1.0],
          ]}
        >
          <OffthreadVideo
            src={RUSH_SRC}
            startFrom={sec(RUSH.camStart)}
            endAt={sec(RUSH.camEnd)}
            style={{ width: "100%", height: "100%", objectFit: "contain" }}
          />
        </ZoomLayer>
        <Callout
          enterAt={sec(0.5)}
          exitAt={sec(7)}
          eyebrow="Browser"
          body="`tofa cam` opens a local URL that streams your webcam."
        />
        <Callout
          enterAt={sec(15)}
          exitAt={sec(22)}
          eyebrow="Detected"
          body="The third QR is decoded the moment it's centred."
        />
        <Callout
          enterAt={sec(22)}
          exitAt={sec(30.5)}
          eyebrow="Vault"
          body="Three accounts now ticking down in your terminal."
          position="bottom-right"
        />
      </Sequence>

      {/* Outro */}
      <Sequence from={OUTRO_START} durationInFrames={OUTRO_FRAMES}>
        <BrandCard
          title="Get TOFA"
          subtitle="Open-source TOTP for your terminal, TUI, and menu bar"
          cta="brew install --cask tofa"
          footer="docs.tofa.stratif.io"
        />
      </Sequence>
    </AbsoluteFill>
  );
};
