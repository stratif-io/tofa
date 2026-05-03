# Multi-Format OTP Import + Origin Tracking — Design

## Goal

Support importing TOTP accounts from every major authenticator app, track where each account came from (app, file, date), and surface that origin in the TUI detail screen and Tauri detail modal.

## Scope

- New parsers: Aegis, 2FAS, Raivo, Bitwarden, 1Password (.1pux + CSV), Apple Passwords (CSV)
- Origin tracking for all existing entry points: QR screen scan, camera scan, image file, URI/manual add
- `ImportSource` struct added to `VaultEntry` (backwards-compatible)
- CLI auto-detect by file extension + content sniffing
- TUI detail screen: new "Source" row
- Tauri app: new `import_file` command (bytes + filename); detail modal: new source row
- Test fixtures for every format under `tofa-core/tests/fixtures/imports/`

Out of scope: Microsoft Authenticator (proprietary encrypted format, no public spec).

---

## Data Model

### `ImportSource` (new, in `tofa-core/src/store.rs`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSource {
    pub app: String,         // "Aegis", "1Password", "Google Authenticator", "QR Scan", "Manual", …
    pub file: String,        // filename, "screen capture", "camera", or "" for manual
    pub imported_at: String, // "YYYY-MM-DD"
}
```

### `VaultEntry` — new optional field

```rust
#[serde(default, skip_serializing_if = "Option::is_none")]
pub source: Option<ImportSource>,
```

`skip_serializing_if` keeps existing vault files unchanged. Old entries deserialise with `source: None`.

### Source values by entry point

| Entry point | `app` | `file` |
|---|---|---|
| Screen QR scan — Google Auth migration | `"Google Authenticator"` | `"screen capture"` |
| Screen QR scan — plain otpauth:// | `"QR Scan"` | `"screen capture"` |
| Camera — Google Auth migration | `"Google Authenticator"` | `"camera"` |
| Camera — plain otpauth:// | `"QR Scan"` | `"camera"` |
| Image file — Google Auth migration | `"Google Authenticator"` | filename |
| Image file — plain otpauth:// | `"QR Scan"` | filename |
| Aegis JSON import | `"Aegis"` | filename |
| 2FAS import | `"2FAS"` | filename |
| Raivo ZIP import | `"Raivo"` | filename |
| Bitwarden JSON import | `"Bitwarden"` | filename |
| 1Password .1pux import | `"1Password"` | filename |
| 1Password CSV import | `"1Password"` | filename |
| Apple Passwords CSV import | `"Apple Passwords"` | filename |
| Manual URI / Base32 | `"Manual"` | `""` |

---

## Architecture

### `tofa-core/src/import/`  (new module)

```
import/
  mod.rs          — ImportFormat enum, detect(path, bytes) -> ImportFormat, parse(path, bytes, filename) -> Vec<OtpSecret>
  aegis.rs        — plain JSON { version, entries[] } or encrypted (password = "test" in fixtures)
  two_fas.rs      — .2fas: JSON { services: [{ otp: { account, issuer, secret, tokenType, period, digits, algorithm } }] }
  raivo.rs        — ZIP containing accounts.json: [{ issuer, account, secret, algorithm, digits, timer }]
  bitwarden.rs    — JSON { items: [{ login: { totp: "otpauth://…" or raw secret } }] }
  one_password.rs — .1pux: ZIP containing export.data JSON; CSV: columns include "one-time password"
  apple.rs        — CSV columns: Title, Username, Password, OTPAuth, URL, Notes
```

`OtpSecret` gains a `source_app: String` field so each parser stamps its own name.

`detect()` rules (in order):
1. Extension `.2fas` → TwoFas
2. Extension `.1pux` → OnePasswordPux
3. Extension `.zip` → Raivo (check ZIP contains `accounts.json`)
4. Extension `.csv` → sniff header row: `OTPAuth` → Apple; `one-time password` → OnePasswordCsv
5. Extension `.json` → sniff top-level keys: `db` → AegisEncrypted; `entries` → AegisPlain; `items` → Bitwarden; `services` → TwoFas fallback
6. Unknown / unrecognised → error "Unsupported import format"

### Changes to existing code

- `tofa-core/src/qr.rs` — `parse_migration` and `parse_input` stay unchanged; callers pass `source_app` when building `VaultEntry`
- `tofa-core/src/lib.rs` — re-export `import::parse` and `import::detect`
- `tofa/src/cli/commands/import.rs` — call `import::detect` + `import::parse`; pass `filename` and today's date when building `VaultEntry`
- `tofa/src/tui/screens/otp_detail.rs` — add "Source" row when `entry.source.is_some()`
- `tofa-app/src-tauri/src/commands.rs`:
  - `scan_screen` / `scan_image_bytes` / `add_from_uri` — propagate `ImportSource` when building `VaultEntry`
  - new `import_file(bytes: Vec<u8>, filename: String)` command — calls `import::parse`, stamps source
- `tofa-app/src/main.js` — `handleImportFile(file)` reads bytes, calls `import_file`; detail modal shows source row
- `tofa-app/src/index.html` — detail modal: source row

---

## Test Fixtures

All secrets use `JBSWY3DPEHPK3PXP` (RFC test vector) or `JBSWY3DPEHPK3PXQ`.

```
tofa-core/tests/fixtures/imports/
  aegis_plain.json          — 2 TOTP entries, unencrypted
  aegis_encrypted.json      — 2 TOTP entries, password-encrypted (password: "test")
  two_fas.2fas              — 2 entries
  raivo.zip                 — ZIP with accounts.json, 2 entries
  bitwarden.json            — vault export with 2 TOTP items
  one_password.1pux         — 1pux ZIP with 2 TOTP items
  one_password.csv          — CSV with 2 TOTP rows
  apple_passwords.csv       — CSV with 2 TOTP rows
```

Each fixture is generated by a `gen_import_fixtures.rs` script (similar to existing `gen_fixtures.rs`).

---

## Testing

One test file per parser in `tofa-core/tests/`:

```
import_aegis.rs
import_two_fas.rs
import_raivo.rs
import_bitwarden.rs
import_one_password.rs
import_apple.rs
```

Each test asserts:
- Correct number of entries parsed
- `secret` matches expected Base32
- `meta.issuer` and `meta.account` populated correctly
- `meta.algorithm`, `meta.digits`, `meta.period` match fixture values
- `source_app` equals the expected app name string

Plus one `import_detect.rs` test that verifies `detect()` returns the right format for each fixture file.

CLI integration tests in `tofa/tests/cli_import_formats.rs` cover round-trip: import fixture → list → verify entry present with correct source.

---

## UI Changes

### TUI — `otp_detail.rs`

New row below the algorithm line:

```
Source   Google Authenticator · screen capture · 2026-05-03
```

Hidden when `entry.source.is_none()` (manually added entries). Modal height grows by 1 row when source is present.

### Tauri app — detail modal (`index.html` / `main.js`)

New row in the detail modal:

```
Source   Aegis · aegis-export.json · 2026-05-03
```

Hidden via `display:none` when source is absent.

### Tauri app — new `import_file` command

```rust
pub async fn import_file(
    bytes: Vec<u8>,
    filename: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<String>, String>
```

Called from JS when user drags or selects a non-image file. Auto-detects format from filename + bytes. Returns list of added entry names. Existing `scan_image_bytes` remains unchanged for image files; `handleImageOrImport(file)` in JS routes by MIME type / extension.

---

## Backwards Compatibility

- Existing vault files with no `source` field deserialise cleanly (`Option::None`).
- No vault migration needed.
- Existing CLI `tofa import file.json` continues to work — `detect()` will recognise the existing tofa JSON format as a new `ImportFormat::TofaJson` variant and handle it as before.
