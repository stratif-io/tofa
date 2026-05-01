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
        let vault_path = Self::load_vault_path();
        Self {
            cache: PassphraseCache::new(),
            vault_path,
        }
    }

    fn load_vault_path() -> PathBuf {
        let path = settings_path();
        if path.exists() {
            if let Ok(s) = std::fs::read_to_string(&path) {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&s) {
                    if let Some(p) = v["vault_path"].as_str() {
                        return PathBuf::from(p);
                    }
                }
            }
        }
        default_vault_path()
    }
}

pub fn default_vault_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("tofa")
        .join("vault.enc")
}

pub fn settings_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("tofa")
        .join("tofa-app-settings.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cache_is_locked() {
        let mut cache = PassphraseCache::new();
        assert!(cache.get().is_none());
        assert!(cache.get().is_none());
    }

    #[test]
    fn unlock_makes_passphrase_available() {
        let mut cache = PassphraseCache::new();
        cache.unlock("secret".to_string());
        assert!(cache.get().is_some());
        assert_eq!(cache.get(), Some("secret"));
    }

    #[test]
    fn lock_clears_passphrase() {
        let mut cache = PassphraseCache::new();
        cache.unlock("secret".to_string());
        cache.lock();
        assert!(cache.get().is_none());
        assert!(cache.get().is_none());
    }
}
