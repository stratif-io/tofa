# tofa list

List every account in the vault, sorted by id. Pass `--codes` to also print
the current TOTP and time remaining.

<!-- BEGIN auto:help -->
**Synopsis**

```
tofa list [FLAGS]
```

**Flags**

| Flag | Description |
|---|---|
| `--codes <CODES>` | Also display current codes and time remaining |

<!-- END auto:help -->

## Examples

Names only (fastest):

```console
$ tofa list
Passphrase: ********
GitHub:you
Discord:you
Slack:work
```

With live codes and countdown:

```console
$ tofa list --codes
Passphrase: ********
┌─────────────────────────────────────┬──────────┬─────────┐
│ name                                │ code     │ expires │
├─────────────────────────────────────┼──────────┼─────────┤
│ GitHub:you                          │ 482 913  │ 21s     │
│ Discord:you                         │ 705 224  │ 21s     │
│ Slack:work                          │ 169 380  │ 21s     │
└─────────────────────────────────────┴──────────┴─────────┘
```

## Notes

- Output is sorted alphabetically by id.
- `--codes` is a switch (no value); the `<CODES>` placeholder in the
  auto-generated table above is a clap quirk for boolean flags. Just write
  `--codes`.
- For a single account in a script, prefer [`tofa code`](./code.md).

## See also

- **[`tofa code`](./code.md)** — single account.
- **[`tofa add`](./add.md)** — add a new entry.
