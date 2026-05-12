import { AbsoluteFill, interpolate, spring, useCurrentFrame, useVideoConfig } from "remotion";

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
    config: { damping: 18, stiffness: 140, mass: 0.6 },
    durationInFrames: 20,
  });
  const exitOpacity = interpolate(frame, [exitAt, exitAt + 10], [1, 0], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });
  const opacity = Math.min(enterSpring, exitOpacity);
  const translateY = interpolate(enterSpring, [0, 1], [16, 0]);

  const posStyles: Record<NonNullable<CalloutProps["position"]>, React.CSSProperties> = {
    "bottom-left": { left: 36, bottom: 36 },
    "bottom-right": { right: 36, bottom: 36 },
    "top-right": { right: 36, top: 36 },
  };

  return (
    <AbsoluteFill style={{ pointerEvents: "none" }}>
      <div
        style={{
          position: "absolute",
          ...posStyles[position],
          opacity,
          transform: `translateY(${translateY}px)`,
          background: "rgba(14, 12, 20, 0.86)",
          backdropFilter: "blur(8px)",
          border: "1px solid rgba(184, 158, 255, 0.35)",
          borderRadius: 10,
          padding: "12px 18px",
          maxWidth: 520,
          fontFamily:
            "ui-sans-serif, system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif",
          boxShadow: "0 8px 32px rgba(0, 0, 0, 0.45)",
        }}
      >
        {eyebrow && (
          <div
            style={{
              color: "#b89eff",
              fontFamily:
                "ui-monospace, 'SF Mono', Menlo, Consolas, monospace",
              fontSize: 13,
              letterSpacing: 1.4,
              textTransform: "uppercase",
              marginBottom: 4,
              fontWeight: 600,
            }}
          >
            {eyebrow}
          </div>
        )}
        <div style={{ color: "#f1eef8", fontSize: 20, lineHeight: 1.3, fontWeight: 500 }}>
          {body}
        </div>
      </div>
    </AbsoluteFill>
  );
};
