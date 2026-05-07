# tofa export

Dump every account in the vault as **plain text** — either re-importable
JSON (the default) or a list of `otpauth://` URIs. Both formats contain
your secrets unencrypted. Use only for backups or migration.

<!-- BEGIN auto:help -->
**Synopsis**

```
tofa export [FLAGS]
```

**Flags**

| Flag | Description |
|---|---|
| `--output <PATH>` | Write to a file instead of stdout |
| `--format <FORMAT>` | Output format. Defaults to `json` for backwards compatibility |

<!-- END auto:help -->

## Examples

To stdout:

```console
$ tofa export
Passphrase: ********
{
  "version": 1,
  "entries": [
    { "id": "GitHub:you", "name": "GitHub:you", "secret": "JBSWY3DPEHPK3PXP", "issuer": "GitHub", ... }
  ]
}
```

To a file (recommended — set restrictive permissions immediately):

```console
$ tofa export --output ~/tofa-backup.json
Passphrase: ********
✓ wrote ~/tofa-backup.json (3 accounts)
$ chmod 600 ~/tofa-backup.json
```

Pipe into another tool (e.g., `jq`):

```bash
tofa export | jq '.entries | length'
```

Export as a plain-text URI list (one `otpauth://` per line). The
default output filename becomes `tofa-export-<date>.txt`:

```console
$ tofa export --format uris
Passphrase: ********
3 account(s) exported to tofa-export-2026-05-07.txt
```

That file round-trips back through [`tofa import`](./import.md) — and
also through the desktop app's drop / picker and the TUI's file
picker, so you can hand it to a teammate (over a secure channel) and
they can import in one step.

## Notes

- The output is **not encrypted**. Treat it like a password file: `chmod 600`,
  store on encrypted media, delete after use.
- JSON output has a `version` field and is the safest long-term backup
  shape — it carries every entry's metadata.
- The URI-list format (`--format uris`) preserves
  period / digits / algorithm too (encoded in each URI's query
  string), but is friendlier for piping or sharing one URI at a time.

## See also

- **[`tofa import`](./import.md)** — reads both JSON and URI-list
  formats.
- **[`tofa code <name> --uri`](./code.md)** — print or copy a single
  entry's `otpauth://` URI without exporting the whole vault.
- **[Recipe: import from Aegis / andOTP](../recipes/import-from-aegis.md)**
  — same JSON shape.
