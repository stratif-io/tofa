import { AbsoluteFill, interpolate, spring, useCurrentFrame, useVideoConfig } from "remotion";
import { tokens } from "../theme/tokens";

interface TitleCardProps {
  /** Eyebrow label, e.g. "Step 1 of 2". */
  eyebrow?: string;
  /** Bold command-style title, rendered in monospace. */
  command: string;
  /** Short tagline under the title. */
  subtitle: string;
}

/**
 * Full-bleed title card between scenes. Eyebrow fades, headline springs,
 * subtitle and accent underline follow, everything fades out at the tail.
 */
export const TitleCard: React.FC<TitleCardProps> = ({ eyebrow, command, subtitle }) => {
  const frame = useCurrentFrame();
  const { durationInFrames, fps } = useVideoConfig();

  const headlineSpring = spring({
    fps,
    frame,
    config: tokens.spring,
    durationInFrames: 20,
  });
  const headlineScale = interpolate(headlineSpring, [0, 1], [0.92, 1]);
  const headlineOpacity = interpolate(headlineSpring, [0, 1], [0, 1]);

  const eyebrowFade = interpolate(frame, [0, 12], [0, 1], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });
  const underlineGrow = interpolate(frame, [10, 28], [0, 1], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });
  const subtitleFade = interpolate(frame, [16, 30], [0, 1], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });
  const fadeOut = interpolate(
    frame,
    [durationInFrames - 10, durationInFrames],
    [1, 0],
    { extrapolateLeft: "clamp", extrapolateRight: "clamp" },
  );

  return (
    <AbsoluteFill
      style={{
        backgroundColor: tokens.color.bg,
        alignItems: "center",
        justifyContent: "center",
        textAlign: "center",
      }}
    >
      <div style={{ opacity: fadeOut }}>
        {eyebrow && (
          <div
            style={{
              color: tokens.color.brand,
              fontFamily: tokens.font.mono,
              fontSize: tokens.type.eyebrow,
              letterSpacing: 3,
              textTransform: "uppercase",
              fontWeight: 600,
              opacity: eyebrowFade,
              marginBottom: tokens.s[6],
            }}
          >
            {eyebrow}
          </div>
        )}

        <div
          style={{
            color: tokens.color.brand,
            fontFamily: tokens.font.mono,
            fontSize: tokens.type.display,
            fontWeight: 700,
            letterSpacing: -1.5,
            opacity: headlineOpacity,
            transform: `scale(${headlineScale})`,
          }}
        >
          {command}
        </div>

        <div
          style={{
            width: 240,
            height: 2,
            margin: `${tokens.s[5]}px auto ${tokens.s[5]}px`,
            background:
              "linear-gradient(90deg, transparent, rgba(184, 158, 255, 0.7), transparent)",
            transform: `scaleX(${underlineGrow})`,
            transformOrigin: "center",
          }}
        />

        <div
          style={{
            color: tokens.color.text,
            fontSize: tokens.type.subtitle,
            fontFamily: tokens.font.body,
            opacity: subtitleFade,
          }}
        >
          {subtitle}
        </div>
      </div>
    </AbsoluteFill>
  );
};
