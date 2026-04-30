use crate::crypto::{decrypt, encrypt, CryptoError};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use thiserror::Error;

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
    pub name: String,
    pub secret: String,
    pub created_at: String,
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

    pub fn add_entry(&mut self, entry: VaultEntry) {
        self.data.entries.push(entry);
    }

    pub fn remove_entry(&mut self, index: usize) {
        if index < self.data.entries.len() {
            self.data.entries.remove(index);
        }
    }

    pub fn entries(&self) -> &[VaultEntry] {
        &self.data.entries
    }
}
