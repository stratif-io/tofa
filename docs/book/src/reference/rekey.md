# tofa rekey

Change the vault passphrase. Asks for the current passphrase, then the new
one twice.

<!-- BEGIN auto:help -->
**Synopsis**

```
tofa rekey
```

<!-- END auto:help -->

## Examples

```console
$ tofa rekey
Current passphrase: ********
New passphrase: ************
Confirm new passphrase: ************
✓ vault re-encrypted
```

## Notes

- The vault is rewritten atomically: temp file then rename. If anything goes
  wrong mid-write, the old vault is intact.
- A fresh nonce and a fresh KDF salt are generated, so the new ciphertext
  is unrelated to the old one even if accounts are unchanged.
- There is no "set a new passphrase without knowing the old one" — that would
  defeat the encryption.

## See also

- **[Vault & passphrase](../getting-started/vault.md)** — broader explanation.
- **[Security model](../security.md)** — Argon2id and AES-256-GCM details.
