# Quick start

## 1. Create a vault

```bash
tofa init
```

You'll be asked for a passphrase. The vault is created at
`~/Library/Application Support/tofa/vault.enc` on macOS or
`~/.config/tofa/vault.enc` on Linux.

## 2. Add an account

From a base32 secret:

```bash
tofa add --name "GitHub:you" --secret JBSWY3DPEHPK3PXP
```

From an `otpauth://` URI:

```bash
tofa add --uri "otpauth://totp/GitHub:you?secret=JBSWY3DPEHPK3PXP&issuer=GitHub"
```

From a QR image on disk:

```bash
tofa add --qr ~/Downloads/github-qr.png
```

## 3. Get a code

```bash
tofa code GitHub:you
```

Pipe to your clipboard:

```bash
tofa code GitHub:you | pbcopy   # macOS
tofa code GitHub:you | xclip    # Linux
```

## 4. Open the TUI

```bash
tofa
```

Live countdown bars, mouse support, violet accent.
