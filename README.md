<div align="center">
  <img src="tofa-app/src-tauri/icons/128x128@2x.png" width="96" alt="TOFA" />
  <h1>TOFA</h1>
  <p><strong>Offline, encrypted 2FA — CLI, TUI, and a macOS menu bar app.</strong></p>

  [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
  [![Rust](https://img.shields.io/badge/rust-1.78%2B-orange.svg)](https://www.rust-lang.org)
  [![Tauri](https://img.shields.io/badge/tauri-v2-purple.svg)](https://tauri.app)
  [![Platform](https://img.shields.io/badge/platform-macOS-lightgrey.svg)](#)
  [![Docs](https://img.shields.io/badge/docs-live-blue.svg)](https://stratif-io.github.io/tofa/)
</div>

---

TOFA is a command-line 2FA tool with a full-featured terminal TUI. Secrets stay encrypted on disk with AES-256-GCM — no cloud, no account, no telemetry. A macOS menu bar app is the first GUI built on top of it.

## Why TOFA

You're mid-flow at your desk and a site asks for a 6-digit OTP. You unlock your phone, open the authenticator app, wait for a code, squint at it, and type it in. That interruption is annoying and unnecessary when your Mac is right in front of you.

TOFA lives in your terminal — or in one click from the menu bar. No phone needed.

## CLI & TUI

<!-- demo-cli: replace with an actual GIF or video -->
<div align="center">
  <img src="docs/demo-cli.gif" alt="TOFA TUI demo" width="640" />
</div>

```bash
cargo install --path tofa
tofa          # open the interactive TUI
tofa --help   # CLI reference
```

**Full reference:** <https://stratif-io.github.io/tofa/>

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

The TUI shows all accounts with live countdown bars, violet accent, and mouse support.

## macOS menu bar app

<!-- demo-app: replace with an actual GIF or video -->
<div align="center">
  <img src="docs/demo-app.gif" alt="TOFA menu bar app demo" width="320" />
</div>

The first GUI built on `tofa-core`. One click from the tray icon opens a compact popover with live codes — no phone, no alt-tab. Leave your phone in your pocket.

```bash
git clone https://github.com/stratif-io/tofa
cd tofa/tofa-app
cargo tauri build        # → src-tauri/target/release/bundle/macos/tofa.app
```

**Adding accounts:**

| Method | How |
|--------|-----|
| **Open file** | QR image (PNG/JPG), Aegis or andOTP JSON export, TXT (one URI per line), or ZIP |
| **Scan screen** | Captures your screen and detects any QR code on it |
| **Camera** | Opens a browser-based QR scanner using your webcam |
| **Paste URI** | Type or paste an `otpauth://totp/…` URI directly |

The vault locks after 10 minutes of inactivity or manually from the tray menu. Supports dark/light/auto theme and a configurable vault path.

## Architecture

| Package | Role |
|---|---|
| `tofa-core` | Rust library — crypto, TOTP generation, QR parsing, vault I/O |
| `tofa` | Clap CLI + Ratatui TUI — thin shell over `tofa-core` |
| `tofa-app` | Tauri v2 menu bar app — thin shell over `tofa-core` |

All business logic lives in `tofa-core`. The CLI, TUI, and app are pure UI layers.

## Vault security

- Key derivation: **Argon2id** (m=64 MiB, t=3, p=1)
- Encryption: **AES-256-GCM** with a random 96-bit nonce per write
- Atomic writes: vault saved to a temp file then renamed — no partial writes
- Passphrase cached in memory with a 10-minute TTL; zeroed on lock via `zeroize`

## Built with

[Rust](https://www.rust-lang.org) · [Tauri v2](https://tauri.app) · [Ratatui](https://ratatui.rs) · [totp-rs](https://github.com/constantoine/totp-rs) · [Argon2](https://github.com/RustCrypto/password-hashes) · [rqrr](https://github.com/WanzenBug/rqrr) · [Clap](https://github.com/clap-rs/clap)

## License

MIT © Carlo Abi Chahine
