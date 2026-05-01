use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{Manager, State};
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
pub fn unlock(passphrase: String, state: State<Mutex<AppState>>) -> Result<Vec<OtpEntry>, String> {
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
    entries_from_vault(&vault)
}

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
    let code_raw = tofa_core::totp::generate_code_now(entry)
        .map_err(|e| e.to_string())?;
    let code = format_code(&code_raw);
    use tauri_plugin_clipboard_manager::ClipboardExt;
    app.clipboard().write_text(code).map_err(|e| e.to_string())?;
    if let Some(win) = app.get_webview_window("popover") {
        let _ = win.hide();
    }
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
pub fn delete_entry(name: String, state: State<Mutex<AppState>>) -> Result<(), String> {
    let mut s = state.lock().map_err(|e| e.to_string())?;
    let passphrase = s.cache.get().ok_or("locked")?.to_string();
    let mut vault = tofa_core::store::Vault::load(&s.vault_path, &passphrase)
        .map_err(|e| e.to_string())?;
    let idx = vault.entries().iter().position(|e| e.name == name)
        .ok_or_else(|| format!("entry '{}' not found", name))?;
    vault.remove_entry(idx);
    vault.save(&s.vault_path, &passphrase).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn scan_screen(app: tauri::AppHandle, state: State<Mutex<AppState>>) -> Result<Vec<String>, String> {
    if let Some(win) = app.get_webview_window("popover") {
        let _ = win.hide();
    }
    std::thread::sleep(std::time::Duration::from_millis(300));
    let tmp = std::env::temp_dir().join("tofa_scan.png");
    std::process::Command::new("screencapture")
        .args(["-x", "-t", "png", tmp.to_str().unwrap_or_default()])
        .status()
        .map_err(|e| e.to_string())?;
    let uris = tofa_core::qr::scan_all_qr_uris(&tmp)
        .map_err(|_| "No QR code found on screen.".to_string())?;
    let _ = std::fs::remove_file(&tmp);
    // Auto-import all found URIs
    let mut s = state.lock().map_err(|e| e.to_string())?;
    let passphrase = s.cache.get().ok_or("locked")?.to_string();
    let mut vault = tofa_core::store::Vault::load(&s.vault_path, &passphrase)
        .map_err(|e| e.to_string())?;
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let mut added = Vec::new();
    for uri in &uris {
        if let Ok(otp) = tofa_core::qr::parse_input(uri) {
            let name = match (&otp.meta.issuer, &otp.meta.account) {
                (Some(i), Some(a)) => format!("{i}:{a}"),
                (Some(i), None) => i.clone(),
                (None, Some(a)) => a.clone(),
                _ => format!("Imported-{}", vault.entries().len() + 1),
            };
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
    }
    vault.save(&s.vault_path, &passphrase).map_err(|e| e.to_string())?;
    if let Some(win) = app.get_webview_window("popover") {
        let _ = win.show();
        let _ = win.set_focus();
    }
    if added.is_empty() {
        Err("No valid OTP QR codes found.".to_string())
    } else {
        Ok(added)
    }
}

#[tauri::command]
pub fn scan_image_data(data: String) -> Result<String, String> {
    use base64::{Engine, engine::general_purpose::STANDARD};
    let bytes = STANDARD.decode(&data).map_err(|e| e.to_string())?;
    let tmp = std::env::temp_dir().join("tofa_camera_frame.png");
    std::fs::write(&tmp, &bytes).map_err(|e| e.to_string())?;
    let uri = tofa_core::qr::scan_qr_uri(&tmp)
        .map_err(|_| "No QR code detected.".to_string())?;
    let _ = std::fs::remove_file(&tmp);
    Ok(uri)
}

#[tauri::command]
pub fn add_from_uri(uri: String, name: String, state: State<Mutex<AppState>>) -> Result<(), String> {
    let mut s = state.lock().map_err(|e| e.to_string())?;
    let passphrase = s.cache.get().ok_or("locked")?.to_string();
    let mut vault = tofa_core::store::Vault::load(&s.vault_path, &passphrase)
        .map_err(|e| e.to_string())?;
    let otp = tofa_core::qr::parse_input(&uri).map_err(|e| e.to_string())?;
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let entry_name = if name.trim().is_empty() {
        match (&otp.meta.issuer, &otp.meta.account) {
            (Some(i), Some(a)) => format!("{i}:{a}"),
            (Some(i), None) => i.clone(),
            (None, Some(a)) => a.clone(),
            _ => "Imported".to_string(),
        }
    } else {
        name
    };
    vault.add_entry(tofa_core::store::VaultEntry {
        name: entry_name,
        secret: otp.secret,
        created_at: today,
        period: otp.meta.period.unwrap_or(30),
        digits: otp.meta.digits.unwrap_or(6),
        algorithm: otp.meta.algorithm.unwrap_or_else(|| "SHA1".to_string()),
    });
    vault.save(&s.vault_path, &passphrase).map_err(|e| e.to_string())
}

fn format_code(raw: &str) -> String {
    tofa_core::totp::format_code(raw)
}

fn entries_from_vault(vault: &tofa_core::store::Vault) -> Result<Vec<OtpEntry>, String> {
    vault.entries().iter().map(|entry| {
        let code_raw = tofa_core::totp::generate_code_now(entry)
            .map_err(|e| e.to_string())?;
        let code = format_code(&code_raw);
        let seconds_left = tofa_core::totp::seconds_remaining_now(entry);
        let (issuer, account) = if let Some(pos) = entry.name.find(':') {
            (entry.name[..pos].to_string(), entry.name[pos + 1..].to_string())
        } else {
            (entry.name.clone(), String::new())
        };
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
