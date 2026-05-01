use std::{path::PathBuf, time::Instant};
use zeroize::Zeroizing;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Unlock,
    List,
    Fullscreen,
    AddForm,
    AddName,
    DeleteConfirm,
    FilePicker,
    OtpDetail,
    Export,
    ExportQr,
    ScanningQr,
}

pub struct OtpMetaDisplay {
    pub issuer: Option<String>,
    pub account: Option<String>,
    pub algorithm: Option<String>,
    pub digits: Option<u8>,
    pub period: Option<u32>,
}

pub struct AppState {
    pub screen: Screen,
    pub passphrase_input: Zeroizing<String>,
    pub selected_index: usize,
    pub status_message: Option<String>,
    pub status_message_at: Option<Instant>,
    pub add_name: String,
    pub add_secret_input: String,
    pub add_parsed_secret: Zeroizing<String>,
    pub add_meta: Option<OtpMetaDisplay>,
    pub add_focused_field: usize,
    pub unlock_error: bool,
    pub unlock_error_msg: Option<String>,
    pub is_new_vault: bool,
    pub unlock_confirming: bool,
    pub passphrase_confirm: Zeroizing<String>,
    pub vault_key_cache: Option<Zeroizing<Vec<u8>>>,
    pub show_codes: bool,
    // export
    pub export_checked: Vec<bool>,
    pub export_selected: usize,
    pub export_qr_lines: Vec<String>,
    // file picker
    pub fp_path: PathBuf,
    pub fp_entries: Vec<(String, bool)>,
    pub fp_selected: usize,
    pub fp_query: String,
    // pending QR scan
    pub pending_scan_path: Option<PathBuf>,
    // OTP detail secret reveal
    pub detail_revealing: bool,
    pub detail_passphrase: Zeroizing<String>,
    pub detail_secret_visible: bool,
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
            passphrase_input: Zeroizing::new(String::new()),
            selected_index: 0,
            status_message: None,
            status_message_at: None,
            add_name: String::new(),
            add_secret_input: String::new(),
            add_parsed_secret: Zeroizing::new(String::new()),
            add_meta: None,
            add_focused_field: 0,
            unlock_error: false,
            unlock_error_msg: None,
            is_new_vault: false,
            unlock_confirming: false,
            passphrase_confirm: Zeroizing::new(String::new()),
            vault_key_cache: None,
            show_codes: true,
            export_checked: Vec::new(),
            export_selected: 0,
            export_qr_lines: Vec::new(),
            fp_path: PathBuf::new(),
            fp_entries: Vec::new(),
            fp_selected: 0,
            fp_query: String::new(),
            pending_scan_path: None,
            detail_revealing: false,
            detail_passphrase: Zeroizing::new(String::new()),
            detail_secret_visible: false,
        }
    }

    pub fn reset_detail_reveal(&mut self) {
        self.detail_revealing = false;
        self.detail_passphrase = Zeroizing::new(String::new());
        self.detail_secret_visible = false;
    }

    pub fn clear_add_form(&mut self) {
        self.add_name.clear();
        self.add_secret_input.clear();
        self.add_parsed_secret = Zeroizing::new(String::new());
        self.add_meta = None;
        self.add_focused_field = 0;
        self.status_message = None;
    }
}
