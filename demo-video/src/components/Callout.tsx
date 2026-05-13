import { AbsoluteFill, interpolate, spring, useCurrentFrame, useVideoConfig } from "remotion";
import { tokens } from "../theme/tokens";

interface CalloutProps {
  /** Frame (relative to the parent Sequence) when the callout enters. */
  enterAt: number;
  /** Frame when the callout starts exiting. */
  exitAt: number;
  /** Short label, e.g. "Step 1 of 2". */
  eyebrow?: string;
  /** Main copy, one sentence max. */
  body: string;
  /** Position on screen. */
  position?: "bottom-left" | "bottom-right" | "top-right";
}

/**
 * Lower-third style annotation that springs in over the footage,
 * holds, then fades out.
 */
export const Callout: React.FC<CalloutProps> = ({
  enterAt,
  exitAt,
  eyebrow,
  body,
  position = "bottom-left",
}) => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  if (frame < enterAt - 6 || frame > exitAt + 12) return null;

  const enterSpring = spring({
    fps,
    frame: frame - enterAt,
    config: tokens.spring,
    durationInFrames: 20,
  });
  const exitOpacity = interpolate(frame, [exitAt, exitAt + 10], [1, 0], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });
  const opacity = Math.min(enterSpring, exitOpacity);
  const translateY = interpolate(enterSpring, [0, 1], [16, 0]);

  const posStyles: Record<NonNullable<CalloutProps["position"]>, React.CSSProperties> = {
    "bottom-left": { left: tokens.s[6], bottom: tokens.s[6] },
    "bottom-right": { right: tokens.s[6], bottom: tokens.s[6] },
    "top-right": { right: tokens.s[6], top: tokens.s[6] },
  };

  return (
    <AbsoluteFill style={{ pointerEvents: "none" }}>
      <div
        style={{
          position: "absolute",
          ...posStyles[position],
          opacity,
          transform: `translateY(${translateY}px)`,
          background: tokens.color.calloutBackdrop,
          backdropFilter: "blur(8px)",
          border: `1px solid ${tokens.color.borderBrand}`,
          borderRadius: tokens.r.lg,
          padding: `${tokens.s[3]}px ${tokens.s[4]}px`,
          maxWidth: 520,
          fontFamily: tokens.font.body,
          boxShadow: "0 8px 32px rgba(0, 0, 0, 0.45)",
        }}
      >
        {eyebrow && (
          <div
            style={{
              color: tokens.color.brand,
              fontFamily: tokens.font.mono,
              fontSize: tokens.type.eyebrow,
              letterSpacing: 1.5,
              textTransform: "uppercase",
              marginBottom: tokens.s[1],
              fontWeight: 600,
            }}
          >
            {eyebrow}
          </div>
        )}
        <div
          style={{
            color: tokens.color.text,
            fontSize: tokens.type.body,
            lineHeight: 1.4,
            fontWeight: 500,
          }}
        >
          {body}
        </div>
      </div>
    </AbsoluteFill>
  );
};
