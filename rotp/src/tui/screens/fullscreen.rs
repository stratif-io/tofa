use ratatui::{layout::Rect, Frame};
use rotp_core::store::Vault;
use crate::tui::state::AppState;

pub fn render(f: &mut Frame, area: Rect, state: &AppState, vault: &Vault) {
    let _ = (f, area, state, vault);
}
