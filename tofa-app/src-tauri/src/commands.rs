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
