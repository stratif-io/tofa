use std::{path::PathBuf, time::Instant};
use tofa_core::qr::OtpMeta;
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
    Export,
    ExportQr,
    ExportOtpauthList,
    ExportUriList,
    ScanningQr,
}

#[derive(Debug)]
pub enum PendingVaultAction {
    DeleteEntry(usize),
    AddEntry,
}

// `OtpMetaDisplay` was a TUI-local struct identical to
// `tofa_core::qr::OtpMeta`. Use the core type directly to avoid the
// near-duplicate.

pub struct AppState {
    pub screen: Screen,
    pub passphrase_input: Zeroizing<String>,
    pub selected_index: usize,
    pub status_message: Option<String>,
    pub status_message_at: Option<Instant>,
    pub add_name: String,
    pub add_secret_input: String,
    pub add_parsed_secret: Zeroizing<String>,
    pub add_meta: Option<OtpMeta>,
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
    /// Where Esc on the ExportQr screen returns to. Defaults to Export
    /// (multi-account selection flow); set to Fullscreen when the QR
    /// is opened for the currently-selected entry from the view.
    pub export_qr_back: Screen,
    // multi-otpauth list export
    pub otpauth_list_qrs: Vec<Vec<String>>,
    pub otpauth_list_titles: Vec<String>,
    pub otpauth_list_index: usize,
    // URI list export (`u` from the export screen): pairs of
    // (display name, otpauth:// URI) shown as plain text so the user
    // can select with the mouse natively or copy all with `y`.
    pub export_uri_list: Vec<(String, String)>,
    pub export_uri_scroll: u16,
    // file picker
    pub fp_path: PathBuf,
    pub fp_entries: Vec<(String, bool)>,
    pub fp_selected: usize,
    pub fp_query: String,
    /// Files explicitly checked by the user (Space-toggle). Holds full
    /// paths so a check survives navigating into other directories,
    /// letting the user compose a multi-folder import in one go.
    pub fp_checked: Vec<PathBuf>,
    // pending QR scan
    pub pending_scan_path: Option<PathBuf>,
    // pending vault write (deferred one tick so "Saving…" toast renders first)
    pub pending_vault_action: Option<PendingVaultAction>,
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
    #[must_use]
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
            export_qr_back: Screen::Export,
            otpauth_list_qrs: Vec::new(),
            otpauth_list_titles: Vec::new(),
            otpauth_list_index: 0,
            export_uri_list: Vec::new(),
            export_uri_scroll: 0,
            fp_path: PathBuf::new(),
            fp_entries: Vec::new(),
            fp_selected: 0,
            fp_query: String::new(),
            fp_checked: Vec::new(),
            pending_scan_path: None,
            pending_vault_action: None,
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
