#!/usr/bin/env node
// Pre-cut the raw rush into per-scene clips with cuts and speed changes
// baked in via ffmpeg. Tour.tsx then consumes the resulting clips and only
// concerns itself with composition timing (titles, callouts, zoom, pan).
//
// Re-run with `npm run cut-rush` after editing the EDIT spec below.

import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const PUBLIC = resolve(__dirname, "..", "public");
const SOURCE = "scan-cam.mov";

// =============================================================================
// EDIT — editorial decisions live here. Each scene becomes one output clip.
// Times are in **source seconds** (positions in scan-cam.mov). The script
// concatenates segments at the requested speeds; speed > 1 fast-forwards,
// speed < 1 slow-mo's, speed defaults to 1.
// =============================================================================

const EDIT = {
  scan: {
    output: "scan.mov",
    segments: [
      { in: 1.0, out: 9.28 }, // pre-passphrase typing
      { in: 10.58, out: 27.3}, // pre-passphrase typing
    ],
  },
  cam: {
    output: "cam.mov",
    segments: [
      { in: 32.0, out: 44.267 },
      { in: 50.267, out: 54.28 },
      { in: 58.73, out: 61.0 },

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

// The raw rush is variable-framerate (~11 fps avg). `trim` works on input PTS
// and produces erratic output durations on VFR sources, so we normalise to a
// constant 30 fps first and split into N branches before trimming.
const OUTPUT_FPS = 30;

function buildFilterGraph(segments) {
  const splitLabels = segments.map((_, i) => `[v${i}]`).join("");
  const lines = [
    `[0:v]fps=${OUTPUT_FPS},split=${segments.length}${splitLabels}`,
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
  const inputPath = resolve(PUBLIC, SOURCE);
  const outputPath = resolve(PUBLIC, spec.output);
  if (!existsSync(inputPath)) {
    throw new Error(`Source rush not found: ${inputPath}`);
  }
  mkdirSync(dirname(outputPath), { recursive: true });

  const filter = buildFilterGraph(spec.segments);
  const args = [
    "-y",
    "-i", inputPath,
    "-filter_complex", filter,
    "-map", "[out]",
    ...ENCODE_ARGS,
    outputPath,
  ];

  console.log(`\nCutting ${name} → ${spec.output}`);
  console.log(`  segments: ${spec.segments.length}, expected duration: ${clipDuration(spec.segments).toFixed(2)}s`);
  execFileSync("ffmpeg", args, { stdio: ["ignore", "ignore", "inherit"] });

  const probe = execFileSync(
    "ffprobe",
    ["-v", "error", "-show_entries", "format=duration", "-of", "default=noprint_wrappers=1:nokey=1", outputPath],
    { encoding: "utf8" },
  ).trim();
  console.log(`  actual duration:    ${parseFloat(probe).toFixed(2)}s`);
}

for (const [name, spec] of Object.entries(EDIT)) {
  cut(name, spec);
}

console.log("\nDone. Re-run `npm run cut-rush` after editing the EDIT spec.");
