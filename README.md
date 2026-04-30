# rotp

Eye-candy terminal OTP manager written in Rust. Secure, offline-first, works everywhere.

## Features

- Live 6-digit TOTP codes with expiry timer (green → orange → red)
- AES-256-GCM encrypted vault, passphrase-derived with Argon2id
- Add OTPs via QR code image, `otpauth://` URI, or raw Base32 secret
- Fullscreen code view for easy phone scanning
- Copy to clipboard with `y`
- Atomic vault writes — no corruption on crash

## Install

**macOS**
```sh
brew install carlo/tap/rotp
```

**Linux**
```sh
# AUR
yay -S rotp

# Cargo
cargo install rotp

# Install script
curl -fsSL https://github.com/carlo/rotp/releases/latest/download/install.sh | sh
```

**Windows**
```sh
scoop install rotp
winget install rotp
```

## Usage

```sh
rotp        # open TUI
rotp --help
rotp --version
```

### Keyboard shortcuts

| Key | Action |
|-----|--------|
| `↑` / `↓` | Navigate |
| `Enter` | Fullscreen code view |
| `a` | Add OTP |
| `d` | Delete (with confirmation) |
| `y` | Copy code to clipboard |
| `q` | Quit |
| `Esc` | Back / cancel |

### Adding OTPs

Press `a` and provide either:
- A file path to a QR code image (PNG/JPG)
- An `otpauth://totp/...` URI
- A raw Base32 TOTP secret

> **Drag & drop:** In iTerm2, Kitty, and WezTerm, dragging a QR image into the terminal pastes its path automatically.

## Vault location

`~/.config/rotp/vault.enc`

## Security

- Argon2id (m=64MB, t=3, p=1) key derivation
- AES-256-GCM encryption with random salt + nonce per save
- Keys zeroized from memory after use
- Atomic vault writes (tmp → fsync → rename)
