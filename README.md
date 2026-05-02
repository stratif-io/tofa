<div align="center">
  <img src="tofa-app/src-tauri/icons/128x128@2x.png" width="96" alt="tofa" />
  <h1>tofa</h1>
  <p><strong>Offline, encrypted 2FA for macOS — menu bar app and eye-candy terminal TUI.</strong></p>

  [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
  [![Rust](https://img.shields.io/badge/rust-1.78%2B-orange.svg)](https://www.rust-lang.org)
  [![Tauri](https://img.shields.io/badge/tauri-v2-purple.svg)](https://tauri.app)
  [![Platform](https://img.shields.io/badge/platform-macOS-lightgrey.svg)](#)
</div>

---

tofa is a local-first TOTP authenticator with two interfaces: a macOS menu bar app and a full-featured terminal TUI. Every secret stays encrypted on disk — no cloud sync, no account, no telemetry.

## Features

- **Always one click away** — tray icon opens a compact popover with live codes and countdown rings
- **Eye-candy TUI** — violet accent, per-entry progress bars, mouse support, live countdown
- **AES-256-GCM vault** encrypted with an Argon2id-derived key; secrets never hit disk in plaintext
- **QR scanning** — scan your screen, drop an image, or use the camera to add accounts instantly
- **Google Authenticator migration** — import `otpauth-migration://` QR codes directly
- **Manual entry** — paste a raw Base32 secret or a full `otpauth://` URI
- **Session lock** — auto-locks after 5 minutes of inactivity; lock manually from the menu
- **Scriptable CLI** — add, list, copy codes, import/export, and pipe into scripts

## Quick start

```bash
# Clone and build the menu bar app
git clone https://github.com/cabichahine/tofa
cd tofa/tofa-app
cargo tauri build        # → src-tauri/target/release/bundle/macos/tofa.app
```

Or build the CLI + TUI:

```bash
cargo install --path tofa
tofa          # open the TUI
tofa --help   # CLI reference
```

## CLI & TUI

```
tofa                               # open the interactive TUI
tofa init                          # create a new encrypted vault
tofa add --name GitHub:you --secret JBSWY3DPEHPK3PXP
tofa add --uri "otpauth://totp/..."
tofa add --qr ~/Downloads/qr.png
tofa list                          # show all entries
tofa code GitHub:you               # print current TOTP code
tofa code GitHub:you | pbcopy      # copy to clipboard
tofa remove GitHub:you
tofa rename GitHub:you GitHub:me
tofa rekey                         # change vault passphrase
tofa export                        # dump vault as JSON
tofa import accounts.json
tofa qr GitHub:you                 # display QR in terminal
tofa completions zsh               # shell completions
```

## Architecture

| Package | Role |
|---|---|
| `tofa-core` | Rust library — crypto, TOTP generation, QR parsing, vault I/O |
| `tofa-app` | Tauri v2 menu bar app — thin shell over `tofa-core` |
| `tofa` | Clap CLI + Ratatui TUI — thin shell over `tofa-core` |

All business logic lives in `tofa-core`. The app and CLI are pure UI layers.

## Vault security

- Key derivation: **Argon2id** (m=64 MiB, t=3, p=1)
- Encryption: **AES-256-GCM** with a random 96-bit nonce per write
- Atomic writes: vault saved to a temp file then renamed — no partial writes
- Passphrase cached in memory with a 5-minute TTL; zeroed on lock via `zeroize`

## Built with

[Rust](https://www.rust-lang.org) · [Tauri v2](https://tauri.app) · [Ratatui](https://ratatui.rs) · [totp-rs](https://github.com/constantoine/totp-rs) · [Argon2](https://github.com/RustCrypto/password-hashes) · [rqrr](https://github.com/WanzenBug/rqrr) · [Clap](https://github.com/clap-rs/clap)

## License

MIT © Carlo Abi Chahine
