#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

command -v ffmpeg >/dev/null || { echo "ffmpeg not found — brew install ffmpeg"; exit 1; }

for f in public/demos/*.mp4; do
  out="${f%.mp4}-poster.png"
  echo "→ $out"
  # Frame at 0.5s, scaled to 1280px wide, lossless PNG
  ffmpeg -y -ss 0.5 -i "$f" -frames:v 1 -vf "scale=1280:-1:flags=lanczos" "$out" -loglevel error
done
echo "✓ posters extracted"
