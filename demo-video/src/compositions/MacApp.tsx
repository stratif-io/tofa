import React from "react";
import {
  AbsoluteFill,
  OffthreadVideo,
  Sequence,
  staticFile,
} from "remotion";
import { BrandCard, type CTAGroup } from "../components/BrandCard";
import { Callout } from "../components/Callout";
import { tokens } from "../theme/tokens";

// =============================================================================
// EDIT SCRIPT — directorial values for the macOS menu-bar app demo.
//
// The clip (public/mac-app.mov) is produced by `npm run cut-rush macApp` and
// is already cropped, trimmed, and 30 fps. Times in this file are clip
// seconds; the renderer maps them to comp frames via SPEC.speed.
// =============================================================================

const FPS = 30;

type CalloutSpec = {
  enter: number;
  exit: number;
  eyebrow?: string;
  body: string;
  position?: "bottom-left" | "bottom-right" | "top-right";
};

type MacAppSpec = {
  speed: number;
  intro: { title: string; subtitle: string; durationSec: number };
  scene: {
    src: string;
    durationSec: number;
    callouts?: ReadonlyArray<CalloutSpec>;
  };
  outro: {
    title: string;
    subtitle: string;
    cta?: string | CTAGroup[];
    footer?: string;
    durationSec: number;
  };
};

const SPEC: MacAppSpec = {
  speed: 1.5,
  intro: {
    title: "TOFA",
    subtitle: "Now in your macOS menu bar",
    durationSec: 3,
  },
  scene: {
    src: "mac-app.mov",
    durationSec: 58,
    callouts: [
      {
        enter: 0.5,
        exit: 7,
        eyebrow: "Menu bar",
        body: "Click the wink to drop the vault from the top of your screen.",
      },
      {
        enter: 7,
        exit: 14,
        eyebrow: "Unlock",
        body: "One passphrase. No phone in the loop.",
      },
      {
        enter: 14,
        exit: 24,
        eyebrow: "Live codes",
        body: "Each row ticks down a real OTP. Click to copy.",
        position: "bottom-right",
      },
      {
        enter: 24,
        exit: 40,
        eyebrow: "Same vault",
        body: "The exact accounts you ran from the CLI, in a native app.",
      },
      {
        enter: 40,
        exit: 58,
        eyebrow: "Lock back up",
        body: "Lock the vault when you're done — or quit; it locks on close.",
        position: "bottom-right",
      },
    ],
  },
  outro: {
    title: "Get TOFA",
    subtitle: "Open-source TOTP for your terminal, TUI, and menu bar",
    cta: [
      { commands: [{ command: "cargo install tofa" }] },
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
// Derivation.
// =============================================================================

const secToFrame = (s: number) => Math.round((s * FPS) / SPEC.speed);

const INTRO_FRAMES = secToFrame(SPEC.intro.durationSec);
const SCENE_FRAMES = secToFrame(SPEC.scene.durationSec);
const OUTRO_FRAMES = secToFrame(SPEC.outro.durationSec);

const OFFSETS = {
  intro: 0,
  scene: INTRO_FRAMES,
  outro: INTRO_FRAMES + SCENE_FRAMES,
};

export const MAC_APP_TOTAL_FRAMES = OFFSETS.outro + OUTRO_FRAMES;

// =============================================================================
// Composition.
// =============================================================================

export const MacAppDemo: React.FC = () => {
  return (
    <AbsoluteFill style={{ backgroundColor: tokens.color.bg }}>
      <Sequence from={OFFSETS.intro} durationInFrames={INTRO_FRAMES}>
        <BrandCard title={SPEC.intro.title} subtitle={SPEC.intro.subtitle} />
      </Sequence>

      <Sequence from={OFFSETS.scene} durationInFrames={SCENE_FRAMES}>
        <AbsoluteFill>
          <OffthreadVideo
            src={staticFile(SPEC.scene.src)}
            playbackRate={SPEC.speed}
            style={{
              width: "100%",
              height: "100%",
              objectFit: "cover",
              objectPosition: "center top",
            }}
          />
          {(SPEC.scene.callouts ?? []).map((c, i) => (
            <Callout
              key={i}
              enterAt={Math.max(0, secToFrame(c.enter))}
              exitAt={Math.min(secToFrame(c.exit), SCENE_FRAMES - 10)}
              eyebrow={c.eyebrow}
              body={c.body}
              position={c.position}
            />
          ))}
        </AbsoluteFill>
      </Sequence>

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
