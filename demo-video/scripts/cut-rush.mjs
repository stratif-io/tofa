#!/usr/bin/env node
// Pre-cut raw rushes into per-scene clips with cuts, speed changes, and
// optional cropping baked in via ffmpeg. The compositions in
// src/compositions/*.tsx consume the resulting clips and only concern
// themselves with composition timing (titles, callouts, zoom, pan).
//
// Re-run with `npm run cut-rush` after editing the EDIT spec below.

import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const PUBLIC = resolve(__dirname, "..", "public");

// =============================================================================
// EDIT — editorial decisions live here. Each entry becomes one output clip.
//
// Times are in **source seconds** (positions in the source rush). The script
// concatenates segments at the requested speeds; speed > 1 fast-forwards,
// speed < 1 slow-mo's, speed defaults to 1.
//
// Optional `crop` { x, y, w, h } applies once to the source before segmenting.
// =============================================================================

const EDIT = {
  scan: {
    source: "scan-cam.mov",
    output: "scan.mov",
    segments: [
      { in: 1.0, out: 9.28 },
      { in: 10.58, out: 27.3 },
    ],
  },
  cam: {
    source: "scan-cam.mov",
    output: "cam.mov",
    segments: [
      { in: 32.0, out: 44.267 },
      { in: 50.267, out: 54.28 },
      { in: 58.73, out: 61.0 },
    ],
  },
  macApp: {
    source: "mac-app-raw.mov",
    output: "mac-app.mov",
    // Source is a wide screen recording (3448×1284). Take a 1728×1030 window
    // anchored to the right side so the popover sits at ~60% horizontal with
    // macOS menu-bar context around it. Height stops just past the Create
    // vault button so the yellow desktop sticky stays out — the slight
    // aspect mismatch with 1280×800 letterboxes onto the ink-900 background.
    segments: [
      { in: 0.0, out: 11.14},
      { in: 14.94, out: 22.0, speed: 5},
      { in: 22.00, out: 50.86 },
      { in: 55.76, out: 59.6 },
      { in: 62, out: 1000 },
    ],
  },
};

// =============================================================================
// Implementation
// =============================================================================

const ENCODE_ARGS = [
  "-c:v", "libx264",
  "-preset", "slow",
  "-crf", "18",
  "-pix_fmt", "yuv420p",
  "-r", "30",
  "-vsync", "cfr",
  "-an",
];

// Source rushes may be variable-framerate. `trim` works on input PTS and
// produces erratic output durations on VFR sources, so we normalise to a
// constant 30 fps first and split into N branches before trimming.
const OUTPUT_FPS = 30;

function buildFilterGraph(segments, crop) {
  const splitLabels = segments.map((_, i) => `[v${i}]`).join("");
  const prefix = crop
    ? `[0:v]crop=${crop.w}:${crop.h}:${crop.x}:${crop.y},fps=${OUTPUT_FPS}`
    : `[0:v]fps=${OUTPUT_FPS}`;
  const lines = [
    `${prefix},split=${segments.length}${splitLabels}`,
  ];
  const concatLabels = [];
  segments.forEach((seg, i) => {
    const label = `s${i}`;
    concatLabels.push(`[${label}]`);
    const setpts = seg.speed && seg.speed !== 1
      ? `setpts=(PTS-STARTPTS)/${seg.speed}`
      : `setpts=PTS-STARTPTS`;
    lines.push(`[v${i}]trim=start=${seg.in}:end=${seg.out},${setpts}[${label}]`);
  });
  lines.push(`${concatLabels.join("")}concat=n=${segments.length}:v=1[out]`);
  return lines.join(";");
}

function clipDuration(segments) {
  return segments.reduce((acc, s) => acc + (s.out - s.in) / (s.speed ?? 1), 0);
}

function cut(name, spec) {
  const inputPath = resolve(PUBLIC, spec.source);
  const outputPath = resolve(PUBLIC, spec.output);
  if (!existsSync(inputPath)) {
    throw new Error(`Source rush not found: ${inputPath}`);
  }
  mkdirSync(dirname(outputPath), { recursive: true });

  const filter = buildFilterGraph(spec.segments, spec.crop);
  const args = [
    "-y",
    "-i", inputPath,
    "-filter_complex", filter,
    "-map", "[out]",
    ...ENCODE_ARGS,
    outputPath,
  ];

  console.log(`\nCutting ${name} → ${spec.output}`);
  console.log(`  source:   ${spec.source}`);
  if (spec.crop) {
    console.log(`  crop:     ${spec.crop.w}×${spec.crop.h} at (${spec.crop.x},${spec.crop.y})`);
  }
  console.log(`  segments: ${spec.segments.length}, expected duration: ${clipDuration(spec.segments).toFixed(2)}s`);
  execFileSync("ffmpeg", args, { stdio: ["ignore", "ignore", "inherit"] });

  const probe = execFileSync(
    "ffprobe",
    ["-v", "error", "-show_entries", "format=duration", "-of", "default=noprint_wrappers=1:nokey=1", outputPath],
    { encoding: "utf8" },
  ).trim();
  console.log(`  actual duration:    ${parseFloat(probe).toFixed(2)}s`);
}

const only = process.argv[2];
const entries = only
  ? Object.entries(EDIT).filter(([name]) => name === only)
  : Object.entries(EDIT);

if (entries.length === 0) {
  console.error(`No EDIT entry matches "${only}". Available: ${Object.keys(EDIT).join(", ")}`);
  process.exit(1);
}

for (const [name, spec] of entries) {
  cut(name, spec);
}

console.log("\nDone. Re-run `npm run cut-rush [name]` after editing the EDIT spec.");
