use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{Manager, State};
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
pub fn save_settings(settings: Settings) -> Result<(), String> {
    let path = settings_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let s = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    std::fs::write(&path, s).map_err(|e| e.to_string())
}

fn format_code(raw: &str) -> String {
    if raw.len() == 6 {
        format!("{} {}", &raw[..3], &raw[3..])
    } else {
        raw.to_string()
    }
}

fn entries_from_vault(vault: &tofa_core::store::Vault) -> Result<Vec<OtpEntry>, String> {
    vault.entries().iter().map(|entry| {
        let code_raw = tofa_core::totp::generate_code_now(entry)
            .map_err(|e| e.to_string())?;
        let code = format_code(&code_raw);
        let seconds_left = tofa_core::totp::seconds_remaining_now(entry);
        Ok(OtpEntry {
            name: entry.name.clone(),
            code,
            seconds_left,
            period: entry.period,
        })
    }).collect()
}
