# tofa rename

Rename an account. Takes the current id and the new id as two positional
arguments.

<!-- BEGIN auto:help -->
**Synopsis**

```
tofa rename
```

<!-- END auto:help -->

## Examples

```console
$ tofa rename GitHub:you GitHub:me
Passphrase: ********
✓ renamed GitHub:you → GitHub:me
```

## Notes

- Refuses if the target id already exists. Remove the conflicting entry first
  if that's what you want.
- The underlying TOTP secret is unchanged — you keep the same codes.

## See also

- **[`tofa list`](./list.md)** — confirm the new id is what you expected.
- **[`tofa remove`](./remove.md)** — if you don't need the entry anymore.
