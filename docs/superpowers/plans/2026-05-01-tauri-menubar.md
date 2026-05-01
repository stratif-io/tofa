# tofa Menu Bar App — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the bash `.app` bundle with a proper Tauri v2 macOS menu bar app that reads the encrypted vault via `tofa-core`, caches the passphrase 10 minutes, and copies OTP codes to the clipboard on click.

**Architecture:** New crate `tofa-app/src-tauri` added to the Cargo workspace. Tauri v2 tray-only app (no Dock icon, no main window). Vanilla JS frontend in `tofa-app/src/`. Deprecated `packaging/macos/` and the hand-crafted DMG CI job are removed.

**Tech Stack:** Tauri v2, `tauri-plugin-clipboard-manager`, `tauri-plugin-positioner`, `tofa-core` (path dependency), `zeroize`, vanilla HTML/CSS/JS.

---

## File Structure

```
tofa-app/
├── src/                          (frontend)
│   ├── index.html
│   ├── style.css
│   └── main.js
└── src-tauri/                    (Rust crate)
    ├── Cargo.toml
    ├── build.rs
    ├── tauri.conf.json
    ├── icons/
    │   └── icon.png              (copied from assets/icon.png at build time)
    └── src/
        ├── main.rs               (Tauri setup, tray, window)
        ├── state.rs              (PassphraseCache, AppState)
        └── commands.rs           (unlock, get_entries, copy_code, get/save settings)
```

Workspace root `Cargo.toml`: add `"tofa-app/src-tauri"` to members.

CI `release.yml`: remove `dmg` job entirely, remove `aarch64-apple-darwin` from CLI build matrix, add `tauri-build` job.

---

### Task 1: Cleanup deprecated files

**Files:**
- Delete: `packaging/macos/tofa-launcher`
- Delete: `packaging/macos/Info.plist.template`
- Delete: `packaging/macos/` directory
- Modify: `.github/workflows/release.yml`
- Modify: `.gitignore`

- [ ] **Step 1: Remove packaging/macos/**

```bash
rm -rf /Users/carlo/my_work/tofa/packaging/macos
rmdir /Users/carlo/my_work/tofa/packaging 2>/dev/null || true
```

- [ ] **Step 2: Remove local .app test artifacts from git tracking**

Add to `.gitignore` (append):
```
# Tauri build outputs and local test artifacts
tofa.app/
tofa.icns
tofa.iconset/
src-tauri/target/
```

Run:
```bash
git rm -r --cached tofa.app tofa.icns tofa.iconset 2>/dev/null || true
```

- [ ] **Step 3: Simplify release.yml — remove dmg job and aarch64-apple-darwin**

In `.github/workflows/release.yml`, remove the entire `dmg:` job block (from `dmg:` to the end of the job including the upload step).

Also remove this entry from the build matrix:
```yaml
          - os: macos-latest
            target: aarch64-apple-darwin
            use_cross: false
```

Also update the `release` job's `needs` from `needs: [build, dmg]` to `needs: build`.

The `release` job's `Dispatch tap update` step: remove `SHA_DMG` extraction and remove `sha_dmg` from the JSON payload (the cask will be updated separately once Tauri produces its own DMG).

The final `build` matrix should have 3 entries: `x86_64-apple-darwin`, `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`.

- [ ] **Step 4: Commit**

```bash
cd /Users/carlo/my_work/tofa
git add -A
git commit -m "chore: remove deprecated .app bundle, simplify release pipeline to 3 targets"
```

---

### Task 2: Workspace scaffold — Tauri crate skeleton

**Files:**
- Modify: `Cargo.toml` (workspace root)
- Create: `tofa-app/src-tauri/Cargo.toml`
- Create: `tofa-app/src-tauri/build.rs`
- Create: `tofa-app/src-tauri/tauri.conf.json`
- Create: `tofa-app/src-tauri/src/main.rs` (stub)
- Create: `tofa-app/src/index.html` (stub)

- [ ] **Step 1: Add tofa-app/src-tauri to workspace**

In `/Users/carlo/my_work/tofa/Cargo.toml`, change:
```toml
[workspace]
members = ["tofa-core", "tofa"]
resolver = "2"
```
to:
```toml
[workspace]
members = ["tofa-core", "tofa", "tofa-app/src-tauri"]
resolver = "2"
```

- [ ] **Step 2: Create tofa-app/src-tauri/Cargo.toml**

```bash
mkdir -p /Users/carlo/my_work/tofa/tofa-app/src-tauri/src
mkdir -p /Users/carlo/my_work/tofa/tofa-app/src-tauri/icons
mkdir -p /Users/carlo/my_work/tofa/tofa-app/src
```

```toml
# tofa-app/src-tauri/Cargo.toml
[package]
name = "tofa-app"
version = "0.1.0"
edition = "2021"

[lib]
name = "tofa_app_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = ["tray-icon", "image-png"] }
tauri-plugin-clipboard-manager = "2"
tauri-plugin-positioner = { version = "2", features = ["tray-icon"] }
tofa-core = { path = "../../tofa-core" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
zeroize = { version = "1", features = ["derive"] }
```

- [ ] **Step 3: Create build.rs**

```rust
// tofa-app/src-tauri/build.rs
fn main() {
    tauri_build::build()
}
```

- [ ] **Step 4: Create tauri.conf.json**

```json
{
  "productName": "tofa",
  "version": "0.1.0",
  "identifier": "com.cabichahine.tofa",
  "build": {
    "frontendDist": "../src"
  },
  "app": {
    "windows": [],
    "security": {
      "csp": null
    },
    "trayIcon": {
      "id": "main",
      "iconPath": "icons/icon.png",
      "iconAsTemplate": true,
      "menuOnLeftClick": false
    },
    "macOSPrivateApi": false
  },
  "bundle": {
    "active": true,
    "targets": ["dmg", "app"],
    "icon": ["icons/icon.png"],
    "macOS": {
      "infoPlist": {
        "LSUIElement": true
      }
    }
  }
}
```

`LSUIElement: true` hides tofa from the Dock — it lives only in the menu bar.

- [ ] **Step 5: Copy icon**

```bash
cp /Users/carlo/my_work/tofa/assets/icon.png /Users/carlo/my_work/tofa/tofa-app/src-tauri/icons/icon.png
```

- [ ] **Step 6: Create stub main.rs**

```rust
// tofa-app/src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 7: Create stub index.html**

```html
<!-- tofa-app/src/index.html -->
<!DOCTYPE html>
<html><body><p>tofa</p></body></html>
```

- [ ] **Step 8: Verify it compiles**

```bash
cd /Users/carlo/my_work/tofa/tofa-app/src-tauri
cargo build 2>&1 | tail -5
```

Expected: `Finished` (may take a few minutes first time — Tauri downloads dependencies).

- [ ] **Step 9: Commit**

```bash
cd /Users/carlo/my_work/tofa
git add -A
git commit -m "feat(tofa-app): scaffold Tauri v2 workspace crate"
```

---

### Task 3: PassphraseCache and AppState

**Files:**
- Create: `tofa-app/src-tauri/src/state.rs`
- Modify: `tofa-app/src-tauri/src/main.rs`

- [ ] **Step 1: Write state.rs**

```rust
// tofa-app/src-tauri/src/state.rs
use std::path::PathBuf;
use std::time::{Duration, Instant};
use zeroize::Zeroizing;

const CACHE_TTL: Duration = Duration::from_secs(10 * 60);

pub struct PassphraseCache {
    passphrase: Zeroizing<String>,
    unlocked_at: Option<Instant>,
}

impl PassphraseCache {
    pub fn new() -> Self {
        Self {
            passphrase: Zeroizing::new(String::new()),
            unlocked_at: None,
        }
    }

    pub fn unlock(&mut self, passphrase: String) {
        *self.passphrase = passphrase;
        self.unlocked_at = Some(Instant::now());
    }

    pub fn get(&self) -> Option<&str> {
        match self.unlocked_at {
            Some(t) if t.elapsed() < CACHE_TTL => Some(self.passphrase.as_str()),
            _ => None,
        }
    }

    pub fn is_locked(&self) -> bool {
        self.get().is_none()
    }

    pub fn lock(&mut self) {
        use zeroize::Zeroize;
        self.passphrase.zeroize();
        self.unlocked_at = None;
    }
}

pub struct AppState {
    pub cache: PassphraseCache,
    pub vault_path: PathBuf,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            cache: PassphraseCache::new(),
            vault_path: default_vault_path(),
        }
    }
}

pub fn default_vault_path() -> PathBuf {
    let mut p = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
    p.push(".config/tofa/vault.enc");
    p
}

pub fn settings_path() -> PathBuf {
    let mut p = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
    p.push(".config/tofa/settings.json");
    p
}
```

Add `dirs = "5"` to `tofa-app/src-tauri/Cargo.toml` under `[dependencies]`.

- [ ] **Step 2: Write unit tests for PassphraseCache**

Append to `state.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cache_is_locked() {
        let cache = PassphraseCache::new();
        assert!(cache.is_locked());
        assert!(cache.get().is_none());
    }

    #[test]
    fn unlock_makes_passphrase_available() {
        let mut cache = PassphraseCache::new();
        cache.unlock("secret".to_string());
        assert!(!cache.is_locked());
        assert_eq!(cache.get(), Some("secret"));
    }

    #[test]
    fn lock_clears_passphrase() {
        let mut cache = PassphraseCache::new();
        cache.unlock("secret".to_string());
        cache.lock();
        assert!(cache.is_locked());
        assert!(cache.get().is_none());
    }
}
```

- [ ] **Step 3: Run tests**

```bash
cd /Users/carlo/my_work/tofa/tofa-app/src-tauri
cargo test state 2>&1
```

Expected:
```
test tests::new_cache_is_locked ... ok
test tests::unlock_makes_passphrase_available ... ok
test tests::lock_clears_passphrase ... ok
test result: ok. 3 passed
```

- [ ] **Step 4: Register state in main.rs**

```rust
// tofa-app/src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;

use std::sync::Mutex;
use state::AppState;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_positioner::init())
        .manage(Mutex::new(AppState::new()))
        .invoke_handler(tauri::generate_handler![
            commands::unlock,
            commands::get_entries,
            commands::copy_code,
            commands::get_settings,
            commands::save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 5: Create commands.rs stub** (so it compiles)

```rust
// tofa-app/src-tauri/src/commands.rs
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::state::{AppState, default_vault_path, settings_path};

#[derive(Serialize)]
pub struct OtpEntry {
    pub name: String,
    pub code: String,
    pub seconds_left: u64,
    pub period: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub vault_path: String,
}

#[tauri::command]
pub fn unlock(_passphrase: String, _state: State<Mutex<AppState>>) -> Result<Vec<OtpEntry>, String> {
    Ok(vec![])
}

#[tauri::command]
pub fn get_entries(_state: State<Mutex<AppState>>) -> Result<Vec<OtpEntry>, String> {
    Ok(vec![])
}

#[tauri::command]
pub fn copy_code(_name: String, _state: State<Mutex<AppState>>, _app: tauri::AppHandle) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_settings() -> Result<Settings, String> {
    Ok(Settings { vault_path: default_vault_path().to_string_lossy().to_string() })
}

#[tauri::command]
pub fn save_settings(_settings: Settings) -> Result<(), String> {
    Ok(())
}
```

- [ ] **Step 6: Verify it compiles**

```bash
cd /Users/carlo/my_work/tofa/tofa-app/src-tauri
cargo build 2>&1 | tail -3
```

Expected: `Finished`

- [ ] **Step 7: Commit**

```bash
cd /Users/carlo/my_work/tofa
git add tofa-app/src-tauri/src/state.rs tofa-app/src-tauri/src/main.rs tofa-app/src-tauri/src/commands.rs tofa-app/src-tauri/Cargo.toml
git commit -m "feat(tofa-app): PassphraseCache with 10 min TTL, AppState, command stubs"
```

---

### Task 4: Tauri commands — real implementation

**Files:**
- Modify: `tofa-app/src-tauri/src/commands.rs`

- [ ] **Step 1: Implement unlock**

Replace the `unlock` stub:

```rust
#[tauri::command]
pub fn unlock(passphrase: String, state: State<Mutex<AppState>>) -> Result<Vec<OtpEntry>, String> {
    let mut s = state.lock().map_err(|e| e.to_string())?;
    let vault = tofa_core::store::Vault::load(&s.vault_path, &passphrase)
        .map_err(|_| "Wrong passphrase".to_string())?;
    s.cache.unlock(passphrase);
    entries_from_vault(&vault)
}
```

Add this private helper after the command definitions:

```rust
fn entries_from_vault(vault: &tofa_core::store::Vault) -> Result<Vec<OtpEntry>, String> {
    vault.entries().iter().map(|entry| {
        let code_raw = tofa_core::totp::generate_code_now(entry)
            .map_err(|e| e.to_string())?;
        let code = format!("{} {}", &code_raw[..3], &code_raw[3..]);
        let seconds_left = tofa_core::totp::seconds_remaining_now(entry);
        Ok(OtpEntry {
            name: entry.name.clone(),
            code,
            seconds_left,
            period: entry.period,
        })
    }).collect()
}
```

- [ ] **Step 2: Implement get_entries**

Replace the `get_entries` stub:

```rust
#[tauri::command]
pub fn get_entries(state: State<Mutex<AppState>>) -> Result<Vec<OtpEntry>, String> {
    let mut s = state.lock().map_err(|e| e.to_string())?;
    let passphrase = s.cache.get()
        .ok_or("locked")?
        .to_string();
    let vault = tofa_core::store::Vault::load(&s.vault_path, &passphrase)
        .map_err(|e| e.to_string())?;
    entries_from_vault(&vault)
}
```

- [ ] **Step 3: Implement copy_code**

Replace the `copy_code` stub:

```rust
#[tauri::command]
pub fn copy_code(name: String, state: State<Mutex<AppState>>, app: tauri::AppHandle) -> Result<(), String> {
    let mut s = state.lock().map_err(|e| e.to_string())?;
    let passphrase = s.cache.get()
        .ok_or("locked")?
        .to_string();
    let vault = tofa_core::store::Vault::load(&s.vault_path, &passphrase)
        .map_err(|e| e.to_string())?;
    let entry = vault.entries().iter()
        .find(|e| e.name == name)
        .ok_or_else(|| format!("entry '{}' not found", name))?;
    let code = tofa_core::totp::generate_code_now(entry)
        .map_err(|e| e.to_string())?;
    tauri_plugin_clipboard_manager::ClipboardExt::clipboard(&app)
        .write_text(code)
        .map_err(|e| e.to_string())?;
    Ok(())
}
```

- [ ] **Step 4: Implement get_settings and save_settings**

Replace the two settings stubs:

```rust
#[tauri::command]
pub fn get_settings() -> Result<Settings, String> {
    let path = settings_path();
    if path.exists() {
        let s = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&s).map_err(|e| e.to_string())
    } else {
        Ok(Settings {
            vault_path: default_vault_path().to_string_lossy().to_string(),
        })
    }
}

#[tauri::command]
pub fn save_settings(settings: Settings) -> Result<(), String> {
    let path = settings_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let s = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    std::fs::write(&path, s).map_err(|e| e.to_string())
}
```

- [ ] **Step 5: Update AppState to load vault path from settings on init**

In `state.rs`, replace `AppState::new()`:

```rust
impl AppState {
    pub fn new() -> Self {
        let vault_path = Self::load_vault_path();
        Self {
            cache: PassphraseCache::new(),
            vault_path,
        }
    }

    fn load_vault_path() -> PathBuf {
        let path = settings_path();
        if path.exists() {
            if let Ok(s) = std::fs::read_to_string(&path) {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&s) {
                    if let Some(p) = v["vault_path"].as_str() {
                        return PathBuf::from(p);
                    }
                }
            }
        }
        default_vault_path()
    }
}
```

- [ ] **Step 6: Verify compile**

```bash
cd /Users/carlo/my_work/tofa/tofa-app/src-tauri
cargo build 2>&1 | tail -3
```

Expected: `Finished`

- [ ] **Step 7: Commit**

```bash
cd /Users/carlo/my_work/tofa
git add tofa-app/src-tauri/src/commands.rs tofa-app/src-tauri/src/state.rs
git commit -m "feat(tofa-app): implement unlock, get_entries, copy_code, settings commands"
```

---

### Task 5: Frontend — base layout + locked view

**Files:**
- Modify: `tofa-app/src/index.html`
- Create: `tofa-app/src/style.css`
- Create: `tofa-app/src/main.js`

- [ ] **Step 1: Write index.html**

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>tofa</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <!-- Locked view -->
  <div id="view-locked" class="view">
    <div class="header">
      <span class="app-name">tofa</span>
    </div>
    <div class="content">
      <form id="form-unlock">
        <label for="input-passphrase">Passphrase</label>
        <input id="input-passphrase" type="password" autocomplete="current-password" autofocus>
        <p id="unlock-error" class="error hidden"></p>
        <button type="submit">Unlock</button>
      </form>
    </div>
  </div>

  <!-- Unlocked view -->
  <div id="view-unlocked" class="view hidden">
    <div class="header">
      <span class="app-name">tofa</span>
      <button id="btn-settings" class="icon-btn" title="Settings">⚙</button>
    </div>
    <div id="otp-list" class="otp-list"></div>
  </div>

  <!-- Settings view -->
  <div id="view-settings" class="view hidden">
    <div class="header">
      <button id="btn-back" class="icon-btn" title="Back">←</button>
      <span class="app-name">Settings</span>
    </div>
    <div class="content">
      <form id="form-settings">
        <label for="input-vault-path">Vault path</label>
        <input id="input-vault-path" type="text" spellcheck="false">
        <p id="settings-error" class="error hidden"></p>
        <button type="submit">Save</button>
      </form>
    </div>
  </div>

  <script src="main.js"></script>
</body>
</html>
```

- [ ] **Step 2: Write style.css**

```css
:root {
  --bg: #0d1117;
  --bg-hover: #161b22;
  --border: #30363d;
  --text: #e6edf3;
  --dim: #8b949e;
  --accent: #58a6ff;
  --urgent: #f85149;
  --radius: 6px;
}

* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  background: var(--bg);
  color: var(--text);
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  font-size: 13px;
  width: 320px;
  min-height: 100px;
  overflow: hidden;
}

.view { display: flex; flex-direction: column; }
.hidden { display: none !important; }

.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border);
}

.app-name {
  font-weight: 600;
  color: var(--accent);
  letter-spacing: 0.5px;
}

.icon-btn {
  background: none;
  border: none;
  color: var(--dim);
  cursor: pointer;
  font-size: 14px;
  padding: 2px 4px;
  border-radius: var(--radius);
}

.icon-btn:hover { color: var(--text); background: var(--bg-hover); }

.content {
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

label {
  color: var(--dim);
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 4px;
  display: block;
}

input[type="password"],
input[type="text"] {
  width: 100%;
  background: var(--bg-hover);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  color: var(--text);
  font-size: 13px;
  padding: 8px 10px;
  outline: none;
  margin-bottom: 8px;
}

input:focus { border-color: var(--accent); }

button[type="submit"] {
  width: 100%;
  background: var(--accent);
  border: none;
  border-radius: var(--radius);
  color: #0d1117;
  font-size: 13px;
  font-weight: 600;
  padding: 8px;
  cursor: pointer;
}

button[type="submit"]:hover { opacity: 0.85; }

.error {
  color: var(--urgent);
  font-size: 12px;
  margin-bottom: 8px;
}

/* OTP list */
.otp-list { display: flex; flex-direction: column; max-height: 420px; overflow-y: auto; }

.otp-entry {
  display: flex;
  flex-direction: column;
  padding: 10px 16px;
  cursor: pointer;
  border-bottom: 1px solid var(--border);
  gap: 4px;
}

.otp-entry:hover { background: var(--bg-hover); }

.otp-row {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
}

.otp-name { color: var(--text); font-weight: 500; }

.otp-code {
  color: var(--accent);
  font-weight: 600;
  font-variant-numeric: tabular-nums;
  letter-spacing: 1px;
}

.otp-bar-wrap {
  height: 2px;
  background: var(--border);
  border-radius: 1px;
  overflow: hidden;
}

.otp-bar {
  height: 100%;
  border-radius: 1px;
  background: var(--accent);
  transition: width 1s linear;
}

.otp-bar.urgent { background: var(--urgent); }

.otp-secs {
  color: var(--dim);
  font-size: 11px;
  text-align: right;
}
```

- [ ] **Step 3: Write main.js — skeleton + locked view**

```javascript
// tofa-app/src/main.js
const { invoke } = window.__TAURI__.core;

// --- Views ---
const views = {
  locked: document.getElementById('view-locked'),
  unlocked: document.getElementById('view-unlocked'),
  settings: document.getElementById('view-settings'),
};

function showView(name) {
  Object.values(views).forEach(v => v.classList.add('hidden'));
  views[name].classList.remove('hidden');
}

// --- Locked view ---
const formUnlock = document.getElementById('form-unlock');
const inputPassphrase = document.getElementById('input-passphrase');
const unlockError = document.getElementById('unlock-error');

formUnlock.addEventListener('submit', async (e) => {
  e.preventDefault();
  unlockError.classList.add('hidden');
  try {
    const entries = await invoke('unlock', { passphrase: inputPassphrase.value });
    inputPassphrase.value = '';
    renderEntries(entries);
    showView('unlocked');
  } catch (err) {
    unlockError.textContent = err;
    unlockError.classList.remove('hidden');
  }
});

// --- Unlocked view ---
const otpList = document.getElementById('otp-list');
const btnSettings = document.getElementById('btn-settings');

function renderEntries(entries) {
  otpList.innerHTML = '';
  entries.forEach(entry => {
    const pct = Math.round((entry.seconds_left / entry.period) * 100);
    const urgent = entry.seconds_left <= 5;

    const el = document.createElement('div');
    el.className = 'otp-entry';
    el.innerHTML = `
      <div class="otp-row">
        <span class="otp-name">${entry.name}</span>
        <span class="otp-code">${entry.code}</span>
      </div>
      <div class="otp-bar-wrap">
        <div class="otp-bar${urgent ? ' urgent' : ''}" style="width:${pct}%"></div>
      </div>
    `;
    el.addEventListener('click', async () => {
      try {
        await invoke('copy_code', { name: entry.name });
        // Tauri will close the window from the backend — handled by closing the popover
      } catch (err) {
        console.error('copy failed:', err);
      }
    });
    otpList.appendChild(el);
  });
}

btnSettings.addEventListener('click', async () => {
  try {
    const s = await invoke('get_settings');
    document.getElementById('input-vault-path').value = s.vault_path;
  } catch (_) {}
  showView('settings');
});

// --- Settings view ---
const formSettings = document.getElementById('form-settings');
const btnBack = document.getElementById('btn-back');
const settingsError = document.getElementById('settings-error');

btnBack.addEventListener('click', () => showView('unlocked'));

formSettings.addEventListener('submit', async (e) => {
  e.preventDefault();
  settingsError.classList.add('hidden');
  try {
    await invoke('save_settings', {
      settings: { vault_path: document.getElementById('input-vault-path').value }
    });
    showView('unlocked');
  } catch (err) {
    settingsError.textContent = err;
    settingsError.classList.remove('hidden');
  }
});

// --- Init ---
showView('locked');
setTimeout(() => inputPassphrase.focus(), 50);
```

- [ ] **Step 4: Commit**

```bash
cd /Users/carlo/my_work/tofa
git add tofa-app/src/
git commit -m "feat(tofa-app): frontend — locked/unlocked/settings views in vanilla JS"
```

---

### Task 6: Tray icon + window management

**Files:**
- Modify: `tofa-app/src-tauri/src/main.rs`
- Modify: `tofa-app/src-tauri/src/commands.rs`

- [ ] **Step 1: Wire up tray and window in main.rs**

Replace the entire `main.rs`:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;

use std::sync::Mutex;
use state::AppState;
use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WebviewUrl, WebviewWindowBuilder,
};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_positioner::init())
        .manage(Mutex::new(AppState::new()))
        .invoke_handler(tauri::generate_handler![
            commands::unlock,
            commands::get_entries,
            commands::copy_code,
            commands::get_settings,
            commands::save_settings,
        ])
        .setup(|app| {
            // Create the popover window, hidden initially
            let window = WebviewWindowBuilder::new(
                app,
                "popover",
                WebviewUrl::App("index.html".into()),
            )
            .decorations(false)
            .always_on_top(true)
            .resizable(false)
            .visible(false)
            .inner_size(320.0, 480.0)
            .build()?;

            // Hide from macOS mission control and expose no title bar
            #[cfg(target_os = "macos")]
            {
                use tauri::TitleBarStyle;
                let _ = window.set_title_bar_style(TitleBarStyle::Overlay);
            }

            // Build tray icon
            let tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;

            tray.on_tray_icon_event(|tray, event| {
                tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);
                if let TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } = event
                {
                    let app = tray.app_handle();
                    if let Some(win) = app.get_webview_window("popover") {
                        let visible = win.is_visible().unwrap_or(false);
                        if visible {
                            let _ = win.hide();
                        } else {
                            let _ = tauri_plugin_positioner::move_window(
                                &win,
                                tauri_plugin_positioner::Position::TrayBottomCenter,
                            );
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            // Close popover when it loses focus
            if let tauri::WindowEvent::Focused(false) = event {
                if window.label() == "popover" {
                    let _ = window.hide();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 2: Close popover from backend after copy_code**

In `commands.rs`, update `copy_code` to hide the window after copying:

```rust
#[tauri::command]
pub fn copy_code(name: String, state: State<Mutex<AppState>>, app: tauri::AppHandle) -> Result<(), String> {
    let s = state.lock().map_err(|e| e.to_string())?;
    let passphrase = s.cache.get()
        .ok_or("locked")?
        .to_string();
    let vault = tofa_core::store::Vault::load(&s.vault_path, &passphrase)
        .map_err(|e| e.to_string())?;
    let entry = vault.entries().iter()
        .find(|e| e.name == name)
        .ok_or_else(|| format!("entry '{}' not found", name))?;
    let code = tofa_core::totp::generate_code_now(entry)
        .map_err(|e| e.to_string())?;
    tauri_plugin_clipboard_manager::ClipboardExt::clipboard(&app)
        .write_text(code)
        .map_err(|e| e.to_string())?;
    // Hide popover
    if let Some(win) = app.get_webview_window("popover") {
        let _ = win.hide();
    }
    Ok(())
}
```

Note: `copy_code` in Task 4 had `mut s` — remove `mut`, it's not needed (we only read from the cache here).

- [ ] **Step 3: Build and run**

```bash
cd /Users/carlo/my_work/tofa/tofa-app/src-tauri
cargo build 2>&1 | tail -3
```

Expected: `Finished`

Run in dev mode to test:
```bash
cd /Users/carlo/my_work/tofa/tofa-app/src-tauri
cargo run 2>&1
```

You should see a padlock icon in the macOS menu bar. Clicking it should open a 320×480 popover with the passphrase form.

- [ ] **Step 4: Commit**

```bash
cd /Users/carlo/my_work/tofa
git add tofa-app/src-tauri/src/main.rs tofa-app/src-tauri/src/commands.rs
git commit -m "feat(tofa-app): tray icon, popover window positioning, focus-to-close"
```

---

### Task 7: CI — tauri-build job in release.yml

**Files:**
- Modify: `.github/workflows/release.yml`

- [ ] **Step 1: Add tauri-build job**

The `release` job currently `needs: build`. After the cleanup in Task 1, the jobs are: `build` (3 CLI targets) and `release`. Add a `tauri-build` job that runs in parallel with `build`, then have `release` need all three.

Add this job after the `build` job in `release.yml`:

```yaml
  tauri-build:
    name: Build macOS App (Tauri)
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: aarch64-apple-darwin

      - uses: Swatinem/rust-cache@v2
        with:
          key: tauri-universal

      - name: Install Tauri CLI
        run: cargo install tauri-cli --version "^2" --locked

      - name: Build universal app
        working-directory: tofa-app/src-tauri
        run: cargo tauri build --target universal-apple-darwin

      - name: Rename DMG
        run: |
          VERSION="${GITHUB_REF_NAME#v}"
          mv tofa-app/src-tauri/target/universal-apple-darwin/release/bundle/dmg/*.dmg \
             "tofa-${VERSION}.dmg"
          echo "DMG_FILE=tofa-${VERSION}.dmg" >> "$GITHUB_ENV"

      - uses: actions/upload-artifact@v4
        with:
          name: tofa-dmg
          path: ${{ env.DMG_FILE }}
```

- [ ] **Step 2: Update release job needs**

Change:
```yaml
  release:
    needs: build
```
to:
```yaml
  release:
    needs: [build, tauri-build]
```

And restore the `SHA_DMG` extraction in the `Dispatch tap update` step (it was removed in Task 1 but the cask needs it again now that we have a Tauri DMG):

```bash
SHA_DMG=$(grep "\.dmg" artifacts/SHA256SUMS | awk '{print $1}')
```

And add it back to the JSON payload:
```json
"sha_dmg": "$SHA_DMG"
```

- [ ] **Step 3: Commit**

```bash
cd /Users/carlo/my_work/tofa
git add .github/workflows/release.yml
git commit -m "ci: add tauri-build job for universal macOS .app DMG"
```

---

### Task 8: End-to-end smoke test

No code changes — validation only.

- [ ] **Step 1: Create a test vault**

```bash
# If you don't have a vault yet, create one via the CLI
cd /Users/carlo/my_work/tofa
cargo run -p tofa -- init
cargo run -p tofa -- add --secret JBSWY3DPEHPK3PXP TestAccount
```

- [ ] **Step 2: Run the app in dev mode**

```bash
cd /Users/carlo/my_work/tofa/tofa-app/src-tauri
cargo run
```

Verify:
- [ ] Padlock icon appears in macOS menu bar
- [ ] Click opens popover with passphrase form
- [ ] Enter correct passphrase → OTP list appears with TestAccount
- [ ] Click entry → code copied to clipboard, popover closes
- [ ] Paste (`⌘V`) in any app → 6-digit code appears
- [ ] Enter wrong passphrase → "Wrong passphrase" error shown
- [ ] Click ⚙ → settings view with current vault path
- [ ] Save settings → back to unlocked view
- [ ] Click outside popover → popover closes

- [ ] **Step 3: Verify TUI CLI still works**

```bash
cd /Users/carlo/my_work/tofa
cargo run -p tofa -- list
```

Expected: table of OTP entries. The TUI app is unaffected.

- [ ] **Step 4: Final commit if any fixes needed**

```bash
git add -A
git commit -m "fix(tofa-app): smoke test fixes"
```
