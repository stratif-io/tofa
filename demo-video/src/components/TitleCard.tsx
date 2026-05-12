import { AbsoluteFill, interpolate, useCurrentFrame, useVideoConfig } from "remotion";

interface TitleCardProps {
  /** Bold command-style title, rendered in monospace. */
  command: string;
  /** Short tagline under the title. */
  subtitle: string;
}

/**
 * Full-bleed title card used between scenes. Fades in, holds, fades out.
 * Background and accent colors match the TOFA TUI's dark theme.
 */
export const TitleCard: React.FC<TitleCardProps> = ({ command, subtitle }) => {
  const frame = useCurrentFrame();
  const { durationInFrames } = useVideoConfig();

  const fadeIn = interpolate(frame, [0, 10], [0, 1], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });
  const fadeOut = interpolate(
    frame,
    [durationInFrames - 10, durationInFrames],
    [1, 0],
    { extrapolateLeft: "clamp", extrapolateRight: "clamp" },
  );
  const opacity = fadeIn * fadeOut;

  const slideUp = interpolate(frame, [0, 14], [16, 0], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });

  return (
    <AbsoluteFill
      style={{
        backgroundColor: "#0e0c14",
        alignItems: "center",
        justifyContent: "center",
        fontFamily:
          "ui-monospace, 'SF Mono', Menlo, Consolas, 'Liberation Mono', monospace",
      }}
    >
      <div
        style={{
          opacity,
          transform: `translateY(${slideUp}px)`,
          textAlign: "center",
        }}
      >
        <div
          style={{
            color: "#b89eff",
            fontSize: 96,
            fontWeight: 700,
            letterSpacing: -2,
          }}
        >
          {command}
        </div>
        <div
          style={{
            color: "#a1a0ad",
            fontSize: 28,
            marginTop: 18,
            fontFamily:
              "ui-sans-serif, system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif",
            fontWeight: 400,
          }}
        >
          {subtitle}
        </div>
      </div>
    </AbsoluteFill>
  );
};
