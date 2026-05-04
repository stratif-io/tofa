# tofa import

Import accounts from a JSON file (Aegis, andOTP, or `tofa export` format) or
from a migration QR image.

<!-- BEGIN auto:help -->
**Synopsis**

```
tofa import
```

<!-- END auto:help -->

## Examples

From a JSON export:

```console
$ tofa import ~/Downloads/aegis-export.json
Passphrase: ********
✓ added GitHub:you
✓ added Discord:you
- skipped Slack:work (already in vault)
imported 2 of 3 entries
```

From a migration QR (Google Authenticator export):

```console
$ tofa import ~/Downloads/migration.png
Passphrase: ********
✓ added GitHub:you
✓ added Discord:you
imported 2 entries
```

## Notes

- Skips entries whose id is already in the vault — re-importing the same file
  is safe.
- Accepts JSON formats from Aegis, andOTP, and `tofa export` interchangeably.
- The source file is **plain text** — delete it after import (`shred -u` on
  Linux, `rm -P` on macOS).

## See also

- **[`tofa export`](./export.md)** — produce a JSON export.
- **[Recipe: import from Aegis / andOTP](../recipes/import-from-aegis.md)** —
  step-by-step migration.
