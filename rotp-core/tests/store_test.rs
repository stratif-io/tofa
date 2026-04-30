use rotp_core::store::{Vault, VaultEntry};
use tempfile::tempdir;

fn make_entry(name: &str) -> VaultEntry {
    VaultEntry {
        name: name.to_string(),
        secret: "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ".to_string(),
        created_at: "2026-01-01".to_string(),
    }
}

#[test]
fn save_and_load_roundtrip() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("vault.enc");
    let passphrase = "correct horse battery staple";

    let mut vault = Vault::new();
    vault.add_entry(make_entry("GitHub"));
    vault.save(&path, passphrase).unwrap();

    let loaded = Vault::load(&path, passphrase).unwrap();
    assert_eq!(loaded.entries().len(), 1);
    assert_eq!(loaded.entries()[0].name, "GitHub");
    assert_eq!(loaded.entries()[0].secret, "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ");
}

#[test]
fn wrong_passphrase_on_load_returns_error() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("vault.enc");

    let mut vault = Vault::new();
    vault.add_entry(make_entry("GitHub"));
    vault.save(&path, "correct").unwrap();

    assert!(Vault::load(&path, "wrong").is_err());
}

#[test]
fn add_and_remove_entry() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("vault.enc");

    let mut vault = Vault::new();
    vault.add_entry(make_entry("GitHub"));
    vault.add_entry(make_entry("AWS"));
    vault.save(&path, "pass").unwrap();

    let mut loaded = Vault::load(&path, "pass").unwrap();
    assert_eq!(loaded.entries().len(), 2);

    loaded.remove_entry(0);
    loaded.save(&path, "pass").unwrap();

    let reloaded = Vault::load(&path, "pass").unwrap();
    assert_eq!(reloaded.entries().len(), 1);
    assert_eq!(reloaded.entries()[0].name, "AWS");
}

#[test]
fn empty_vault_saves_and_loads() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("vault.enc");

    let vault = Vault::new();
    vault.save(&path, "pass").unwrap();

    let loaded = Vault::load(&path, "pass").unwrap();
    assert!(loaded.entries().is_empty());
}

#[test]
fn load_nonexistent_returns_empty_vault() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("nonexistent.enc");

    let vault = Vault::load_or_new(&path, "pass").unwrap();
    assert!(vault.entries().is_empty());
}
