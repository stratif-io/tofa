# Distribution — Plan A: CI/CD & Release Pipeline

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Wire up a fully automated GitHub Actions pipeline that builds 4 native binaries + a macOS DMG on every `git tag v*`, publishes a GitHub Release, and dispatches a tap-update event.

**Architecture:** Two workflows — `ci.yml` (lint/test on every push/PR) and `release.yml` (build matrix → DMG → GitHub Release → repository_dispatch). A macOS `.app` bundle is assembled from a universal binary (`lipo`) and ad-hoc signed. The DMG is created with `create-dmg`.

**Tech Stack:** GitHub Actions, Rust/cargo, cross (Linux musl), lipo, sips/iconutil, create-dmg, softprops/action-gh-release.

---

### Task 1: CI workflow

**Files:**
- Modify: `.github/workflows/ci.yml`

- [ ] **Step 1: Replace the existing stub**

The current `ci.yml` has Windows in the matrix (we don't ship Windows) and doesn't run fmt/clippy before tests. Replace it entirely:

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  check:
    name: Check (${{ matrix.os }})
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: fmt
        run: cargo fmt --check
      - name: clippy
        run: cargo clippy --workspace -- -D warnings
      - name: test
        run: cargo test --workspace
```

- [ ] **Step 2: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: replace stub with fmt/clippy/test on ubuntu+macos"
```

---

### Task 2: Source icon

**Files:**
- Create: `assets/icon.png`
- Create: `assets/generate-icon.sh`

The icon must be 1024×1024 PNG. We generate it programmatically using ImageMagick (available on macOS and the macOS GitHub runner).

- [ ] **Step 1: Write the generation script**

```bash
cat > assets/generate-icon.sh << 'EOF'
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
EOF
chmod +x assets/generate-icon.sh
```

- [ ] **Step 2: Generate the icon locally and commit it**

```bash
bash assets/generate-icon.sh
git add assets/icon.png assets/generate-icon.sh
git commit -m "assets: add source icon 1024x1024 and generation script"
```

---

### Task 3: macOS launcher and Info.plist template

**Files:**
- Create: `packaging/macos/tofa-launcher`
- Create: `packaging/macos/Info.plist.template`

- [ ] **Step 1: Create launcher script**

```bash
mkdir -p packaging/macos
cat > packaging/macos/tofa-launcher << 'EOF'
#!/bin/bash
DIR="$(cd "$(dirname "$0")" && pwd)"
osascript - "$DIR/tofa" <<'APPLESCRIPT'
on run argv
  tell application "Terminal"
    activate
    do script (item 1 of argv)
  end tell
end run
APPLESCRIPT
EOF
chmod +x packaging/macos/tofa-launcher
```

- [ ] **Step 2: Create Info.plist template**

The placeholders `%%VERSION%%` are replaced by the release workflow using `sed`.

```bash
cat > packaging/macos/Info.plist.template << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleIdentifier</key>
    <string>com.cabichahine.tofa</string>
    <key>CFBundleName</key>
    <string>tofa</string>
    <key>CFBundleDisplayName</key>
    <string>tofa</string>
    <key>CFBundleExecutable</key>
    <string>tofa-launcher</string>
    <key>CFBundleIconFile</key>
    <string>tofa</string>
    <key>CFBundleVersion</key>
    <string>%%VERSION%%</string>
    <key>CFBundleShortVersionString</key>
    <string>%%VERSION%%</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>LSUIElement</key>
    <false/>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSMinimumSystemVersion</key>
    <string>12.0</string>
</dict>
</plist>
EOF
```

- [ ] **Step 3: Commit**

```bash
git add packaging/macos/
git commit -m "packaging: add macOS launcher and Info.plist template"
```

---

### Task 4: Release workflow — build matrix

**Files:**
- Modify: `.github/workflows/release.yml`

This step writes the full workflow in one shot. We'll build it up across Tasks 4–6 but commit once at the end (Task 6).

- [ ] **Step 1: Write the build-matrix job**

Replace `.github/workflows/release.yml` with:

```yaml
name: Release

on:
  push:
    tags:
      - 'v[0-9]*'

permissions:
  contents: write

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    name: Build (${{ matrix.target }})
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: macos-latest
            target: x86_64-apple-darwin
            use_cross: false
          - os: macos-latest
            target: aarch64-apple-darwin
            use_cross: false
          - os: ubuntu-latest
            target: x86_64-unknown-linux-musl
            use_cross: true
          - os: ubuntu-latest
            target: aarch64-unknown-linux-musl
            use_cross: true
    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - uses: Swatinem/rust-cache@v2
        with:
          key: ${{ matrix.target }}

      - name: Install cross
        if: matrix.use_cross
        run: cargo install cross --locked

      - name: Build
        run: |
          VERSION="${GITHUB_REF_NAME#v}"
          if [ "${{ matrix.use_cross }}" = "true" ]; then
            cross build --release --target ${{ matrix.target }} -p tofa
          else
            cargo build --release --target ${{ matrix.target }} -p tofa
          fi
          strip target/${{ matrix.target }}/release/tofa || true

      - name: Package
        run: |
          VERSION="${GITHUB_REF_NAME#v}"
          ARCHIVE="tofa-${VERSION}-${{ matrix.target }}.tar.gz"
          cp target/${{ matrix.target }}/release/tofa tofa
          tar czf "$ARCHIVE" tofa
          echo "ARCHIVE=$ARCHIVE" >> "$GITHUB_ENV"

      - uses: actions/upload-artifact@v4
        with:
          name: tofa-${{ matrix.target }}
          path: ${{ env.ARCHIVE }}
```

(Do not commit yet — continue in Task 5.)

---

### Task 5: Release workflow — DMG job

**Files:**
- Modify: `.github/workflows/release.yml` (append)

- [ ] **Step 1: Append the DMG job after the build job**

Append to `release.yml`:

```yaml

  dmg:
    name: Build macOS DMG
    needs: build
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4

      - name: Download macOS binaries
        uses: actions/download-artifact@v4
        with:
          pattern: tofa-*-apple-darwin
          merge-multiple: true
          path: mac-bins

      - name: Extract binaries
        run: |
          VERSION="${GITHUB_REF_NAME#v}"
          tar xf "mac-bins/tofa-${VERSION}-x86_64-apple-darwin.tar.gz" -C mac-bins --transform 's/tofa/tofa-x86_64/'
          tar xf "mac-bins/tofa-${VERSION}-aarch64-apple-darwin.tar.gz" -C mac-bins --transform 's/tofa/tofa-aarch64/'

      - name: Create universal binary
        run: |
          lipo -create mac-bins/tofa-x86_64 mac-bins/tofa-aarch64 -output tofa-universal

      - name: Generate icon
        run: |
          brew install imagemagick 2>/dev/null || true
          bash assets/generate-icon.sh
          mkdir -p tofa.iconset
          for SIZE in 16 32 64 128 256 512 1024; do
            sips -z $SIZE $SIZE assets/icon.png --out "tofa.iconset/icon_${SIZE}x${SIZE}.png"
            [ $SIZE -le 512 ] && sips -z $((SIZE*2)) $((SIZE*2)) assets/icon.png --out "tofa.iconset/icon_${SIZE}x${SIZE}@2x.png"
          done
          iconutil -c icns tofa.iconset -o tofa.icns

      - name: Assemble .app bundle
        run: |
          VERSION="${GITHUB_REF_NAME#v}"
          APP="tofa.app/Contents"
          mkdir -p "$APP/MacOS" "$APP/Resources"
          cp tofa-universal "$APP/MacOS/tofa"
          cp packaging/macos/tofa-launcher "$APP/MacOS/tofa-launcher"
          chmod +x "$APP/MacOS/tofa-launcher"
          cp tofa.icns "$APP/Resources/tofa.icns"
          sed "s/%%VERSION%%/$VERSION/g" packaging/macos/Info.plist.template > "$APP/Info.plist"
          codesign --sign - --force --deep tofa.app

      - name: Create DMG
        run: |
          VERSION="${GITHUB_REF_NAME#v}"
          brew install create-dmg 2>/dev/null || true
          create-dmg \
            --volname "tofa" \
            --background-color "#0d1117" \
            --window-size 500 300 \
            --icon-size 128 \
            --icon "tofa.app" 125 150 \
            --app-drop-link 375 150 \
            "tofa-${VERSION}.dmg" \
            "tofa.app"
          echo "DMG_FILE=tofa-${VERSION}.dmg" >> "$GITHUB_ENV"

      - uses: actions/upload-artifact@v4
        with:
          name: tofa-dmg
          path: ${{ env.DMG_FILE }}
```

---

### Task 6: Release workflow — GitHub Release + tap dispatch

**Files:**
- Modify: `.github/workflows/release.yml` (append)

- [ ] **Step 1: Append the release job**

```yaml

  release:
    name: Publish GitHub Release
    needs: [build, dmg]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/download-artifact@v4
        with:
          path: artifacts
          merge-multiple: true

      - name: Compute SHA256SUMS
        run: |
          cd artifacts
          sha256sum *.tar.gz *.dmg > SHA256SUMS
          cat SHA256SUMS

      - name: Publish release
        uses: softprops/action-gh-release@v2
        with:
          files: |
            artifacts/*.tar.gz
            artifacts/*.dmg
            artifacts/SHA256SUMS
          generate_release_notes: true

      - name: Dispatch tap update
        run: |
          VERSION="${GITHUB_REF_NAME#v}"
          SHA_MAC_ARM=$(grep "aarch64-apple-darwin" artifacts/SHA256SUMS | awk '{print $1}')
          SHA_MAC_X86=$(grep "x86_64-apple-darwin" artifacts/SHA256SUMS | awk '{print $1}')
          SHA_LINUX_ARM=$(grep "aarch64-unknown-linux-musl" artifacts/SHA256SUMS | awk '{print $1}')
          SHA_LINUX_X86=$(grep "x86_64-unknown-linux-musl" artifacts/SHA256SUMS | awk '{print $1}')
          SHA_DMG=$(grep "\.dmg" artifacts/SHA256SUMS | awk '{print $1}')
          curl -s -X POST \
            -H "Authorization: Bearer ${{ secrets.TAP_DISPATCH_TOKEN }}" \
            -H "Accept: application/vnd.github+json" \
            https://api.github.com/repos/cabichahine/homebrew-tofa/dispatches \
            -d "{
              \"event_type\": \"new-release\",
              \"client_payload\": {
                \"version\": \"$VERSION\",
                \"sha_mac_arm\": \"$SHA_MAC_ARM\",
                \"sha_mac_x86\": \"$SHA_MAC_X86\",
                \"sha_linux_arm\": \"$SHA_LINUX_ARM\",
                \"sha_linux_x86\": \"$SHA_LINUX_X86\",
                \"sha_dmg\": \"$SHA_DMG\"
              }
            }"
```

- [ ] **Step 2: Commit the full release workflow**

```bash
git add .github/workflows/release.yml packaging/macos/
git commit -m "ci: complete release pipeline — 4-target build, DMG, GitHub Release, tap dispatch"
```

---

### Task 7: GitHub secret

**Files:** None — GitHub repository setting.

- [ ] **Step 1: Create a PAT and add it as a secret**

1. Go to `https://github.com/settings/tokens` → "Generate new token (classic)"
2. Scopes needed: `repo` (for the `cabichahine/homebrew-tofa` repo dispatch)
3. Token name: `TAP_DISPATCH_TOKEN`
4. Copy the token value
5. Go to `https://github.com/cabichahine/tofa/settings/secrets/actions`
6. "New repository secret" → name `TAP_DISPATCH_TOKEN`, paste value

- [ ] **Step 2: Verify (dry run)**

Push any branch commit (not a tag) and confirm `ci.yml` runs green on both `ubuntu-latest` and `macos-latest`. The release workflow will only run on a tag.

---

### Task 8: Smoke-test the pipeline

- [ ] **Step 1: Bump version and tag**

```bash
# In tofa/Cargo.toml and tofa-core/Cargo.toml, set version = "0.1.0" (or bump if needed)
git add tofa/Cargo.toml tofa-core/Cargo.toml Cargo.lock
git commit -m "chore: bump version to 0.1.0"
git tag v0.1.0
git push origin main --tags
```

- [ ] **Step 2: Watch the Actions run**

Go to `https://github.com/cabichahine/tofa/actions` and verify:
- 4 build jobs complete (check artifacts)
- DMG job produces `tofa-0.1.0.dmg`
- Release job publishes the GitHub Release with all files + `SHA256SUMS`
- Tap dispatch HTTP call returns 204 (check step logs)

- [ ] **Step 3: Verify the release**

```bash
gh release view v0.1.0 --repo cabichahine/tofa
```

Expected output includes `tofa-0.1.0-x86_64-apple-darwin.tar.gz`, `tofa-0.1.0-aarch64-apple-darwin.tar.gz`, `tofa-0.1.0-x86_64-unknown-linux-musl.tar.gz`, `tofa-0.1.0-aarch64-unknown-linux-musl.tar.gz`, `tofa-0.1.0.dmg`, `SHA256SUMS`.
