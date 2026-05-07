# tofa import

Import accounts from any file format other authenticators (and TOFA itself)
emit. One unified dispatcher handles every shape: single- and multi-QR
images, Google Authenticator migration QRs, JSON / CSV / TXT exports from
the major mobile and desktop authenticators, and zip archives mixing any
of the above.

<!-- BEGIN auto:help -->
**Synopsis**

```
tofa import
```

<!-- END auto:help -->

## Supported formats

| Source                                  | Extension(s)            |
|-----------------------------------------|-------------------------|
| Single-QR image                         | `.png` `.jpg` `.gif` `.bmp` `.webp` `.tiff` |
| Multi-QR image (e.g. backup printout)   | same as above           |
| Google Authenticator migration QR       | same as above (or pasted as text) |
| Aegis / andOTP / 2FAS / Bitwarden / FreeOTP+ / Raivo / native `tofa export` | `.json` `.2fas` |
| KeePassXC CSV                           | `.csv`                  |
| Ente Auth plain-text URI list           | `.txt`                  |
| Zip archive of any of the above (recursive) | `.zip`              |

The dispatch is by extension, then by content where the extension is
ambiguous. A `.txt` file containing a single `otpauth-migration://` URI
is expanded into every account it carries.

## Examples

Aegis or andOTP JSON export:

```console
$ tofa import ~/Downloads/aegis-export.json
Passphrase: ********
Imported 12 account(s).
```

A printout (or screenshot) showing many QRs at once:

```console
$ tofa import ~/Downloads/backup-printout.png
Passphrase: ********
Imported 11 account(s).
```

A Google Authenticator export QR:

```console
$ tofa import ~/Downloads/migration.png
Passphrase: ********
Imported 8 account(s).
```

A zip from `tofa-app`'s **Save All** button — round-trips your backup
without manually unzipping:

```console
$ tofa import ~/Downloads/tofa-qrs.zip
Passphrase: ********
Imported 12 account(s).
```

A plain-text list of `otpauth://` URIs (Ente Auth's export format, or
the output of `tofa export --format uris`):

```console
$ tofa import ~/Downloads/tofa-export-2026-05-07.txt
Passphrase: ********
Imported 5 account(s).
```

## Notes

- Skips entries that already exist in the vault (matched on name +
  secret). Re-importing the same file is safe and reports the skip
  count.
- Zip extraction is in-memory: bytes are never written to disk during
  import, so slip-path attacks aren't a concern.
- The source file is **plain text** — delete it after import
  (`shred -u` on Linux, `rm -P` on macOS).

## See also

- **[`tofa export`](./export.md)** — produces JSON or a `.txt` URI
  list, both of which `import` reads back.
- **[`tofa scan`](./scan.md)** — capture screens directly without
  saving QRs to disk first.
- **[Recipe: import from Aegis / andOTP](../recipes/import-from-aegis.md)**
  — step-by-step migration.
