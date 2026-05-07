# Importing from Aegis or andOTP

`tofa import` reads the JSON export format used by Aegis and andOTP — and
also 2FAS, Bitwarden, FreeOTP+, Raivo, KeePassXC CSV, Ente Auth's plain
text URI list, single- and multi-QR images, Google Authenticator
migration QRs, and zip archives mixing any of the above. See the
[`tofa import`](../reference/import.md) reference for the full list.

## Aegis (Android)

1. In Aegis: **Settings → Import & Export → Export → Plain text**.
2. Transfer the file to your machine (`scp`, AirDrop, USB).
3. Import:

```bash
tofa import ~/Downloads/aegis-export.json
```

The file is plain text — delete it after a successful import:

```bash
shred -u ~/Downloads/aegis-export.json   # Linux
rm -P ~/Downloads/aegis-export.json      # macOS
```

## andOTP

Use **Backups → Plain-text → Backup**. Same import command. Same caveat about
the export being plain text.

## Migration QR codes

Google Authenticator exports as a QR-encoded URL (`otpauth-migration://`).
Three ways to bring it in:

```bash
tofa scan                                  # capture every screen, decode all QRs
tofa import ~/Downloads/migration.png      # if you saved the QR to disk
tofa add --qr ~/Downloads/migration.png    # equivalent, single-account flow
```

`tofa scan` is the fastest path when the QR is on screen — it picks up
multi-account migration QRs and printout grids in one pass.

## Backup printouts (multi-QR images)

If you exported your vault from another tool as a printable page of
QRs, pass the PNG / JPG straight to `tofa import` — the dispatcher
finds and imports every QR on the page in one shot. This is also how
the desktop app's **Save All** zip round-trips back into a vault.

```bash
tofa import ~/Downloads/backup-printout.png
```
