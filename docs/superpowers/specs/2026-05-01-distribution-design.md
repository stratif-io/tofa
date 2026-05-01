# tofa Distribution Design

## Goal

Make `tofa` installable via `brew install tofa`, `brew install --cask tofa`, `cargo install tofa`, and `yay -S tofa-bin` (AUR), with a fully automated release pipeline triggered by a git tag.

## Architecture

All distribution channels are fed from a single GitHub Actions pipeline that fires on `git tag v*`:

```
git tag v1.0.0
       │
       ▼
GitHub Actions (release.yml)
       ├── 4 pre-built binaries (tar.gz)
       ├── tofa.dmg  (macOS .app bundle)
       └── SHA256SUMS
              │
       ┌──────┼──────────────┐
       ▼      ▼              ▼
 Homebrew  Homebrew      AUR
 formula   cask        PKGBUILD
(auto-commit) (auto-commit) (manual script)
```

**Repository:** `https://github.com/cabichahine/tofa`  
**Tap repository:** `https://github.com/cabichahine/homebrew-tofa`  
**Version source of truth:** `version` field in `tofa/Cargo.toml`. Git tag must match (`v1.0.0` ↔ `version = "1.0.0"`).

---

## CI/CD Workflows

### `ci.yml` — continuous integration

Triggers on every push to `main` and on pull requests.

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test --workspace`
- Build (debug) on `macos-latest` and `ubuntu-latest`

### `release.yml` — release pipeline

Triggers on `push` with tag matching `v[0-9]+.*`.

**Matrix build (parallel):**

| Target | Runner | Method |
|---|---|---|
| `x86_64-apple-darwin` | `macos-latest` | native |
| `aarch64-apple-darwin` | `macos-latest` | `--target aarch64-apple-darwin` |
| `x86_64-unknown-linux-musl` | `ubuntu-latest` | `cross` |
| `aarch64-unknown-linux-musl` | `ubuntu-latest` | `cross` |

Each binary is stripped (`strip`) and archived as `tofa-{version}-{target}.tar.gz` containing a single `tofa` binary.

**macOS DMG job** (runs after matrix, on `macos-latest`):
1. Download `x86_64-apple-darwin` and `aarch64-apple-darwin` binaries
2. Create universal binary with `lipo`
3. Assemble `tofa.app` bundle (see below)
4. Generate `tofa.icns` icon
5. Ad-hoc sign: `codesign --sign - --force tofa.app`
6. Create `tofa-{version}.dmg` with `hdiutil`

**Release job:**
1. Compute `SHA256SUMS` for all artifacts
2. Publish GitHub Release with tag name, all `.tar.gz` files, `tofa-{version}.dmg`, `SHA256SUMS`
3. Trigger tap update (repository dispatch to `cabichahine/homebrew-tofa`)

---

## macOS `.app` Bundle

```
tofa.app/
├── Contents/
│   ├── Info.plist
│   ├── MacOS/
│   │   ├── tofa             # universal binary (lipo of arm64 + x86_64)
│   │   └── tofa-launcher    # shell script entry point
│   └── Resources/
│       └── tofa.icns        # generated from icon.png
```

**`tofa-launcher`** (the executable the OS calls when user double-clicks):
```bash
#!/bin/bash
DIR="$(cd "$(dirname "$0")" && pwd)"
osascript - "$DIR/tofa" <<'EOF'
on run argv
  tell application "Terminal"
    activate
    do script (item 1 of argv)
  end tell
end run
EOF
```

**`Info.plist`** key fields:
- `CFBundleIdentifier`: `com.cabichahine.tofa`
- `CFBundleVersion` / `CFBundleShortVersionString`: injected from tag at build time
- `CFBundleExecutable`: `tofa-launcher`
- `CFBundleIconFile`: `tofa`
- `LSUIElement`: `false` (appears in Dock while running)

**Icon generation** (in CI, macOS runner):
- Source: `assets/icon.png` (1024×1024, dark `#0d1117` background, `tofa` text in `#58a6ff`)
- Convert to `.iconset` folder with `sips` at all required sizes (16→1024)
- Run `iconutil -c icns` to produce `tofa.icns`

**DMG layout:**
- Background: dark solid `#0d1117`
- `tofa.app` on the left, `/Applications` symlink on the right
- Created with `hdiutil create` → `hdiutil convert` (UDZO compressed)

---

## Homebrew Tap (`cabichahine/homebrew-tofa`)

Repository structure:
```
homebrew-tofa/
├── Formula/
│   └── tofa.rb
└── Casks/
    └── tofa.rb
```

**Formula (`Formula/tofa.rb`):**
```ruby
class Tofa < Formula
  desc "Eye-candy terminal OTP manager"
  homepage "https://github.com/cabichahine/tofa"
  version "VERSION"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/cabichahine/tofa/releases/download/vVERSION/tofa-VERSION-aarch64-apple-darwin.tar.gz"
      sha256 "SHA_MAC_ARM"
    end
    on_intel do
      url "https://github.com/cabichahine/tofa/releases/download/vVERSION/tofa-VERSION-x86_64-apple-darwin.tar.gz"
      sha256 "SHA_MAC_X86"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/cabichahine/tofa/releases/download/vVERSION/tofa-VERSION-aarch64-unknown-linux-musl.tar.gz"
      sha256 "SHA_LINUX_ARM"
    end
    on_intel do
      url "https://github.com/cabichahine/tofa/releases/download/vVERSION/tofa-VERSION-x86_64-unknown-linux-musl.tar.gz"
      sha256 "SHA_LINUX_X86"
    end
  end

  def install
    bin.install "tofa"
    generate_completions_from_executable(bin/"tofa", "completions")
  end

  test do
    assert_match "tofa", shell_output("#{bin}/tofa --version")
  end
end
```

**Cask (`Casks/tofa.rb`):**
```ruby
cask "tofa" do
  version "VERSION"
  sha256 "SHA_DMG"

  url "https://github.com/cabichahine/tofa/releases/download/v#{version}/tofa-#{version}.dmg"

  name "tofa"
  desc "Eye-candy terminal OTP manager"
  homepage "https://github.com/cabichahine/tofa"

  app "tofa.app"

  zap trash: [
    "~/.config/tofa",
  ]
end
```

**Auto-update mechanism:** the release pipeline sends a `repository_dispatch` event to `cabichahine/homebrew-tofa`. A workflow there checks out the tap, replaces VERSION and SHA256 placeholders using `sed`, and commits directly to `main`.

**Installation commands:**
```bash
brew tap cabichahine/tofa
brew install tofa          # formula (CLI)
brew install --cask tofa   # cask (macOS .app)
```

---

## AUR Packages

Two packages maintained under `packaging/aur/` in the main repo:

### `tofa-bin` (pre-built binary — recommended)

```
packaging/aur/tofa-bin/
└── PKGBUILD
```

```bash
pkgname=tofa-bin
pkgver=1.0.0
pkgrel=1
pkgdesc="Eye-candy terminal OTP manager"
arch=('x86_64' 'aarch64')
url="https://github.com/cabichahine/tofa"
license=('MIT')
provides=('tofa')
conflicts=('tofa')

source_x86_64=("$url/releases/download/v$pkgver/tofa-$pkgver-x86_64-unknown-linux-musl.tar.gz")
source_aarch64=("$url/releases/download/v$pkgver/tofa-$pkgver-aarch64-unknown-linux-musl.tar.gz")
sha256sums_x86_64=('SHA_LINUX_X86')
sha256sums_aarch64=('SHA_LINUX_ARM')

package() {
  install -Dm755 tofa "$pkgdir/usr/bin/tofa"
  "$pkgdir/usr/bin/tofa" completions bash | install -Dm644 /dev/stdin "$pkgdir/usr/share/bash-completion/completions/tofa"
  "$pkgdir/usr/bin/tofa" completions zsh  | install -Dm644 /dev/stdin "$pkgdir/usr/share/zsh/site-functions/_tofa"
  "$pkgdir/usr/bin/tofa" completions fish | install -Dm644 /dev/stdin "$pkgdir/usr/share/fish/vendor_completions.d/tofa.fish"
}
```

### `tofa` (build from source)

```
packaging/aur/tofa/
└── PKGBUILD
```

Fetches the release `.tar.gz` of the source, runs `cargo build --release`. Depends on `rust` and `cargo`.

**Update script** (`scripts/publish-aur.sh`): clones the AUR package repo, copies the updated PKGBUILD, updates `.SRCINFO` with `makepkg --printsrcinfo`, commits and pushes. Run manually after each release.

---

## `cargo install`

Works automatically once the repo is public and the crate is published to crates.io:

```bash
cargo install tofa
```

Requires publishing `tofa-core` first (it's a workspace dependency), then `tofa`.

---

## File Layout in Main Repo

```
tofa/
├── .github/
│   └── workflows/
│       ├── ci.yml
│       └── release.yml
├── assets/
│   └── icon.png            # 1024×1024 source icon
├── packaging/
│   └── aur/
│       ├── tofa/
│       │   └── PKGBUILD
│       └── tofa-bin/
│           └── PKGBUILD
├── scripts/
│   └── publish-aur.sh
└── ...
```

---

## Release Process (step by step)

```bash
# 1. Bump version in tofa/Cargo.toml and tofa-core/Cargo.toml
# 2. Commit: "chore: bump version to 1.0.0"
# 3. Tag and push
git tag v1.0.0
git push origin main --tags

# GitHub Actions takes over:
# → builds 4 binaries
# → builds DMG
# → publishes GitHub Release
# → auto-updates homebrew-tofa tap

# 4. Publish to crates.io (manual)
cargo publish -p tofa-core
cargo publish -p tofa

# 5. Update AUR (manual)
bash scripts/publish-aur.sh 1.0.0
```
