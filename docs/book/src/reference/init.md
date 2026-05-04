# tofa init

Create a new encrypted vault. Run this once before any other command.

<!-- BEGIN auto:help -->
**Synopsis**

```
tofa init
```

<!-- END auto:help -->

## Examples

Initialize the default vault:

```console
$ tofa init
Choose a passphrase: ********
Confirm passphrase: ********
✓ vault created at /Users/you/Library/Application Support/tofa/vault.enc
```

Use a custom path:

```console
$ tofa --vault ~/secrets/work-vault.enc init
```

Or via env var:

```console
$ TOFA_VAULT=~/secrets/work-vault.enc tofa init
```

## Notes

- Exit code `0` on success.
- Refuses to overwrite an existing vault file. Move or delete it first if you
  really want to start over (consider [`destroy`](./destroy.md) for that).
- The passphrase is asked twice and must match. There is no recovery — losing
  the passphrase makes the vault unreadable forever.

## See also

- **[`tofa rekey`](./rekey.md)** — change the passphrase later.
- **[`tofa destroy`](./destroy.md)** — wipe the vault.
- **[Vault & passphrase](../getting-started/vault.md)** — broader explanation.
