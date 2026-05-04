# Vault & passphrase

The vault is a single AES-256-GCM-encrypted file containing every account's
secret. The passphrase you set during `tofa init` is the only key — losing it
makes the vault unrecoverable.

## Where it lives

| OS | Default path |
|---|---|
| macOS | `~/Library/Application Support/tofa/vault.enc` |
| Linux | `~/.config/tofa/vault.enc` |

Override with the `--vault <PATH>` flag (works on every command) or the
`TOFA_VAULT` env var.

## Passphrase cache

Once you unlock the vault in the TUI or app, the passphrase is held in memory
for **10 minutes** of inactivity, then zeroed. Each CLI invocation prompts
fresh — there is no daemon.

## Changing the passphrase

```bash
tofa rekey
```

You'll be asked for the current and new passphrases. The vault is rewritten
atomically.

## Destroying the vault

```bash
tofa destroy
```

Permanently deletes the vault file. There is no undo.

## See also

- **[`tofa init`](../reference/init.md)** · **[`tofa rekey`](../reference/rekey.md)** · **[`tofa destroy`](../reference/destroy.md)**
- **[Security model](../security.md)** for the cryptographic details.
