import { loadFont as loadInter } from "@remotion/google-fonts/Inter";
import { loadFont as loadJetBrainsMono } from "@remotion/google-fonts/JetBrainsMono";
import { loadFont as loadFraunces } from "@remotion/google-fonts/Fraunces";

// Trim weights and subsets to what the components actually use so we don't
// hammer Google Fonts with 28 requests on every render.
const { fontFamily: interFamily } = loadInter("normal", {
  weights: ["400", "500"],
  subsets: ["latin"],
});
const { fontFamily: monoFamily } = loadJetBrainsMono("normal", {
  weights: ["400", "500", "600", "700"],
  subsets: ["latin"],
});
const { fontFamily: frauncesFamily } = loadFraunces("normal", {
  weights: ["700"],
  subsets: ["latin"],
});

export const tokens = {
  color: {
    bg: "#0a0a12",
    bgElevated: "#14131f",
    surface: "#14131f",
    surface2: "#25233a",
    border: "#2a2a3a",
    borderStrong: "#3d3d52",
    borderBrand: "rgba(184, 158, 255, 0.35)",

    text: "#e8e6f0",
    textMuted: "#7a7690",
    textSubtle: "#5a5870",

    brand: "#b89eff",
    brandBg: "rgba(184, 158, 255, 0.12)",
    brandHover: "#d4c2ff",

    calloutBackdrop: "rgba(10, 10, 18, 0.86)",
  },

  /** 4px base spacing scale. */
  s: {
    1: 4,
    2: 8,
    3: 12,
    4: 16,
    5: 24,
    6: 32,
    7: 48,
    8: 64,
  },

  r: {
    sm: 4,
    md: 8,
    lg: 12,
    xl: 20,
  },

  font: {
    mono: `${monoFamily}, "SF Mono", Menlo, Consolas, monospace`,
    body: `${interFamily}, -apple-system, "Segoe UI", Roboto, sans-serif`,
    display: `${frauncesFamily}, Georgia, serif`,
  },

  /** Type scale (1.333 modular, snapped to whole pixels). */
  type: {
    eyebrow: 14,
    body: 18,
    bodyLg: 24,
    subtitle: 32,
    h2: 48,
    display: 96,
    displayXl: 128,
  },

  /** Bezier control points for use with Remotion's `Easing.bezier(...)`. */
  ease: {
    out: [0.22, 1, 0.36, 1] as const,
    inOut: [0.65, 0, 0.35, 1] as const,
  },

  /** Shared spring config so every entrance breathes at the same rate. */
  spring: {
    damping: 18,
    stiffness: 130,
    mass: 0.65,
  },
} as const;
