use std::path::PathBuf;
use std::time::{Duration, Instant};
use zeroize::Zeroizing;

const CACHE_TTL: Duration = Duration::from_secs(10 * 60);

pub struct PassphraseCache {
    passphrase: Zeroizing<String>,
    unlocked_at: Option<Instant>,
}

impl PassphraseCache {
    pub fn new() -> Self {
        Self {
            passphrase: Zeroizing::new(String::new()),
            unlocked_at: None,
        }
    }

    pub fn unlock(&mut self, passphrase: String) {
        use zeroize::Zeroize;
        self.passphrase.zeroize(); // scrub old bytes before overwriting
        *self.passphrase = passphrase;
        self.unlocked_at = Some(Instant::now());
    }

    pub fn get(&mut self) -> Option<&str> {
        match self.unlocked_at {
            Some(t) if t.elapsed() < CACHE_TTL => Some(self.passphrase.as_str()),
            _ => {
                // TTL expired — enforce in memory, not just visibility
                self.lock();
                None
            }
        }
    }

    pub fn is_locked(&mut self) -> bool {
        self.get().is_none()
    }

    pub fn lock(&mut self) {
        use zeroize::Zeroize;
        self.passphrase.zeroize();
        self.unlocked_at = None;
    }
}

pub struct AppState {
    pub cache: PassphraseCache,
    pub vault_path: PathBuf,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            cache: PassphraseCache::new(),
            vault_path: default_vault_path(),
        }
    }
}

pub fn default_vault_path() -> PathBuf {
    let mut p = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
    p.push(".config/tofa/vault.enc");
    p
}

pub fn settings_path() -> PathBuf {
    let mut p = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
    p.push(".config/tofa/settings.json");
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cache_is_locked() {
        let mut cache = PassphraseCache::new();
        assert!(cache.is_locked());
        assert!(cache.get().is_none());
    }

    #[test]
    fn unlock_makes_passphrase_available() {
        let mut cache = PassphraseCache::new();
        cache.unlock("secret".to_string());
        assert!(!cache.is_locked());
        assert_eq!(cache.get(), Some("secret"));
    }

    #[test]
    fn lock_clears_passphrase() {
        let mut cache = PassphraseCache::new();
        cache.unlock("secret".to_string());
        cache.lock();
        assert!(cache.is_locked());
        assert!(cache.get().is_none());
    }
}
