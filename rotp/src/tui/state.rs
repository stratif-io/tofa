use zeroize::Zeroizing;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Unlock,
    List,
    Fullscreen,
    AddForm,
    AddName,
    DeleteConfirm,
}

pub struct AppState {
    pub screen: Screen,
    pub passphrase_input: String,
    pub selected_index: usize,
    pub status_message: Option<String>,
    pub add_name: String,
    pub add_secret_input: String,
    pub add_focused_field: usize,
    pub unlock_error: bool,
    pub vault_key_cache: Option<Zeroizing<Vec<u8>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            screen: Screen::Unlock,
            passphrase_input: String::new(),
            selected_index: 0,
            status_message: None,
            add_name: String::new(),
            add_secret_input: String::new(),
            add_focused_field: 0,
            unlock_error: false,
            vault_key_cache: None,
        }
    }

    pub fn clear_add_form(&mut self) {
        self.add_name.clear();
        self.add_secret_input.clear();
        self.add_focused_field = 0;
        self.status_message = None;
    }
}
