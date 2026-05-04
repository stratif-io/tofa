#!/usr/bin/env sh
# Verifies the generator runs and produces a valid 1320x800 PNG.
set -eu
DIR="$(cd "$(dirname "$0")" && pwd)"
"$DIR/generate.sh"
test -f "$DIR/dmg-background.png" || { echo "FAIL: PNG not produced"; exit 1; }
DIMS="$(sips -g pixelWidth -g pixelHeight "$DIR/dmg-background.png" | awk '/pixel(Width|Height)/ {print $2}' | paste -sd 'x' -)"
test "$DIMS" = "1320x800" || { echo "FAIL: expected 1320x800, got $DIMS"; exit 1; }
echo "PASS: dmg-background.png is 1320x800"
