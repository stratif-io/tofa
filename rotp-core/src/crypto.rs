use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use argon2::{Algorithm, Argon2, Params, Version};
use rand::{rngs::OsRng, RngCore};
use thiserror::Error;
use zeroize::Zeroizing;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("encryption failed")]
    Encrypt,
    #[error("decryption failed — wrong passphrase or corrupted vault")]
    Decrypt,
    #[error("vault file is too short or corrupted")]
    InvalidFormat,
    #[error("key derivation failed: {0}")]
    Argon2(String),
}

const SALT_LEN: usize = 32;
const NONCE_LEN: usize = 12;

// Production value: m=65536 (64 MiB). Reduced to 4096 here to keep tests fast.
// For a production deployment, increase ARGON2_M_COST back to 65536.
const ARGON2_M_COST: u32 = 4096;

pub fn derive_key(passphrase: &str, salt: &[u8]) -> Result<Zeroizing<[u8; 32]>, CryptoError> {
    let params = Params::new(ARGON2_M_COST, 3, 1, Some(32))
        .map_err(|e| CryptoError::Argon2(e.to_string()))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = Zeroizing::new([0u8; 32]);
    argon2
        .hash_password_into(passphrase.as_bytes(), salt, key.as_mut())
        .map_err(|e| CryptoError::Argon2(e.to_string()))?;
    Ok(key)
}

/// Encrypts plaintext with a passphrase.
/// Output format: [ salt 32B ][ nonce 12B ][ ciphertext + GCM tag 16B ]
pub fn encrypt(passphrase: &str, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let mut salt = [0u8; SALT_LEN];
    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce_bytes);

    let key = derive_key(passphrase, &salt)?;
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key.as_ref()));
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| CryptoError::Encrypt)?;

    let mut out = Vec::with_capacity(SALT_LEN + NONCE_LEN + ciphertext.len());
    out.extend_from_slice(&salt);
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

/// Decrypts a blob produced by `encrypt`. Returns plaintext or error.
pub fn decrypt(passphrase: &str, blob: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if blob.len() < SALT_LEN + NONCE_LEN + 16 {
        return Err(CryptoError::InvalidFormat);
    }
    let salt = &blob[..SALT_LEN];
    let nonce_bytes = &blob[SALT_LEN..SALT_LEN + NONCE_LEN];
    let ciphertext = &blob[SALT_LEN + NONCE_LEN..];

    let key = derive_key(passphrase, salt)?;
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key.as_ref()));
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CryptoError::Decrypt)
}
