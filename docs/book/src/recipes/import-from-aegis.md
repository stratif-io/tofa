# Importing from Aegis or andOTP

`tofa import` reads the JSON export format used by Aegis and andOTP.

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
Capture the screen showing it and run [`tofa scan`](../reference/scan.md), or
save the QR as a PNG and run `tofa add --qr migration.png`.
