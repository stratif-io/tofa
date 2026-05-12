import { AbsoluteFill, interpolate, spring, useCurrentFrame, useVideoConfig } from "remotion";

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
    config: { damping: 16, stiffness: 130, mass: 0.6 },
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
        backgroundColor: "#0e0c14",
        alignItems: "center",
        justifyContent: "center",
        textAlign: "center",
      }}
    >
      <div style={{ opacity: fadeOut }}>
        {eyebrow && (
          <div
            style={{
              color: "#b89eff",
              fontFamily:
                "ui-monospace, 'SF Mono', Menlo, Consolas, monospace",
              fontSize: 16,
              letterSpacing: 3,
              textTransform: "uppercase",
              fontWeight: 600,
              opacity: eyebrowFade,
              marginBottom: 28,
            }}
          >
            {eyebrow}
          </div>
        )}

        <div
          style={{
            color: "#b89eff",
            fontFamily:
              "ui-monospace, 'SF Mono', Menlo, Consolas, monospace",
            fontSize: 112,
            fontWeight: 700,
            letterSpacing: -2,
            opacity: headlineOpacity,
            transform: `scale(${headlineScale})`,
          }}
        >
          {command}
        </div>

        <div
          style={{
            width: 240,
            height: 3,
            margin: "20px auto 24px",
            background:
              "linear-gradient(90deg, transparent, rgba(184, 158, 255, 0.7), transparent)",
            transform: `scaleX(${underlineGrow})`,
            transformOrigin: "center",
          }}
        />

        <div
          style={{
            color: "#cfcbe0",
            fontSize: 28,
            fontFamily:
              "ui-sans-serif, system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif",
            opacity: subtitleFade,
          }}
        >
          {subtitle}
        </div>
      </div>
    </AbsoluteFill>
  );
};
