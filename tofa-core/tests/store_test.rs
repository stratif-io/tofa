use tempfile::tempdir;
use tofa_core::store::{Vault, VaultEntry};

fn make_entry(name: &str) -> VaultEntry {
    // Per-name secret so multi-entry vault tests don't have duplicate secrets
    // that could mask account-collision bugs.
    let secret = match name {
        "GitHub" => "STORETESTGITHUBA",
        "AWS" => "STORETESTAWSAAAA",
        _ => "STORETESTOTHERAA",
    };
    VaultEntry::new(
        name.to_string(),
        secret.to_string(),
        "2026-01-01".to_string(),
    )
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
    assert_eq!(loaded.entries()[0].secret, "STORETESTGITHUBA");
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

#[test]
fn add_entry_if_unique_returns_true_when_inserted() {
    let mut vault = Vault::new();
    let entry = VaultEntry::new(
        "GitHub:carlo".to_string(),
        "DEDUPTESTAAAAAAA".to_string(),
        "2026-05-07".to_string(),
    );
    let inserted = vault.add_entry_if_unique(entry);
    assert!(inserted, "first insertion should succeed");
    assert_eq!(vault.entries().len(), 1);
}

#[test]
fn add_entry_if_unique_skips_when_name_and_secret_match() {
    // Same (name, secret) is the dedup key — two identical imports of
    // the same QR shouldn't pile up in the vault.
    let mut vault = Vault::new();
    let entry = VaultEntry::new(
        "GitHub:carlo".to_string(),
        "DEDUPTESTAAAAAAA".to_string(),
        "2026-05-07".to_string(),
    );
    assert!(vault.add_entry_if_unique(entry.clone()));
    let inserted_again = vault.add_entry_if_unique(entry);
    assert!(
        !inserted_again,
        "second insertion should be reported as a dup"
    );
    assert_eq!(vault.entries().len(), 1);
}

#[test]
fn add_entry_if_unique_keeps_same_name_with_different_secret() {
    // Two entries can share a name when the secret differs — the user
    // rotated the OTP and wants to keep the old one until they've
    // verified the new one works. (This is also why the dedup key is
    // both fields, not name alone.)
    let mut vault = Vault::new();
    assert!(vault.add_entry_if_unique(VaultEntry::new(
        "GitHub:carlo".to_string(),
        "OLDSECRETAAAAAAA".to_string(),
        "2026-05-07".to_string(),
    )));
    assert!(vault.add_entry_if_unique(VaultEntry::new(
        "GitHub:carlo".to_string(),
        "NEWSECRETAAAAAAA".to_string(),
        "2026-05-07".to_string(),
    )));
    assert_eq!(vault.entries().len(), 2);
}

#[test]
fn add_entry_if_unique_keeps_same_secret_under_different_names() {
    // Same secret under two labels — the user filed the same OTP under
    // both "GitHub" and "Work / GitHub" intentionally. Don't merge.
    let mut vault = Vault::new();
    let secret = "SHAREDSECRETAAAA".to_string();
    assert!(vault.add_entry_if_unique(VaultEntry::new(
        "GitHub:carlo".to_string(),
        secret.clone(),
        "2026-05-07".to_string(),
    )));
    assert!(vault.add_entry_if_unique(VaultEntry::new(
        "Work/GitHub".to_string(),
        secret,
        "2026-05-07".to_string(),
    )));
    assert_eq!(vault.entries().len(), 2);
}
