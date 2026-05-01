pub mod screens;
pub mod state;
pub mod theme;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseEventKind, MouseButton},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tofa_core::{
    generate_migration_uri, uri_to_qr_lines,
    qr::{parse_input, parse_migration, scan_qr_uri},
    store::{Vault, VaultEntry},
    totp::generate_code_now,
};
use state::{AppState, OtpMetaDisplay, Screen};
use std::{
    io,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

const IMAGE_EXTS: &[&str] = &["png", "jpg", "jpeg", "gif", "bmp", "webp"];
use zeroize::Zeroizing;

pub fn vault_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("tofa")
        .join("vault.enc")
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

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
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app_state = AppState::new();
    let mut vault: Option<Vault> = None;
    let path = vault_path();

    loop {
        terminal.draw(|f| {
            let area = f.area();
            match app_state.screen {
                Screen::Unlock => screens::unlock::render(f, area, &app_state),
                Screen::List => screens::list::render(
                    f,
                    area,
                    &app_state,
                    vault
                        .as_ref()
                        .expect("vault must be initialized after unlock"),
                ),
                Screen::Fullscreen => {
                    let v = vault.as_ref().expect("vault must be initialized after unlock");
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
                    let v = vault.as_ref().expect("vault must be initialized after unlock");
                    screens::list::render(f, area, &app_state, v);
                    screens::otp_detail::render(f, area, &app_state, v);
                }
                Screen::Export => {
                    let v = vault.as_ref().expect("vault must be initialized after unlock");
                    screens::list::render(f, area, &app_state, v);
                    screens::export::render(f, area, &app_state, v);
                }
                Screen::ExportQr => screens::export_qr::render(f, area, &app_state),
                Screen::ScanningQr => {
                    let v = vault.as_ref().expect("vault must be initialized after unlock");
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
                let raw = app_state.add_secret_input.trim().to_string();
                let v = vault.as_mut().expect("vault must be initialized after unlock");
                match scan_qr_uri(&fp) {
                    Ok(uri) if uri.starts_with("otpauth-migration://") => {
                        try_import_migration(&mut app_state, &uri, v, &path);
                    }
                    _ => {
                        app_state.screen = Screen::AddForm;
                        try_parse_and_advance(&mut app_state, &raw);
                    }
                }
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
                                // Each list item = 2 lines (content + separator).
                                // Header = 2 rows, so subtract them first.
                                let row = (mouse.row as usize).saturating_sub(2);
                                let clicked = row / 2;
                                if clicked < v.entries().len() {
                                    app_state.selected_index = clicked;
                                    copy_selected_code(&mut app_state, v);
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
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
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
                        vault.as_mut().expect("vault must be initialized after unlock"),
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
                        vault.as_mut().expect("vault must be initialized after unlock"),
                        &path,
                    )?,
                    Screen::OtpDetail => handle_otp_detail_key(
                        key.code,
                        &mut app_state,
                        vault.as_ref().expect("vault must be initialized after unlock"),
                    ),
                    Screen::Export => handle_export_key(
                        key.code,
                        &mut app_state,
                        vault.as_ref().expect("vault must be initialized after unlock"),
                    )?,
                    Screen::ExportQr => {
                        if key.code == KeyCode::Esc {
                            app_state.screen = Screen::Export;
                        }
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
    match key {
        KeyCode::Char(c) => state.passphrase_input.push(c),
        KeyCode::Backspace => {
            state.passphrase_input.pop();
        }
        KeyCode::Enter => match Vault::load_or_new(path, &state.passphrase_input) {
            Ok(v) => {
                state.unlock_error = false;
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
                state.passphrase_input.clear();
            }
        },
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
        KeyCode::Down | KeyCode::Char('j') if !accumulating && len > 0 && state.selected_index < len - 1 => {
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
        // Accumulate only if it looks like a file path (starts with '/' or '~'),
        // or we're already mid-accumulation. Anything else is ignored.
        KeyCode::Char(c) if accumulating || c == '/' || c == '~' => {
            state.add_secret_input.push(c);
            let raw = state.add_secret_input.trim().to_string();
            let unescaped = raw.replace("\\ ", " ");
            if let Some(fp) = [unescaped.as_str(), raw.as_str()]
                .iter()
                .find_map(|s| {
                    let p = std::path::PathBuf::from(s);
                    if p.is_file() { Some(p) } else { None }
                })
            {
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
        KeyCode::Up | KeyCode::Char('k') if state.selected_index > 0 => {
            state.selected_index -= 1;
        }
        KeyCode::Down | KeyCode::Char('j') if len > 0 && state.selected_index < len - 1 => {
            state.selected_index += 1;
        }
        _ => {}
    }
}

fn try_import_migration(
    state: &mut AppState,
    uri: &str,
    vault: &mut Vault,
    path: &Path,
) {
    // Always close any modal and return to List, success or failure.
    state.clear_add_form();
    state.fp_query.clear();
    state.screen = Screen::List;

    match parse_migration(uri) {
        Ok(accounts) => {
            let count = accounts.len();
            let today = chrono::Local::now().format("%Y-%m-%d").to_string();
            for otp in accounts {
                let name = match (&otp.meta.issuer, &otp.meta.account) {
                    (Some(i), Some(a)) => format!("{i}:{a}"),
                    (Some(i), None) => i.clone(),
                    (None, Some(a)) => a.clone(),
                    (None, None) => format!("imported-{}", vault.entries().len() + 1),
                };
                vault.add_entry(VaultEntry {
                    name,
                    secret: otp.secret,
                    created_at: today.clone(),
                    period: otp.meta.period.unwrap_or(30),
                    digits: otp.meta.digits.unwrap_or(6),
                    algorithm: otp.meta.algorithm.unwrap_or_else(|| "SHA1".to_string()),
                });
            }
            save_vault(state, vault, path);
            state.status_message = Some(format!("Imported {count} account(s)."));
        }
        Err(e) => {
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
            if let Some(fp) = [unescaped.as_str(), raw.as_str()]
                .iter()
                .find_map(|s| {
                    let p = std::path::PathBuf::from(s);
                    if p.is_file() { Some(p) } else { None }
                })
            {
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
                if raw.starts_with("otpauth-migration://") {
                    try_import_migration(state, &raw, vault, path);
                } else {
                    // Could be a file path containing a migration QR
                    let fp = std::path::Path::new(&raw);
                    if fp.is_file() {
                        match scan_qr_uri(fp) {
                            Ok(uri) if uri.starts_with("otpauth-migration://") => {
                                try_import_migration(state, &uri, vault, path);
                            }
                            _ => try_parse_and_advance(state, &raw),
                        }
                    } else {
                        try_parse_and_advance(state, &raw);
                    }
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
    vault: &mut Vault,
    path: &Path,
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
            let today = chrono::Local::now().format("%Y-%m-%d").to_string();
            let (period, digits, algorithm) = state.add_meta.as_ref().map(|m| (
                m.period.unwrap_or(30),
                m.digits.unwrap_or(6),
                m.algorithm.clone().unwrap_or_else(|| "SHA1".to_string()),
            )).unwrap_or((30, 6, "SHA1".to_string()));
            vault.add_entry(VaultEntry {
                name,
                secret: state.add_parsed_secret.as_str().to_string(),
                created_at: today,
                period,
                digits,
                algorithm,
            });
            save_vault(state, vault, path);
            state.screen = Screen::List;
            state.clear_add_form();
        }
        _ => {}
    }
    Ok(())
}

fn handle_delete_confirm_key(
    key: KeyCode,
    state: &mut AppState,
    vault: &mut Vault,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    match key {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            vault.remove_entry(state.selected_index);
            if state.selected_index > 0 && state.selected_index >= vault.entries().len() {
                state.selected_index -= 1;
            }
            save_vault(state, vault, path);
            if vault.entries().is_empty() {
                state.clear_add_form();
                state.screen = Screen::AddForm;
            } else {
                state.screen = Screen::List;
                state.status_message = Some("Entry deleted.".to_string());
            }
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
            KeyCode::Backspace => { state.detail_passphrase.pop(); }
            KeyCode::Enter => {
                let correct = state.vault_key_cache
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
            let entries = vault.entries();
            let accounts: Vec<(&str, &str, &str)> = entries
                .iter()
                .enumerate()
                .filter(|(i, _)| state.export_checked.get(*i).copied().unwrap_or(true))
                .map(|(_, e)| {
                    let name = e.name.as_str();
                    // Try to split "issuer:account" → issuer stays separate
                    (name, "", e.secret.as_str())
                })
                .collect();

            if accounts.is_empty() {
                state.status_message = Some("No accounts selected.".to_string());
                state.screen = Screen::List;
                return Ok(());
            }

            let uri = generate_migration_uri(&accounts)?;
            state.export_qr_lines = uri_to_qr_lines(&uri);
            state.screen = Screen::ExportQr;
        }
        _ => {}
    }
    Ok(())
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
                if IMAGE_EXTS.contains(&ext.as_str()) {
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
                    match scan_qr_uri(&file_path) {
                        Ok(uri) if uri.starts_with("otpauth-migration://") => {
                            try_import_migration(state, &uri, vault, path);
                        }
                        Ok(_) | Err(_) => {
                            let raw = file_path.to_string_lossy().to_string();
                            state.add_secret_input = raw.clone();
                            state.screen = Screen::AddForm;
                            try_parse_and_advance(state, &raw);
                        }
                    }
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

fn save_vault(state: &mut AppState, vault: &Vault, path: &Path) {
    if let Some(key_bytes) = &state.vault_key_cache {
        let pass = std::str::from_utf8(key_bytes).unwrap_or("");
        match vault.save(path, pass) {
            Ok(_) => {}
            Err(e) => state.status_message = Some(format!("Save failed: {e}")),
        }
    }
}
