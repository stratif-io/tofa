import React from "react";
import { AbsoluteFill, interpolate, spring, useCurrentFrame, useVideoConfig } from "remotion";

export type CTACommand = {
  /** Shell command, rendered in monospace. */
  command: string;
  /** Short label rendered to the right of the command (e.g. "CLI + TUI"). */
  note?: string;
};

export type CTAGroup = {
  commands: CTACommand[];
};

interface BrandCardProps {
  /** Wordmark, large. */
  title: string;
  /** Subtitle, one short line. */
  subtitle: string;
  /** One or more install/usage CTAs. Multiple groups render stacked with an
   *  "OR" separator. Single string accepted for backwards compatibility. */
  cta?: string | CTAGroup[];
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

  // Normalise CTA into the structured form.
  const ctaGroups: CTAGroup[] | undefined = !cta
    ? undefined
    : typeof cta === "string"
      ? [{ commands: [{ command: cta }] }]
      : cta;

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

      {ctaGroups && (
        <div
          style={{
            marginTop: 40,
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            gap: 14,
            opacity: ctaFade * fadeOut,
          }}
        >
          {ctaGroups.map((group, i) => (
            <React.Fragment key={i}>
              {i > 0 && (
                <div
                  style={{
                    color: "#7d7a8a",
                    fontFamily:
                      "ui-monospace, 'SF Mono', Menlo, Consolas, monospace",
                    fontSize: 16,
                    letterSpacing: 4,
                    textTransform: "uppercase",
                    fontWeight: 600,
                  }}
                >
                  or
                </div>
              )}
              <div
                style={{
                  padding: "16px 28px",
                  border: "1px solid rgba(184, 158, 255, 0.4)",
                  borderRadius: 10,
                  background: "rgba(184, 158, 255, 0.08)",
                  color: "#f1eef8",
                  fontFamily:
                    "ui-monospace, 'SF Mono', Menlo, Consolas, monospace",
                  fontSize: 24,
                  display: "flex",
                  flexDirection: "column",
                  gap: 6,
                  textAlign: "left",
                  minWidth: 520,
                }}
              >
                {group.commands.map((c, j) => (
                  <div
                    key={j}
                    style={{
                      display: "flex",
                      justifyContent: "space-between",
                      alignItems: "baseline",
                      gap: 32,
                    }}
                  >
                    <span>{c.command}</span>
                    {c.note && (
                      <span
                        style={{
                          color: "#7d7a8a",
                          fontFamily:
                            "ui-sans-serif, system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif",
                          fontSize: 16,
                          fontStyle: "italic",
                        }}
                      >
                        {c.note}
                      </span>
                    )}
                  </div>
                ))}
              </div>
            </React.Fragment>
          ))}
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
