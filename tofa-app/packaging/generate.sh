#!/usr/bin/env sh
# Regenerates dmg-background.png from dmg-background.svg.
# Requires: rsvg-convert (brew install librsvg).
set -eu
DIR="$(cd "$(dirname "$0")" && pwd)"
rsvg-convert \
  --width 1320 --height 800 \
  --format png \
  --output "$DIR/dmg-background.png" \
  "$DIR/dmg-background.svg"
echo "Wrote $DIR/dmg-background.png"
