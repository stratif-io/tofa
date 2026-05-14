# Tofa — Network Egress Audit

**Audit date:** 2026-05-14
**Audited revision:** `main` @ `db1ae90`
**Fixes applied on:** `fix/security-audit` (this branch — FINDINGS 1 and 2 resolved)
**Scope:** Every code path in the workspace that could send data off the device, with a focus on OTP secrets, QR payloads, and vault contents.

## The claim being audited

> Tofa's OTP secrets, QR contents, and vault entries never leave the device. The only outbound network traffic is the Tauri auto-updater checking GitHub for new releases.

This document enumerates the egress surface, identifies what does and does not cross the network, and lists the findings that contradict or weaken the claim.

## TL;DR

| Surface | Verdict |
|---|---|
| `tofa-core` (vault, crypto, QR, TOTP) | ✅ Zero network deps. Pure local logic. |
| `tofa` CLI | ✅ Zero HTTP clients. Localhost-only TCP listener for `cam`. |
| `tofa-app` Rust backend | ⚠️ Only egress is `tauri-plugin-updater` → public GitHub release URL. No vault data in request. |
| `tofa-app` webview (UI) | ✅ jsQR vendored locally (FINDING 1 resolved). Fonts self-hosted (FINDING 2 resolved). |
| Telemetry / analytics / crash reporting | ✅ None present. |

**Net assessment (post-fix):** on this branch, the OTP secret bytes themselves do not leave the device, **and** the webview no longer pulls any third-party assets at runtime. The only remaining outbound traffic is the Tauri updater's check against the public GitHub release URL, which carries no vault data. The three defense-in-depth findings (CSP, redacted `Debug`, macOS entitlements) remain open and are tracked below.

## Methodology

Three independent passes:

1. **Dependency tree** — `cargo tree -e=normal -p <crate>` per workspace member, grepped for known HTTP/socket crates and telemetry SDKs.
2. **Source grep** — every reference to `reqwest`, `ureq`, `hyper`, `TcpStream`, `TcpListener`, `std::net`, `fetch(`, `XMLHttpRequest`, `WebSocket`, `EventSource`, `sendBeacon`, external `http(s)://` URLs in strings, `<script src>` and `<link href>` with external hosts.
3. **Configuration** — `tauri.conf.json`, `capabilities/*.json`, `Info.plist`, `*.entitlements`.

Commands used are listed in [§ Reproducing the audit](#reproducing-the-audit) at the bottom so anyone can re-run them.

## Egress surface, enumerated

### 1. `tofa-core` — clean

`cargo tree -e=normal -p tofa-core` contains no `reqwest`, `ureq`, `hyper`, `surf`, `isahc`, `attohttpc`, websocket, telemetry, or crash-reporting crate. The library is pure local logic: argon2 for KDF, AES-GCM for vault encryption, base32 for TOTP, image/zip/etc. for QR decoding.

### 2. `tofa` CLI — clean (with one localhost socket)

No HTTP clients in the dep tree. One socket usage:

- **`tofa/src/cli/commands/cam.rs:31`** — `TcpListener::bind("127.0.0.1:0")`. This is a **localhost-only** mini-HTTP server used by the `cam` subcommand to receive a decoded QR URI from a browser tab on the same machine. The browser POSTs to `http://127.0.0.1:<port>/result`. The kernel will refuse any non-loopback connection.

### 3. `tofa-app` Rust backend — one update channel, one localhost socket

#### 3a. Tauri auto-updater

- Plugin: `tauri-plugin-updater` 2.10.1.
- Endpoint (in `tofa-app/src-tauri/tauri.conf.json`): `https://github.com/stratif-io/tofa/releases/latest/download/latest.json` — a static, public GitHub release artifact.
- Request method: HTTP GET. **No body, no per-user fields.**
- The response is a JSON manifest; if a newer version is announced, the updater downloads the `.app.tar.gz` from the URL in the manifest and verifies its minisign signature against the public key embedded in `tauri.conf.json` (`pubkey`).
- The `dialog: false` config means the updater never opens a system dialog asking the user.
- **What GitHub sees:** client IP, default Tauri/`reqwest` User-Agent string, the timing of update checks. **It does not see any vault data.**
- Call sites: `tofa-app/src-tauri/src/lib.rs:146` (plugin registration), `lib.rs:382-389` (startup check), `commands.rs:1100-1135` (manual check via `check_for_update` Tauri command).

#### 3b. Localhost camera-relay listener

- **`tofa-app/src-tauri/src/commands.rs:609`** — `TcpListener::bind("127.0.0.1:0")`. Same pattern as the CLI's `cam` command: a localhost-only HTTP server that serves `cam.html` and receives the decoded QR via POST `/result`. The webview opens `http://127.0.0.1:<port>` in the system browser via `open(1)` on macOS. The listener accepts loopback connections only.

### 4. `tofa-app` webview (UI assets) — two external dependencies (FINDINGS)

The frontend lives in `tofa-app/src-tauri/ui/` (HTML/CSS/JS, no build tool) plus `tofa-app/src-tauri/src/cam.html` (string-embedded in the binary for the camera scan flow).

Three `fetch(...)` call sites in the UI; only one resolves to an external host (FINDING 1):

| File | Line | URL | External? |
|---|---|---|---|
| `tofa-app/src-tauri/ui/js/sprite.js` | 43 | `assets/svg/issuers/<name>.svg` (relative) | No — local asset |
| `tofa-app/src-tauri/src/cam.html` | 164 | `/result` (relative — POSTs decoded QR back to localhost server) | No — loopback |
| `tofa-app/src-tauri/src/cam.html` | 131 | `/jsQR.min.js` (relative — served by the localhost server from `tofa_core::JSQR_MIN_JS`) | No — loopback *(was FINDING 1, fixed on this branch)* |

External CSS import: **none** *(was FINDING 2, fixed on this branch)*. `tokens.css` now imports `../fonts/fonts.css`, which references the WOFF2 files bundled under `tofa-app/src-tauri/ui/assets/fonts/`.

### 5. Tauri capabilities (IPC surface) — minimal

`tofa-app/src-tauri/capabilities/default.json` grants the webview these permissions only:

- `core:default`
- `clipboard-manager:allow-write-text` — write OTP code to clipboard
- `dialog:allow-open` — open a file picker
- `updater:default` — trigger update check

**Notably absent:** `http` plugin, `shell` plugin, broad `fs` permissions. The webview has no Tauri-mediated way to make arbitrary HTTP requests through the Rust backend. (It can still issue `fetch()` from JS, see FINDINGS 1–3.)

The 30+ `#[tauri::command]` functions in `tofa-app/src-tauri/src/commands.rs` were spot-checked: none take a URL parameter or invoke `reqwest` themselves. Only the `check_for_update` and `download_and_install_update` commands trigger network I/O, both via the audited `tauri-plugin-updater`.

### 6. macOS bundle — no entitlements file, not sandboxed

`tofa-app/src-tauri/Info.plist` declares only:

- `NSCameraUsageDescription` (camera prompt)
- `NSScreenCaptureUsageDescription` (screen-capture prompt)
- `LSUIElement` (menu-bar-only app)

There is no `.entitlements` file and no `com.apple.security.app-sandbox` entitlement, which means the app runs **outside** the macOS App Sandbox. The OS does not restrict its outbound network access. See FINDING 5.

### 7. Telemetry / analytics / crash reporters — none

Grep across the workspace for `sentry`, `posthog`, `mixpanel`, `segment`, `amplitude`, `datadog`, `telemetry`, `analytics`, `crash[_-]?report` returned no matches in dependency manifests or source code. (One match in `ui/assets/css/styles.css` is a CSS class named `.settings-segmented` — unrelated.)

## Secret-handling review

### `OtpSecret` (`tofa-core/src/qr.rs:61`)

```rust
#[derive(Debug, Clone)]
pub struct OtpSecret {
    pub secret: String,
    pub meta: OtpMeta,
}
```

- **Serialize**: not derived. The struct cannot be JSON-encoded directly.
- **Debug**: derived. The raw secret string would print under `{:?}` formatting. No current call site does this; defense-in-depth fix is to write a redacted `Debug` impl (FINDING 4).
- **Zeroize / Drop**: no explicit zeroize on drop. The `String` holding the secret is heap-allocated and will be freed (but not wiped) when the struct drops. `OtpSecret` is short-lived in practice — it is converted to `VaultEntry` via `into_vault_entry` (`qr.rs:73`) almost immediately. The converted `VaultEntry` does zeroize on drop.

### `VaultEntry` (`tofa-core/src/store.rs:19`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultEntry { ..., pub secret: String, ... }

impl Drop for VaultEntry {
    fn drop(&mut self) { self.secret.zeroize(); }
}
```

- **Serialize**: derived. This is used for (a) encrypted on-disk vault storage and (b) Tauri IPC return values to the webview. Tauri IPC is a **local** named-pipe/stdin channel between webview and Rust on the same machine — it does not cross the network.
- **Debug**: derived. Same caveat as `OtpSecret` — no current logging call site emits a `{:?}` `VaultEntry`, but a custom redacted Debug would be safer.
- **Zeroize**: explicit `Drop` impl wipes the secret. ✅

The KDF + AES-GCM vault encryption layer (`tofa-core/src/crypto.rs`) uses `zeroize::Zeroizing` for derived key material.

## Findings

### FINDING 1 — Third-party CDN dependency in the QR-scan flow

**Status: ✅ Resolved on `fix/security-audit`.**
**Severity:** High *(in the QR-scan flow specifically; no impact when not scanning)*
**Location:** `tofa-app/src-tauri/src/cam.html:131`, `tofa/src/cli/commands/cam.rs:258` (same code, two surfaces)

The camera-based QR scanner loads its decoder library from a public CDN:

```html
s.src = 'https://cdn.jsdelivr.net/npm/jsqr@1.4.0/dist/jsQR.min.js';
```

**Why this is a problem.** The library that decodes the QR — which contains the raw OTP seed — is fetched at runtime from a third party. If `cdn.jsdelivr.net` (or its Cloudflare edge) is compromised, or if the user is on a hostile network (coffee shop, hotel) that can MITM the request and intercept the TLS connection, an attacker can serve a malicious `jsQR.min.js` that:

1. Decodes the QR locally as expected, and
2. `fetch()`s the decoded `otpauth://...` URI to an attacker-controlled host.

There is no CSP restricting `connect-src` (see FINDING 3) and the page is served from a loopback HTTP origin without restrictions, so a malicious script can call any URL.

Even without compromise, every QR scan reveals to jsdelivr/Cloudflare that the user is running Tofa and is at the QR-scanning step (timing signal).

**Fix.** Vendor `jsQR.min.js` (17 KB minified) into `tofa-app/src-tauri/ui/assets/js/` and `tofa-app/src-tauri/src/` for the CLI variant. Reference it with a relative path. Add a `Subresource Integrity` hash if you keep it remote as a fallback. Pin the version and re-verify on upgrade.

**Resolution (this branch).** The full minified `jsqr@1.4.0` (130 KB) is vendored at `tofa-core/vendor/jsQR.min.js` and exposed as a `&'static str` via `tofa_core::JSQR_MIN_JS` (`tofa-core/src/lib.rs`). Both the CLI's localhost mini-server (`tofa/src/cli/commands/cam.rs`) and the app's `scan_camera` localhost server (`tofa-app/src-tauri/src/commands.rs`) now serve `GET /jsQR.min.js` from that constant, and the `<script>` tags in `cam.html` / `build_html()` reference the relative path `/jsQR.min.js`. The QR-scanning flow makes zero outbound HTTP requests.

### FINDING 2 — Google Fonts loaded on every app launch

**Status: ✅ Resolved on `fix/security-audit`.**
**Severity:** Medium *(privacy, not secret exposure)*
**Location:** `tofa-app/src-tauri/ui/assets/css/tokens.css:1`

```css
@import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:...&family=Fraunces:...&family=Inter:...&display=swap');
```

Every time the Tofa app window renders, the webview fetches CSS from `fonts.googleapis.com` and (via that CSS) WOFF2 files from `fonts.gstatic.com`. Google sees the client IP, User-Agent, the `Referer` (or its absence), and the timing of every launch.

No OTP seed or vault data is sent. But this contradicts a strict "fully offline" privacy stance, and a privacy-conscious user installing a TOTP manager would reasonably expect zero outbound traffic at idle.

**Fix.** Self-host the three font families. Use `pyftsubset` (or the Google Fonts subset URL with `?text=...&format=woff2`) to subset to ASCII + the small set of glyphs the UI actually uses; the total drops to a few KB. Reference with `@font-face` in `tokens.css`. Once removed, the app makes zero outbound requests at launch (only the updater check at a configured interval).

**Resolution (this branch).** The three families (Fraunces, Inter, JetBrains Mono) are vendored as variable-font WOFF2 files under `tofa-app/src-tauri/ui/assets/fonts/`, one file per family-subset (latin + latin-ext), 6 files totalling 316 KB. Vietnamese subset is dropped (not used by the UI). A local `fonts.css` declares one `@font-face` per family-subset with `font-weight: 100 900` so the variable font covers every weight the UI requests. `tokens.css` now imports `../fonts/fonts.css` instead of `fonts.googleapis.com`. The app makes zero outbound requests at launch.

### FINDING 3 — No Content-Security-Policy on the webview

**Severity:** Low *(defense-in-depth)*
**Location:** `tofa-app/src-tauri/tauri.conf.json` (`app.security.csp: null`)

With CSP disabled, the webview is free to load scripts, styles, and make XHRs to any origin. This means:

- If FINDING 1 or 2 were exploited, no second-line defense.
- Any future accidental introduction of an external script tag or fetch goes undetected.

**Fix.** After bundling the assets in FINDINGS 1–2, set a tight CSP:

```json
"security": {
  "csp": "default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; object-src 'none'; base-uri 'self'; frame-ancestors 'none'"
}
```

The `'self'` origin in a Tauri webview resolves to the app's bundled assets — no external host matches.

### FINDING 4 — `OtpSecret` derives `Debug` without redaction

**Severity:** Low *(defense-in-depth; no current exploit path)*
**Location:** `tofa-core/src/qr.rs:60-64`

`#[derive(Debug)]` on a struct whose `secret: String` field is the raw OTP seed means any `{:?}`, `dbg!()`, or `tracing::debug!("{otp:?}")` call would emit the seed. No current call site does this, but the type's surface area makes the foot-gun available.

**Fix.** Replace the derive with a manual impl:

```rust
impl std::fmt::Debug for OtpSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OtpSecret").field("secret", &"<redacted>").field("meta", &self.meta).finish()
    }
}
```

Same change for `VaultEntry` in `tofa-core/src/store.rs`.

### FINDING 5 — App is not macOS App-Sandboxed

**Severity:** Low *(defense-in-depth)*
**Location:** `tofa-app/src-tauri/` (no `*.entitlements` file)

Without `com.apple.security.app-sandbox`, the OS does not constrain the app's outbound network access. If a transitive dependency ever made an unexpected outbound call, nothing at the OS level would block it.

**Fix.** Adding App Sandbox is a non-trivial change (some Tauri APIs may require additional entitlements). A more practical interim fix: ship a minimal `tofa.entitlements` declaring only `com.apple.security.network.client` (for the updater) and `com.apple.security.device.camera` + `com.apple.security.device.screen-capture` (for QR scanning). Even outside the sandbox, declaring entitlements explicitly is good hygiene and helps with future Gatekeeper changes.

## Reproducing the audit

Anyone can re-run these checks from a clean checkout:

```bash
# 1. Dependency tree per workspace member
for p in tofa-core tofa tofa-app; do
  echo "=== $p ==="
  cargo tree -e=normal -p $p | rg 'reqwest|ureq|hyper|surf|isahc|attohttpc|tokio-tungstenite|async-tungstenite|websocket|curl|sentry|posthog|mixpanel|segment|amplitude|datadog'
done

# 2. HTTP/socket call sites in own code
rg -nP '\b(reqwest|ureq|hyper::|TcpStream|TcpListener|UdpSocket|std::net)\b' tofa-core tofa tofa-app/src-tauri/src

# 3. Frontend egress primitives
rg -nP "\b(fetch\(|XMLHttpRequest|WebSocket|EventSource|sendBeacon|new Image\()\b" tofa-app

# 4. External URLs in frontend
rg -nP 'https?://(?!127\.0\.0\.1|localhost|w3\.org/2000/svg)[a-zA-Z0-9.-]+' tofa-app/src-tauri/ui tofa-app/src-tauri/src

# 5. Tauri config
cat tofa-app/src-tauri/tauri.conf.json
cat tofa-app/src-tauri/capabilities/*.json
cat tofa-app/src-tauri/Info.plist

# 6. Telemetry SDKs
rg -nPi 'sentry|posthog|mixpanel|segment|amplitude|datadog|telemetry|analytics|crash[_-]?report' tofa-core tofa tofa-app/src-tauri
```

A runtime audit complements this by running the app behind `mitmproxy --mode regular` with the system proxy set, exercising every flow (add via QR, view OTP, edit, delete, lock/unlock, idle 10 min, quit), and confirming only the updater endpoint appears in the captured traffic.

## Continuous gating

Two CI gates recommended to keep this property:

1. **Ban HTTP clients in `tofa-core` and `tofa`.** Add to `deny.toml`:

   ```toml
   [bans]
   deny = [
     { crate = "reqwest", wrappers = ["tauri", "tauri-plugin-updater"] },
     { crate = "ureq" },
     { crate = "isahc" },
     { crate = "attohttpc" },
   ]
   ```

   Today this passes (the only `reqwest` path is via Tauri). The moment a future change introduces `reqwest` directly into `tofa-core` or `tofa`, the `Security audit` CI job fails.

2. **Forbid external script/style/font references in the bundled webview.** Once FINDINGS 1–2 are fixed, add a simple grep-based test that fails CI if a new external URL appears in `tofa-app/src-tauri/{ui,src}`:

   ```bash
   ! rg -nP 'https?://(?!127\.0\.0\.1|localhost|w3\.org/2000/svg)[a-zA-Z0-9.-]+' tofa-app/src-tauri/ui tofa-app/src-tauri/src
   ```

## Summary table of recommendations

| # | Finding | Severity | Status |
|---|---|---|---|
| 1 | Vendor jsQR locally instead of CDN | High | ✅ Resolved on this branch |
| 2 | Self-host Google Fonts | Medium | ✅ Resolved on this branch |
| 3 | Set a tight CSP after #1 and #2 | Low | Open — unblocked by #1 and #2, can land next |
| 4 | Redact `Debug` on `OtpSecret` and `VaultEntry` | Low | Open |
| 5 | Add explicit macOS entitlements file | Low | Open |
| — | Add the two CI gates above | — | Open |

With #3–5 also addressed, the audit's headline claim becomes defensible without caveats.
