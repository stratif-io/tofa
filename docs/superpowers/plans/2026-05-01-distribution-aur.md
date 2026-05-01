# Distribution — Plan C: AUR Packages

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Publish two AUR packages — `tofa-bin` (pre-built binary from GitHub Releases) and `tofa` (builds from source) — and provide a `scripts/publish-aur.sh` script to update them after each release.

**Architecture:** Two separate AUR packages live under `packaging/aur/` in the main repo. After each release, the maintainer runs `scripts/publish-aur.sh <version>` locally. The script clones the AUR package repo, replaces version/sha in `PKGBUILD`, regenerates `.SRCINFO`, commits, and pushes to AUR.

**Tech Stack:** AUR (SSH push to `aur.archlinux.org`), `makepkg`, `updpkgsums`, bash.

**Prerequisite:** Plan A completed (GitHub Release exists with binaries). An AUR account at `https://aur.archlinux.org`. SSH key uploaded to the AUR account.

---

### Task 1: `tofa-bin` PKGBUILD

**Files:**
- Create: `packaging/aur/tofa-bin/PKGBUILD` (in main `tofa` repo)

- [ ] **Step 1: Write the PKGBUILD**

```bash
mkdir -p packaging/aur/tofa-bin
cat > packaging/aur/tofa-bin/PKGBUILD << 'EOF'
# Maintainer: Carlo Abi Chahine <carlo.abichahine@gmail.com>
pkgname=tofa-bin
pkgver=0.1.0
pkgrel=1
pkgdesc="Eye-candy terminal OTP manager (pre-built binary)"
arch=('x86_64' 'aarch64')
url="https://github.com/cabichahine/tofa"
license=('MIT')
provides=('tofa')
conflicts=('tofa')

source_x86_64=("$url/releases/download/v$pkgver/tofa-$pkgver-x86_64-unknown-linux-musl.tar.gz")
source_aarch64=("$url/releases/download/v$pkgver/tofa-$pkgver-aarch64-unknown-linux-musl.tar.gz")
sha256sums_x86_64=('SKIP')
sha256sums_aarch64=('SKIP')

package() {
  install -Dm755 tofa "$pkgdir/usr/bin/tofa"
  "$pkgdir/usr/bin/tofa" completions bash | install -Dm644 /dev/stdin \
    "$pkgdir/usr/share/bash-completion/completions/tofa"
  "$pkgdir/usr/bin/tofa" completions zsh | install -Dm644 /dev/stdin \
    "$pkgdir/usr/share/zsh/site-functions/_tofa"
  "$pkgdir/usr/bin/tofa" completions fish | install -Dm644 /dev/stdin \
    "$pkgdir/usr/share/fish/vendor_completions.d/tofa.fish"
}
EOF
```

Note: `sha256sums` start as `'SKIP'` and will be filled by `updpkgsums` in the publish script.

- [ ] **Step 2: Commit**

```bash
git add packaging/aur/tofa-bin/PKGBUILD
git commit -m "packaging: add tofa-bin AUR PKGBUILD"
```

---

### Task 2: `tofa` (source) PKGBUILD

**Files:**
- Create: `packaging/aur/tofa/PKGBUILD`

- [ ] **Step 1: Write the PKGBUILD**

```bash
mkdir -p packaging/aur/tofa
cat > packaging/aur/tofa/PKGBUILD << 'EOF'
# Maintainer: Carlo Abi Chahine <carlo.abichahine@gmail.com>
pkgname=tofa
pkgver=0.1.0
pkgrel=1
pkgdesc="Eye-candy terminal OTP manager"
arch=('x86_64' 'aarch64')
url="https://github.com/cabichahine/tofa"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
provides=('tofa')
conflicts=('tofa-bin')

source=("$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "tofa-$pkgver"
  cargo build --release -p tofa
}

package() {
  cd "tofa-$pkgver"
  install -Dm755 "target/release/tofa" "$pkgdir/usr/bin/tofa"
  "$pkgdir/usr/bin/tofa" completions bash | install -Dm644 /dev/stdin \
    "$pkgdir/usr/share/bash-completion/completions/tofa"
  "$pkgdir/usr/bin/tofa" completions zsh | install -Dm644 /dev/stdin \
    "$pkgdir/usr/share/zsh/site-functions/_tofa"
  "$pkgdir/usr/bin/tofa" completions fish | install -Dm644 /dev/stdin \
    "$pkgdir/usr/share/fish/vendor_completions.d/tofa.fish"
}
EOF
```

- [ ] **Step 2: Commit**

```bash
git add packaging/aur/tofa/PKGBUILD
git commit -m "packaging: add tofa (from source) AUR PKGBUILD"
```

---

### Task 3: Publish script

**Files:**
- Create: `scripts/publish-aur.sh`

- [ ] **Step 1: Write the script**

```bash
mkdir -p scripts
cat > scripts/publish-aur.sh << 'EOF'
#!/usr/bin/env bash
# Usage: bash scripts/publish-aur.sh <version>
# Example: bash scripts/publish-aur.sh 1.2.3
# Requires: git, makepkg (Arch Linux or docker), SSH key for AUR
set -euo pipefail

VERSION="${1:?Usage: $0 <version>}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

publish_pkg() {
  local pkgname="$1"
  local src_pkgbuild="$REPO_ROOT/packaging/aur/$pkgname/PKGBUILD"

  echo "==> Publishing $pkgname v$VERSION"

  # Clone or update AUR repo
  local aur_dir="$TMP_DIR/$pkgname"
  git clone "ssh://aur@aur.archlinux.org/$pkgname.git" "$aur_dir"

  # Copy PKGBUILD and bump version
  cp "$src_pkgbuild" "$aur_dir/PKGBUILD"
  sed -i "s/^pkgver=.*/pkgver=$VERSION/" "$aur_dir/PKGBUILD"
  sed -i "s/^pkgrel=.*/pkgrel=1/" "$aur_dir/PKGBUILD"

  # Fill real checksums (requires internet access)
  cd "$aur_dir"
  updpkgsums

  # Regenerate .SRCINFO
  makepkg --printsrcinfo > .SRCINFO

  # Sync back to repo for review
  cp "$aur_dir/PKGBUILD" "$src_pkgbuild"

  # Commit and push to AUR
  git config user.name "Carlo Abi Chahine"
  git config user.email "carlo.abichahine@gmail.com"
  git add PKGBUILD .SRCINFO
  git commit -m "Update to v$VERSION"
  git push

  echo "==> $pkgname published"
}

publish_pkg tofa-bin
publish_pkg tofa

echo ""
echo "Done! Both AUR packages updated to v$VERSION."
echo "Check: https://aur.archlinux.org/packages/tofa-bin"
echo "Check: https://aur.archlinux.org/packages/tofa"
EOF
chmod +x scripts/publish-aur.sh
```

- [ ] **Step 2: Commit**

```bash
git add scripts/publish-aur.sh
git commit -m "scripts: add publish-aur.sh for AUR package updates"
```

---

### Task 4: Register packages on AUR

This is a one-time manual step performed on an Arch Linux machine (or in an Arch Docker container).

- [ ] **Step 1: Create SSH key for AUR (if not already done)**

```bash
ssh-keygen -t ed25519 -C "aur-tofa" -f ~/.ssh/aur_tofa
# Add the public key to your AUR account:
# https://aur.archlinux.org/account/<username>/edit → SSH Public Key
```

Add to `~/.ssh/config`:

```
Host aur.archlinux.org
    IdentityFile ~/.ssh/aur_tofa
    User aur
```

- [ ] **Step 2: Register `tofa-bin`**

```bash
cd /tmp
git clone ssh://aur@aur.archlinux.org/tofa-bin.git
# This creates an empty repo (new package)
cp ~/my_work/tofa/packaging/aur/tofa-bin/PKGBUILD tofa-bin/
cd tofa-bin
# Fill real checksums for version 0.1.0
sed -i "s/^pkgver=.*/pkgver=0.1.0/" PKGBUILD
updpkgsums
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Initial release v0.1.0"
git push
```

- [ ] **Step 3: Register `tofa`**

```bash
cd /tmp
git clone ssh://aur@aur.archlinux.org/tofa.git
cp ~/my_work/tofa/packaging/aur/tofa/PKGBUILD tofa/
cd tofa
sed -i "s/^pkgver=.*/pkgver=0.1.0/" PKGBUILD
updpkgsums
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Initial release v0.1.0"
git push
```

- [ ] **Step 4: Verify**

```bash
# On an Arch Linux machine or in docker:
yay -S tofa-bin   # or: paru -S tofa-bin
tofa --version
# Expected: tofa 0.1.0
```

---

### Task 5: Test the publish script end-to-end

After v0.2.0 is released on GitHub:

- [ ] **Step 1: Run the script**

```bash
bash scripts/publish-aur.sh 0.2.0
```

Expected output:
```
==> Publishing tofa-bin v0.2.0
...
==> tofa-bin published
==> Publishing tofa v0.2.0
...
==> tofa published

Done! Both AUR packages updated to v0.2.0.
```

- [ ] **Step 2: Verify on AUR web**

Visit `https://aur.archlinux.org/packages/tofa-bin` — version should show `0.2.0`.
