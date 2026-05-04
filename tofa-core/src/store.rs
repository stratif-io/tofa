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

fn default_period() -> u32 {
    30
}
fn default_digits() -> u8 {
    6
}
fn default_algorithm() -> String {
    "SHA1".to_string()
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
            period: 30,
            digits: 6,
            algorithm: "SHA1".to_string(),
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

    pub fn rename_entry(&mut self, index: usize, new_name: String) {
        if let Some(entry) = self.data.entries.get_mut(index) {
            entry.name = new_name;
        }
    }

    pub fn entries(&self) -> &[VaultEntry] {
        &self.data.entries
    }
}
