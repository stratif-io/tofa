#!/bin/sh
set -e

REPO="stratif-io/tofa"
BIN_NAME="tofa"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# ── Detect OS and arch ────────────────────────────────────────────────────────
detect_target() {
  OS="$(uname -s)"
  ARCH="$(uname -m)"

  case "$OS" in
    Darwin)
      case "$ARCH" in
        arm64)   echo "aarch64-apple-darwin" ;;
        x86_64)  echo "x86_64-apple-darwin" ;;
        *)       echo "Unsupported macOS arch: $ARCH" >&2; exit 1 ;;
      esac
      ;;
    Linux)
      case "$ARCH" in
        x86_64)          echo "x86_64-unknown-linux-musl" ;;
        aarch64|arm64)   echo "aarch64-unknown-linux-musl" ;;
        *)               echo "Unsupported Linux arch: $ARCH" >&2; exit 1 ;;
      esac
      ;;
    *)
      echo "Unsupported OS: $OS" >&2
      exit 1
      ;;
  esac
}

# ── Resolve version ───────────────────────────────────────────────────────────
resolve_version() {
  if [ -n "${VERSION:-}" ]; then
    echo "$VERSION"
    return
  fi

  # Fetch latest release whose tag starts with "v" (excludes tofa-macos-* and tofa-core-*)
  LATEST=$(curl -fsSL "https://api.github.com/repos/$REPO/releases" \
    | grep '"tag_name"' \
    | grep '"v[0-9]' \
    | grep -v '"tofa-macos-' \
    | grep -v '"tofa-core-' \
    | head -1 \
    | sed 's/.*"tag_name": *"v\([^"]*\)".*/\1/')

  if [ -z "$LATEST" ]; then
    echo "Could not determine latest tofa version" >&2
    exit 1
  fi

  echo "$LATEST"
}

# ── Download helper (curl or wget) ────────────────────────────────────────────
download() {
  URL="$1"
  DEST="$2"
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$URL" -o "$DEST"
  elif command -v wget >/dev/null 2>&1; then
    wget -q "$URL" -O "$DEST"
  else
    echo "curl or wget is required" >&2
    exit 1
  fi
}

# ── Verify checksum ───────────────────────────────────────────────────────────
verify_checksum() {
  ARCHIVE="$1"
  SUMS_FILE="$2"

  BASENAME="$(basename "$ARCHIVE")"

  if command -v sha256sum >/dev/null 2>&1; then
    EXPECTED=$(grep "  $BASENAME$" "$SUMS_FILE" | awk '{print $1}')
    ACTUAL=$(sha256sum "$ARCHIVE" | awk '{print $1}')
  elif command -v shasum >/dev/null 2>&1; then
    EXPECTED=$(grep "  $BASENAME$" "$SUMS_FILE" | awk '{print $1}')
    ACTUAL=$(shasum -a 256 "$ARCHIVE" | awk '{print $1}')
  else
    echo "sha256sum or shasum is required" >&2
    exit 1
  fi

  if [ "$EXPECTED" != "$ACTUAL" ]; then
    echo "Checksum mismatch for $BASENAME" >&2
    echo "  expected: $EXPECTED" >&2
    echo "  actual:   $ACTUAL" >&2
    exit 1
  fi
}

# ── Main ──────────────────────────────────────────────────────────────────────
main() {
  TARGET="$(detect_target)"
  VERSION="$(resolve_version)"
  if [ -z "$VERSION" ]; then
    echo "Could not determine version to install" >&2
    exit 1
  fi
  TAG="v${VERSION}"
  ARCHIVE_NAME="${BIN_NAME}-${VERSION}-${TARGET}.tar.gz"
  BASE_URL="https://github.com/$REPO/releases/download/$TAG"

  TMPDIR="$(mktemp -d)"
  trap 'rm -rf "$TMPDIR"' EXIT

  echo "Installing tofa $VERSION for $TARGET..."

  download "$BASE_URL/$ARCHIVE_NAME"     "$TMPDIR/$ARCHIVE_NAME"
  download "$BASE_URL/SHA256SUMS"        "$TMPDIR/SHA256SUMS"

  verify_checksum "$TMPDIR/$ARCHIVE_NAME" "$TMPDIR/SHA256SUMS"

  tar -xzf "$TMPDIR/$ARCHIVE_NAME" -C "$TMPDIR"

  if ! mkdir -p "$INSTALL_DIR" 2>/dev/null; then
    echo "Cannot create install directory: $INSTALL_DIR" >&2
    exit 1
  fi
  if [ ! -w "$INSTALL_DIR" ]; then
    echo "Cannot write to $INSTALL_DIR" >&2
    exit 1
  fi

  BINARY=$(find "$TMPDIR" -name "$BIN_NAME" -type f | head -1)
  if [ -z "$BINARY" ]; then
    echo "Binary not found in archive" >&2
    exit 1
  fi
  mv "$BINARY" "$INSTALL_DIR/$BIN_NAME"
  chmod +x "$INSTALL_DIR/$BIN_NAME"

  echo "tofa $VERSION installed to $INSTALL_DIR/$BIN_NAME"

  # PATH hint
  case ":${PATH}:" in
    *":$INSTALL_DIR:"*) ;;
    *)
      echo ""
      echo "NOTE: $INSTALL_DIR is not in your PATH."
      echo "Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
      echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
      ;;
  esac
}

main
