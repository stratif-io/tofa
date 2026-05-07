# tofa scan

Capture every connected display and import every QR code visible —
single accounts, Google Authenticator migration QRs, or a printout
showing many at once. A spinner reports per-pass progress while the
scanner runs.

> **Experimental.** Real-world Retina captures of dense backup
> printouts can occasionally miss a QR that lands at the edge of the
> detector's threshold (typically 0–1 out of ~10). If a code is
> missing, rerun the scan or import that one separately via
> `tofa import <file>`. The CLI prints this caveat on every run.

<!-- BEGIN auto:help -->
**Synopsis**

```
tofa scan [FLAGS]
```

**Flags**

| Flag | Description |
|---|---|
| `--name <NAME>` | Override the account name (only applied when exactly one entry is found) |

<!-- END auto:help -->

## Examples

Show a TOTP QR somewhere on screen (browser, password manager, etc.), then:

```console
$ tofa scan
⚠  Experimental — screen scan may miss QR codes at the edge of rqrr's
   detection threshold. If a code is missing, rerun the scan or import
   that one separately.
⠹ screen 1/2 • pass @ 3840px • 7 found
Passphrase: ********
Imported 11 account(s) from 2 screen(s).
```

Override the account name (only applied when the scan yields exactly
one entry):

```console
$ tofa scan --name "GitHub:work"
Passphrase: ********
Imported 1 account(s) from 1 screen(s).
```

## How it works

- **Capture.** macOS uses `screencapture -D N` per display (one PNG
  per monitor). Wayland uses `grim`; X11 uses `scrot -m` and falls
  back to `gnome-screenshot`. The CLI captures every connected
  display; multi-monitor setups are first-class.
- **Decode.** Each capture is run through a small resolution ladder
  (native ≤3840 → 1920 with both Lanczos3 and Triangle filters → 1280
  → 960). Filter diversity helps marginal QRs decode that would
  otherwise sit just below the detector's threshold. Early termination
  stops once two consecutive passes find nothing new.
- **Progress.** A stderr spinner shows the current screen, current
  rescale width, and running count of decoded URIs.

## Notes

- On first macOS run, the system prompts for **Screen Recording**
  permission. Grant it in System Settings → Privacy & Security if the
  scan returns nothing.
- Linux requires one of `grim` (Wayland), `scrot` (X11), or
  `gnome-screenshot` (single-display fallback) on `$PATH`.
- All visible QRs are imported. Migration QRs are expanded into their
  constituent accounts; vCards / URLs / other QR payloads are silently
  ignored.

## See also

- **[`tofa cam`](./cam.md)** — webcam-based scanner.
- **[`tofa import`](./import.md)** — same dispatcher, but for files
  (single-QR, multi-QR, zip, JSON, …).
- **[`tofa add`](./add.md)** — `--qr <PATH>` for a single QR image on
  disk.
