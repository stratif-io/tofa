# tofa cam

Open a browser-based webcam scanner and add the first QR detected.

<!-- BEGIN auto:help -->
**Synopsis**

```
tofa cam [FLAGS]
```

**Flags**

| Flag | Description |
|---|---|
| `--camera <INDEX>` | Camera index passed to the browser (default: 0) |
| `--name <NAME>` | Override the account name (default: derived from QR metadata) |

<!-- END auto:help -->

## Examples

Default camera:

```console
$ tofa cam
Passphrase: ********
Open http://127.0.0.1:54321 in your browser…
Added GitHub:you
Current code: 482 913  (21s)
```

Pick a specific camera (e.g., the second one):

```console
$ tofa cam --camera 1
```

A migration QR captured with the camera expands into all of its
accounts in one shot:

```console
$ tofa cam
...
Imported 3 account(s).
```

## Notes

- The browser scanner runs locally on a random port — nothing leaves your
  machine.
- Grant the browser permission to use the camera when prompted.
- Once a QR decodes, the page closes itself and `tofa` continues with the
  detected account.
- Like every TOFA import surface, `cam` dedups on (name, secret).
  Pointing the camera at a QR you've already imported errors out for
  a single account, or reports `N duplicate(s) skipped` for a
  migration QR — never silently double-adds.

## See also

- **[`tofa scan`](./scan.md)** — screen capture variant (no camera needed).
- **[`tofa add --qr`](./add.md)** — for an image on disk.
