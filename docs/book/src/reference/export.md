# tofa export

Dump every account in the vault as **plain-text** JSON. Use only for backups
or migrating to another tool — the output contains your secrets unencrypted.

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

## Notes

- The output is **not encrypted**. Treat it like a password file: `chmod 600`,
  store on encrypted media, delete after use.
- Format is stable JSON with a `version` field — safe to keep around for
  later import.

## See also

- **[`tofa import`](./import.md)** — read this format back.
- **[Recipe: import from Aegis / andOTP](../recipes/import-from-aegis.md)** —
  same JSON shape.
