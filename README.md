# rotp

> Eye-candy terminal OTP manager — encrypted vault, beautiful TUI, full CLI.

```
rotp                     # open the TUI
rotp code github         # get your GitHub code instantly
rotp add --qr ~/qr.png  # scan a QR code
```

---

## Features

- **Encrypted vault** — AES-256-GCM + Argon2id key derivation
- **Beautiful TUI** — violet accent, per-entry progress bars, mouse click to copy
- **Full CLI** — scriptable, pipeable, shell-completion ready
- **QR import/export** — Google Authenticator migration format
- **Zero cloud** — your secrets never leave your machine

---

## Installation

```bash
cargo install rotp
```

Or build from source:

```bash
git clone https://github.com/carlo/rotp
cd rotp
cargo build --release
# binary at target/release/rotp
```

### Shell completions

```bash
rotp completions bash >> ~/.bashrc
rotp completions zsh  > ~/.zfunc/_rotp   # add ~/.zfunc to $fpath
rotp completions fish > ~/.config/fish/completions/rotp.fish
```

---

## Quick start

```bash
# Create your vault
rotp init

# Add accounts
rotp add --uri "otpauth://totp/GitHub:you?secret=YOURSECRET"
rotp add --qr  ~/Downloads/github-qr.png
rotp add --secret JBSWY3DPEHPK3PXP --name "GitHub:you"

# Get a code
rotp code github           # → 480 152
rotp code github --raw     # → 480152  (for scripts)
rotp code github --copy    # copies to clipboard
rotp code github --watch   # live countdown

# Manage accounts
rotp list
rotp list --codes
rotp rename "GitHub:you" "GitHub:work"
rotp remove "GitHub:work"

# Export / backup
rotp qr github                        # display QR in terminal
rotp qr --all --output backup.png     # migration QR for all accounts
rotp export --output backup.json      # plain-text JSON (keep safe!)
rotp import backup.json               # restore from JSON
rotp import migration-qr.png          # import from Google Authenticator

# Vault management
rotp rekey                # change passphrase
rotp destroy              # delete vault
```

---

## Command reference

| Command | Description |
|---|---|
| `rotp` | Launch TUI |
| `rotp init` | Create a new vault |
| `rotp destroy` | Delete the vault |
| `rotp list [--codes]` | List accounts (optionally with codes) |
| `rotp code <name> [--raw\|--copy\|--watch]` | Show current TOTP code |
| `rotp add [--name] [--secret\|--uri\|--qr]` | Add an account |
| `rotp remove <name>` | Remove an account |
| `rotp rename <name> <new>` | Rename an account |
| `rotp qr <name\|--all> [--output]` | Export QR code |
| `rotp rekey` | Change passphrase |
| `rotp completions <bash\|zsh\|fish>` | Print shell completions |
| `rotp export [--output]` | Export vault as plain-text JSON |
| `rotp import <file>` | Import from JSON or migration QR |

---

## Vault location

Default: `~/.config/rotp/vault.enc`

Override with `--vault <path>` or `ROTP_VAULT` environment variable.

---

## Environment variables

| Variable | Description |
|---|---|
| `ROTP_VAULT` | Override vault path |
| `ROTP_PASSPHRASE` | Passphrase for non-interactive use (shows a warning) |
| `ROTP_NEW_PASSPHRASE` | New passphrase for `rotp rekey` (non-interactive) |

---

## Licence

MIT
