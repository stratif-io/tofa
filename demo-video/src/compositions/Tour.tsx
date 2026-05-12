import { AbsoluteFill, Easing, interpolate, OffthreadVideo, Sequence, staticFile, useCurrentFrame } from "remotion";
import { BrandCard } from "../components/BrandCard";
import { Callout } from "../components/Callout";
import { TitleCard } from "../components/TitleCard";

const FPS = 30;
// Playback multiplier applied to the whole tour. The rush plays at `SPEED`x
// via OffthreadVideo.playbackRate, and every other duration is divided by
// the same factor here so cards/callouts stay in sync.
const SPEED = 2;

// Composition-time frames for a duration expressed in original-tempo seconds.
// Use for: Sequence durations, callout enter/exit, ZoomLayer keyframes.
const sec = (s: number) => Math.round((s * FPS) / SPEED);

// Position inside the source rush, expressed in composition frames. Use only
// for OffthreadVideo `startFrom` / `endAt` (paired with playbackRate=SPEED).
const src = (s: number) => Math.round(s * FPS);

// Trim points on the source rush. The user types `tofa cam` around 0:32,
// which is the natural seam between the two scenes.
const RUSH = {
  scanStart: 1.0,
  scanEnd: 32.0,
  camStart: 32.0,
  camEnd: 63.0,
} as const;

// Scan-scene content cuts. Times are scene-relative rush seconds, matching
// the keyframe scale. Mid-scene cut: skip the passphrase prompt typing.
// Tail trim: drop the last few seconds (extra `tofa list` admin) so the
// scene wraps right after the import result has been read.
const SCAN_CUT_START_SEC = 16.20;
const SCAN_CUT_END_SEC = 19.10;
const SCAN_CUT_LEN_SEC = SCAN_CUT_END_SEC - SCAN_CUT_START_SEC;
const SCAN_TAIL_TRIM_SEC = 4.70;
const SCAN_TOTAL_SEC =
  (RUSH.scanEnd - RUSH.scanStart) - SCAN_CUT_LEN_SEC - SCAN_TAIL_TRIM_SEC;

// Cam scene speed-up: time-lapse the webcam-scan moment. Window picked to
// match the un-sped tour's frames 700 → 790 (scene-rush 12.27s → 18.27s).
// `CAM_FAST_X` is the requested multiplier on top of `SPEED`; the actual
// playback rate is clamped to MAX_PLAYBACK_RATE (Chromium's HTMLMediaElement
// hard limit), so the segment length follows whatever rate we can really set.
const MAX_PLAYBACK_RATE = 16;
const CAM_FAST_START_SEC = 12.2667;
const CAM_FAST_END_SEC = 18.2667;
const CAM_FAST_X = 10;
const CAM_FAST_RATE = Math.min(SPEED * CAM_FAST_X, MAX_PLAYBACK_RATE);
const CAM_FAST_SRC_LEN_SEC = CAM_FAST_END_SEC - CAM_FAST_START_SEC;
const CAM_PRE_FRAMES = sec(CAM_FAST_START_SEC);
const CAM_FAST_FRAMES = Math.max(
  1,
  Math.round((CAM_FAST_SRC_LEN_SEC * FPS) / CAM_FAST_RATE),
);
const CAM_POST_FRAMES = sec(
  (RUSH.camEnd - RUSH.camStart) - CAM_FAST_END_SEC,
);

const INTRO_FRAMES = sec(4);
const SCAN_INTRO_FRAMES = sec(4);
const SCAN_CLIP_FRAMES = sec(SCAN_TOTAL_SEC);
const MID_CARD_FRAMES = sec(3);
const CAM_CLIP_FRAMES = CAM_PRE_FRAMES + CAM_FAST_FRAMES + CAM_POST_FRAMES;
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
 * Scale + pan wrapper. `keyframes` drives scale over time; `originKeyframes`
 * (optional) animates the transform-origin as `[frame, [x%, y%]]`, allowing
 * pans within a held zoom. If only a static `origin` is supplied the pan is
 * skipped. Origin interpolation uses ease-in-out so pans feel cinematic.
 */
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
    <AbsoluteFill style={{ transform: `scale(${scale})`, transformOrigin: resolvedOrigin }}>
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
          subtitle="Two ways to capture a QR with CLI"
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
          origin="95% 0%"
          keyframes={[
            [0, 1],
            [sec(0.4), 1.8],
            [sec(7), 1.88 ],
            [sec(8), 1.0 ],
            [sec(11 ), 1.0  ],
            [sec(15.), 1.88 ],
            [SCAN_CLIP_FRAMES,  1.88],
          ]}
        >
          {/* Part A: rush 1.0 → 17.20 (= scene-rush 0 → 16.20). */}
          <Sequence from={0} durationInFrames={sec(SCAN_CUT_START_SEC)}>
            <OffthreadVideo
              src={RUSH_SRC}
              startFrom={src(RUSH.scanStart)}
              endAt={src(RUSH.scanStart + SCAN_CUT_START_SEC)}
              playbackRate={SPEED}
              style={{ width: "100%", height: "100%", objectFit: "contain" }}
            />
          </Sequence>
          {/* Part B: rush 20.10 → (32.0 - tail trim) = scene-rush 16.20 → end. */}
          <Sequence
            from={sec(SCAN_CUT_START_SEC)}
            durationInFrames={SCAN_CLIP_FRAMES - sec(SCAN_CUT_START_SEC)}
          >
            <OffthreadVideo
              src={RUSH_SRC}
              startFrom={src(RUSH.scanStart + SCAN_CUT_END_SEC)}
              endAt={src(RUSH.scanEnd - SCAN_TAIL_TRIM_SEC)}
              playbackRate={SPEED}
              style={{ width: "100%", height: "100%", objectFit: "contain" }}
            />
          </Sequence>
        </ZoomLayer>
        <Callout
          enterAt={sec(0.4)}
          exitAt={sec(5)}
          eyebrow="On screen"
          body="Two QR codes on desktop."
        />
        <Callout
          enterAt={sec(15)}
          exitAt={sec(22 - SCAN_CUT_LEN_SEC)}
          eyebrow="One command"
          body="`tofa scan` captures every display and decodes every QR."
        />
        <Callout
          enterAt={sec(22 - SCAN_CUT_LEN_SEC)}
          exitAt={SCAN_CLIP_FRAMES - 10}
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
            [0, 1.88],
            [CAM_CLIP_FRAMES, 1.88],
          ]}
          originKeyframes={[
            // Hold on the terminal (top-right) while `tofa cam` is typed and
            // the browser is opening.
            [0, [95, 0]],
            [sec(2), [95, 0]],
            // Pan over 1.5s to the left, where the browser permission popup
            // and webcam preview appear. Eased for a smooth glide.
            [sec(3), [0, 0]],
            [sec(8), [0, 0]],
            [sec(10 ), [95, 0]],
            [CAM_CLIP_FRAMES, [95, 0]],
          ]}
        >
          {/* Pre-fast: rush 32.00 → 44.27. */}
          <Sequence from={0} durationInFrames={CAM_PRE_FRAMES}>
            <OffthreadVideo
              src={RUSH_SRC}
              startFrom={src(RUSH.camStart)}
              endAt={src(RUSH.camStart + CAM_FAST_START_SEC)}
              playbackRate={SPEED}
              style={{ width: "100%", height: "100%", objectFit: "contain" }}
            />
          </Sequence>
          {/* Fast: rush 44.27 → 50.27 played at CAM_FAST_RATE (capped at 16x). */}
          <Sequence from={CAM_PRE_FRAMES} durationInFrames={CAM_FAST_FRAMES}>
            <OffthreadVideo
              src={RUSH_SRC}
              startFrom={src(RUSH.camStart + CAM_FAST_START_SEC)}
              endAt={src(RUSH.camStart + CAM_FAST_END_SEC)}
              playbackRate={CAM_FAST_RATE}
              style={{ width: "100%", height: "100%", objectFit: "contain" }}
            />
          </Sequence>
          {/* Post-fast: rush 50.27 → 63.00. */}
          <Sequence
            from={CAM_PRE_FRAMES + CAM_FAST_FRAMES}
            durationInFrames={CAM_POST_FRAMES}
          >
            <OffthreadVideo
              src={RUSH_SRC}
              startFrom={src(RUSH.camStart + CAM_FAST_END_SEC)}
              endAt={src(RUSH.camEnd)}
              playbackRate={SPEED}
              style={{ width: "100%", height: "100%", objectFit: "contain" }}
            />
          </Sequence>
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
          exitAt={Math.min(sec(30.5), CAM_CLIP_FRAMES - 10)}
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
