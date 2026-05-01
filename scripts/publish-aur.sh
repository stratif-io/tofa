#!/usr/bin/env bash
# Usage: bash scripts/publish-aur.sh <version>
# Example: bash scripts/publish-aur.sh 1.2.3
# Requires: git, makepkg (Arch Linux), updpkgsums, SSH key for AUR (~/.ssh/config with Host aur.archlinux.org)
set -euo pipefail

VERSION="${1:?Usage: $0 <version>}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

publish_pkg() {
  local pkgname="$1"
  local src_pkgbuild="$REPO_ROOT/packaging/aur/$pkgname/PKGBUILD"

  echo "==> Publishing $pkgname v$VERSION"

  # Clone the AUR repo (creates empty repo if package is new)
  local aur_dir="$TMP_DIR/$pkgname"
  git clone "ssh://aur@aur.archlinux.org/$pkgname.git" "$aur_dir"

  # Copy PKGBUILD and set version
  cp "$src_pkgbuild" "$aur_dir/PKGBUILD"
  sed -i "s/^pkgver=.*/pkgver=$VERSION/" "$aur_dir/PKGBUILD"
  sed -i "s/^pkgrel=.*/pkgrel=1/" "$aur_dir/PKGBUILD"

  # Fill real checksums (downloads the source archives)
  cd "$aur_dir"
  updpkgsums

  # Regenerate .SRCINFO
  makepkg --printsrcinfo > .SRCINFO

  # Sync updated PKGBUILD back to the main repo for review
  cp "$aur_dir/PKGBUILD" "$src_pkgbuild"

  # Commit and push to AUR
  git config user.name "Carlo Abi Chahine"
  git config user.email "carlo.abichahine@gmail.com"
  git add PKGBUILD .SRCINFO
  git commit -m "Update to v$VERSION"
  git push

  echo "==> $pkgname published successfully"
}

publish_pkg tofa-bin
publish_pkg tofa

echo ""
echo "Done! Both AUR packages updated to v$VERSION."
echo "  https://aur.archlinux.org/packages/tofa-bin"
echo "  https://aur.archlinux.org/packages/tofa"
