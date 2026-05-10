use crate::state::{default_vault_path, settings_path, AppState};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{Emitter, Manager, State};
use zeroize::Zeroizing;

#[derive(Serialize)]
pub struct OtpEntry {
    pub id: String,
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
    #[serde(default = "default_theme")]
    pub theme: String, // "system" | "dark" | "light"
}

fn default_theme() -> String {
    "system".to_string()
}

#[tauri::command]
pub fn vault_exists(state: State<Mutex<AppState>>) -> bool {
    state.lock().map(|s| s.vault_path.exists()).unwrap_or(false)
}

#[tauri::command]
pub async fn create_vault(
    passphrase: String,
    state: State<'_, Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<Vec<OtpEntry>, String> {
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
    })
    .await
    .map_err(|e| e.to_string())??;

    let mut s = state.lock().map_err(|e| e.to_string())?;
    let vault = tofa_core::store::Vault::load(&s.vault_path, &passphrase)
        .map_err(|_| "Failed to open new vault.".to_string())?;
    s.cache.unlock(passphrase);
    let _ = app.emit("session-unlocked", ());
    entries_from_vault(&vault)
}

#[tauri::command]
pub fn unlock(
    passphrase: String,
    state: State<Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<Vec<OtpEntry>, String> {
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
        let p = s
            .cache
            .with_passphrase(|p| Zeroizing::new(p.to_string()))
            .ok_or("locked")?;
        (s.vault_path.clone(), p)
    };
    tokio::task::spawn_blocking(move || {
        let vault =
            tofa_core::store::Vault::load(&vault_path, &passphrase).map_err(|e| e.to_string())?;
        entries_from_vault(&vault)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn copy_code(
    id: String,
    state: State<'_, Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let (vault_path, passphrase) = {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        let p = s
            .cache
            .with_passphrase(|p| Zeroizing::new(p.to_string()))
            .ok_or("locked")?;
        (s.vault_path.clone(), p)
    };
    let code = tokio::task::spawn_blocking(move || {
        let vault =
            tofa_core::store::Vault::load(&vault_path, &passphrase).map_err(|e| e.to_string())?;
        let entry = vault
            .entry_by_id(&id)
            .ok_or_else(|| format!("entry '{}' not found", id))?
            .clone();
        let raw = tofa_core::totp::generate_code_now(&entry).map_err(|e| e.to_string())?;
        Ok::<String, String>(tofa_core::totp::format_code(&raw))
    })
    .await
    .map_err(|e| e.to_string())??;
    use tauri_plugin_clipboard_manager::ClipboardExt;
    app.clipboard()
        .write_text(code)
        .map_err(|e| e.to_string())?;
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
            theme: default_theme(),
        })
    }
}

#[tauri::command]
pub async fn pick_and_import_file(
    window: tauri::Window,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<String>, String> {
    let handle = window.app_handle().clone();
    let path = tokio::task::spawn_blocking(move || {
        use tauri_plugin_dialog::DialogExt;
        Ok::<_, String>(
            handle
                .dialog()
                .file()
                .set_title("Open QR image or import file")
                .add_filter(
                    "Supported files",
                    &[
                        "png", "jpg", "jpeg", "gif", "bmp", "webp", "tiff", "json", "2fas", "txt",
                        "csv", "zip",
                    ],
                )
                .blocking_pick_file(),
        )
    })
    .await
    .map_err(|e| e.to_string())??;

    let path = match path {
        None => return Ok(vec![]),
        Some(p) => p,
    };

    let path = path.into_path().map_err(|e| e.to_string())?;
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();
    let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;

    let (vault_path, passphrase) = {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        let p = s
            .cache
            .with_passphrase(|p| Zeroizing::new(p.to_string()))
            .ok_or("locked")?;
        (s.vault_path.clone(), p)
    };

    tokio::task::spawn_blocking(move || {
        let otps = extract_otps_from_bytes(&filename, &bytes)?;
        let mut vault =
            tofa_core::store::Vault::load(&vault_path, &passphrase).map_err(|e| e.to_string())?;
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let mut added = Vec::new();
        for otp in otps {
            let name = otp.meta.derive_name();
            vault.add_entry(tofa_core::store::VaultEntry {
                id: String::new(),
                name: name.clone(),
                secret: otp.secret,
                created_at: today.clone(),
                period: otp.meta.period.unwrap_or(30),
                digits: otp.meta.digits.unwrap_or(6),
                algorithm: otp.meta.algorithm.unwrap_or_else(|| "SHA1".to_string()),
            });
            added.push(name);
        }
        vault
            .save(&vault_path, &passphrase)
            .map_err(|e| e.to_string())?;
        Ok(added)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn pick_vault_folder(window: tauri::Window) -> Result<Option<String>, String> {
    let handle = window.app_handle().clone();
    tokio::task::spawn_blocking(move || {
        use tauri_plugin_dialog::DialogExt;
        let folder = handle
            .dialog()
            .file()
            .set_title("Select vault folder")
            .blocking_pick_folder();
        Ok(folder.map(|p| p.to_string()))
    })
    .await
    .map_err(|e| e.to_string())?
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
pub fn get_secret(
    id: String,
    passphrase: String,
    state: State<Mutex<AppState>>,
) -> Result<String, String> {
    let s = state.lock().map_err(|e| e.to_string())?;
    let vault = tofa_core::store::Vault::load(&s.vault_path, &passphrase)
        .map_err(|_| "Wrong passphrase.".to_string())?;
    let entry = vault
        .entry_by_id(&id)
        .ok_or_else(|| format!("Entry '{}' not found.", id))?;
    Ok(entry.secret.clone())
}

#[tauri::command]
pub fn lock(state: State<Mutex<AppState>>, app: tauri::AppHandle) -> Result<(), String> {
    let mut s = state.lock().map_err(|e| e.to_string())?;
    s.cache.lock();
    let _ = app.emit("session-locked", ());
    Ok(())
}

#[tauri::command]
pub async fn delete_entry(id: String, state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let (vault_path, passphrase) = {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        let p = s
            .cache
            .with_passphrase(|p| Zeroizing::new(p.to_string()))
            .ok_or("locked")?;
        (s.vault_path.clone(), p)
    };
    tokio::task::spawn_blocking(move || {
        let mut vault =
            tofa_core::store::Vault::load(&vault_path, &passphrase).map_err(|e| e.to_string())?;
        if !vault.remove_by_id(&id) {
            return Err(format!("entry '{}' not found", id));
        }
        vault
            .save(&vault_path, &passphrase)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn scan_screen(
    app: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<String>, String> {
    // 1. Request screen recording permission (macOS Sequoia requires explicit API call)
    #[cfg(target_os = "macos")]
    {
        extern "C" {
            fn CGRequestScreenCaptureAccess() -> bool;
        }
        let granted = unsafe { CGRequestScreenCaptureAccess() };
        if !granted {
            return Err("Screen recording permission denied. Enable it in System Settings → Privacy & Security → Screen Recording.".to_string());
        }
    }

    // 2. Hide window so it doesn't appear in the screenshot
    if let Some(win) = app.get_webview_window("popover") {
        let _ = win.hide();
    }
    std::thread::sleep(std::time::Duration::from_millis(250));

    // 3. Take screenshot
    let tmp = std::env::temp_dir().join(format!("tofa_scan_{}.png", std::process::id()));
    std::process::Command::new("screencapture")
        .args(["-x", "-t", "png", tmp.to_str().unwrap_or_default()])
        .status()
        .map_err(|e| e.to_string())?;

    // 4. Window reappears immediately — user sees progress while we process
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
    let _ = app.emit(
        "scan-step",
        format!("Found {} code(s) — decoding…", uris.len()),
    );

    // 5. Save to vault
    let (vault_path, passphrase) = {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        let p = s
            .cache
            .with_passphrase(|p| Zeroizing::new(p.to_string()))
            .ok_or("locked")?;
        (s.vault_path.clone(), p)
    };

    let app_handle = app.clone();
    let added = tokio::task::spawn_blocking(move || {
        let mut vault =
            tofa_core::store::Vault::load(&vault_path, &passphrase).map_err(|e| e.to_string())?;
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
                id: String::new(),
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
        vault
            .save(&vault_path, &passphrase)
            .map_err(|e| e.to_string())?;
        Ok::<Vec<String>, String>(added)
    })
    .await
    .map_err(|e| e.to_string())??;

    if added.is_empty() {
        Err("No valid OTP QR codes found.".to_string())
    } else {
        Ok(added)
    }
}

#[tauri::command]
pub async fn scan_image_bytes(
    b64: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<String>, String> {
    use base64::Engine;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(&b64)
        .map_err(|e| e.to_string())?;
    let tmp = std::env::temp_dir().join(format!("tofa_drop_{}.png", std::process::id()));
    std::fs::write(&tmp, &bytes).map_err(|e| e.to_string())?;

    let uris =
        tofa_core::qr::scan_all_qr_uris(&tmp).map_err(|_| "No QR code found in image.".to_string());
    let _ = std::fs::remove_file(&tmp);
    let uris = uris?;

    let (vault_path, passphrase) = {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        let p = s
            .cache
            .with_passphrase(|p| Zeroizing::new(p.to_string()))
            .ok_or("locked")?;
        (s.vault_path.clone(), p)
    };

    let added = tokio::task::spawn_blocking(move || {
        let mut vault =
            tofa_core::store::Vault::load(&vault_path, &passphrase).map_err(|e| e.to_string())?;
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
                id: String::new(),
                name: name.clone(),
                secret: otp.secret,
                created_at: today.clone(),
                period: otp.meta.period.unwrap_or(30),
                digits: otp.meta.digits.unwrap_or(6),
                algorithm: otp.meta.algorithm.unwrap_or_else(|| "SHA1".to_string()),
            });
            added.push(name);
        }

        vault
            .save(&vault_path, &passphrase)
            .map_err(|e| e.to_string())?;
        Ok::<Vec<String>, String>(added)
    })
    .await
    .map_err(|e| e.to_string())??;

    if added.is_empty() {
        Err("No valid OTP QR codes found in image.".to_string())
    } else {
        Ok(added)
    }
}

#[tauri::command]
pub async fn add_from_uri(
    uri: String,
    name: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<String>, String> {
    let (vault_path, passphrase) = {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        let p = s
            .cache
            .with_passphrase(|p| Zeroizing::new(p.to_string()))
            .ok_or("locked")?;
        (s.vault_path.clone(), p)
    };
    tokio::task::spawn_blocking(move || {
        let mut vault =
            tofa_core::store::Vault::load(&vault_path, &passphrase).map_err(|e| e.to_string())?;
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
                id: String::new(),
                name: entry_name.clone(),
                secret: otp.secret,
                created_at: today.clone(),
                period: otp.meta.period.unwrap_or(30),
                digits: otp.meta.digits.unwrap_or(6),
                algorithm: otp.meta.algorithm.unwrap_or_else(|| "SHA1".to_string()),
            });
            added.push(entry_name);
        }

        vault
            .save(&vault_path, &passphrase)
            .map_err(|e| e.to_string())?;
        Ok(added)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn scan_camera(state: State<'_, Mutex<AppState>>) -> Result<Vec<String>, String> {
    use std::io::{BufRead, BufReader, Write as _};
    use std::net::TcpListener;

    let (vault_path, passphrase) = {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        let p = s
            .cache
            .with_passphrase(|p| Zeroizing::new(p.to_string()))
            .ok_or("locked")?;
        (s.vault_path.clone(), p)
    };

    let html = cam_html();

    let uri = tokio::task::spawn_blocking(move || -> Result<String, String> {
        let listener = TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
        let port = listener.local_addr().map_err(|e| e.to_string())?.port();
        let url = format!("http://127.0.0.1:{port}");

        #[cfg(target_os = "macos")]
        std::process::Command::new("open").arg(&url).spawn().ok();

        for stream in listener.incoming() {
            let Ok(mut stream) = stream else { continue };
            let mut reader = BufReader::new(stream.try_clone().map_err(|e| e.to_string())?);
            let mut req_line = String::new();
            reader.read_line(&mut req_line).ok();
            let mut content_length = 0usize;
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).ok();
                if line.trim().is_empty() { break; }
                if let Some(v) = line.trim().strip_prefix("Content-Length:") {
                    content_length = v.trim().parse().unwrap_or(0);
                }
            }
            if req_line.starts_with("GET /") {
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
                    html.len(), html
                );
                stream.write_all(resp.as_bytes()).ok();
            } else if req_line.starts_with("POST /result") && content_length > 0 && content_length <= 8192 {
                let mut body = vec![0u8; content_length];
                use std::io::Read;
                reader.read_exact(&mut body).ok();
                stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n").ok();
                return Ok(String::from_utf8_lossy(&body).to_string());
            } else {
                stream.write_all(b"HTTP/1.1 204 No Content\r\nContent-Length: 0\r\n\r\n").ok();
            }
        }
        Err("No QR code received.".to_string())
    }).await.map_err(|e| e.to_string())??;

    if uri.is_empty() {
        return Err("No QR code received.".to_string());
    }

    tokio::task::spawn_blocking(move || {
        let mut vault =
            tofa_core::store::Vault::load(&vault_path, &passphrase).map_err(|e| e.to_string())?;
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let mut added = Vec::new();

        let otps: Vec<tofa_core::qr::OtpSecret> = if uri.starts_with("otpauth-migration://") {
            tofa_core::qr::parse_migration(&uri).map_err(|e| e.to_string())?
        } else {
            vec![tofa_core::qr::parse_input(&uri).map_err(|e| e.to_string())?]
        };

        for otp in otps {
            let name = otp.meta.derive_name();
            vault.add_entry(tofa_core::store::VaultEntry {
                id: String::new(),
                name: name.clone(),
                secret: otp.secret,
                created_at: today.clone(),
                period: otp.meta.period.unwrap_or(30),
                digits: otp.meta.digits.unwrap_or(6),
                algorithm: otp.meta.algorithm.unwrap_or_else(|| "SHA1".to_string()),
            });
            added.push(name);
        }

        vault
            .save(&vault_path, &passphrase)
            .map_err(|e| e.to_string())?;
        Ok(added)
    })
    .await
    .map_err(|e| e.to_string())?
}

fn cam_html() -> String {
    include_str!("cam.html").to_string()
}

/// Extract OtpSecret entries from raw bytes + filename hint.
/// Supports: images (QR), JSON (Aegis / andOTP / plain URI list), TXT (URI lines), ZIP (recursive).
fn extract_otps_from_bytes(
    filename: &str,
    bytes: &[u8],
) -> Result<Vec<tofa_core::qr::OtpSecret>, String> {
    let ext = std::path::Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "tiff" => {
            let tmp =
                std::env::temp_dir().join(format!("tofa_import_{}.{}", std::process::id(), ext));
            std::fs::write(&tmp, bytes).map_err(|e| e.to_string())?;
            let uris = tofa_core::qr::scan_all_qr_uris(&tmp);
            let _ = std::fs::remove_file(&tmp);
            let uris = uris.map_err(|_| "No QR code found in image.".to_string())?;
            collect_otps_from_uris(&uris)
        }
        "json" | "2fas" => parse_json_import(bytes),
        "csv" => {
            let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
            tofa_core::import::parse_csv(text)
        }
        "txt" => {
            let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
            let uris: Vec<String> = text
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty())
                .map(String::from)
                .collect();
            collect_otps_from_uris(&uris)
        }
        "zip" => {
            use std::io::Read;
            let cursor = std::io::Cursor::new(bytes);
            let mut archive = zip::ZipArchive::new(cursor).map_err(|e| e.to_string())?;
            let mut all = Vec::new();
            let names: Vec<String> = (0..archive.len())
                .filter_map(|i| archive.by_index(i).ok().map(|f| f.name().to_string()))
                .collect();
            for name in names {
                if let Ok(mut file) = archive.by_name(&name) {
                    let mut buf = Vec::new();
                    file.read_to_end(&mut buf).ok();
                    if let Ok(mut otps) = extract_otps_from_bytes(&name, &buf) {
                        all.append(&mut otps);
                    }
                }
            }
            if all.is_empty() {
                Err("No OTP accounts found in ZIP.".to_string())
            } else {
                Ok(all)
            }
        }
        _ => Err(format!(
            "Unsupported file type: .{ext}. Try PNG, JPG, JSON, TXT, or ZIP."
        )),
    }
}

fn collect_otps_from_uris(uris: &[String]) -> Result<Vec<tofa_core::qr::OtpSecret>, String> {
    let mut otps = Vec::new();
    for uri in uris {
        if uri.starts_with("otpauth-migration://") {
            if let Ok(accounts) = tofa_core::qr::parse_migration(uri) {
                otps.extend(accounts);
            }
        } else if let Ok(otp) = tofa_core::qr::parse_input(uri) {
            otps.push(otp);
        }
    }
    if otps.is_empty() {
        Err("No valid OTP URIs found.".to_string())
    } else {
        Ok(otps)
    }
}

fn parse_json_import(bytes: &[u8]) -> Result<Vec<tofa_core::qr::OtpSecret>, String> {
    tofa_core::qr::parse_json_bytes(bytes)
}

#[tauri::command]
pub async fn import_file(
    filename: String,
    b64: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<String>, String> {
    use base64::Engine;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(&b64)
        .map_err(|e| e.to_string())?;
    let (vault_path, passphrase) = {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        let p = s
            .cache
            .with_passphrase(|p| Zeroizing::new(p.to_string()))
            .ok_or("locked")?;
        (s.vault_path.clone(), p)
    };
    tokio::task::spawn_blocking(move || {
        let otps = extract_otps_from_bytes(&filename, &bytes)?;
        let mut vault =
            tofa_core::store::Vault::load(&vault_path, &passphrase).map_err(|e| e.to_string())?;
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let mut added = Vec::new();
        for otp in otps {
            let name = otp.meta.derive_name();
            vault.add_entry(tofa_core::store::VaultEntry {
                id: String::new(),
                name: name.clone(),
                secret: otp.secret,
                created_at: today.clone(),
                period: otp.meta.period.unwrap_or(30),
                digits: otp.meta.digits.unwrap_or(6),
                algorithm: otp.meta.algorithm.unwrap_or_else(|| "SHA1".to_string()),
            });
            added.push(name);
        }
        vault
            .save(&vault_path, &passphrase)
            .map_err(|e| e.to_string())?;
        Ok(added)
    })
    .await
    .map_err(|e| e.to_string())?
}

fn entries_from_vault(vault: &tofa_core::store::Vault) -> Result<Vec<OtpEntry>, String> {
    vault
        .entries()
        .iter()
        .map(|entry| {
            let code = tofa_core::totp::generate_code_now(entry)
                .map(|raw| tofa_core::totp::format_code(&raw))
                .unwrap_or_else(|_| "------".to_string());
            let seconds_left = tofa_core::totp::seconds_remaining_now(entry);
            // Use split_name: no colon → account=name, issuer="" (not the reverse)
            let (issuer, account) = tofa_core::qr::OtpMeta::split_name(&entry.name);
            Ok(OtpEntry {
                id: entry.id.clone(),
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
        })
        .collect()
}

/// Generate a QR code PNG for a single vault entry and return it as a base64 data URI.
/// Encodes the entry as an otpauth:// URI so any authenticator can scan it.
#[tauri::command]
pub async fn generate_entry_qr(
    id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<String, String> {
    let (vault_path, passphrase) = {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        let p = s
            .cache
            .with_passphrase(|p| Zeroizing::new(p.to_string()))
            .ok_or("locked")?;
        (s.vault_path.clone(), p)
    };

    tokio::task::spawn_blocking(move || {
        let vault =
            tofa_core::store::Vault::load(&vault_path, &passphrase).map_err(|e| e.to_string())?;
        let entry = vault
            .entry_by_id(&id)
            .ok_or_else(|| format!("entry '{}' not found", id))?;

        let uri = tofa_core::qr::build_otpauth_uri(entry);

        let tmp = std::env::temp_dir().join(format!("tofa_qr_{}.png", entry.id));
        tofa_core::qr::uri_to_qr_png(&uri, &tmp).map_err(|e| e.to_string())?;
        let bytes = std::fs::read(&tmp).map_err(|e| e.to_string())?;
        let _ = std::fs::remove_file(&tmp);
        Ok(format!(
            "data:image/png;base64,{}",
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes)
        ))
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Generate a QR code PNG for a selection of vault entries using the Google
/// Authenticator migration format, returned as a base64 data URI.
#[tauri::command]
pub async fn generate_selection_qr(
    ids: Vec<String>,
    state: State<'_, Mutex<AppState>>,
) -> Result<String, String> {
    let (vault_path, passphrase) = {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        let p = s
            .cache
            .with_passphrase(|p| Zeroizing::new(p.to_string()))
            .ok_or("locked")?;
        (s.vault_path.clone(), p)
    };

    tokio::task::spawn_blocking(move || {
        let vault =
            tofa_core::store::Vault::load(&vault_path, &passphrase).map_err(|e| e.to_string())?;

        let selection: Vec<tofa_core::store::VaultEntry> = ids
            .iter()
            .filter_map(|id| vault.entry_by_id(id).cloned())
            .collect();

        if selection.is_empty() {
            return Err("No matching entries found.".to_string());
        }

        let uri = tofa_core::build_selection_uri(&selection).map_err(|e| e.to_string())?;

        let tmp = std::env::temp_dir().join("tofa_qr_export.png");
        tofa_core::qr::uri_to_qr_png(&uri, &tmp).map_err(|e| e.to_string())?;
        let bytes = std::fs::read(&tmp).map_err(|e| e.to_string())?;
        let _ = std::fs::remove_file(&tmp);
        Ok(format!(
            "data:image/png;base64,{}",
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes)
        ))
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Per-entry QR result for the multi-otpauth export.
#[derive(Serialize)]
pub struct OtpauthQrItem {
    pub name: String,
    pub data_uri: String,
}

/// Generate one otpauth:// QR PNG per selected entry, returned as a list
/// so the frontend can paginate. Each PNG preserves period/algorithm/digits
/// for its entry — use this when the migration QR can't combine the selection
/// (e.g., mixed 30s and non-30s periods).
#[tauri::command]
pub async fn generate_otpauth_list(
    ids: Vec<String>,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<OtpauthQrItem>, String> {
    let (vault_path, passphrase) = {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        let p = s
            .cache
            .with_passphrase(|p| Zeroizing::new(p.to_string()))
            .ok_or("locked")?;
        (s.vault_path.clone(), p)
    };

    tokio::task::spawn_blocking(move || {
        let vault =
            tofa_core::store::Vault::load(&vault_path, &passphrase).map_err(|e| e.to_string())?;

        let selection: Vec<tofa_core::store::VaultEntry> = ids
            .iter()
            .filter_map(|id| vault.entry_by_id(id).cloned())
            .collect();

        if selection.is_empty() {
            return Err("No matching entries found.".to_string());
        }

        let mut items = Vec::with_capacity(selection.len());
        for entry in &selection {
            let uri = tofa_core::qr::build_otpauth_uri(entry);
            let tmp = std::env::temp_dir().join(format!("tofa_qr_list_{}.png", entry.id));
            tofa_core::qr::uri_to_qr_png(&uri, &tmp).map_err(|e| e.to_string())?;
            let bytes = std::fs::read(&tmp).map_err(|e| e.to_string())?;
            let _ = std::fs::remove_file(&tmp);
            let data_uri = format!(
                "data:image/png;base64,{}",
                base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes)
            );
            items.push(OtpauthQrItem {
                name: entry.name.clone(),
                data_uri,
            });
        }
        Ok(items)
    })
    .await
    .map_err(|e| e.to_string())?
}

/// One QR PNG to package into the zip.
#[derive(Deserialize)]
pub struct QrZipItem {
    pub name: String,
    pub data_uri: String,
}

/// Bundle a list of QR PNGs into a single zip with a printable one-pager.
/// Asks the user once for the zip's destination path, then writes:
/// - `<NN>-<sanitized-name>.png` per item
/// - `print.html` — a self-contained one-pager that lists all QRs in a
///   responsive grid and is print-stylesheet-friendly (open in any
///   browser, hit Cmd-P, print or save as PDF).
#[tauri::command]
pub async fn save_qr_zip(window: tauri::Window, items: Vec<QrZipItem>) -> Result<(), String> {
    if items.is_empty() {
        return Err("nothing to save".to_string());
    }

    let handle = window.app_handle().clone();
    let path = tokio::task::spawn_blocking(move || {
        use tauri_plugin_dialog::DialogExt;
        handle
            .dialog()
            .file()
            .set_title("Save QR Codes")
            .set_file_name("tofa-qrs.zip")
            .add_filter("Zip archive", &["zip"])
            .blocking_save_file()
    })
    .await
    .map_err(|e| e.to_string())?;

    let path = match path {
        None => return Ok(()), // user cancelled
        Some(p) => p.into_path().map_err(|e| e.to_string())?,
    };

    let bytes = build_qr_zip(&items)?;
    std::fs::write(&path, &bytes).map_err(|e| e.to_string())
}

fn build_qr_zip(items: &[QrZipItem]) -> Result<Vec<u8>, String> {
    use std::io::Write;
    use zip::write::SimpleFileOptions;

    let mut buf = std::io::Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(&mut buf);
    let opts = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let mut filenames = Vec::with_capacity(items.len());
    for (i, item) in items.iter().enumerate() {
        let png = item
            .data_uri
            .strip_prefix("data:image/png;base64,")
            .ok_or_else(|| format!("item {} is not a PNG data URI", i))?;
        let png_bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, png)
            .map_err(|e| format!("base64 decode failed for item {}: {}", i, e))?;

        let filename = format!("{:02}-{}.png", i + 1, sanitize_filename_for_zip(&item.name));
        zip.start_file(&filename, opts).map_err(|e| e.to_string())?;
        zip.write_all(&png_bytes).map_err(|e| e.to_string())?;
        filenames.push((filename, item.name.clone()));
    }

    zip.start_file("print.html", opts)
        .map_err(|e| e.to_string())?;
    zip.write_all(build_print_html(&filenames).as_bytes())
        .map_err(|e| e.to_string())?;

    zip.finish().map_err(|e| e.to_string())?;
    Ok(buf.into_inner())
}

fn sanitize_filename_for_zip(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' => c,
            _ => '_',
        })
        .collect()
}

fn build_print_html(items: &[(String, String)]) -> String {
    let cells: String = items
        .iter()
        .map(|(filename, label)| {
            format!(
                r#"<figure><img src="{}" alt="{}"><figcaption>{}</figcaption></figure>"#,
                html_escape(filename),
                html_escape(label),
                html_escape(label),
            )
        })
        .collect();

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Tofa QR codes</title>
<style>
  :root {{ color-scheme: light; }}
  body {{
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    margin: 24px;
    color: #111;
    background: #fff;
  }}
  h1 {{ font-size: 18px; margin: 0 0 4px; }}
  p.intro {{ font-size: 12px; color: #666; margin: 0 0 24px; }}
  .grid {{
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 24px;
  }}
  figure {{
    margin: 0;
    border: 1px solid #ddd;
    border-radius: 6px;
    padding: 12px;
    page-break-inside: avoid;
    break-inside: avoid;
    text-align: center;
  }}
  figure img {{
    width: 100%;
    max-width: 180px;
    height: auto;
    image-rendering: pixelated;
    background: #fff;
  }}
  figcaption {{
    font-family: ui-monospace, "SF Mono", Menlo, monospace;
    font-size: 11px;
    word-break: break-all;
    margin-top: 8px;
  }}
  @media print {{
    body {{ margin: 12mm; }}
    .grid {{ gap: 12px; }}
    figure {{ border-color: #000; }}
  }}
</style>
</head>
<body>
<h1>Tofa OTP backup</h1>
<p class="intro">Each QR encodes an otpauth:// URI for one account. Scan with any authenticator app.</p>
<div class="grid">{}</div>
</body>
</html>
"#,
        cells
    )
}

fn html_escape(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#39;".to_string(),
            c => c.to_string(),
        })
        .collect()
}

#[tauri::command]
pub fn open_release_url(url: String) -> Result<(), String> {
    if !url.starts_with("https://github.com/") {
        return Err("refusing to open non-github URL".into());
    }
    open::that(&url).map_err(|e| e.to_string())
}

#[derive(Serialize)]
pub struct Versions {
    pub app: String,
    pub core: String,
}

#[tauri::command]
pub fn get_versions() -> Versions {
    Versions {
        app: env!("CARGO_PKG_VERSION").to_string(),
        core: tofa_core::VERSION.to_string(),
    }
}

use crate::updater::{self, UpdateStatus, UpdaterState};
use semver::Version;

#[derive(Serialize, Deserialize, Clone)]
pub struct CheckResult {
    pub current: String,
    pub latest: Option<String>,
    pub release_url: Option<String>,
    pub update_available: bool,
    pub error: Option<String>,
}

impl CheckResult {
    pub(crate) fn from_status(s: &UpdateStatus) -> Self {
        Self {
            current: s.current.to_string(),
            latest: s.latest.as_ref().map(|v| v.to_string()),
            release_url: s.release_url.clone(),
            update_available: s.is_update_available(),
            error: None,
        }
    }

    fn error(current: &Version, msg: String) -> Self {
        Self {
            current: current.to_string(),
            latest: None,
            release_url: None,
            update_available: false,
            error: Some(msg),
        }
    }
}

#[tauri::command]
pub fn get_cached_update_status(state: State<'_, Mutex<UpdaterState>>) -> Option<CheckResult> {
    let s = state.lock().ok()?;
    s.last_status.as_ref().map(CheckResult::from_status)
}

#[tauri::command]
pub async fn check_for_updates(
    state: State<'_, Mutex<UpdaterState>>,
    app: tauri::AppHandle,
) -> Result<CheckResult, String> {
    let current = Version::parse(env!("CARGO_PKG_VERSION"))
        .map_err(|e| format!("invalid app version: {}", e))?;

    {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        if s.in_flight {
            // Return whatever we have; don't fire a parallel request.
            return Ok(s
                .last_status
                .as_ref()
                .map(CheckResult::from_status)
                .unwrap_or_else(|| {
                    CheckResult::error(&current, "check already in progress".into())
                }));
        }
        s.in_flight = true;
    }

    let http = updater::build_http_client(&current);
    let result = updater::fetch_and_check(&http, &current).await;

    let mut s = state.lock().map_err(|e| e.to_string())?;
    s.in_flight = false;

    match result {
        Ok(status) => {
            let payload = CheckResult::from_status(&status);
            s.last_status = Some(status);
            if payload.update_available {
                let _ = app.emit("update-available", payload.clone());
            }
            Ok(payload)
        }
        Err(e) => Ok(CheckResult::error(&current, e.to_string())),
    }
}

#[derive(serde::Serialize, Clone)]
pub struct UpdateInfo {
    pub current: String,
    pub latest: Option<String>,
    pub available: bool,
}

#[tauri::command]
pub async fn check_for_updates_v2(app: tauri::AppHandle) -> Result<UpdateInfo, String> {
    use tauri_plugin_updater::UpdaterExt;
    let current = env!("CARGO_PKG_VERSION").to_string();
    match app.updater().map_err(|e| e.to_string())?.check().await {
        Ok(Some(update)) => Ok(UpdateInfo {
            current,
            latest: Some(update.version.clone()),
            available: true,
        }),
        Ok(None) => Ok(UpdateInfo {
            current,
            latest: None,
            available: false,
        }),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn download_and_install(app: tauri::AppHandle) -> Result<(), String> {
    use tauri_plugin_updater::UpdaterExt;
    let update = app
        .updater()
        .map_err(|e| e.to_string())?
        .check()
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "no update available".to_string())?;
    update
        .download_and_install(|_chunk, _total| {}, || {})
        .await
        .map_err(|e| e.to_string())?;
    app.restart();
}

/// Save a base64-encoded PNG to a user-chosen location via native save dialog.
#[tauri::command]
pub async fn save_qr_png(
    window: tauri::Window,
    data_uri: String,
    filename: String,
) -> Result<(), String> {
    let bytes = data_uri
        .strip_prefix("data:image/png;base64,")
        .ok_or("invalid data URI")?;
    let bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, bytes)
        .map_err(|e| e.to_string())?;

    let handle = window.app_handle().clone();
    let path = tokio::task::spawn_blocking(move || {
        use tauri_plugin_dialog::DialogExt;
        handle
            .dialog()
            .file()
            .set_title("Save QR Code")
            .set_file_name(&filename)
            .add_filter("PNG image", &["png"])
            .blocking_save_file()
    })
    .await
    .map_err(|e| e.to_string())?;

    let path = match path {
        None => return Ok(()), // user cancelled
        Some(p) => p.into_path().map_err(|e| e.to_string())?,
    };

    std::fs::write(&path, &bytes).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    fn item(name: &str) -> QrZipItem {
        // 1x1 transparent PNG, base64-encoded — not a valid QR but a valid PNG
        // for the encoder's "decode this base64" path.
        let png_b64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=";
        QrZipItem {
            name: name.to_string(),
            data_uri: format!("data:image/png;base64,{png_b64}"),
        }
    }

    fn zip_filenames(zip_bytes: &[u8]) -> Vec<String> {
        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(zip_bytes)).expect("read zip");
        (0..archive.len())
            .map(|i| archive.by_index(i).unwrap().name().to_string())
            .collect()
    }

    fn zip_file_contents(zip_bytes: &[u8], name: &str) -> Vec<u8> {
        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(zip_bytes)).expect("read zip");
        let mut file = archive.by_name(name).expect("file in zip");
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).expect("read file");
        buf
    }

    #[test]
    fn build_qr_zip_writes_one_png_per_item_plus_print_html() {
        let items = vec![item("Foo:alice"), item("Bar:bob"), item("Baz:carol")];
        let zip = build_qr_zip(&items).expect("build zip");
        let names = zip_filenames(&zip);
        assert!(names.contains(&"01-Foo_alice.png".to_string()));
        assert!(names.contains(&"02-Bar_bob.png".to_string()));
        assert!(names.contains(&"03-Baz_carol.png".to_string()));
        assert!(names.contains(&"print.html".to_string()));
    }

    #[test]
    fn build_qr_zip_print_html_references_each_png() {
        let items = vec![item("Foo:alice"), item("Bar:bob")];
        let zip = build_qr_zip(&items).expect("build zip");
        let html = String::from_utf8(zip_file_contents(&zip, "print.html")).expect("utf8");
        assert!(html.contains("<img src=\"01-Foo_alice.png\""));
        assert!(html.contains("<img src=\"02-Bar_bob.png\""));
        assert!(html.contains("Foo:alice"));
        assert!(html.contains("Bar:bob"));
    }

    #[test]
    fn build_qr_zip_html_escapes_entry_names() {
        // Names with HTML-special characters must not break the page or open
        // an injection vector when the user opens print.html in a browser.
        let evil_item = QrZipItem {
            name: "Evil <script>".to_string(),
            data_uri: item("x").data_uri,
        };
        let zip = build_qr_zip(&[evil_item]).expect("build zip");
        let html = String::from_utf8(zip_file_contents(&zip, "print.html")).expect("utf8");
        assert!(!html.contains("<script>"), "raw <script> found in html");
        assert!(html.contains("Evil &lt;script&gt;"));
    }

    #[test]
    fn build_qr_zip_rejects_non_png_data_uri() {
        let bad = QrZipItem {
            name: "x".to_string(),
            data_uri: "data:text/plain;base64,aGVsbG8=".to_string(),
        };
        assert!(build_qr_zip(&[bad]).is_err());
    }

    #[test]
    fn sanitize_filename_for_zip_strips_path_separators_and_colons() {
        assert_eq!(
            sanitize_filename_for_zip("Issuer:account"),
            "Issuer_account"
        );
        assert_eq!(sanitize_filename_for_zip("a/b\\c"), "a_b_c");
        assert_eq!(sanitize_filename_for_zip("plain"), "plain");
    }
}
