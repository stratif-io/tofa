# tofa add

Add a new account to the vault. Accepts a base32 `--secret`, an `otpauth://`
URI, or a path to a QR image.

<!-- BEGIN auto:help -->
**Synopsis**

```
tofa add [FLAGS]
```

**Flags**

| Flag | Description |
|---|---|
| `--name <NAME>` | Account name (required when using --secret) |
| `--secret <BASE32>` | Base32-encoded TOTP secret |
| `--uri <URI>` | otpauth:// URI |
| `--qr <PATH>` | Path to a QR code image |

<!-- END auto:help -->

## Examples

From a base32 secret:

```console
$ tofa add --name "GitHub:you" --secret JBSWY3DPEHPK3PXP
Passphrase: ********
✓ added GitHub:you
```

From an `otpauth://` URI (issuer and label parsed automatically):

```console
$ tofa add --uri "otpauth://totp/GitHub:you?secret=JBSWY3DPEHPK3PXP&issuer=GitHub"
Passphrase: ********
✓ added GitHub:you
```

From a QR image:

```console
$ tofa add --qr ~/Downloads/github-qr.png
Passphrase: ********
✓ added GitHub:you (issuer=GitHub, label=you)
```

A migration QR (Google Authenticator export, contains many accounts) imports
all of them at once:

```console
$ tofa add --qr ~/Downloads/migration.png
Passphrase: ********
Imported 3 account(s).
```

If the migration QR includes accounts you've already imported, the
duplicates are skipped silently and the count reflects only the new
ones:

```console
$ tofa add --qr ~/Downloads/migration.png
Passphrase: ********
Imported 1 account(s) (2 duplicate(s) skipped).
```

Re-adding an account that's already in the vault errors out instead
of creating a duplicate row:

```console
$ tofa add --uri "otpauth://totp/GitHub:you?secret=JBSWY3DPEHPK3PXP&issuer=GitHub"
Passphrase: ********
Error: "GitHub:you" is already in the vault.
```

## Notes

- Exactly one of `--secret`, `--uri`, or `--qr` is required.
- `--secret` requires `--name`; `--uri` and `--qr` derive the name themselves
  (override with `--name`).
- An entry is a duplicate when its **name** *and* **secret** both match a
  row already in the vault. Rotating the secret (same name, new secret)
  or filing the same secret under a second name still goes through.
  This is the same rule the TUI, desktop app, and `tofa import` use.
- The vault is rewritten atomically: a temp file is written and then renamed,
  so the old vault is never partially overwritten.
- Exit code `0` on success, non-zero on parse errors, wrong passphrase, or
  an attempt to re-add an exact duplicate.

## See also

- **[`tofa scan`](./scan.md)** — capture the screen instead of supplying a file.
- **[`tofa cam`](./cam.md)** — webcam capture.
- **[`tofa import`](./import.md)** — bulk import from a JSON export.
