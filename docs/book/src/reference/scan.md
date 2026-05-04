# tofa scan

Capture the entire screen, scan it for QR codes, and add the first account
found.

<!-- BEGIN auto:help -->
**Synopsis**

```
tofa scan [FLAGS]
```

**Flags**

| Flag | Description |
|---|---|
| `--name <NAME>` | Override the account name (default: derived from QR metadata) |

<!-- END auto:help -->

## Examples

Show a TOTP QR somewhere on screen (browser, password manager, etc.), then:

```console
$ tofa scan
Passphrase: ********
✓ added GitHub:you (issuer=GitHub, label=you)
```

Override the account name:

```console
$ tofa scan --name "GitHub:work"
Passphrase: ********
✓ added GitHub:work
```

## Notes

- macOS only (uses ScreenCaptureKit). On Linux, save the QR to a file and use
  [`tofa add --qr`](./add.md) instead.
- On first use, macOS prompts for **Screen Recording** permission. Grant it
  in System Settings → Privacy & Security if the scan returns nothing.
- If multiple QRs are visible, the first one decoded wins.

## See also

- **[`tofa cam`](./cam.md)** — webcam-based scanner.
- **[`tofa add`](./add.md)** — `--qr <PATH>` for QR images on disk.
