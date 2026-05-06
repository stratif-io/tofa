# tofa qr

Print a QR code for one account in the terminal, or export QRs for one or
all accounts to disk.

<!-- BEGIN auto:help -->
**Synopsis**

```
tofa qr [FLAGS]
```

**Flags**

| Flag | Description |
|---|---|
| `--all <ALL>` | Export all accounts as a migration QR |
| `--multi <MULTI>` | Emit one otpauth:// QR per entry instead of a single migration QR. Requires `--all` and `--output-dir`. Preserves period/algorithm/digits for every entry — use this when the migration format would refuse because the selection mixes 30s and non-30s entries |
| `--output <PATH>` | Save QR as PNG instead of displaying in terminal (single-QR modes) |
| `--output-dir <DIR>` | Directory to write per-entry PNGs into when using `--multi` |

<!-- END auto:help -->

## Examples

Print one account's QR in the terminal (scannable from a phone camera held
up to the screen):

```console
$ tofa qr GitHub:you
Passphrase: ********
█▀▀▀▀▀█ ▀▀█▀▀ █▀▀▀▀▀█
█ ███ █  ▀ ▀▀ █ ███ █
█ ▀▀▀ █ ▀█▀▄  █ ▀▀▀ █
▀▀▀▀▀▀▀ ▀ ▀▀▀ ▀▀▀▀▀▀▀
...
```

Save as a PNG instead of printing:

```console
$ tofa qr GitHub:you --output github-you.png
Passphrase: ********
✓ wrote github-you.png
```

Export every account as a single migration QR (Google Authenticator format —
scan it with the Authenticator app to import everything at once):

```console
$ tofa qr --all --output migration.png
Passphrase: ********
✓ wrote migration.png (3 accounts)
```

## Notes

- `--all` is a switch; the `<ALL>` placeholder above is a clap quirk for bool
  flags.
- Terminal QRs use Unicode block characters and need a font with full block
  support (most modern fonts do).
- The migration format is the same one Google Authenticator uses — any reader
  that handles `otpauth-migration://` will accept the result.

## See also

- **[`tofa export`](./export.md)** — JSON dump for offline backups.
- **[`tofa add`](./add.md)** with `--qr` — the inverse: read a QR image.
