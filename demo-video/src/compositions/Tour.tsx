import { AbsoluteFill, OffthreadVideo, Sequence, staticFile } from "remotion";
import { TitleCard } from "../components/TitleCard";

const FPS = 30;
const sec = (s: number) => Math.round(s * FPS);

// Scene timing on the source rush (scan-cam.mov).
// The user types `tofa cam` around 0:32, so we use that as the natural split.
const RUSH = {
  scanStart: 1.0, // trim 1s of head dead air
  scanEnd: 32.0,
  camStart: 32.0,
  camEnd: 63.0,
};

const SCAN_INTRO_FRAMES = sec(1.5);
const SCAN_CLIP_FRAMES = sec(RUSH.scanEnd - RUSH.scanStart);
const MID_CARD_FRAMES = sec(1.5);
const CAM_CLIP_FRAMES = sec(RUSH.camEnd - RUSH.camStart);

export const TOTAL_FRAMES =
  SCAN_INTRO_FRAMES + SCAN_CLIP_FRAMES + MID_CARD_FRAMES + CAM_CLIP_FRAMES;

const SCENE1_START = SCAN_INTRO_FRAMES;
const MID_CARD_START = SCENE1_START + SCAN_CLIP_FRAMES;
const SCENE2_START = MID_CARD_START + MID_CARD_FRAMES;

const RUSH_SRC = staticFile("scan-cam.mov");

export const ScanCamTour: React.FC = () => {
  return (
    <AbsoluteFill style={{ backgroundColor: "#0e0c14" }}>
      <Sequence from={0} durationInFrames={SCAN_INTRO_FRAMES}>
        <TitleCard command="tofa scan" subtitle="Capture every QR on every screen at once" />
      </Sequence>

      <Sequence from={SCENE1_START} durationInFrames={SCAN_CLIP_FRAMES}>
        <OffthreadVideo
          src={RUSH_SRC}
          startFrom={sec(RUSH.scanStart)}
          endAt={sec(RUSH.scanEnd)}
          style={{ width: "100%", height: "100%", objectFit: "contain" }}
        />
      </Sequence>

      <Sequence from={MID_CARD_START} durationInFrames={MID_CARD_FRAMES}>
        <TitleCard command="tofa cam" subtitle="Scan with your laptop webcam in the browser" />
      </Sequence>

      <Sequence from={SCENE2_START} durationInFrames={CAM_CLIP_FRAMES}>
        <OffthreadVideo
          src={RUSH_SRC}
          startFrom={sec(RUSH.camStart)}
          endAt={sec(RUSH.camEnd)}
          style={{ width: "100%", height: "100%", objectFit: "contain" }}
        />
      </Sequence>
    </AbsoluteFill>
  );
};
