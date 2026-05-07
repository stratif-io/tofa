# CLI reference

Every `tofa` subcommand documented in one place. Auto-generated synopsis and
flags are kept in sync with the source by an `xtask` tool — they cannot drift.

## Global options

These work on every subcommand:

| Flag | Description |
|---|---|
| `--vault <PATH>` | Override the vault path. Reads from `TOFA_VAULT` env var as fallback. |
| `--help`, `-h` | Print help. |
| `--version`, `-V` | Print the version. |

## Environment variables

| Variable | Effect |
|---|---|
| `TOFA_VAULT` | Default vault path when `--vault` is not given. |
| `TOFA_PASSPHRASE` | If set, used instead of an interactive prompt. **Avoid in production** — `tofa` prints a warning to stderr when it reads this. |

## Commands

| Command | Purpose |
|---|---|
| [`init`](./init.md) | Create a new encrypted vault. |
| [`list`](./list.md) | List every account. |
| [`code`](./code.md) | Print the current TOTP code for one account. |
| [`add`](./add.md) | Add an account from secret, URI, or QR. |
| [`remove`](./remove.md) | Remove an account. |
| [`rename`](./rename.md) | Rename an account. |
| [`qr`](./qr.md) | Print a QR code for one account or all. |
| [`rekey`](./rekey.md) | Change the vault passphrase. |
| [`export`](./export.md) | Dump every account as JSON or as a list of `otpauth://` URIs. |
| [`import`](./import.md) | Import from any supported file: single- or multi-QR images, migration QRs, JSON / CSV / TXT exports from other authenticators, or a zip archive of any of the above. |
| [`scan`](./scan.md) | Capture every connected display and import every QR visible on screen. |
| [`cam`](./cam.md) | Open the webcam and wait for a QR. |
| [`completions`](./completions.md) | Print shell completions. |
| [`destroy`](./destroy.md) | Permanently delete the vault. |
