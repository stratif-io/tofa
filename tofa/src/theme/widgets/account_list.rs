use crate::theme::theme::ThemeMode;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, StatefulWidget},
};

pub struct AccountEntry {
    pub issuer: String,
    pub account: String,
    pub code: Option<String>, // None = masked
    pub seconds: u64,
}

pub struct AccountListState {
    pub list_state: ListState,
}

impl Default for AccountListState {
    fn default() -> Self {
        Self::new()
    }
}

impl AccountListState {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default(),
        }
    }
    pub fn select(&mut self, i: usize) {
        self.list_state.select(Some(i));
    }
    pub fn selected(&self) -> Option<usize> {
        self.list_state.selected()
    }
}

pub struct AccountList {
    pub entries: Vec<AccountEntry>,
    pub theme: ThemeMode,
}

impl AccountList {
    pub fn new(entries: Vec<AccountEntry>) -> Self {
        Self {
            entries,
            theme: ThemeMode::Dark,
        }
    }
    pub fn theme(mut self, t: ThemeMode) -> Self {
        self.theme = t;
        self
    }
}

impl StatefulWidget for AccountList {
    type State = AccountListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let brand = self.theme.brand();
        let muted = self.theme.text_muted();
        let selected = state.list_state.selected();

        let items: Vec<ListItem> = self
            .entries
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let is_selected = selected == Some(i);
                let code_str = if is_selected {
                    e.code.clone().unwrap_or_else(|| "──────".to_string())
                } else {
                    "●●●●●●".to_string()
                };
                let code_color = if is_selected { brand } else { muted };
                let line = Line::from(vec![
                    Span::styled(
                        format!("{:<14}", e.issuer),
                        Style::default().fg(if is_selected {
                            self.theme.text()
                        } else {
                            muted
                        }),
                    ),
                    Span::styled(
                        code_str,
                        Style::default().fg(code_color).add_modifier(Modifier::BOLD),
                    ),
                ]);
                ListItem::new(line)
            })
            .collect();

        let list =
            List::new(items).highlight_style(Style::default().bg(self.theme.surface()).fg(brand));
        StatefulWidget::render(list, area, buf, &mut state.list_state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, Terminal};

    fn make_entries() -> Vec<AccountEntry> {
        vec![
            AccountEntry {
                issuer: "GitHub".into(),
                account: "u@ex.com".into(),
                code: Some("847 392".into()),
                seconds: 20,
            },
            AccountEntry {
                issuer: "AWS".into(),
                account: "root".into(),
                code: Some("128 045".into()),
                seconds: 15,
            },
        ]
    }

    #[test]
    fn selected_entry_shows_code() {
        let backend = TestBackend::new(30, 5);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut list_state = AccountListState::new();
        list_state.select(0);
        terminal
            .draw(|f| {
                let w = AccountList::new(make_entries());
                f.render_stateful_widget(w, f.area(), &mut list_state);
            })
            .unwrap();
        let content: String = terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|c| c.symbol().to_string())
            .collect();
        assert!(content.contains("847 392"));
    }

    #[test]
    fn unselected_entry_shows_masked() {
        let backend = TestBackend::new(30, 5);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut list_state = AccountListState::new();
        list_state.select(0);
        terminal
            .draw(|f| {
                let w = AccountList::new(make_entries());
                f.render_stateful_widget(w, f.area(), &mut list_state);
            })
            .unwrap();
        let content: String = terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|c| c.symbol().to_string())
            .collect();
        assert!(content.contains("●"));
    }
}
