# tofa Menu Bar App — Design Spec

## Goal

A macOS menu bar app (Tauri v2) that lives in the top bar, shows the OTP vault contents in a popover, caches the passphrase for 10 minutes, and copies a code to the clipboard on click.

## Architecture

New crate `tofa-app` added to the Cargo workspace alongside the existing `tofa` CLI and `tofa-core` library.

```
tofa/                        (workspace root)
├── tofa-core/               (shared library — unchanged)
├── tofa/                    (CLI TUI — unchanged)
└── tofa-app/                (NEW — Tauri v2 menu bar app)
    ├── src/
    │   ├── main.rs          (Tauri setup, tray icon, Tauri commands)
    │   └── state.rs         (PassphraseCache — Zeroizing<String> + Instant, 10 min TTL)
    ├── frontend/
    │   ├── index.html       (single page — locked or unlocked state)
    │   ├── style.css        (dark theme matching tofa palette)
    │   └── main.js          (state machine: locked ↔ unlocked ↔ settings)
    └── tauri.conf.json      (tray-only app, no main window, macOS target)
```

`tofa-app` depends on `tofa-core` as a workspace path dependency. No subprocess calls — the backend uses `tofa-core` APIs directly.

## Tray Icon

- Locked state: padlock icon (template image, adapts to light/dark menu bar)
- Unlocked state: small blue dot overlay on the padlock
- Source: generated from `assets/icon.png` at build time (16×16 and 32×32 @2x)
- Click on icon → opens/closes the popover window

## Popover

A Tauri window (`decorations: false`, `always_on_top: true`, `resizable: false`, fixed size ~320×480) positioned below the tray icon on click. Closes when focus is lost or after a copy action.

### Locked State

```
┌────────────────────────────┐
│  tofa                      │
│                            │
│  Passphrase                │
│  [_____________________]   │
│                            │
│  [ Unlock ]                │
│                            │
│  Wrong passphrase          │  ← shown on error
└────────────────────────────┘
```

- Password input, submit on Enter or button click
- Calls Tauri command `unlock(passphrase: String) -> Result<Vec<OtpEntry>, String>`
- On success: cache passphrase, transition to unlocked state
- On failure: show "Wrong passphrase" inline

### Unlocked State

```
┌────────────────────────────┐
│  tofa                 [⚙]  │
│  ─────────────────────     │
│  GitHub          127 834   │
│  ██████████████░░░░  28s   │
│  ─────────────────────     │
│  Google          891 234   │
│  ████████░░░░░░░░░░  18s   │
│  ─────────────────────     │
│  ...                       │
└────────────────────────────┘
```

- Each row: name (left) + formatted code (right, space in the middle)
- Below each row: expiry progress bar + remaining seconds
- Click on row → `copy_code(name: String)` → clipboard → popover closes
- Codes loaded fresh on every popover open (no continuous polling)
- ⚙ icon → settings state

### Settings State

```
┌────────────────────────────┐
│  ← Settings                │
│                            │
│  Vault path                │
│  [~/.config/tofa/vault.enc]│
│                            │
│  [ Save ]                  │
└────────────────────────────┘
```

- Single text field for vault path
- Persisted to `~/.config/tofa/settings.json`
- Loaded at app startup; default is `~/.config/tofa/vault.enc`

## Backend (Rust)

### `state.rs` — PassphraseCache

```rust
pub struct PassphraseCache {
    passphrase: Zeroizing<String>,
    unlocked_at: Option<Instant>,
}

impl PassphraseCache {
    pub fn unlock(&mut self, passphrase: String) { ... }
    pub fn get(&self) -> Option<&str> { /* None if expired or never set */ }
    pub fn is_locked(&self) -> bool { self.get().is_none() }
    pub fn lock(&mut self) { self.passphrase.zeroize(); self.unlocked_at = None; }
}
```

TTL: 10 minutes. On every Tauri command, `get()` is called first — if `None`, the command returns an error that triggers the locked state in the frontend.

### Tauri Commands

```rust
// Try passphrase, return OTP entries on success
#[tauri::command]
fn unlock(passphrase: String, state: State<Mutex<AppState>>) -> Result<Vec<OtpEntry>, String>

// Get current OTP entries (requires cached passphrase)
#[tauri::command]
fn get_entries(state: State<Mutex<AppState>>) -> Result<Vec<OtpEntry>, String>

// Copy code for named entry to clipboard, returns Ok(()) on success
#[tauri::command]
fn copy_code(name: String, state: State<Mutex<AppState>>, app: AppHandle) -> Result<(), String>

// Read settings (vault path)
#[tauri::command]
fn get_settings() -> Result<Settings, String>

// Write settings (vault path)
#[tauri::command]
fn save_settings(settings: Settings) -> Result<(), String>
```

### OtpEntry (shared type)

```rust
#[derive(Serialize)]
pub struct OtpEntry {
    pub name: String,
    pub code: String,       // "127 834" (space-formatted)
    pub seconds_left: u64,
    pub period: u32,
}
```

## Frontend

Vanilla HTML/CSS/JS — no framework. Single `index.html` with three views toggled by JS:
- `#view-locked`
- `#view-unlocked`
- `#view-settings`

Dark palette matching the TUI: background `#0d1117`, text `#e6edf3`, accent `#58a6ff`, dim `#8b949e`.

Font: `-apple-system` (native macOS system font).

Progress bar: CSS `width` set inline as `calc(${seconds_left} / ${period} * 100%)`.

## Cleanup (Deprecated Removal)

Remove from the repository:
- `packaging/macos/` (tofa-launcher, Info.plist.template) — replaced by Tauri bundler
- `tofa.app/`, `tofa.icns`, `tofa.iconset` — local test artifacts, add to `.gitignore`
- `release.yml` changes: remove `dmg` job (lipo, sips, iconutil, create-dmg, codesign), remove `aarch64-apple-darwin` from build matrix (Tauri handles universal binary internally via `--target universal-apple-darwin`)
- The Homebrew cask (`Casks/tofa.rb` in homebrew-tofa) now points to the Tauri-produced DMG instead of the hand-crafted one

## Release Integration

`tauri build --target universal-apple-darwin` produces:
- `tofa-app_VERSION_universal.dmg` — drag-to-Applications installer
- `tofa-app_VERSION_universal.app.tar.gz` — for Homebrew cask

This replaces the hand-crafted DMG job. The `release.yml` `dmg` job is removed; a new `tauri-build` job runs `tauri build` on `macos-latest` and uploads the artifacts.

## Settings File Format

```json
{
  "vault_path": "/Users/alice/.config/tofa/vault.enc"
}
```

Stored at `~/.config/tofa/settings.json`. Created with defaults on first launch if absent.
