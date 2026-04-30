pub mod theme;
pub mod state;
pub mod screens;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use rotp_core::{
    qr::parse_input,
    store::{Vault, VaultEntry},
    totp::generate_code_now,
};
use state::{AppState, Screen};
use std::{io, path::PathBuf, time::Duration};
use zeroize::Zeroizing;

pub fn vault_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("rotp")
        .join("vault.enc")
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
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
                Screen::List => {
                    screens::list::render(f, area, &app_state, vault.as_ref().unwrap())
                }
                Screen::Fullscreen => {
                    screens::fullscreen::render(f, area, &app_state, vault.as_ref().unwrap())
                }
                Screen::AddForm => screens::add_form::render(f, area, &app_state),
                Screen::DeleteConfirm => {
                    screens::delete_confirm::render(f, area, &app_state, vault.as_ref().unwrap())
                }
            }
        })?;

        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
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
                        if handle_list_key(key.code, &mut app_state, vault.as_mut().unwrap(), &path)? {
                            return Ok(());
                        }
                    }
                    Screen::Fullscreen => {
                        handle_fullscreen_key(key.code, &mut app_state, vault.as_ref().unwrap())
                    }
                    Screen::AddForm => {
                        handle_add_form_key(key.code, &mut app_state, vault.as_mut().unwrap(), &path)?
                    }
                    Screen::DeleteConfirm => {
                        handle_delete_confirm_key(key.code, &mut app_state, vault.as_mut().unwrap(), &path)?
                    }
                }
            }
        }
    }
}

fn handle_unlock_key(
    key: KeyCode,
    state: &mut AppState,
    vault: &mut Option<Vault>,
    path: &PathBuf,
) {
    match key {
        KeyCode::Char(c) => state.passphrase_input.push(c),
        KeyCode::Backspace => {
            state.passphrase_input.pop();
        }
        KeyCode::Enter => match Vault::load_or_new(path, &state.passphrase_input) {
            Ok(v) => {
                *vault = Some(v);
                state.screen = Screen::List;
                state.unlock_error = false;
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
    path: &PathBuf,
) -> Result<bool, Box<dyn std::error::Error>> {
    let len = vault.entries().len();
    match key {
        KeyCode::Char('q') => return Ok(true),
        KeyCode::Up | KeyCode::Char('k') => {
            if state.selected_index > 0 {
                state.selected_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if len > 0 && state.selected_index < len - 1 {
                state.selected_index += 1;
            }
        }
        KeyCode::Enter => {
            if len > 0 {
                state.screen = Screen::Fullscreen;
            }
        }
        KeyCode::Char('a') => {
            state.clear_add_form();
            state.screen = Screen::AddForm;
        }
        KeyCode::Char('d') => {
            if len > 0 {
                state.screen = Screen::DeleteConfirm;
            }
        }
        KeyCode::Char('y') => {
            copy_selected_code(state, vault);
        }
        _ => {}
    }
    Ok(false)
}

fn handle_fullscreen_key(key: KeyCode, state: &mut AppState, vault: &Vault) {
    match key {
        KeyCode::Esc | KeyCode::Char('q') => state.screen = Screen::List,
        KeyCode::Char('y') => copy_selected_code(state, vault),
        _ => {}
    }
}

fn handle_add_form_key(
    key: KeyCode,
    state: &mut AppState,
    vault: &mut Vault,
    path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    match key {
        KeyCode::Esc => state.screen = Screen::List,
        KeyCode::Tab => {
            state.add_focused_field = if state.add_focused_field == 0 { 1 } else { 0 };
        }
        KeyCode::Char(c) => {
            if state.add_focused_field == 0 {
                state.add_name.push(c);
            } else {
                state.add_secret_input.push(c);
            }
        }
        KeyCode::Backspace => {
            if state.add_focused_field == 0 {
                state.add_name.pop();
            } else {
                state.add_secret_input.pop();
            }
        }
        KeyCode::Enter => {
            if state.add_focused_field == 0 {
                state.add_focused_field = 1;
            } else {
                let name = state.add_name.trim().to_string();
                let raw = state.add_secret_input.trim().to_string();
                if !name.is_empty() && !raw.is_empty() {
                    match parse_input(&raw) {
                        Ok(otp) => {
                            let today = chrono::Local::now().format("%Y-%m-%d").to_string();
                            vault.add_entry(VaultEntry {
                                name,
                                secret: otp.secret,
                                created_at: today,
                            });
                            save_vault(state, vault, path);
                            state.screen = Screen::List;
                            state.clear_add_form();
                        }
                        Err(e) => {
                            state.status_message = Some(format!("Error: {e}"));
                        }
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_delete_confirm_key(
    key: KeyCode,
    state: &mut AppState,
    vault: &mut Vault,
    path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    match key {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            vault.remove_entry(state.selected_index);
            if state.selected_index > 0 && state.selected_index >= vault.entries().len() {
                state.selected_index -= 1;
            }
            save_vault(state, vault, path);
            state.screen = Screen::List;
            state.status_message = Some("Entry deleted.".to_string());
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
        match generate_code_now(&entry.secret) {
            Ok(code) => {
                match arboard::Clipboard::new().and_then(|mut cb| cb.set_text(code)) {
                    Ok(_) => state.status_message = Some("Copied to clipboard.".to_string()),
                    Err(_) => state.status_message = Some("Clipboard unavailable.".to_string()),
                }
            }
            Err(e) => state.status_message = Some(format!("Error: {e}")),
        }
    }
}

fn save_vault(state: &mut AppState, vault: &Vault, path: &PathBuf) {
    if let Some(key_bytes) = &state.vault_key_cache {
        let pass = std::str::from_utf8(key_bytes).unwrap_or("");
        match vault.save(path, pass) {
            Ok(_) => {}
            Err(e) => state.status_message = Some(format!("Save failed: {e}")),
        }
    }
}
