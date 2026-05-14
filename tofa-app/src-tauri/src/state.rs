use std::path::PathBuf;
use std::time::{Duration, Instant};
use zeroize::Zeroizing;

pub const DEFAULT_LOCK_AFTER_SECONDS: u64 = 10 * 60;
const DEFAULT_LOCK_AFTER: Duration = Duration::from_secs(DEFAULT_LOCK_AFTER_SECONDS);

pub struct PassphraseCache {
    passphrase: Zeroizing<String>,
    unlocked_at: Option<Instant>,
    /// `None` means "never auto-lock" — the passphrase stays in memory until
    /// `lock()` is called explicitly (or the process exits).
    lock_after: Option<Duration>,
}

impl PassphraseCache {
    pub fn new(lock_after: Option<Duration>) -> Self {
        Self {
            passphrase: Zeroizing::new(String::new()),
            unlocked_at: None,
            lock_after,
        }
    }

    pub fn unlock(&mut self, passphrase: String) {
        use zeroize::Zeroize;
        self.passphrase.zeroize(); // scrub old bytes before overwriting
        *self.passphrase = passphrase;
        self.unlocked_at = Some(Instant::now());
    }

    pub fn get(&mut self) -> Option<&str> {
        match (self.unlocked_at, self.lock_after) {
            // Never-expire mode: alive as long as we've ever unlocked.
            (Some(_), None) => Some(self.passphrase.as_str()),
            // TTL mode: alive within the window.
            (Some(t), Some(ttl)) if t.elapsed() < ttl => Some(self.passphrase.as_str()),
            // Locked, or TTL elapsed — scrub and return None.
            _ => {
                self.lock();
                None
            }
        }
    }

    /// True iff the cache has been unlocked, a finite TTL is configured,
    /// and that TTL has now elapsed. Returns `false` when `lock_after` is
    /// `None` (never-expire mode), even though the cache is unlocked.
    ///
    /// Pure read — does not mutate state or self-lock. The next call to
    /// `get()` will perform the actual lock-and-scrub.
    pub fn is_expired(&self) -> bool {
        match (self.unlocked_at, self.lock_after) {
            (Some(t), Some(ttl)) => t.elapsed() >= ttl,
            _ => false,
        }
    }

    /// Update the auto-lock window. Called when the user saves a new value
    /// in Settings. Does not change `unlocked_at` — if the new TTL is
    /// already shorter than `elapsed`, the next `get()` will lock.
    pub fn set_lock_after(&mut self, lock_after: Option<Duration>) {
        self.lock_after = lock_after;
    }

    /// Run `f` with the passphrase borrowed in-place — no heap clone, no unprotected String.
    pub fn with_passphrase<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(&str) -> R,
    {
        let p = self.get()?;
        Some(f(p))
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
        let (vault_path, lock_after) = Self::load_from_settings();
        Self {
            cache: PassphraseCache::new(lock_after),
            vault_path,
        }
    }

    /// Load both `vault_path` and the `lock_after_seconds` TTL from
    /// `tofa-app-settings.json` in one read. Missing fields fall back to:
    /// vault_path → `default_vault_path()`, lock_after → 10 min.
    fn load_from_settings() -> (PathBuf, Option<Duration>) {
        let default_lock = Some(DEFAULT_LOCK_AFTER);
        let path = settings_path();
        let Ok(s) = std::fs::read_to_string(&path) else {
            return (default_vault_path(), default_lock);
        };
        let Ok(v) = serde_json::from_str::<serde_json::Value>(&s) else {
            return (default_vault_path(), default_lock);
        };

        let vp = v["vault_path"]
            .as_str()
            .map(PathBuf::from)
            .unwrap_or_else(default_vault_path);

        // Four states for lock_after_seconds in the JSON:
        //   - missing            → default to 10 min
        //   - null               → never expire (None)
        //   - 0                  → Some(Duration::ZERO), i.e. expire on
        //                          next access. Probably a footgun for
        //                          hand-edited files, but consistent with
        //                          the in-memory semantics; the UI only
        //                          surfaces non-zero preset durations.
        //   - positive number    → Some(Duration::from_secs(n))
        let lock_after = match v.get("lock_after_seconds") {
            None => default_lock,
            Some(serde_json::Value::Null) => None,
            Some(n) => n.as_u64().map(Duration::from_secs).or(default_lock),
        };

        (vp, lock_after)
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

    fn ten_min() -> Option<Duration> {
        Some(DEFAULT_LOCK_AFTER)
    }

    #[test]
    fn new_cache_is_locked() {
        let mut cache = PassphraseCache::new(ten_min());
        assert!(cache.get().is_none());
        assert!(cache.get().is_none());
    }

    #[test]
    fn unlock_makes_passphrase_available() {
        let mut cache = PassphraseCache::new(ten_min());
        cache.unlock("secret".to_string());
        assert!(cache.get().is_some());
        assert_eq!(cache.get(), Some("secret"));
    }

    #[test]
    fn lock_clears_passphrase() {
        let mut cache = PassphraseCache::new(ten_min());
        cache.unlock("secret".to_string());
        cache.lock();
        assert!(cache.get().is_none());
        assert!(cache.get().is_none());
    }

    #[test]
    fn cache_with_zero_ttl_expires_immediately() {
        let mut cache = PassphraseCache::new(Some(Duration::ZERO));
        cache.unlock("secret".to_string());
        assert!(cache.is_expired());
        assert!(
            cache.get().is_none(),
            "expired cache should self-lock on get"
        );
    }

    #[test]
    fn cache_with_none_ttl_never_expires() {
        let mut cache = PassphraseCache::new(None);
        cache.unlock("secret".to_string());
        assert!(!cache.is_expired());
        assert_eq!(cache.get(), Some("secret"));
    }

    #[test]
    fn is_expired_false_when_locked() {
        let cache = PassphraseCache::new(Some(Duration::ZERO));
        // Never unlocked — is_expired must be false even with a zero TTL,
        // because "expired" only makes sense for an unlocked cache.
        assert!(!cache.is_expired());
    }

    #[test]
    fn set_lock_after_takes_effect() {
        let mut cache = PassphraseCache::new(None);
        cache.unlock("secret".to_string());
        cache.set_lock_after(Some(Duration::ZERO));
        assert!(cache.is_expired());
        assert!(cache.get().is_none());
    }
}
