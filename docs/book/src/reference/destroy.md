# tofa destroy

Permanently delete the vault file. **There is no undo.**

<!-- BEGIN auto:help -->
**Synopsis**

```
tofa destroy
```

<!-- END auto:help -->

## Examples

```console
$ tofa destroy
This will permanently delete /Users/you/Library/Application Support/tofa/vault.enc.
Type "destroy" to confirm: destroy
✓ vault destroyed
```

Anything other than the literal word `destroy` aborts:

```console
$ tofa destroy
Type "destroy" to confirm: yes
Aborted.
```

## Notes

- Irreversible. There is no encrypted backup, no soft-delete, no trash.
- The file is removed with a normal `unlink`. If you want to wipe the
  underlying blocks too, run `shred -u` (Linux) or `rm -P` (macOS) on the
  vault path before calling `destroy`.

## See also

- **[`tofa init`](./init.md)** — start fresh after destroy.
- **[`tofa rekey`](./rekey.md)** — change passphrase without losing data.
