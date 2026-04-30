# rotp — OTP Manager TUI: Design Spec

**Date:** 2026-04-30  
**Status:** Approved

---

## Overview

`rotp` is a terminal-based OTP (TOTP) manager written in Rust. It stores encrypted OTP secrets locally and displays live 6-digit codes with expiry timers in an eye-candy TUI. The vault is protected by a passphrase using Argon2id + AES-256-GCM.

---

## Architecture

### Approach

Two-crate Cargo workspace:

- **`rotp-core`** — pure library crate (crypto, TOTP, storage, QR decoding). Fully testable in isolation.
- **`rotp`** — binary crate (TUI, CLI argument parsing). Depends on `rotp-core`.

### Project Structure

```
rotp/
├── Cargo.toml                  # workspace root
├── rotp-core/
│   └── src/
│       ├── crypto.rs           # Argon2id key derivation + AES-256-GCM encrypt/decrypt
│       ├── totp.rs             # TOTP code generation (RFC 6238)
│       ├── store.rs            # vault read/write (atomic writes)
│       ├── qr.rs               # QR image → otpauth:// URI decoding
│       └── lib.rs
│   └── tests/                  # integration tests
├── rotp/
│   └── src/
│       ├── main.rs
│       ├── tui/                # Ratatui widgets and screens
│       └── cli.rs              # clap argument parsing
├── .github/
│   └── workflows/
│       ├── ci.yml              # test + lint on every PR
│       └── release.yml         # cargo-dist release pipeline
└── README.md
```

### Key Dependencies

| Crate | Purpose |
|---|---|
| `argon2` | Passphrase → 256-bit key (Argon2id) |
| `aes-gcm` | AES-256-GCM encryption/decryption |
| `totp-rs` | TOTP code generation |
| `rqrr` | QR code image decoding |
| `image` | Image file loading |
| `serde` + `serde_json` | Vault serialization |
| `ratatui` | TUI framework |
| `crossterm` | Cross-platform keyboard/terminal events |
| `clap` | CLI argument parsing |
| `zeroize` | Secure memory zeroing after key use |
| `arboard` | Cross-platform clipboard access |

---

## Security

### Passphrase & Key Derivation

- Passphrase prompted at every launch, masked on screen (`•••`)
- Derived to a 256-bit key using **Argon2id** (m=64MB, t=3, p=1)
- Key **zeroized** from memory immediately after vault decrypt/encrypt
- Wrong passphrase → AES-GCM authentication tag failure → error message → exit

### Vault Format

Binary file at `~/.config/rotp/vault.enc`:

```
[ salt 32B ] [ nonce 12B ] [ ciphertext ] [ GCM tag 16B ]
```

Plaintext (JSON) structure:

```json
{
  "version": 1,
  "entries": [
    { "name": "GitHub", "secret": "BASE32SECRET", "created_at": "2024-01-15" }
  ]
}
```

### Atomic Writes

All vault writes follow: write to temp file → `fsync` → `rename`. No vault corruption on crash.

---

## Features

### 1. Unlock Screen

- Full-screen passphrase prompt at launch
- Input masked
- `[ Enter ]` to unlock, `[ Ctrl+C ]` to quit
- Error shown on wrong passphrase, re-prompts

### 2. Main List Screen

Compact list layout (one line per account):

```
rotp                                      3 accounts
▶ GitHub                    123 456        ⏱ 18s
  AWS                       789 012        ⏱ 8s
  Google                    345 678        ⏱ 25s

[ ↑↓ ] navigate  [ Enter ] fullscreen  [ a ] add  [ d ] delete  [ y ] copy  [ q ] quit
```

- Selected entry highlighted with `▶` and left border
- Code visible only on selected entry (others dimmed)
- Timer color: green (>20s), orange (10–20s), red (<10s)
- Codes refresh every second

### 3. Fullscreen Code View

Triggered by `[ Enter ]` on a selected entry:

- Account name displayed in large spaced uppercase
- Code displayed at ~64px equivalent, with digit spacing
- Progress bar showing time remaining
- `[ y ]` to copy to clipboard without leaving screen
- `[ Esc ]` or `[ q ]` to return to list

### 4. Add OTP

`[ a ]` from the list opens a two-field form:

1. **Name** — free text
2. **QR code path or URI/secret** — accepts:
   - File path to a QR code image (PNG, JPG) → decoded via `rqrr`
   - `otpauth://totp/...` URI pasted directly
   - Raw Base32 TOTP secret

Date of creation saved automatically.

> Note: In terminals that support file drag & drop (iTerm2, Kitty, WezTerm), dragging a QR image pastes its path automatically. Documented in README.

### 5. Delete OTP

`[ d ]` from the list opens a confirmation dialog:

```
⚠ Delete this account?
GitHub will be permanently removed from the vault.

[ y ] Yes    [ n ] No
```

Only deleted on explicit `[ y ]` confirmation.

### 6. Copy to Clipboard

`[ y ]` from either the list or fullscreen view copies the current code to the system clipboard. No visual feedback disrupts the TUI (a brief status line update suffices). On Linux headless environments where a clipboard is unavailable, `rotp` prints an error in the status line and does not crash.

---

## Visual Style

- **Theme:** Hacker Green — `#00ff41` on `#0d0d0d`
- **Accent colors:** timer uses green/orange/red only
- **Font:** monospace (terminal default)
- **Language:** English throughout (UI, errors, docs)

---

## Testing Strategy (TDD)

All features in `rotp-core` are written test-first:

- `crypto.rs` — encrypt/decrypt roundtrip, wrong passphrase rejection, key zeroization
- `totp.rs` — known TOTP vectors (RFC 6238 test vectors), timer boundary conditions
- `store.rs` — vault read/write roundtrip, atomic write behavior, version field
- `qr.rs` — QR decoding from test images, URI parsing, raw secret parsing

TUI code (`rotp` crate) is tested via integration tests that drive the event loop with synthetic key events.

---

## Distribution

### Release Pipeline

`git tag vX.Y.Z` → GitHub Actions → `cargo-dist` builds and publishes:

| Platform | Target |
|---|---|
| macOS (Intel) | `x86_64-apple-darwin` |
| macOS (Apple Silicon) | `aarch64-apple-darwin` |
| Linux (glibc) | `x86_64-unknown-linux-gnu` |
| Linux (static/musl) | `x86_64-unknown-linux-musl` |
| Windows | `x86_64-pc-windows-msvc` |

GitHub Release created automatically with binaries + SHA256 checksums.

### Install Methods

| Platform | Method |
|---|---|
| macOS | `brew install carlo/tap/rotp` |
| Linux | `yay -S rotp` (AUR), `cargo install rotp`, install script |
| Windows | `scoop install rotp`, `winget install rotp` |
| Universal | `cargo install rotp`, GitHub Releases binary |

### CI (every PR)

- `cargo test` — unit + integration
- `cargo clippy` — linting
- `cargo fmt --check` — formatting
- `cargo audit` — security advisories
- Matrix: ubuntu-latest, macos-latest, windows-latest
