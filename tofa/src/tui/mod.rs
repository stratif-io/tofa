pub mod screens;
pub mod state;

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseButton,
        MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use state::{AppState, OtpMetaDisplay, Screen};
use std::{
    io,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use tofa_core::{
    qr::{parse_input, OtpSecret},
    store::{Vault, VaultEntry},
    totp::generate_code_now,
    uri_to_qr_lines,
};

/// Extensions the file picker exposes. Mirrors the desktop app's drop
/// handler so users see the same surface across TUI and app. Anything
/// the unified `tofa_core::import::parse_file` dispatcher can handle
/// belongs here.
const SUPPORTED_EXTS: &[&str] = &[
    // Images: png/jpg/etc. with one or many QRs (otpauth or migration).
    "png", "jpg", "jpeg", "gif", "bmp", "webp", "tiff",
    // Archive of QR images (round-trips the desktop app's "Save All").
    "zip", // Other-app exports.
    "json", "2fas", "csv", "txt",
];
use zeroize::Zeroizing;

pub fn default_vault_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("tofa")
        .join("vault.enc")
}

pub fn run(vault_override: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, vault_override);

    // Always restore terminal regardless of result
    let _ = disable_raw_mode();
    let _ = execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );
    let _ = terminal.show_cursor();

    res
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    vault_override: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app_state = AppState::new();
    let mut vault: Option<Vault> = None;
    let path = vault_override
        .map(|p| {
            if p.starts_with("~") {
                dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join(p.strip_prefix("~").unwrap_or(&p))
            } else {
                p
            }
        })
        .unwrap_or_else(default_vault_path);
    app_state.is_new_vault = !path.exists();

    loop {
        terminal.draw(|f| {
            let area = f.area();
            match app_state.screen {
                Screen::Unlock => screens::unlock::render(f, area, &app_state, &path),
                Screen::List => screens::list::render(
                    f,
                    area,
                    &app_state,
                    vault
                        .as_ref()
                        .expect("vault must be initialized after unlock"),
                ),
                Screen::Fullscreen => {
                    let v = vault
                        .as_ref()
                        .expect("vault must be initialized after unlock");
                    screens::list::render(f, area, &app_state, v);
                    screens::fullscreen::render(f, area, &app_state, v);
                }
                Screen::AddForm => screens::add_form::render(f, area, &app_state),
                Screen::AddName => screens::add_name::render(f, area, &app_state),
                Screen::DeleteConfirm => screens::delete_confirm::render(
                    f,
                    area,
                    &app_state,
                    vault
                        .as_ref()
                        .expect("vault must be initialized after unlock"),
                ),
                Screen::FilePicker => screens::file_picker::render(f, area, &app_state),
                Screen::OtpDetail => {
                    let v = vault
                        .as_ref()
                        .expect("vault must be initialized after unlock");
                    screens::list::render(f, area, &app_state, v);
                    screens::otp_detail::render(f, area, &app_state, v);
                }
                Screen::Export => {
                    let v = vault
                        .as_ref()
                        .expect("vault must be initialized after unlock");
                    screens::list::render(f, area, &app_state, v);
                    screens::export::render(f, area, &app_state, v);
                }
                Screen::ExportQr => screens::export_qr::render(f, area, &app_state),
                Screen::ExportOtpauthList => {
                    screens::export_otpauth_list::render(f, area, &app_state)
                }
                Screen::ScanningQr => {
                    let v = vault
                        .as_ref()
                        .expect("vault must be initialized after unlock");
                    screens::list::render(f, area, &app_state, v);
                    screens::scanning_qr::render(f, area, &app_state);
                }
            }
        })?;

        // Auto-dismiss toast after 1.5s
        if let Some(at) = app_state.status_message_at {
            if at.elapsed() >= Duration::from_millis(1500) {
                app_state.status_message = None;
                app_state.status_message_at = None;
            }
        }

        // Two-phase vault write: first iteration shows "Saving…" toast,
        // second iteration executes the actual save (Argon2 ~300ms).
        if let Some(action) = app_state.pending_vault_action.take() {
            let v = vault.as_mut().expect("vault initialized");
            use state::PendingVaultAction;
            match action {
                PendingVaultAction::DeleteEntry(idx) => {
                    v.remove_entry(idx);
                    if app_state.selected_index > 0 && app_state.selected_index >= v.entries().len()
                    {
                        app_state.selected_index -= 1;
                    }
                    if save_vault(&mut app_state, v, &path) {
                        if v.entries().is_empty() {
                            app_state.clear_add_form();
                            app_state.screen = Screen::AddForm;
                        } else {
                            app_state.screen = Screen::List;
                            app_state.status_message = Some("Deleted.".to_string());
                            app_state.status_message_at = Some(Instant::now());
                        }
                    }
                    // On save failure: status_message is already set by save_vault (persistent)
                }
                PendingVaultAction::AddEntry => {
                    let name = app_state.add_name.trim().to_string();
                    let secret = app_state.add_parsed_secret.clone();
                    let meta = app_state.add_meta.take();
                    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
                    v.add_entry(tofa_core::store::VaultEntry {
                        id: String::new(),
                        name: name.clone(),
                        secret: secret.to_string(),
                        created_at: today,
                        period: meta.as_ref().and_then(|m| m.period).unwrap_or(30),
                        digits: meta.as_ref().and_then(|m| m.digits).unwrap_or(6),
                        algorithm: meta
                            .as_ref()
                            .and_then(|m| m.algorithm.clone())
                            .unwrap_or_else(|| "SHA1".to_string()),
                    });
                    if save_vault(&mut app_state, v, &path) {
                        app_state.selected_index = v.entries().len().saturating_sub(1);
                        app_state.clear_add_form();
                        app_state.screen = Screen::List;
                        app_state.status_message = Some(format!("Added: {name}"));
                        app_state.status_message_at = Some(Instant::now());
                    }
                    // On save failure: revert the in-memory add to prevent divergence
                    else if let Some(idx) = v.entries().iter().rposition(|e| e.name == name) {
                        v.remove_entry(idx);
                    }
                }
            }
            continue;
        }

        // Two-phase scan: first iteration switches to ScanningQr (draws loader),
        // second iteration does the actual scan and transitions.
        if matches!(app_state.screen, Screen::List | Screen::AddForm)
            && app_state.pending_scan_path.is_some()
        {
            app_state.screen = Screen::ScanningQr;
            continue; // redraw with loader visible
        }
        if app_state.screen == Screen::ScanningQr {
            if let Some(fp) = app_state.pending_scan_path.take() {
                std::thread::sleep(Duration::from_millis(120));
                let v = vault
                    .as_mut()
                    .expect("vault must be initialized after unlock");
                // Files always route through the unified dispatcher: image,
                // zip, json, csv, txt — all bulk-import with auto-derived
                // names. Typed `otpauth://` URIs still go through the
                // AddName flow via `try_parse_and_advance` (handled in the
                // Enter key handler), so the interactive naming UX is
                // preserved for that case.
                try_import_file(&mut app_state, &fp, v, &path);
            }
            continue;
        }

        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Event::Mouse(mouse) => {
                    if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                        // Dismiss toast on any click
                        if app_state.status_message.is_some() {
                            app_state.status_message = None;
                            app_state.status_message_at = None;
                        } else {
                            match app_state.screen {
                                Screen::List => {
                                    let v = vault.as_ref().expect("vault initialized");
                                    let row = mouse.row as usize;
                                    let col = mouse.column as usize;
                                    let list_start = 2;
                                    let list_end = list_start + v.entries().len() * 2;
                                    let content_width = list_row_content_width(v);
                                    if row >= list_start && row < list_end && col < content_width {
                                        let entry_row = row - list_start;
                                        let is_separator = entry_row % 2 == 1;
                                        if !is_separator {
                                            let clicked = entry_row / 2;
                                            app_state.selected_index = clicked;
                                            copy_selected_code(&mut app_state, v);
                                        }
                                    }
                                }
                                Screen::Fullscreen => {
                                    app_state.screen = Screen::List;
                                }
                                Screen::OtpDetail => {
                                    app_state.reset_detail_reveal();
                                    app_state.screen = Screen::List;
                                }
                                Screen::Export => {
                                    app_state.screen = Screen::List;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Event::Key(key) => {
                    if key.modifiers.contains(KeyModifiers::CONTROL)
                        && key.code == KeyCode::Char('c')
                    {
                        return Ok(());
                    }

                    match app_state.screen {
                        Screen::Unlock => {
                            handle_unlock_key(key.code, &mut app_state, &mut vault, &path)
                        }
                        Screen::List => {
                            if handle_list_key(
                                key.code,
                                &mut app_state,
                                vault
                                    .as_mut()
                                    .expect("vault must be initialized after unlock"),
                            )? {
                                return Ok(());
                            }
                        }
                        Screen::Fullscreen => handle_fullscreen_key(
                            key.code,
                            &mut app_state,
                            vault
                                .as_ref()
                                .expect("vault must be initialized after unlock"),
                        ),
                        Screen::AddForm => handle_add_form_key(
                            key.code,
                            &mut app_state,
                            vault
                                .as_mut()
                                .expect("vault must be initialized after unlock"),
                            &path,
                        )?,
                        Screen::AddName => handle_add_name_key(
                            key.code,
                            &mut app_state,
                            vault
                                .as_mut()
                                .expect("vault must be initialized after unlock"),
                            &path,
                        )?,
                        Screen::FilePicker => handle_file_picker_key(
                            key.code,
                            &mut app_state,
                            vault
                                .as_mut()
                                .expect("vault must be initialized after unlock"),
                            &path,
                        )?,
                        Screen::OtpDetail => handle_otp_detail_key(
                            key.code,
                            &mut app_state,
                            vault
                                .as_ref()
                                .expect("vault must be initialized after unlock"),
                        ),
                        Screen::Export => handle_export_key(
                            key.code,
                            &mut app_state,
                            vault
                                .as_ref()
                                .expect("vault must be initialized after unlock"),
                        )?,
                        Screen::ExportQr => {
                            if key.code == KeyCode::Esc {
                                app_state.screen = Screen::Export;
                            }
                        }
                        Screen::ExportOtpauthList => {
                            handle_export_otpauth_list_key(key.code, &mut app_state);
                        }
                        Screen::DeleteConfirm => handle_delete_confirm_key(
                            key.code,
                            &mut app_state,
                            vault
                                .as_mut()
                                .expect("vault must be initialized after unlock"),
                            &path,
                        )?,
                        Screen::ScanningQr => {} // inputs blocked while scanning
                    }
                }
                _ => {}
            }
        }
    }
}

fn handle_unlock_key(key: KeyCode, state: &mut AppState, vault: &mut Option<Vault>, path: &Path) {
    // Tab copies vault path to clipboard
    if key == KeyCode::Tab {
        let path_str = path.to_string_lossy().to_string();
        let _ = arboard::Clipboard::new().and_then(|mut cb| cb.set_text(path_str));
        return;
    }

    if state.unlock_confirming {
        // Second step: confirm passphrase
        match key {
            KeyCode::Char(c) => state.passphrase_confirm.push(c),
            KeyCode::Backspace => {
                state.passphrase_confirm.pop();
            }
            KeyCode::Esc => {
                state.unlock_confirming = false;
                state.passphrase_confirm = Zeroizing::new(String::new());
                state.unlock_error = false;
                state.unlock_error_msg = None;
            }
            KeyCode::Enter => {
                if *state.passphrase_input == *state.passphrase_confirm {
                    match Vault::load_or_new(path, &state.passphrase_input) {
                        Ok(v) => {
                            state.unlock_error = false;
                            state.unlock_error_msg = None;
                            state.is_new_vault = false;
                            state.unlock_confirming = false;
                            state.screen = if v.entries().is_empty() {
                                state.clear_add_form();
                                Screen::AddForm
                            } else {
                                Screen::List
                            };
                            let pass_bytes =
                                Zeroizing::new(state.passphrase_input.as_bytes().to_vec());
                            state.vault_key_cache = Some(pass_bytes);
                            state.passphrase_input.clear();
                            state.passphrase_confirm = Zeroizing::new(String::new());
                            *vault = Some(v);
                        }
                        Err(e) => {
                            state.unlock_error = true;
                            state.unlock_error_msg = Some(format!("Error: {e}"));
                            state.passphrase_confirm = Zeroizing::new(String::new());
                        }
                    }
                } else {
                    state.unlock_error = true;
                    state.unlock_error_msg = Some("Passphrases do not match.".to_string());
                    state.passphrase_confirm = Zeroizing::new(String::new());
                }
            }
            _ => {}
        }
        return;
    }

    match key {
        KeyCode::Char(c) => state.passphrase_input.push(c),
        KeyCode::Backspace => {
            state.passphrase_input.pop();
        }
        KeyCode::Enter => {
            if state.is_new_vault {
                // First-time: move to confirmation step
                if !state.passphrase_input.is_empty() {
                    state.unlock_confirming = true;
                    state.unlock_error = false;
                    state.unlock_error_msg = None;
                }
            } else {
                match Vault::load_or_new(path, &state.passphrase_input) {
                    Ok(v) => {
                        state.unlock_error = false;
                        state.unlock_error_msg = None;
                        state.screen = if v.entries().is_empty() {
                            state.clear_add_form();
                            Screen::AddForm
                        } else {
                            Screen::List
                        };
                        *vault = Some(v);
                        let pass_bytes = Zeroizing::new(state.passphrase_input.as_bytes().to_vec());
                        state.vault_key_cache = Some(pass_bytes);
                        state.passphrase_input.clear();
                    }
                    Err(_) => {
                        state.unlock_error = true;
                        state.unlock_error_msg = None;
                        state.passphrase_input.clear();
                    }
                }
            }
        }
        _ => {}
    }
}

fn handle_list_key(
    key: KeyCode,
    state: &mut AppState,
    vault: &mut Vault,
) -> Result<bool, Box<dyn std::error::Error>> {
    let len = vault.entries().len();
    // If we're accumulating a drag-dropped path, only the Char catch-all and Esc are relevant.
    let accumulating = !state.add_secret_input.is_empty();
    match key {
        KeyCode::Esc if accumulating => {
            state.add_secret_input.clear();
            state.pending_scan_path = None;
        }
        KeyCode::Char('q') if !accumulating => return Ok(true),
        KeyCode::Up | KeyCode::Char('k') if !accumulating && state.selected_index > 0 => {
            state.selected_index -= 1;
        }
        KeyCode::Up | KeyCode::Char('k') if !accumulating => {}
        KeyCode::Down | KeyCode::Char('j')
            if !accumulating && len > 0 && state.selected_index < len - 1 =>
        {
            state.selected_index += 1;
        }
        KeyCode::Down | KeyCode::Char('j') if !accumulating => {}
        KeyCode::Enter if !accumulating => {}
        KeyCode::Char(' ') if !accumulating && len > 0 => {
            state.screen = Screen::Fullscreen;
        }
        KeyCode::Char(' ') if !accumulating => {}
        KeyCode::Char('a') if !accumulating => {
            state.clear_add_form();
            state.screen = Screen::AddForm;
        }
        KeyCode::Char('d') if !accumulating && len > 0 => {
            state.screen = Screen::DeleteConfirm;
        }
        KeyCode::Char('d') if !accumulating => {}
        KeyCode::Char('h') if !accumulating => {
            state.show_codes = !state.show_codes;
        }
        KeyCode::Char('i') if !accumulating && len > 0 => {
            state.reset_detail_reveal();
            state.screen = Screen::OtpDetail;
        }
        KeyCode::Char('i') if !accumulating => {}
        KeyCode::Char('e') if !accumulating && len > 0 => {
            state.export_checked = vec![true; len];
            state.export_selected = state.selected_index.min(len.saturating_sub(1));
            state.screen = Screen::Export;
        }
        KeyCode::Char('y') if !accumulating => {
            copy_selected_code(state, vault);
        }
        KeyCode::Char('l') if !accumulating => {
            lock_screen(state);
        }
        // Accumulate only if it looks like a file path (starts with '/' or '~'),
        // or we're already mid-accumulation. Anything else is ignored.
        KeyCode::Char(c) if accumulating || c == '/' || c == '~' => {
            state.add_secret_input.push(c);
            let raw = state.add_secret_input.trim().to_string();
            let unescaped = raw.replace("\\ ", " ");
            if let Some(fp) = [unescaped.as_str(), raw.as_str()].iter().find_map(|s| {
                let p = std::path::PathBuf::from(s);
                if p.is_file() {
                    Some(p)
                } else {
                    None
                }
            }) {
                state.pending_scan_path = Some(fp);
            }
        }
        _ => {}
    }
    Ok(false)
}

fn handle_fullscreen_key(key: KeyCode, state: &mut AppState, vault: &Vault) {
    let len = vault.entries().len();
    match key {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char(' ') => state.screen = Screen::List,
        KeyCode::Char('i') if len > 0 => {
            state.reset_detail_reveal();
            state.screen = Screen::OtpDetail;
        }
        KeyCode::Char('y') => copy_selected_code(state, vault),
        KeyCode::Char('l') => lock_screen(state),
        KeyCode::Up | KeyCode::Char('k') if state.selected_index > 0 => {
            state.selected_index -= 1;
        }
        KeyCode::Down | KeyCode::Char('j') if len > 0 && state.selected_index < len - 1 => {
            state.selected_index += 1;
        }
        _ => {}
    }
}

/// Add already-parsed secrets to the vault with duplicate detection,
/// save, and surface a status toast. All of TUI's import paths
/// (file drop, file picker, typed migration URI) funnel through here.
fn try_import_secrets(
    state: &mut AppState,
    secrets: Vec<OtpSecret>,
    vault: &mut Vault,
    path: &Path,
) {
    state.clear_add_form();
    state.fp_query.clear();
    state.screen = Screen::List;

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let mut imported = 0usize;
    let mut skipped = 0usize;
    for otp in secrets {
        let name = match (&otp.meta.issuer, &otp.meta.account) {
            (Some(i), Some(a)) => format!("{i}:{a}"),
            (Some(i), None) => i.clone(),
            (None, Some(a)) => a.clone(),
            (None, None) => format!("imported-{}", vault.entries().len() + 1),
        };
        let dup = vault
            .entries()
            .iter()
            .any(|e| e.name == name && e.secret == otp.secret);
        if dup {
            skipped += 1;
            continue;
        }
        vault.add_entry(VaultEntry {
            id: String::new(),
            name,
            secret: otp.secret,
            created_at: today.clone(),
            period: otp.meta.period.unwrap_or(30),
            digits: otp.meta.digits.unwrap_or(6),
            algorithm: otp.meta.algorithm.unwrap_or_else(|| "SHA1".to_string()),
        });
        imported += 1;
    }
    if imported == 0 && skipped > 0 {
        state.status_message = Some(format!("Already imported ({skipped} duplicate(s))."));
        return;
    }
    if save_vault(state, vault, path) {
        let mut msg = format!("Imported {imported} account(s).");
        if skipped > 0 {
            msg.push_str(&format!(" Skipped {skipped} duplicate(s)."));
        }
        state.status_message = Some(msg);
    }
}

/// Detect whether the AddForm input looks like a multi-line list of
/// `otpauth://` URIs (Ente-style paste). Two or more lines starting
/// with `otpauth://` is unambiguous — a single URI never has a newline,
/// and migration URIs are caught earlier with their own prefix branch.
fn is_multi_otpauth_paste(raw: &str) -> bool {
    raw.lines()
        .map(str::trim)
        .filter(|l| l.starts_with("otpauth://"))
        .count()
        >= 2
}

/// Run the unified file dispatcher (image / zip / json / csv / txt) and
/// import everything it returns. Used by every TUI surface that takes a
/// file path: drag-dropped path, file-picker selection, typed path in
/// the AddForm.
fn try_import_file(state: &mut AppState, file: &Path, vault: &mut Vault, vault_path: &Path) {
    match tofa_core::import::parse_file(file) {
        Ok(secrets) => try_import_secrets(state, secrets, vault, vault_path),
        Err(e) => {
            state.clear_add_form();
            state.fp_query.clear();
            state.screen = Screen::List;
            state.status_message = Some(format!("Import failed: {e}"));
        }
    }
}

fn try_parse_and_advance(state: &mut AppState, raw: &str) {
    match parse_input(raw) {
        Ok(otp) => {
            state.status_message = None;
            state.add_parsed_secret = Zeroizing::new(otp.secret);
            state.add_name = match (&otp.meta.issuer, &otp.meta.account) {
                (Some(i), Some(a)) => format!("{i}:{a}"),
                (Some(i), None) => i.clone(),
                (None, Some(a)) => a.clone(),
                (None, None) => String::new(),
            };
            state.add_meta = Some(OtpMetaDisplay {
                issuer: otp.meta.issuer,
                account: otp.meta.account,
                algorithm: otp.meta.algorithm,
                digits: otp.meta.digits,
                period: otp.meta.period,
            });
            state.screen = Screen::AddName;
        }
        Err(e) => {
            state.status_message = Some(format!("Error: {e}"));
        }
    }
}

fn handle_add_form_key(
    key: KeyCode,
    state: &mut AppState,
    vault: &mut Vault,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    match key {
        KeyCode::Esc => {
            state.clear_add_form();
            state.screen = Screen::List;
        }
        KeyCode::Tab => {
            open_file_picker(state);
        }
        KeyCode::Char(c) => {
            state.add_secret_input.push(c);
            let raw = state.add_secret_input.trim().to_string();
            let unescaped = raw.replace("\\ ", " ");
            if let Some(fp) = [unescaped.as_str(), raw.as_str()].iter().find_map(|s| {
                let p = std::path::PathBuf::from(s);
                if p.is_file() {
                    Some(p)
                } else {
                    None
                }
            }) {
                state.pending_scan_path = Some(fp);
                state.screen = Screen::ScanningQr;
            }
        }
        KeyCode::Backspace => {
            state.add_secret_input.pop();
        }
        KeyCode::Enter => {
            let raw = state.add_secret_input.trim().to_string();
            if !raw.is_empty() {
                let fp = std::path::Path::new(&raw);
                if fp.is_file() {
                    try_import_file(state, fp, vault, path);
                } else if raw.starts_with("otpauth-migration://") {
                    // Pasted Google-Authenticator export URI — bulk import
                    // every account in one go, no per-entry naming step.
                    match tofa_core::import::parse_migration_uri(&raw) {
                        Ok(secrets) => try_import_secrets(state, secrets, vault, path),
                        Err(e) => {
                            state.status_message = Some(format!("Import failed: {e}"));
                        }
                    }
                } else if is_multi_otpauth_paste(&raw) {
                    // Pasted list of otpauth:// URIs (one per line, e.g.
                    // an Ente Auth export pasted directly into the form).
                    // Bulk import — single-URI naming UX doesn't fit when
                    // there are several at once.
                    match tofa_core::import::parse_text_uris(&raw) {
                        Ok(secrets) => try_import_secrets(state, secrets, vault, path),
                        Err(e) => {
                            state.status_message = Some(format!("Import failed: {e}"));
                        }
                    }
                } else {
                    // Single otpauth:// URI: keep the interactive AddName
                    // flow so the user can review and rename before saving.
                    try_parse_and_advance(state, &raw);
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_add_name_key(
    key: KeyCode,
    state: &mut AppState,
    _vault: &mut Vault,
    _path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    match key {
        KeyCode::Esc => {
            state.screen = Screen::AddForm;
            state.status_message = None;
        }
        KeyCode::Char(c) => state.add_name.push(c),
        KeyCode::Backspace => {
            state.add_name.pop();
        }
        KeyCode::Enter => {
            let name = state.add_name.trim().to_string();
            if name.is_empty() {
                state.status_message = Some("Name is required.".to_string());
                return Ok(());
            }
            state.add_name = name;
            state.pending_vault_action = Some(state::PendingVaultAction::AddEntry);
            state.status_message = Some("Saving…".to_string());
            state.status_message_at = None;
            state.screen = Screen::List;
        }
        _ => {}
    }
    Ok(())
}

fn handle_delete_confirm_key(
    key: KeyCode,
    state: &mut AppState,
    _vault: &mut Vault,
    _path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    match key {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            state.pending_vault_action =
                Some(state::PendingVaultAction::DeleteEntry(state.selected_index));
            state.status_message = Some("Saving…".to_string());
            state.status_message_at = None; // don't auto-dismiss until save completes
            state.screen = Screen::List;
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            state.screen = Screen::List;
        }
        _ => {}
    }
    Ok(())
}

fn copy_selected_code(state: &mut AppState, vault: &Vault) {
    if let Some(entry) = vault.entries().get(state.selected_index) {
        match generate_code_now(entry) {
            Ok(code) => match arboard::Clipboard::new().and_then(|mut cb| cb.set_text(code)) {
                Ok(_) => {
                    state.status_message = Some("Copied to clipboard".to_string());
                    state.status_message_at = Some(Instant::now());
                }
                Err(_) => {
                    state.status_message = Some("Clipboard unavailable".to_string());
                    state.status_message_at = Some(Instant::now());
                }
            },
            Err(e) => {
                state.status_message = Some(format!("Error: {e}"));
                state.status_message_at = Some(Instant::now());
            }
        }
    }
}

fn handle_otp_detail_key(key: KeyCode, state: &mut AppState, vault: &Vault) {
    let len = vault.entries().len();

    // Passphrase reveal sub-mode
    if state.detail_revealing {
        match key {
            KeyCode::Esc => {
                state.detail_revealing = false;
                state.detail_passphrase.clear();
            }
            KeyCode::Char(c) => state.detail_passphrase.push(c),
            KeyCode::Backspace => {
                state.detail_passphrase.pop();
            }
            KeyCode::Enter => {
                let correct = state
                    .vault_key_cache
                    .as_ref()
                    .and_then(|k| std::str::from_utf8(k).ok())
                    .map(|k| k == state.detail_passphrase.as_str())
                    .unwrap_or(false);
                if correct {
                    state.detail_secret_visible = true;
                    state.detail_revealing = false;
                    state.detail_passphrase.clear();
                } else {
                    state.status_message = Some("Wrong passphrase".to_string());
                    state.status_message_at = Some(std::time::Instant::now());
                    state.detail_passphrase.clear();
                }
            }
            _ => {}
        }
        return;
    }

    match key {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('i') => {
            state.reset_detail_reveal();
            state.screen = Screen::List;
        }
        KeyCode::Char(' ') if !state.detail_revealing && len > 0 => {
            state.reset_detail_reveal();
            state.screen = Screen::Fullscreen;
        }
        KeyCode::Char('y') => copy_selected_code(state, vault),
        KeyCode::Char('l') => lock_screen(state),
        KeyCode::Char('s') => {
            if state.detail_secret_visible {
                state.detail_secret_visible = false;
            } else {
                state.detail_revealing = true;
                state.detail_passphrase.clear();
            }
        }
        KeyCode::Up | KeyCode::Char('k') if state.selected_index > 0 => {
            state.reset_detail_reveal();
            state.selected_index -= 1;
        }
        KeyCode::Down | KeyCode::Char('j') if len > 0 && state.selected_index < len - 1 => {
            state.reset_detail_reveal();
            state.selected_index += 1;
        }
        _ => {}
    }
}

fn handle_export_key(
    key: KeyCode,
    state: &mut AppState,
    vault: &Vault,
) -> Result<(), Box<dyn std::error::Error>> {
    let len = vault.entries().len();
    match key {
        KeyCode::Esc => state.screen = Screen::List,
        KeyCode::Up | KeyCode::Char('k') if state.export_selected > 0 => {
            state.export_selected -= 1;
        }
        KeyCode::Down | KeyCode::Char('j') if state.export_selected < len.saturating_sub(1) => {
            state.export_selected += 1;
        }
        KeyCode::Char(' ') => {
            if let Some(v) = state.export_checked.get_mut(state.export_selected) {
                *v = !*v;
            }
        }
        KeyCode::Enter => {
            let selection: Vec<tofa_core::VaultEntry> = checked_selection(state, vault);

            match tofa_core::build_selection_uri(&selection) {
                Ok(uri) => {
                    state.export_qr_lines = uri_to_qr_lines(&uri);
                    state.screen = Screen::ExportQr;
                }
                Err(tofa_core::SelectionExportError::Empty) => {
                    state.status_message = Some("No accounts selected.".to_string());
                    state.screen = Screen::List;
                }
                Err(err) => {
                    state.status_message = Some(err.to_string());
                    state.screen = Screen::List;
                }
            }
        }
        KeyCode::Char('m') => {
            // 'm' = list-of-otpauth multi export. Each entry becomes its own QR
            // shown one at a time on the next screen — preserves period for
            // every entry, and works when the migration QR would refuse the
            // selection.
            let selection = checked_selection(state, vault);
            if selection.is_empty() {
                state.status_message = Some("No accounts selected.".to_string());
                state.screen = Screen::List;
                return Ok(());
            }
            state.otpauth_list_qrs = selection
                .iter()
                .map(|e| uri_to_qr_lines(&tofa_core::qr::build_otpauth_uri(e)))
                .collect();
            state.otpauth_list_titles = selection.iter().map(|e| e.name.clone()).collect();
            state.otpauth_list_index = 0;
            state.screen = Screen::ExportOtpauthList;
        }
        _ => {}
    }
    Ok(())
}

fn checked_selection(state: &AppState, vault: &Vault) -> Vec<tofa_core::VaultEntry> {
    vault
        .entries()
        .iter()
        .enumerate()
        .filter(|(i, _)| state.export_checked.get(*i).copied().unwrap_or(true))
        .map(|(_, e)| e.clone())
        .collect()
}

fn handle_export_otpauth_list_key(key: KeyCode, state: &mut AppState) {
    let total = state.otpauth_list_qrs.len();
    match key {
        KeyCode::Esc => state.screen = Screen::Export,
        KeyCode::Left | KeyCode::Char('h') if state.otpauth_list_index > 0 => {
            state.otpauth_list_index -= 1;
        }
        KeyCode::Right | KeyCode::Char('l')
            if total > 0 && state.otpauth_list_index < total - 1 =>
        {
            state.otpauth_list_index += 1;
        }
        _ => {}
    }
}

fn open_file_picker(state: &mut AppState) {
    let start = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    state.fp_path = start;
    state.fp_selected = 0;
    state.fp_query.clear();
    refresh_fp_entries(state);
    state.screen = Screen::FilePicker;
}

fn refresh_fp_entries(state: &mut AppState) {
    let mut entries: Vec<(String, bool)> = Vec::new();

    // Parent directory entry
    if state.fp_path.parent().is_some() {
        entries.push(("..".to_string(), true));
    }

    if let Ok(read_dir) = std::fs::read_dir(&state.fp_path) {
        let mut dirs: Vec<String> = Vec::new();
        let mut files: Vec<String> = Vec::new();

        for entry in read_dir.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }
            let Ok(meta) = entry.metadata() else { continue };
            if meta.is_dir() {
                dirs.push(name);
            } else if meta.is_file() {
                let ext = entry
                    .path()
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();
                if SUPPORTED_EXTS.contains(&ext.as_str()) {
                    files.push(name);
                }
            }
        }

        dirs.sort();
        files.sort();
        entries.extend(dirs.into_iter().map(|n| (n, true)));
        entries.extend(files.into_iter().map(|n| (n, false)));
    }

    state.fp_entries = entries;
    state.fp_selected = 0;
}

fn handle_file_picker_key(
    key: KeyCode,
    state: &mut AppState,
    vault: &mut Vault,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    use screens::file_picker::filtered;

    match key {
        KeyCode::Esc | KeyCode::Tab => {
            state.screen = Screen::AddForm;
        }
        KeyCode::Backspace => {
            state.fp_query.pop();
            state.fp_selected = 0;
        }
        KeyCode::Up if state.fp_selected > 0 => {
            state.fp_selected -= 1;
        }
        KeyCode::Down => {
            let visible_len = filtered(&state.fp_entries, &state.fp_query).len();
            if visible_len > 0 && state.fp_selected < visible_len - 1 {
                state.fp_selected += 1;
            }
        }
        KeyCode::Enter => {
            let visible: Vec<_> = filtered(&state.fp_entries, &state.fp_query);
            if let Some((name, is_dir)) = visible.get(state.fp_selected).copied() {
                let (name, is_dir) = (name.clone(), *is_dir);
                if is_dir {
                    let new_path = if name == ".." {
                        state.fp_path.parent().unwrap().to_path_buf()
                    } else {
                        state.fp_path.join(&name)
                    };
                    state.fp_path = new_path;
                    state.fp_query.clear();
                    refresh_fp_entries(state);
                } else {
                    let file_path = state.fp_path.join(&name);
                    try_import_file(state, &file_path, vault, path);
                }
            }
        }
        KeyCode::Char(c) => {
            state.fp_query.push(c);
            // Reset selection when query changes, keep it in bounds
            let visible_len = filtered(&state.fp_entries, &state.fp_query).len();
            if state.fp_selected >= visible_len {
                state.fp_selected = 0;
            }
        }
        _ => {}
    }
    Ok(())
}

fn lock_screen(state: &mut AppState) {
    use zeroize::Zeroize;
    if let Some(k) = &mut state.vault_key_cache {
        k.zeroize();
    }
    state.vault_key_cache = None;
    state.passphrase_input.zeroize();
    state.passphrase_input = Zeroizing::new(String::new());
    state.unlock_error = false;
    state.unlock_error_msg = None;
    state.unlock_confirming = false;
    state.screen = Screen::Unlock;
}

fn list_row_content_width(vault: &Vault) -> usize {
    let entries = vault.entries();
    let max_label_w = entries
        .iter()
        .map(|e| {
            if let Some(pos) = e.name.find(':') {
                let issuer = &e.name[..pos];
                let account = &e.name[pos + 1..];
                if account.is_empty() {
                    e.name.chars().count()
                } else {
                    issuer.chars().count() + 3 + account.chars().count()
                }
            } else {
                e.name.chars().count()
            }
        })
        .max()
        .unwrap_or(0);
    let max_code_w: usize = entries
        .iter()
        .map(|e| if e.digits == 8 { 9usize } else { 7 })
        .max()
        .unwrap_or(7);
    const BAR_LEN: usize = 20;
    // cursor(2) + label(max_label_w) + gap(2) + code(max_code_w) + bar(1+BAR_LEN) + space(1) + secs("16s"=3)
    2 + max_label_w + 2 + max_code_w + 1 + BAR_LEN + 1 + 3
}

/// Returns true on success. On failure sets a persistent (non-auto-dismissing) error message.
fn save_vault(state: &mut AppState, vault: &Vault, path: &Path) -> bool {
    let key_bytes = match &state.vault_key_cache {
        Some(k) => k.clone(),
        None => {
            // No key in cache — vault-disk divergence would happen silently; refuse loudly.
            state.status_message =
                Some("SAVE ERROR: vault is locked — changes not persisted!".to_string());
            state.status_message_at = None; // persistent, never auto-dismiss
            return false;
        }
    };
    let pass = match std::str::from_utf8(&key_bytes) {
        Ok(p) => p.to_string(),
        Err(_) => {
            state.status_message = Some("SAVE ERROR: corrupted passphrase cache".to_string());
            state.status_message_at = None;
            return false;
        }
    };
    match vault.save(path, &pass) {
        Ok(_) => true,
        Err(e) => {
            state.status_message = Some(format!("SAVE ERROR: {e}"));
            state.status_message_at = None; // persistent until dismissed
            false
        }
    }
}
