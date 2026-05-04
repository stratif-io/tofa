#!/usr/bin/env bash
set -euo pipefail

SVG="$(dirname "$0")/../docs/design/assets/svg/tofa-wink-dark.svg"
ICONS_DIR="$(dirname "$0")/../tofa-app/src-tauri/icons"
ICONSET="$ICONS_DIR/AppIcon.iconset"

echo "Generating icons from $SVG"
mkdir -p "$ICONSET"

# PNG sizes needed for Tauri + macOS iconset
SIZES=(16 32 64 128 256 512 1024)
for SIZE in "${SIZES[@]}"; do
  echo "  → ${SIZE}x${SIZE}"
  rsvg-convert -w "$SIZE" -h "$SIZE" "$SVG" -o "$ICONS_DIR/${SIZE}x${SIZE}.png"
done

# 128x128@2x = 256px
cp "$ICONS_DIR/256x256.png" "$ICONS_DIR/128x128@2x.png"

# Tauri bundle icon
cp "$ICONS_DIR/512x512.png" "$ICONS_DIR/icon.png"

# macOS .icns via iconset
cp "$ICONS_DIR/16x16.png"     "$ICONSET/icon_16x16.png"
cp "$ICONS_DIR/32x32.png"     "$ICONSET/icon_16x16@2x.png"
cp "$ICONS_DIR/32x32.png"     "$ICONSET/icon_32x32.png"
cp "$ICONS_DIR/64x64.png"     "$ICONSET/icon_32x32@2x.png"
cp "$ICONS_DIR/128x128.png"   "$ICONSET/icon_128x128.png"
cp "$ICONS_DIR/256x256.png"   "$ICONSET/icon_128x128@2x.png"
cp "$ICONS_DIR/256x256.png"   "$ICONSET/icon_256x256.png"
cp "$ICONS_DIR/512x512.png"   "$ICONSET/icon_256x256@2x.png"
cp "$ICONS_DIR/512x512.png"   "$ICONSET/icon_512x512.png"
cp "$ICONS_DIR/1024x1024.png" "$ICONSET/icon_512x512@2x.png"

iconutil -c icns "$ICONSET" -o "$ICONS_DIR/icon.icns"
echo "  → icon.icns generated"
rm -rf "$ICONSET"

# Tray icon
rsvg-convert -w 22 -h 22 "$SVG" -o "$ICONS_DIR/tray_icon.png"
rsvg-convert -w 44 -h 44 "$SVG" -o "$ICONS_DIR/tray_icon@2x.png"

echo "Done. Icons written to $ICONS_DIR"
