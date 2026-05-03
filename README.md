<div align="center">
  <img src="tofa-app/src-tauri/icons/128x128@2x.png" width="96" alt="tofa" />
  <h1>tofa</h1>
  <p><strong>Stop reaching for your phone every time a site asks for a 6-digit code.</strong></p>

  [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
  [![Rust](https://img.shields.io/badge/rust-1.78%2B-orange.svg)](https://www.rust-lang.org)
  [![Tauri](https://img.shields.io/badge/tauri-v2-purple.svg)](https://tauri.app)
  [![Platform](https://img.shields.io/badge/platform-macOS-lightgrey.svg)](#)
</div>

---

tofa keeps all your 2FA codes one click away in the macOS menu bar — no phone, no cloud, no account. Secrets stay encrypted on disk with AES-256-GCM. There is also a full-featured terminal TUI for keyboard-first workflows.

## Why tofa

You're mid-flow at your desk and a site asks for a 6-digit OTP. You unlock your phone, open the authenticator app, wait for a code, squint at it, and type it in. That interruption is annoying and unnecessary when your Mac is right in front of you.

tofa lives in your menu bar. One click, one copy, back to work.

## Features

- **One click away** — menu bar icon opens a compact popover with live codes and countdown bars
- **Dark / light / auto theme** — matches your system appearance or override it in Settings
- **AES-256-GCM vault** encrypted with an Argon2id-derived key; secrets never hit disk in plaintext
- **Multi-format import** — open a QR image, a JSON export (Aegis, andOTP), a plain URI list (TXT), or a ZIP archive; scan your screen; or use the camera
- **Google Authenticator migration** — import `otpauth-migration://` QR codes directly
- **Manual entry** — paste a raw Base32 secret or a full `otpauth://` URI
- **Configurable vault path** — choose where the encrypted vault lives from the Settings screen
- **Session lock** — auto-locks after 10 minutes of inactivity; lock manually from the tray menu
- **Eye-candy TUI** — violet accent, per-entry progress bars, mouse support, live countdown
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

## Menu bar app

Click the tray icon to open the popover. On first launch, create a passphrase to protect your vault. Then add accounts from the **+** button:

| Method | How |
|--------|-----|
| **Open file** | Pick a QR image (PNG/JPG), an Aegis or andOTP JSON export, a TXT file with one `otpauth://` URI per line, or a ZIP containing any of the above |
| **Scan screen** | Captures your screen and detects any QR code on it |
| **Camera** | Opens a browser-based QR scanner using your webcam |
| **Paste URI** | Type or paste an `otpauth://totp/…` URI directly |

Codes refresh automatically. Click any entry to copy it to your clipboard. The vault locks after 10 minutes of inactivity or via the tray menu.

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
- Passphrase cached in memory with a 10-minute TTL; zeroed on lock via `zeroize`

## Built with

[Rust](https://www.rust-lang.org) · [Tauri v2](https://tauri.app) · [Ratatui](https://ratatui.rs) · [totp-rs](https://github.com/constantoine/totp-rs) · [Argon2](https://github.com/RustCrypto/password-hashes) · [rqrr](https://github.com/WanzenBug/rqrr) · [Clap](https://github.com/clap-rs/clap)

## License

MIT © Carlo Abi Chahine
