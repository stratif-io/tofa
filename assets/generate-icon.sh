#!/usr/bin/env bash
# Generates assets/icon.png — requires ImageMagick (brew install imagemagick)
set -euo pipefail
DIR="$(cd "$(dirname "$0")" && pwd)"

# Prefer an absolute font path that works on macOS (local + GitHub Actions runners).
# Fall back to a generic name if the file isn't found.
FONT="/System/Library/Fonts/Supplemental/Verdana Bold.ttf"
if [ ! -f "$FONT" ]; then
  FONT="Verdana-Bold"
fi

magick -size 1024x1024 xc:'#0d1117' \
  -font "$FONT" -pointsize 240 \
  -fill '#58a6ff' -gravity Center \
  -annotate 0 'tofa' \
  "$DIR/icon.png"
echo "Generated $DIR/icon.png"
