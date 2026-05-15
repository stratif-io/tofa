use crate::crypto::{decrypt, encrypt, CryptoError};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use thiserror::Error;
use zeroize::Zeroize;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("crypto error: {0}")]
    Crypto(#[from] CryptoError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultEntry {
    /// Stable unique identifier: `{name}@{unix_ts}` or `{name}@{unix_ts}#{n}` on collision.
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub secret: String,
    pub created_at: String,
    #[serde(default = "default_period")]
    pub period: u32,
    #[serde(default = "default_digits")]
    pub digits: u8,
    #[serde(default = "default_algorithm")]
    pub algorithm: String,
}

/// Default TOTP period (seconds) used when an import or paste doesn't
/// specify one. Matches RFC 6238's default and Google Authenticator's
/// migration protobuf, which has no period field.
pub const DEFAULT_PERIOD: u32 = 30;

/// Default TOTP digit count used when an import or paste doesn't
/// specify one. RFC 6238 default; 8 is the only other widely-supported
/// alternative.
pub const DEFAULT_DIGITS: u8 = 6;

/// Default TOTP algorithm used when an import or paste doesn't specify
/// one. Every authenticator we've seen defaults to SHA1; the migration
/// protobuf encodes 1 = SHA1.
pub const DEFAULT_ALGORITHM: &str = "SHA1";

/// Format string used for `VaultEntry::created_at`. Centralised so a
/// future change (e.g. switching to RFC 3339) lands in one place
/// instead of the dozen-plus call sites that mint timestamps.
pub const CREATED_AT_FORMAT: &str = "%Y-%m-%d";

/// Today's date in the format `VaultEntry::created_at` expects. Use
/// this everywhere a new entry is being added so the field is shaped
/// the same way regardless of which surface (CLI / TUI / app) created
/// the entry.
#[must_use]
pub fn today_iso() -> String {
    chrono::Local::now().format(CREATED_AT_FORMAT).to_string()
}

fn default_period() -> u32 {
    DEFAULT_PERIOD
}
fn default_digits() -> u8 {
    DEFAULT_DIGITS
}
fn default_algorithm() -> String {
    DEFAULT_ALGORITHM.to_string()
}

impl Drop for VaultEntry {
    fn drop(&mut self) {
        self.secret.zeroize();
    }
}

impl VaultEntry {
    pub fn new(name: String, secret: String, created_at: String) -> Self {
        Self {
            id: String::new(),
            name,
            secret,
            created_at,
            period: DEFAULT_PERIOD,
            digits: DEFAULT_DIGITS,
            algorithm: DEFAULT_ALGORITHM.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct VaultData {
    version: u32,
    entries: Vec<VaultEntry>,
}

#[derive(Debug)]
pub struct Vault {
    data: VaultData,
}

impl Default for Vault {
    fn default() -> Self {
        Self::new()
    }
}

impl Vault {
    pub fn new() -> Self {
        Self {
            data: VaultData {
                version: 1,
                entries: Vec::new(),
            },
        }
    }

    pub fn load(path: &Path, passphrase: &str) -> Result<Self, StoreError> {
        let blob = std::fs::read(path)?;
        let plaintext = decrypt(passphrase, &blob)?;
        let data: VaultData = serde_json::from_slice(&plaintext)?;
        Ok(Self { data })
    }

    pub fn load_or_new(path: &Path, passphrase: &str) -> Result<Self, StoreError> {
        if path.exists() {
            Self::load(path, passphrase)
        } else {
            Ok(Self::new())
        }
    }

    pub fn save(&self, path: &Path, passphrase: &str) -> Result<(), StoreError> {
        let plaintext = serde_json::to_vec(&self.data)?;
        let blob = encrypt(passphrase, &plaintext)?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let tmp = path.with_extension("enc.tmp");
        {
            let mut f = std::fs::File::create(&tmp)?;
            f.write_all(&blob)?;
            f.sync_all()?;
        }
        std::fs::rename(&tmp, path)?;
        Ok(())
    }

    pub fn add_entry(&mut self, mut entry: VaultEntry) {
        entry.id = self.generate_id(&entry.name);
        self.data.entries.push(entry);
    }

    /// Add `entry` to the vault unless an existing entry already
    /// matches both `name` and `secret`. Returns `true` if it was
    /// inserted, `false` if it was a duplicate. Single source of
    /// truth for the dedup rule used by every import surface (CLI,
    /// TUI, desktop app drop / picker / paste).
    ///
    /// Why `(name, secret)` and not one or the other:
    /// - Same name, different secret → user rotated the OTP and
    ///   wants both rows until the new one is verified.
    /// - Same secret, different name → user filed the same account
    ///   under two labels intentionally.
    ///
    /// Both are kept; only an exact duplicate is dropped.
    pub fn add_entry_if_unique(&mut self, entry: VaultEntry) -> bool {
        if self
            .data
            .entries
            .iter()
            .any(|e| e.name == entry.name && e.secret == entry.secret)
        {
            return false;
        }
        self.add_entry(entry);
        true
    }

    fn generate_id(&self, name: &str) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let base = format!("{}@{}", name, ts);
        if !self.data.entries.iter().any(|e| e.id == base) {
            return base;
        }
        let mut n = 2u32;
        loop {
            let candidate = format!("{}#{}", base, n);
            if !self.data.entries.iter().any(|e| e.id == candidate) {
                return candidate;
            }
            n += 1;
        }
    }

    pub fn remove_entry(&mut self, index: usize) {
        if index < self.data.entries.len() {
            self.data.entries.remove(index);
        }
    }

    pub fn remove_by_id(&mut self, id: &str) -> bool {
        if let Some(idx) = self.data.entries.iter().position(|e| e.id == id) {
            self.data.entries.remove(idx);
            true
        } else {
            false
        }
    }

    pub fn entry_by_id(&self, id: &str) -> Option<&VaultEntry> {
        self.data.entries.iter().find(|e| e.id == id)
    }

    pub fn rename_entry(&mut self, index: usize, new_name: String) {
        if let Some(entry) = self.data.entries.get_mut(index) {
            entry.name = new_name;
        }
    }

    #[must_use]
    pub fn entries(&self) -> &[VaultEntry] {
        &self.data.entries
    }
}
