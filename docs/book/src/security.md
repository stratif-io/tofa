# Security model

## Threat model

`tofa` protects your TOTP secrets from:

- **Disk theft** — the vault is AES-256-GCM-encrypted; without the passphrase
  the file is opaque.
- **Casual snooping** — secrets never appear in plain text on disk during
  normal operation; only `export` produces a plain-text dump (and warns).

`tofa` does **not** protect from:

- A keylogger or process running under your user — it sees the passphrase.
- A malicious binary masquerading as `tofa` — verify your install source.

## Cryptography

| Primitive | Choice |
|---|---|
| Key derivation | Argon2id (m = 64 MiB, t = 3, p = 1) |
| Encryption | AES-256-GCM with a fresh random 96-bit nonce per write |
| Integrity | Built into GCM (16-byte auth tag) |

The salt is stored alongside the ciphertext. Each save uses a fresh nonce, so
re-saving the same vault produces a different ciphertext.

## Atomic writes

The vault is written to a temp file in the same directory and then renamed
into place. A crash mid-write leaves either the old vault or the new vault
intact — never a half-written file.

## Memory hygiene

The decrypted passphrase is held in a `Zeroizing` buffer with a 10-minute TTL.
On lock (manual, timeout, or process exit) the buffer is zeroed.

## Unsigned build

The macOS app is **not yet notarized** by Apple. Releases are built and
distributed unsigned, which means macOS Gatekeeper will quarantine the app on
first launch and refuse to open it.

To allow it through, run once after install:

```bash
xattr -dr com.apple.quarantine /Applications/tofa.app
```

This removes the quarantine flag that macOS adds to apps downloaded from the
internet. Alternatively, right-click the app in Finder, choose **Open**, and
confirm the dialog — same effect.

Notarization is on the roadmap. Until then, you can verify the build by
inspecting the source on
[GitHub](https://github.com/stratif-io/tofa) and running
`cargo tauri build` locally; the binary you produce is byte-for-byte
reproducible from a given commit on the same toolchain.

## Reporting a vulnerability

Open a private security advisory at
<https://github.com/stratif-io/tofa/security/advisories/new>.
