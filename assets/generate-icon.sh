#!/usr/bin/env bash
# Generates assets/icon.png — requires ImageMagick (brew install imagemagick)
set -euo pipefail
DIR="$(cd "$(dirname "$0")" && pwd)"
magick -size 1024x1024 xc:'#0d1117' \
  -font Helvetica-Bold -pointsize 240 \
  -fill '#58a6ff' -gravity Center \
  -annotate 0 'tofa' \
  "$DIR/icon.png"
echo "Generated $DIR/icon.png"
