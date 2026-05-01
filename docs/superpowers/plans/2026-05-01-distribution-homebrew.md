# Distribution — Plan B: Homebrew Tap

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create the `cabichahine/homebrew-tofa` tap repository with a Homebrew formula (pre-built binaries) and a Homebrew cask (macOS DMG), plus an auto-update workflow triggered by `repository_dispatch` from the main release pipeline.

**Architecture:** A separate GitHub repo (`homebrew-tofa`) structured as a Homebrew tap. A GitHub Actions workflow there listens for `repository_dispatch` events from `cabichahine/tofa`, replaces VERSION/SHA placeholders in the formula and cask files using `sed`, and commits directly to `main`. Plan A (CI/CD) must be completed first — this plan depends on the tap dispatch event sent at the end of `release.yml`.

**Tech Stack:** Ruby (Homebrew DSL), GitHub Actions, `sed`, `gh` CLI.

**Prerequisite:** Plan A completed and `TAP_DISPATCH_TOKEN` secret configured in `cabichahine/tofa`.

---

### Task 1: Create the tap repository

- [ ] **Step 1: Create `cabichahine/homebrew-tofa` on GitHub**

```bash
gh repo create cabichahine/homebrew-tofa --public --description "Homebrew tap for tofa"
```

- [ ] **Step 2: Clone it locally**

```bash
git clone https://github.com/cabichahine/homebrew-tofa ~/homebrew-tofa
cd ~/homebrew-tofa
```

- [ ] **Step 3: Create the directory structure**

```bash
mkdir -p Formula Casks
```

---

### Task 2: Homebrew formula

**Files:**
- Create: `Formula/tofa.rb` (in the `homebrew-tofa` repo)

- [ ] **Step 1: Write the formula**

```ruby
class Tofa < Formula
  desc "Eye-candy terminal OTP manager"
  homepage "https://github.com/cabichahine/tofa"
  version "PLACEHOLDER_VERSION"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/cabichahine/tofa/releases/download/v#{version}/tofa-#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA_MAC_ARM"
    end
    on_intel do
      url "https://github.com/cabichahine/tofa/releases/download/v#{version}/tofa-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA_MAC_X86"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/cabichahine/tofa/releases/download/v#{version}/tofa-#{version}-aarch64-unknown-linux-musl.tar.gz"
      sha256 "PLACEHOLDER_SHA_LINUX_ARM"
    end
    on_intel do
      url "https://github.com/cabichahine/tofa/releases/download/v#{version}/tofa-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "PLACEHOLDER_SHA_LINUX_X86"
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

- [ ] **Step 2: Write the cask**

```ruby
# Casks/tofa.rb
cask "tofa" do
  version "PLACEHOLDER_VERSION"
  sha256 "PLACEHOLDER_SHA_DMG"

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

- [ ] **Step 3: Commit**

```bash
git add Formula/tofa.rb Casks/tofa.rb
git commit -m "feat: add formula and cask with placeholder versions"
git push origin main
```

---

### Task 3: Auto-update workflow in the tap repo

**Files:**
- Create: `.github/workflows/update.yml` (in `homebrew-tofa`)

- [ ] **Step 1: Write the update workflow**

This workflow fires on `repository_dispatch` from the main repo and updates both files in one commit.

```yaml
name: Update tap

on:
  repository_dispatch:
    types: [new-release]

jobs:
  update:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          token: ${{ secrets.GITHUB_TOKEN }}

      - name: Update formula
        run: |
          VERSION="${{ github.event.client_payload.version }}"
          SHA_MAC_ARM="${{ github.event.client_payload.sha_mac_arm }}"
          SHA_MAC_X86="${{ github.event.client_payload.sha_mac_x86 }}"
          SHA_LINUX_ARM="${{ github.event.client_payload.sha_linux_arm }}"
          SHA_LINUX_X86="${{ github.event.client_payload.sha_linux_x86 }}"

          sed -i "s/PLACEHOLDER_VERSION/$VERSION/g" Formula/tofa.rb
          # If updating from a real previous version, replace the old version string too:
          sed -i "s/version \"[0-9.]*\"/version \"$VERSION\"/" Formula/tofa.rb
          sed -i "s/PLACEHOLDER_SHA_MAC_ARM/$SHA_MAC_ARM/" Formula/tofa.rb
          sed -i "s/sha256 \"[a-f0-9]*\" # mac_arm/sha256 \"$SHA_MAC_ARM\" # mac_arm/" Formula/tofa.rb

          # Simpler approach: rewrite with known placeholder names
          sed -i "s/PLACEHOLDER_SHA_MAC_ARM/$SHA_MAC_ARM/g" Formula/tofa.rb
          sed -i "s/PLACEHOLDER_SHA_MAC_X86/$SHA_MAC_X86/g" Formula/tofa.rb
          sed -i "s/PLACEHOLDER_SHA_LINUX_ARM/$SHA_LINUX_ARM/g" Formula/tofa.rb
          sed -i "s/PLACEHOLDER_SHA_LINUX_X86/$SHA_LINUX_X86/g" Formula/tofa.rb

      - name: Update cask
        run: |
          VERSION="${{ github.event.client_payload.version }}"
          SHA_DMG="${{ github.event.client_payload.sha_dmg }}"
          sed -i "s/PLACEHOLDER_VERSION/$VERSION/g" Casks/tofa.rb
          sed -i "s/PLACEHOLDER_SHA_DMG/$SHA_DMG/g" Casks/tofa.rb

      - name: Commit and push
        run: |
          VERSION="${{ github.event.client_payload.version }}"
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"
          git add Formula/tofa.rb Casks/tofa.rb
          git commit -m "chore: update to v$VERSION"
          git push
```

**Important:** After the first release, the `PLACEHOLDER_*` strings are replaced with real values. Subsequent releases need a different sed strategy — replace the existing version/sha lines. Rewrite the update step to use line-targeted replacement for subsequent runs:

```yaml
      - name: Update formula (idempotent)
        run: |
          VERSION="${{ github.event.client_payload.version }}"
          SHA_MAC_ARM="${{ github.event.client_payload.sha_mac_arm }}"
          SHA_MAC_X86="${{ github.event.client_payload.sha_mac_x86 }}"
          SHA_LINUX_ARM="${{ github.event.client_payload.sha_linux_arm }}"
          SHA_LINUX_X86="${{ github.event.client_payload.sha_linux_x86 }}"

          # Replace version line
          sed -i "s/^  version \".*\"/  version \"$VERSION\"/" Formula/tofa.rb
          # Replace sha256 lines by position (order: mac_arm, mac_x86, linux_arm, linux_x86)
          # Use python for reliable multi-pattern replacement
          python3 - "$SHA_MAC_ARM" "$SHA_MAC_X86" "$SHA_LINUX_ARM" "$SHA_LINUX_X86" << 'PYEOF'
          import sys, re

          sha_mac_arm, sha_mac_x86, sha_linux_arm, sha_linux_x86 = sys.argv[1:]

          with open("Formula/tofa.rb") as f:
              content = f.read()

          # Replace each sha256 line in order of appearance
          shas = [sha_mac_arm, sha_mac_x86, sha_linux_arm, sha_linux_x86]
          idx = 0
          def replacer(m):
              global idx
              result = m.group(0)[:m.start("sha") - m.start()] + shas[idx] + '"'
              idx += 1
              return f'      sha256 "{shas[idx-1]}"'
          content = re.sub(r'sha256 "[a-f0-9]+"', lambda m: f'sha256 "{shas.pop(0)}"', content)

          with open("Formula/tofa.rb", "w") as f:
              f.write(content)
          PYEOF
```

Actually, the simpler approach: keep named comment markers in the formula. Rewrite the formula to use comment markers:

The definitive, robust approach — rewrite `Formula/tofa.rb` to use named markers as comments on the sha256 lines:

```ruby
      sha256 "PLACEHOLDER_SHA_MAC_ARM" # SHA_MAC_ARM
```

Then sed targets those comments:

```bash
sed -i "s/\"[^\"]*\" # SHA_MAC_ARM/\"$SHA_MAC_ARM\" # SHA_MAC_ARM/" Formula/tofa.rb
```

- [ ] **Step 2: Update formula to use named comment markers**

Rewrite `Formula/tofa.rb` with comment markers:

```ruby
class Tofa < Formula
  desc "Eye-candy terminal OTP manager"
  homepage "https://github.com/cabichahine/tofa"
  version "PLACEHOLDER_VERSION" # VERSION
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/cabichahine/tofa/releases/download/v#{version}/tofa-#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA_MAC_ARM" # SHA_MAC_ARM
    end
    on_intel do
      url "https://github.com/cabichahine/tofa/releases/download/v#{version}/tofa-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA_MAC_X86" # SHA_MAC_X86
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/cabichahine/tofa/releases/download/v#{version}/tofa-#{version}-aarch64-unknown-linux-musl.tar.gz"
      sha256 "PLACEHOLDER_SHA_LINUX_ARM" # SHA_LINUX_ARM
    end
    on_intel do
      url "https://github.com/cabichahine/tofa/releases/download/v#{version}/tofa-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "PLACEHOLDER_SHA_LINUX_X86" # SHA_LINUX_X86
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

Same for `Casks/tofa.rb`:

```ruby
cask "tofa" do
  version "PLACEHOLDER_VERSION" # VERSION
  sha256 "PLACEHOLDER_SHA_DMG" # SHA_DMG

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

- [ ] **Step 3: Write the final update workflow**

```yaml
name: Update tap

on:
  repository_dispatch:
    types: [new-release]

jobs:
  update:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          token: ${{ secrets.GITHUB_TOKEN }}

      - name: Update formula and cask
        run: |
          VERSION="${{ github.event.client_payload.version }}"
          SHA_MAC_ARM="${{ github.event.client_payload.sha_mac_arm }}"
          SHA_MAC_X86="${{ github.event.client_payload.sha_mac_x86 }}"
          SHA_LINUX_ARM="${{ github.event.client_payload.sha_linux_arm }}"
          SHA_LINUX_X86="${{ github.event.client_payload.sha_linux_x86 }}"
          SHA_DMG="${{ github.event.client_payload.sha_dmg }}"

          # Formula
          sed -i "s/\"[^\"]*\" # VERSION/\"$VERSION\" # VERSION/" Formula/tofa.rb
          sed -i "s/\"[^\"]*\" # SHA_MAC_ARM/\"$SHA_MAC_ARM\" # SHA_MAC_ARM/" Formula/tofa.rb
          sed -i "s/\"[^\"]*\" # SHA_MAC_X86/\"$SHA_MAC_X86\" # SHA_MAC_X86/" Formula/tofa.rb
          sed -i "s/\"[^\"]*\" # SHA_LINUX_ARM/\"$SHA_LINUX_ARM\" # SHA_LINUX_ARM/" Formula/tofa.rb
          sed -i "s/\"[^\"]*\" # SHA_LINUX_X86/\"$SHA_LINUX_X86\" # SHA_LINUX_X86/" Formula/tofa.rb

          # Cask
          sed -i "s/\"[^\"]*\" # VERSION/\"$VERSION\" # VERSION/" Casks/tofa.rb
          sed -i "s/\"[^\"]*\" # SHA_DMG/\"$SHA_DMG\" # SHA_DMG/" Casks/tofa.rb

      - name: Commit and push
        run: |
          VERSION="${{ github.event.client_payload.version }}"
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"
          git add Formula/tofa.rb Casks/tofa.rb
          git diff --staged --quiet || git commit -m "chore: update to v$VERSION"
          git push
```

- [ ] **Step 4: Commit**

```bash
git add .github/workflows/update.yml Formula/tofa.rb Casks/tofa.rb
git commit -m "feat: add auto-update workflow with named comment markers"
git push origin main
```

---

### Task 4: Verify installation after first release

After Plan A's smoke test (Task 8) succeeds and the tap gets auto-updated:

- [ ] **Step 1: Tap and install formula**

```bash
brew tap cabichahine/tofa
brew install cabichahine/tofa/tofa
tofa --version
# Expected: tofa 0.1.0
```

- [ ] **Step 2: Test cask**

```bash
brew install --cask cabichahine/tofa/tofa
# Opens Finder — drag tofa.app to /Applications
open /Applications/tofa.app
# Terminal should open running tofa
```

- [ ] **Step 3: Test completions (formula)**

```bash
# zsh
tofa completions zsh | head -5
# Expected: #compdef tofa
```
