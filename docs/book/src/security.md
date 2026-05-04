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

## Reporting a vulnerability

Open a private security advisory at
<https://github.com/stratif-io/tofa/security/advisories/new>.
