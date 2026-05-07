# tofa code

Print the current TOTP code for one account. Takes the account id (or a
prefix) as the first positional argument.

<!-- BEGIN auto:help -->
**Synopsis**

```
tofa code [FLAGS]
```

**Flags**

| Flag | Description |
|---|---|
| `--raw <RAW>` | Output bare digits without space (for scripting) |
| `--copy <COPY>` | Copy to clipboard (the code by default; the otpauth:// URI when --uri is set) |
| `--watch <WATCH>` | Refresh every second until Ctrl+C |
| `--uri <URI>` | Print/copy the entry's `otpauth://` URI instead of the current code. Useful for moving an account to another authenticator app or piping into `tofa add --uri` |

<!-- END auto:help -->

## Examples

Print the code (with the conventional space):

```console
$ tofa code GitHub:you
Passphrase: ********
482 913   (21s left)
```

Bare digits for scripts:

```console
$ tofa code GitHub:you --raw
Passphrase: ********
482913
```

Copy to clipboard (uses your platform's clipboard handler):

```console
$ tofa code GitHub:you --copy
Passphrase: ********
✓ copied
```

Live-watch a code (handy when typing it elsewhere):

```console
$ tofa code GitHub:you --watch
482 913   (21s left)
482 913   (20s left)
...
```

Pipe to your own clipboard tool if `--copy` doesn't fit:

```bash
tofa code GitHub:you --raw | pbcopy   # macOS
tofa code GitHub:you --raw | xclip    # Linux
```

Print or copy the entry's `otpauth://` URI instead of the current
code. Useful for moving one account to another authenticator without
exporting the whole vault:

```console
$ tofa code GitHub:you --uri
otpauth://totp/GitHub%3Ayou?secret=JBSWY3DPEHPK3PXP&issuer=GitHub&algorithm=SHA1&digits=6&period=30

$ tofa code GitHub:you --uri --copy
otpauth://totp/GitHub%3Ayou?secret=JBSWY3DPEHPK3PXP&issuer=GitHub&algorithm=SHA1&digits=6&period=30
✓ copied
```

The URI carries every parameter (period / digits / algorithm), so the
receiving authenticator gets an exact copy of the entry.

## Notes

- The first argument is the account **id or name** — partial matches work.
  If a prefix is ambiguous, `tofa` lists the candidates and exits non-zero.
- `--raw`, `--copy`, `--watch` are all switches; the `<RAW>` etc. placeholders
  in the auto-generated table above are a clap quirk for booleans.
- Exit code `1` on missing account, wrong passphrase, or ambiguous prefix.

## See also

- **[`tofa list`](./list.md)** — see all accounts at once.
- **[`tofa export --format uris`](./export.md)** — bulk-emit every
  entry as `otpauth://` URIs in one file.
- **[Recipe: clipboard](../recipes/clipboard.md)** — copy patterns by platform.
