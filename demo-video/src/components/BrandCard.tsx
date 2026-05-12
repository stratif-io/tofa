import React from "react";
import { AbsoluteFill, interpolate, spring, useCurrentFrame, useVideoConfig } from "remotion";
import { tokens } from "../theme/tokens";

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
  /** Font face for the wordmark. `display` (Fraunces) for brand titles like
   *  "TOFA" / "Get TOFA"; `mono` (JetBrains Mono) for command-style titles. */
  displayFont?: "display" | "mono";
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
export const BrandCard: React.FC<BrandCardProps> = ({
  title,
  subtitle,
  displayFont = "display",
  cta,
  footer,
}) => {
  const frame = useCurrentFrame();
  const { durationInFrames, fps } = useVideoConfig();

  const titleSpring = spring({
    fps,
    frame,
    config: tokens.spring,
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

  const titleFontFamily =
    displayFont === "mono" ? tokens.font.mono : tokens.font.display;
  // Fraunces benefits from looser tracking; mono titles still tighten slightly.
  const titleLetterSpacing = displayFont === "mono" ? -1.5 : -1;

  return (
    <AbsoluteFill
      style={{
        backgroundColor: tokens.color.bg,
        alignItems: "center",
        justifyContent: "center",
        textAlign: "center",
      }}
    >
      <div
        style={{
          color: tokens.color.brand,
          fontFamily: titleFontFamily,
          fontSize: tokens.type.displayXl,
          fontWeight: 700,
          letterSpacing: titleLetterSpacing,
          opacity: titleOpacity,
          transform: `scale(${titleScale})`,
        }}
      >
        {title}
      </div>

      <div
        style={{
          color: tokens.color.text,
          fontSize: tokens.type.subtitle,
          marginTop: tokens.s[5],
          fontFamily: tokens.font.body,
          opacity: subtitleFade * fadeOut,
        }}
      >
        {subtitle}
      </div>

      {ctaGroups && (
        <div
          style={{
            marginTop: tokens.s[7],
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            gap: tokens.s[4],
            opacity: ctaFade * fadeOut,
          }}
        >
          {ctaGroups.map((group, i) => (
            <React.Fragment key={i}>
              {i > 0 && (
                <div
                  style={{
                    color: tokens.color.textMuted,
                    fontFamily: tokens.font.mono,
                    fontSize: tokens.type.eyebrow,
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
                  padding: `${tokens.s[4]}px ${tokens.s[5]}px`,
                  border: `1px solid ${tokens.color.borderBrand}`,
                  borderRadius: tokens.r.lg,
                  background: tokens.color.brandBg,
                  color: tokens.color.text,
                  fontFamily: tokens.font.mono,
                  fontSize: tokens.type.bodyLg,
                  fontWeight: 500,
                  display: "flex",
                  flexDirection: "column",
                  gap: tokens.s[2],
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
                      gap: tokens.s[6],
                    }}
                  >
                    <span>{c.command}</span>
                    {c.note && (
                      <span
                        style={{
                          color: tokens.color.textMuted,
                          fontFamily: tokens.font.body,
                          fontSize: tokens.type.eyebrow,
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
            color: tokens.color.textMuted,
            fontSize: tokens.type.body,
            marginTop: tokens.s[5],
            fontFamily: tokens.font.body,
            opacity: footerFade * fadeOut,
          }}
        >
          {footer}
        </div>
      )}
    </AbsoluteFill>
  );
};
