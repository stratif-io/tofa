# tofa remove

Remove an account from the vault. Takes the id (or a prefix) as the first
positional argument and prompts for confirmation.

<!-- BEGIN auto:help -->
**Synopsis**

```
tofa remove
```

<!-- END auto:help -->

## Examples

By exact id:

```console
$ tofa remove GitHub:you
Passphrase: ********
Remove "GitHub:you"? [y/N] y
Removed "GitHub:you".
```

By prefix (works when only one entry matches):

```console
$ tofa remove GitHub
Passphrase: ********
Remove "GitHub:you"? [y/N] y
Removed "GitHub:you".
```

Cancel at the prompt:

```console
$ tofa remove GitHub:you
Passphrase: ********
Remove "GitHub:you"? [y/N] n
Aborted.
```

## Notes

- The prompt always defaults to **No** — pressing Enter alone keeps the entry.
- Ambiguous prefixes (matching multiple entries) print the candidates and exit
  non-zero without modifying anything.
- The vault is rewritten atomically.

## See also

- **[`tofa list`](./list.md)** — find the right id first.
- **[`tofa rename`](./rename.md)** — keep the entry but rename it.
