use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{Emitter, Manager, State};
use zeroize::Zeroizing;
use crate::state::{AppState, default_vault_path, settings_path};

#[derive(Serialize)]
pub struct OtpEntry {
    pub name: String,
    pub issuer: String,
    pub account: String,
    pub code: String,
    pub seconds_left: u64,
    pub period: u32,
    pub digits: u32,
    pub algorithm: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub vault_path: String,
}

#[tauri::command]
pub fn vault_exists(state: State<Mutex<AppState>>) -> bool {
    state.lock().map(|s| s.vault_path.exists()).unwrap_or(false)
}

#[tauri::command]
pub async fn create_vault(passphrase: String, state: State<'_, Mutex<AppState>>, app: tauri::AppHandle) -> Result<Vec<OtpEntry>, String> {
    let vault_path = {
        let s = state.lock().map_err(|e| e.to_string())?;
        s.vault_path.clone()
    };
    if vault_path.exists() {
        return Err("Vault already exists.".to_string());
    }
    tokio::task::spawn_blocking({
        let vault_path = vault_path.clone();
        let passphrase = passphrase.clone();
        move || {
            tofa_core::store::Vault::new()
                .save(&vault_path, &passphrase)
                .map_err(|e| e.to_string())
        }
    }).await.map_err(|e| e.to_string())??;

    let mut s = state.lock().map_err(|e| e.to_string())?;
    let vault = tofa_core::store::Vault::load(&s.vault_path, &passphrase)
        .map_err(|_| "Failed to open new vault.".to_string())?;
    s.cache.unlock(passphrase);
    let _ = app.emit("session-unlocked", ());
    entries_from_vault(&vault)
}

#[tauri::command]
pub fn unlock(passphrase: String, state: State<Mutex<AppState>>, app: tauri::AppHandle) -> Result<Vec<OtpEntry>, String> {
    let mut s = state.lock().map_err(|e| e.to_string())?;
    if !s.vault_path.exists() {
        return Err(format!(
            "Vault not found at {}. Open Settings to set the correct path.",
            s.vault_path.display()
        ));
    }
    let vault = tofa_core::store::Vault::load(&s.vault_path, &passphrase)
        .map_err(|_| "Wrong passphrase.".to_string())?;
    s.cache.unlock(passphrase);
    let _ = app.emit("session-unlocked", ());
    entries_from_vault(&vault)
}

#[tauri::command]
pub async fn get_entries(state: State<'_, Mutex<AppState>>) -> Result<Vec<OtpEntry>, String> {
    let (vault_path, passphrase) = {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        let p = s.cache.with_passphrase(|p| Zeroizing::new(p.to_string()))
            .ok_or("locked")?;
        (s.vault_path.clone(), p)
    };
    tokio::task::spawn_blocking(move || {
        let vault = tofa_core::store::Vault::load(&vault_path, &passphrase)
            .map_err(|e| e.to_string())?;
        entries_from_vault(&vault)
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn copy_code(name: String, state: State<'_, Mutex<AppState>>, app: tauri::AppHandle) -> Result<(), String> {
    let (vault_path, passphrase) = {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        let p = s.cache.with_passphrase(|p| Zeroizing::new(p.to_string()))
            .ok_or("locked")?;
        (s.vault_path.clone(), p)
    };
    let code = tokio::task::spawn_blocking(move || {
        let vault = tofa_core::store::Vault::load(&vault_path, &passphrase)
            .map_err(|e| e.to_string())?;
        let entry = vault.entries().iter()
            .find(|e| e.name == name)
            .ok_or_else(|| format!("entry '{}' not found", name))?
            .clone();
        let raw = tofa_core::totp::generate_code_now(&entry).map_err(|e| e.to_string())?;
        Ok::<String, String>(tofa_core::totp::format_code(&raw))
    }).await.map_err(|e| e.to_string())??;
    use tauri_plugin_clipboard_manager::ClipboardExt;
    app.clipboard().write_text(code).map_err(|e| e.to_string())?;
    Ok(())
}

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
pub fn save_settings(settings: Settings, state: State<Mutex<AppState>>) -> Result<(), String> {
    let path = settings_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let s = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    std::fs::write(&path, s).map_err(|e| e.to_string())?;
    let mut st = state.lock().map_err(|e| e.to_string())?;
    st.vault_path = std::path::PathBuf::from(&settings.vault_path);
    st.cache.lock();
    Ok(())
}

#[tauri::command]
pub fn lock(state: State<Mutex<AppState>>, app: tauri::AppHandle) -> Result<(), String> {
    let mut s = state.lock().map_err(|e| e.to_string())?;
    s.cache.lock();
    let _ = app.emit("session-locked", ());
    Ok(())
}

#[tauri::command]
pub async fn delete_entry(name: String, state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let (vault_path, passphrase) = {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        let p = s.cache.with_passphrase(|p| Zeroizing::new(p.to_string()))
            .ok_or("locked")?;
        (s.vault_path.clone(), p)
    };
    tokio::task::spawn_blocking(move || {
        let mut vault = tofa_core::store::Vault::load(&vault_path, &passphrase)
            .map_err(|e| e.to_string())?;
        let idx = vault.entries().iter().position(|e| e.name == name)
            .ok_or_else(|| format!("entry '{}' not found", name))?;
        vault.remove_entry(idx);
        vault.save(&vault_path, &passphrase).map_err(|e| e.to_string())
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn scan_screen(app: tauri::AppHandle, state: State<'_, Mutex<AppState>>) -> Result<Vec<String>, String> {
    // 1. Hide window so it doesn't appear in the screenshot
    if let Some(win) = app.get_webview_window("popover") {
        let _ = win.hide();
    }
    std::thread::sleep(std::time::Duration::from_millis(250));

    // 2. Take screenshot
    let tmp = std::env::temp_dir().join(format!("tofa_scan_{}.png", std::process::id()));
    std::process::Command::new("screencapture")
        .args(["-x", "-t", "png", tmp.to_str().unwrap_or_default()])
        .status()
        .map_err(|e| e.to_string())?;

    // 3. Window reappears immediately — user sees progress while we process
    if let Some(win) = app.get_webview_window("popover") {
        let _ = win.show();
        let _ = win.set_focus();
    }
    let _ = app.emit("scan-step", "Reading screenshot…");

    // 4. Decode QR codes from screenshot
    let _ = app.emit("scan-step", "Detecting QR codes…");
    let uris = tofa_core::qr::scan_all_qr_uris(&tmp)
        .map_err(|_| "No QR code found on screen.".to_string());
    let _ = std::fs::remove_file(&tmp);
    let uris = uris?;
    let _ = app.emit("scan-step", format!("Found {} code(s) — decoding…", uris.len()));

    // 5. Save to vault
    let (vault_path, passphrase) = {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        let p = s.cache.with_passphrase(|p| Zeroizing::new(p.to_string()))
            .ok_or("locked")?;
        (s.vault_path.clone(), p)
    };

    let app_handle = app.clone();
    let added = tokio::task::spawn_blocking(move || {
        let mut vault = tofa_core::store::Vault::load(&vault_path, &passphrase)
            .map_err(|e| e.to_string())?;
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let mut added = Vec::new();

        // Expand each scanned URI — migration QRs contain multiple accounts
        let mut otps: Vec<tofa_core::qr::OtpSecret> = Vec::new();
        for uri in &uris {
            if uri.starts_with("otpauth-migration://") {
                if let Ok(accounts) = tofa_core::qr::parse_migration(uri) {
                    otps.extend(accounts);
                }
            } else if let Ok(otp) = tofa_core::qr::parse_input(uri) {
                otps.push(otp);
            }
        }

        for otp in otps {
            let name = otp.meta.derive_name();
            vault.add_entry(tofa_core::store::VaultEntry {
                name: name.clone(),
                secret: otp.secret,
                created_at: today.clone(),
                period: otp.meta.period.unwrap_or(30),
                digits: otp.meta.digits.unwrap_or(6),
                algorithm: otp.meta.algorithm.unwrap_or_else(|| "SHA1".to_string()),
            });
            added.push(name);
        }

        let _ = app_handle.emit("scan-step", "Saving to vault…");
        vault.save(&vault_path, &passphrase).map_err(|e| e.to_string())?;
        Ok::<Vec<String>, String>(added)
    }).await.map_err(|e| e.to_string())??;

    if added.is_empty() {
        Err("No valid OTP QR codes found.".to_string())
    } else {
        Ok(added)
    }
}

#[tauri::command]
pub async fn scan_image_bytes(bytes: Vec<u8>, state: State<'_, Mutex<AppState>>) -> Result<Vec<String>, String> {
    // Write bytes to a temp file, scan, then delete
    let tmp = std::env::temp_dir().join(format!("tofa_drop_{}.png", std::process::id()));
    std::fs::write(&tmp, &bytes).map_err(|e| e.to_string())?;

    let uris = tofa_core::qr::scan_all_qr_uris(&tmp)
        .map_err(|_| "No QR code found in image.".to_string());
    let _ = std::fs::remove_file(&tmp);
    let uris = uris?;

    let (vault_path, passphrase) = {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        let p = s.cache.with_passphrase(|p| Zeroizing::new(p.to_string()))
            .ok_or("locked")?;
        (s.vault_path.clone(), p)
    };

    let added = tokio::task::spawn_blocking(move || {
        let mut vault = tofa_core::store::Vault::load(&vault_path, &passphrase)
            .map_err(|e| e.to_string())?;
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let mut added = Vec::new();

        let mut otps: Vec<tofa_core::qr::OtpSecret> = Vec::new();
        for uri in &uris {
            if uri.starts_with("otpauth-migration://") {
                if let Ok(accounts) = tofa_core::qr::parse_migration(uri) {
                    otps.extend(accounts);
                }
            } else if let Ok(otp) = tofa_core::qr::parse_input(uri) {
                otps.push(otp);
            }
        }

        for otp in otps {
            let name = otp.meta.derive_name();
            vault.add_entry(tofa_core::store::VaultEntry {
                name: name.clone(),
                secret: otp.secret,
                created_at: today.clone(),
                period: otp.meta.period.unwrap_or(30),
                digits: otp.meta.digits.unwrap_or(6),
                algorithm: otp.meta.algorithm.unwrap_or_else(|| "SHA1".to_string()),
            });
            added.push(name);
        }

        vault.save(&vault_path, &passphrase).map_err(|e| e.to_string())?;
        Ok::<Vec<String>, String>(added)
    }).await.map_err(|e| e.to_string())??;

    if added.is_empty() {
        Err("No valid OTP QR codes found in image.".to_string())
    } else {
        Ok(added)
    }
}

#[tauri::command]
pub async fn add_from_uri(uri: String, name: String, state: State<'_, Mutex<AppState>>) -> Result<Vec<String>, String> {
    let (vault_path, passphrase) = {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        let p = s.cache.with_passphrase(|p| Zeroizing::new(p.to_string()))
            .ok_or("locked")?;
        (s.vault_path.clone(), p)
    };
    tokio::task::spawn_blocking(move || {
        let mut vault = tofa_core::store::Vault::load(&vault_path, &passphrase)
            .map_err(|e| e.to_string())?;
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let mut added = Vec::new();

        // Expand migration URIs into individual accounts
        let otps: Vec<tofa_core::qr::OtpSecret> = if uri.starts_with("otpauth-migration://") {
            tofa_core::qr::parse_migration(&uri).map_err(|e| e.to_string())?
        } else {
            vec![tofa_core::qr::parse_input(&uri).map_err(|e| e.to_string())?]
        };

        for otp in otps {
            let entry_name = if !name.trim().is_empty() && added.is_empty() {
                name.clone()
            } else {
                otp.meta.derive_name()
            };
            vault.add_entry(tofa_core::store::VaultEntry {
                name: entry_name.clone(),
                secret: otp.secret,
                created_at: today.clone(),
                period: otp.meta.period.unwrap_or(30),
                digits: otp.meta.digits.unwrap_or(6),
                algorithm: otp.meta.algorithm.unwrap_or_else(|| "SHA1".to_string()),
            });
            added.push(entry_name);
        }

        vault.save(&vault_path, &passphrase).map_err(|e| e.to_string())?;
        Ok(added)
    }).await.map_err(|e| e.to_string())?
}

fn entries_from_vault(vault: &tofa_core::store::Vault) -> Result<Vec<OtpEntry>, String> {
    vault.entries().iter().map(|entry| {
        let code_raw = tofa_core::totp::generate_code_now(entry)
            .map_err(|e| e.to_string())?;
        let code = tofa_core::totp::format_code(&code_raw);
        let seconds_left = tofa_core::totp::seconds_remaining_now(entry);
        // Use split_name: no colon → account=name, issuer="" (not the reverse)
        let (issuer, account) = tofa_core::qr::OtpMeta::split_name(&entry.name);
        Ok(OtpEntry {
            name: entry.name.clone(),
            issuer,
            account,
            code,
            seconds_left,
            period: entry.period,
            digits: entry.digits as u32,
            algorithm: entry.algorithm.clone(),
            created_at: entry.created_at.clone(),
        })
    }).collect()
}
