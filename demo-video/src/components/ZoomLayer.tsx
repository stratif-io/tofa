import React from "react";
import { AbsoluteFill, Easing, interpolate, useCurrentFrame } from "remotion";
import { tokens } from "../theme/tokens";

/**
 * Scales + pans the wrapped content using keyframes in comp-frame space.
 * Pan keyframes use the design-system ease-out curve.
 */
export const ZoomLayer: React.FC<
  React.PropsWithChildren<{
    keyframes: ReadonlyArray<readonly [number, number]>;
    origin?: string;
    originKeyframes?: ReadonlyArray<readonly [number, readonly [number, number]]>;
  }>
> = ({ keyframes, origin = "center", originKeyframes, children }) => {
  const frame = useCurrentFrame();
  const frames = keyframes.map(([f]) => f);
  const scales = keyframes.map(([, s]) => s);
  const scale = interpolate(frame, frames, scales, {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });

  let resolvedOrigin = origin;
  if (originKeyframes && originKeyframes.length > 0) {
    const oFrames = originKeyframes.map(([f]) => f);
    const xs = originKeyframes.map(([, [x]]) => x);
    const ys = originKeyframes.map(([, [, y]]) => y);
    const ease = Easing.bezier(...tokens.ease.out);
    const x = interpolate(frame, oFrames, xs, {
      extrapolateLeft: "clamp",
      extrapolateRight: "clamp",
      easing: ease,
    });
    const y = interpolate(frame, oFrames, ys, {
      extrapolateLeft: "clamp",
      extrapolateRight: "clamp",
      easing: ease,
    });
    resolvedOrigin = `${x}% ${y}%`;
  }

  return (
    <AbsoluteFill
      style={{ transform: `scale(${scale})`, transformOrigin: resolvedOrigin }}
    >
      {children}
    </AbsoluteFill>
  );
};

/** Build a keyframe list that's guaranteed to satisfy `interpolate`. */
export function withEndpoints<T>(
  kfs: ReadonlyArray<readonly [number, T]>,
  fallback: T,
  total: number,
): Array<readonly [number, T]> {
  if (kfs.length === 0) return [[0, fallback], [total, fallback]];
  const out: Array<readonly [number, T]> = [...kfs];
  if (out[0][0] > 0) out.unshift([0, out[0][1]]);
  if (out[out.length - 1][0] < total) out.push([total, out[out.length - 1][1]]);
  return out;
}
