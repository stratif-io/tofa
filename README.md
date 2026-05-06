<div align="center">
  <img src="tofa-app/src-tauri/icons/128x128@2x.png" width="96" alt="TOFA" />
  <h1>TOFA</h1>
  <p><strong>The 2FA app that lives in your terminal — and your menu bar.</strong></p>
  <p>Import your phone's authenticator once. Stay in flow.</p>

  [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
  [![crates.io](https://img.shields.io/crates/v/tofa.svg)](https://crates.io/crates/tofa)
  [![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)
  [![Platform](https://img.shields.io/badge/platform-macOS%20%C2%B7%20Linux-lightgrey.svg)](#)
  [![Tauri](https://img.shields.io/badge/tauri-v2-purple.svg)](https://tauri.app)
  [![Docs](https://img.shields.io/badge/docs-live-blue.svg)](https://docs.tofa.stratif.io/)

  <p>
    <a href="https://docs.tofa.stratif.io/">Docs</a> ·
    <a href="#quick-start">Install</a> ·
    <a href="https://github.com/stratif-io/tofa/releases">Releases</a> ·
    <a href="https://github.com/stratif-io/tofa/discussions">Discussions</a>
  </p>
</div>

---

<div align="center">
  <img src="docs/demo-cli.gif" alt="TOFA TUI demo" width="640" />
  <p><em>The TUI: live countdown bars, click any row to copy.</em></p>
</div>

<a id="quick-start"></a>

## ⚡ Quick Start

### CLI + TUI (macOS, Linux)

```bash
# Homebrew (macOS or Linux)
brew tap stratif-io/tofa
brew install tofa

# Cargo (any platform with Rust)
cargo install tofa

tofa          # open the TUI
tofa --help   # CLI reference
```

### macOS menu bar app

```bash
brew tap stratif-io/tofa
brew install --cask tofa
```

<div align="center">
  <img src="docs/demo-app.gif" alt="TOFA macOS menu bar app demo" width="320" />
  <p><em>The macOS app: tray icon → popover → click code.</em></p>
</div>

> **First launch:** TOFA isn't notarized yet, so macOS quarantines it.
> After install, run once:
> ```bash
> xattr -dr com.apple.quarantine /Applications/tofa.app
> ```
> Or right-click the app in Finder and choose **Open**, then confirm.
> See [why](https://docs.tofa.stratif.io/security.html#unsigned-build).

## 🔑 Why TOFA

- **Phone stays in your pocket.** Import your authenticator once and add new
  TOTPs from your computer. Your phone keeps working — TOFA is additive, not
  a replacement.
- **Offline by design.** No account, no cloud sync, no telemetry. Your secrets
  live in a single AES-256-GCM file unlocked by your passphrase.
- **Three faces, one vault.** A `tofa` CLI for scripts, a Ratatui TUI with
  click-to-copy codes, and a macOS menu bar app. All read the same vault.

## ✨ Features

**Get codes in**

- **Click any TUI row to copy.** No memorizing names, no typing. The TUI also shows live countdown bars per code.
- **Import otpauth URIs.** Paste a standard `otpauth://totp/…` URI from anywhere — `tofa add --uri` or the app's "Paste URI".
- **Import QR images.** Drop a PNG/JPG into the app, or `tofa add --qr screenshot.png` from the CLI.
- **Import from 9 authenticators.** Aegis, andOTP, 2FAS, Bitwarden, Raivo, Ente, KeePassXC, FreeOTP, and Google Authenticator's migration QR — all parsed directly from their export formats.

**Use TOFA day-to-day**

- **Export to QR.** Re-encode any account (or your whole vault) as scannable QR codes for backup or device transfer.
- **Encrypted vault.** AES-256-GCM with an Argon2id-derived key. Auto-locks after 10 min idle, or on demand.
- **Scriptable.** `tofa code github | pbcopy` works (`xclip -selection clipboard` on Linux). `TOFA_PASSPHRASE` env var unlocks the vault for CI/automation.
- **Open source, MIT.** No account, no telemetry, no cloud. Audit the crypto in `tofa-core`.

## 📥 Migrate from your phone

TOFA's job is to be *the last 2FA migration you do for a while*. Pull every
account from your existing authenticator into your TOFA vault once, then add
new ones from your computer.

| Source                            | How                                | Status                            |
|-----------------------------------|------------------------------------|-----------------------------------|
| **Aegis** (Android, FOSS)         | JSON export                        | ✓ direct                          |
| **andOTP** (Android, FOSS)        | JSON export                        | ✓ direct                          |
| **2FAS** (iOS / Android)          | JSON backup                        | ✓ direct                          |
| **Google Authenticator**          | Multi-account migration QR         | ✓ direct                          |
| **Bitwarden**                     | JSON export                        | ✓ direct                          |
| **Raivo OTP** (iOS)               | JSON / ZIP export                  | ✓ direct                          |
| **Ente Auth**                     | Plain-text export                  | ✓ direct                          |
| **KeePassXC**                     | CSV export                         | ✓ direct                          |
| **FreeOTP / FreeOTP+**            | TXT URI export                     | ✓ direct                          |
| Any app with **QR display**       | Screenshot or camera               | ✓ via `tofa add --qr` or the app  |
| **1Password**                     | TOTP fields export                 | ⏳ Coming soon                     |
| **Apple Passwords**               | CSV export (macOS 15+)             | ⏳ Coming soon                     |
| **Authy**                         | No user-facing export              | ✗ Not possible                    |
| **Microsoft Authenticator**       | Cloud-only backup, no plain export | ✗ Not possible                    |

> **Per-vendor migration guides** — with screenshots and step-by-step import —
> are coming to the [docs site](https://docs.tofa.stratif.io/). Want a
> vendor prioritized?
> [Open an issue](https://github.com/stratif-io/tofa/issues/new).

## 📊 How TOFA compares

|                                 | TOFA | Authy | Google Auth | Microsoft Auth | 1Password | Aegis |
|---------------------------------|:----:|:-----:|:-----------:|:--------------:|:---------:|:-----:|
| Open source                     |  ✓   |   ✗   |     ✗       |       ✗        |    ✗      |  ✓    |
| Works without an account        |  ✓   |   ✗   |     ✓       |       ✗        |    ✗      |  ✓    |
| Encrypted local vault           |  ✓   |   ✗   |     ✗       |       ✗        |    ✓      |  ✓    |
| Native CLI                      |  ✓   |   ✗   |     ✗       |       ✗        |    ✓      |  ✗    |
| Native TUI                      |  ✓   |   ✗   |     ✗       |       ✗        |    ✗      |  ✗    |
| Desktop app (no phone needed)   |  ✓   |   ✓   |     ✗       |       ✓        |    ✓      |  ✗    |
| No telemetry                    |  ✓   |   ✗   |     ✗       |       ✗        |    ✗      |  ✓    |
| Import otpauth URI / QR         |  ✓   |   ✗   |     ✓       |       ✗        |    ✗      |  ✓    |
| Export your secrets             |  ✓   |   ✗   |     ✓       |       ✗        |    ✗      |  ✓    |

> Comparison reflects published behavior as of 2026. Corrections welcome —
> open a [PR](https://github.com/stratif-io/tofa/pulls).

## 🔒 Your vault, your responsibility

TOFA stores everything in a single file (default
`~/.local/share/tofa/vault.json`, configurable), encrypted with
**AES-256-GCM** under a key derived from your passphrase via **Argon2id**.
The passphrase never touches disk; it lives in memory with a 10-minute TTL
and is zeroed on lock.

**No cloud means no recovery.** If you lose the file, no one can restore it.
If you forget the passphrase, no one can decrypt it. There is no "forgot
password" link, and that's the point.

**Back up the vault yourself.** A few patterns that work:

- Drop the file in **iCloud Drive**, **Dropbox**, or any folder your OS syncs
- Run `tofa export` and stash the JSON in a password manager
- Use `tofa qr <name>` to print a paper backup of any single account

See the [security model](https://docs.tofa.stratif.io/security.html) for
the full threat model and crypto choices.

## 🤝 Contributing

Bug reports, vendor migration requests, and PRs are all welcome. Start with
[CONTRIBUTING.md](./CONTRIBUTING.md) or open a
[discussion](https://github.com/stratif-io/tofa/discussions).

If TOFA saved you a phone-grab today, a ⭐ on GitHub helps others find it.

## ⭐ Star history

[![Star History Chart](https://api.star-history.com/svg?repos=stratif-io/tofa&type=Date)](https://star-history.com/#stratif-io/tofa&Date)

## 🛠️ Built with

[Rust](https://www.rust-lang.org) ·
[Tauri v2](https://tauri.app) ·
[Ratatui](https://ratatui.rs) ·
[Clap](https://github.com/clap-rs/clap) ·
[totp-rs](https://github.com/constantoine/totp-rs) ·
[Argon2](https://github.com/RustCrypto/password-hashes) ·
[AES-GCM](https://github.com/RustCrypto/AEADs) ·
[rqrr](https://github.com/WanzenBug/rqrr) ·
[arboard](https://github.com/1Password/arboard)

## License

[MIT](./LICENSE) © Carlo Abi Chahine
