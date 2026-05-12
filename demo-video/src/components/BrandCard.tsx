import { AbsoluteFill, interpolate, spring, useCurrentFrame, useVideoConfig } from "remotion";

interface BrandCardProps {
  /** Wordmark, large. */
  title: string;
  /** Subtitle, one short line. */
  subtitle: string;
  /** Optional install / CTA strip (monospace). */
  cta?: string;
  /** Optional small footer line. */
  footer?: string;
}

/**
 * Intro/outro card with brand-styled big wordmark, subtitle, and optional CTA.
 * Springs in from below, fades out at the tail.
 */
export const BrandCard: React.FC<BrandCardProps> = ({ title, subtitle, cta, footer }) => {
  const frame = useCurrentFrame();
  const { durationInFrames, fps } = useVideoConfig();

  const titleSpring = spring({
    fps,
    frame,
    config: { damping: 18, stiffness: 110, mass: 0.7 },
    durationInFrames: 24,
  });
  const subtitleFade = interpolate(frame, [12, 26], [0, 1], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });
  const ctaFade = interpolate(frame, [22, 36], [0, 1], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });
  const footerFade = interpolate(frame, [30, 44], [0, 1], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });

  const fadeOut = interpolate(
    frame,
    [durationInFrames - 12, durationInFrames],
    [1, 0],
    { extrapolateLeft: "clamp", extrapolateRight: "clamp" },
  );

  const titleScale = interpolate(titleSpring, [0, 1], [0.92, 1]);
  const titleOpacity = interpolate(titleSpring, [0, 1], [0, 1]) * fadeOut;

  return (
    <AbsoluteFill
      style={{
        backgroundColor: "#0e0c14",
        alignItems: "center",
        justifyContent: "center",
        textAlign: "center",
      }}
    >
      <div
        style={{
          color: "#b89eff",
          fontFamily:
            "ui-monospace, 'SF Mono', Menlo, Consolas, monospace",
          fontSize: 120,
          fontWeight: 700,
          letterSpacing: -3,
          opacity: titleOpacity,
          transform: `scale(${titleScale})`,
        }}
      >
        {title}
      </div>

      <div
        style={{
          color: "#cfcbe0",
          fontSize: 30,
          marginTop: 20,
          fontFamily:
            "ui-sans-serif, system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif",
          opacity: subtitleFade * fadeOut,
        }}
      >
        {subtitle}
      </div>

      {cta && (
        <div
          style={{
            marginTop: 48,
            padding: "16px 28px",
            border: "1px solid rgba(184, 158, 255, 0.4)",
            borderRadius: 10,
            background: "rgba(184, 158, 255, 0.08)",
            color: "#f1eef8",
            fontFamily:
              "ui-monospace, 'SF Mono', Menlo, Consolas, monospace",
            fontSize: 26,
            letterSpacing: 0,
            opacity: ctaFade * fadeOut,
          }}
        >
          {cta}
        </div>
      )}

      {footer && (
        <div
          style={{
            color: "#7d7a8a",
            fontSize: 20,
            marginTop: 22,
            fontFamily:
              "ui-sans-serif, system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif",
            opacity: footerFade * fadeOut,
          }}
        >
          {footer}
        </div>
      )}
    </AbsoluteFill>
  );
};
