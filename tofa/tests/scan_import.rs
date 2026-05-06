use tofa::cli::commands::scan::import_uris_into_vault;
use tofa_core::qr::{build_otpauth_uri, generate_migration_uri, MigrationAccount};
use tofa_core::store::{Vault, VaultEntry};

fn entry(name: &str, secret: &str, algorithm: &str, digits: u8, period: u32) -> VaultEntry {
    VaultEntry {
        id: String::new(),
        name: name.to_string(),
        secret: secret.to_string(),
        created_at: String::new(),
        period,
        digits,
        algorithm: algorithm.to_string(),
    }
}

#[test]
fn empty_uri_list_imports_nothing() {
    let mut vault = Vault::new();
    let count = import_uris_into_vault(&[], &mut vault, None).expect("ok");
    assert_eq!(count, 0);
    assert_eq!(vault.entries().len(), 0);
}

#[test]
fn single_otpauth_uri_imports_one_entry() {
    let e = entry("GitHub:alice", "JBSWY3DPEHPK3PXP", "SHA256", 8, 60);
    let uri = build_otpauth_uri(&e);

    let mut vault = Vault::new();
    let count = import_uris_into_vault(&[uri], &mut vault, None).expect("ok");

    assert_eq!(count, 1);
    let added = &vault.entries()[0];
    assert_eq!(added.secret, "JBSWY3DPEHPK3PXP");
    assert_eq!(added.algorithm, "SHA256");
    assert_eq!(added.digits, 8);
    assert_eq!(added.period, 60);
}

#[test]
fn migration_uri_imports_each_account() {
    let accounts = [
        MigrationAccount {
            name: "alice@example.com",
            issuer: "Foo",
            secret_b32: "JBSWY3DPEHPK3PXP",
            algorithm: "SHA1",
            digits: 6,
        },
        MigrationAccount {
            name: "bob@example.com",
            issuer: "Bar",
            secret_b32: "MFRGGZDFM5XW6YTBOI",
            algorithm: "SHA1",
            digits: 6,
        },
    ];
    let uri = generate_migration_uri(&accounts).expect("ok");

    let mut vault = Vault::new();
    let count = import_uris_into_vault(&[uri], &mut vault, None).expect("ok");

    assert_eq!(count, 2);
    assert_eq!(vault.entries().len(), 2);
    let secrets: Vec<&str> = vault.entries().iter().map(|e| e.secret.as_str()).collect();
    assert!(secrets.contains(&"JBSWY3DPEHPK3PXP"));
    assert!(secrets.contains(&"MFRGGZDFM5XW6YTBOI"));
}

#[test]
fn mixed_otpauth_and_migration_uris_are_all_imported() {
    let solo_uri = build_otpauth_uri(&entry(
        "Solo:carol",
        "ORSXG5BAONUGCZDPN5SGSZJB",
        "SHA1",
        6,
        30,
    ));
    let migration_uri = generate_migration_uri(&[
        MigrationAccount {
            name: "alice@example.com",
            issuer: "Foo",
            secret_b32: "JBSWY3DPEHPK3PXP",
            algorithm: "SHA1",
            digits: 6,
        },
        MigrationAccount {
            name: "bob@example.com",
            issuer: "Bar",
            secret_b32: "MFRGGZDFM5XW6YTBOI",
            algorithm: "SHA1",
            digits: 6,
        },
    ])
    .expect("ok");

    let mut vault = Vault::new();
    let count = import_uris_into_vault(&[solo_uri, migration_uri], &mut vault, None).expect("ok");
    assert_eq!(count, 3);
    assert_eq!(vault.entries().len(), 3);
}

#[test]
fn name_override_applies_when_exactly_one_entry_imported() {
    let e = entry("DerivedName:alice", "JBSWY3DPEHPK3PXP", "SHA1", 6, 30);
    let uri = build_otpauth_uri(&e);

    let mut vault = Vault::new();
    let count = import_uris_into_vault(&[uri], &mut vault, Some("MyCustomName")).expect("ok");
    assert_eq!(count, 1);
    assert_eq!(vault.entries()[0].name, "MyCustomName");
}

#[test]
fn name_override_is_ignored_when_multiple_entries_imported() {
    // --name doesn't make sense when scanning yields >1 entry; the helper
    // should fall back to per-entry derived names rather than collapse them.
    let migration_uri = generate_migration_uri(&[
        MigrationAccount {
            name: "alice@example.com",
            issuer: "Foo",
            secret_b32: "JBSWY3DPEHPK3PXP",
            algorithm: "SHA1",
            digits: 6,
        },
        MigrationAccount {
            name: "bob@example.com",
            issuer: "Bar",
            secret_b32: "MFRGGZDFM5XW6YTBOI",
            algorithm: "SHA1",
            digits: 6,
        },
    ])
    .expect("ok");

    let mut vault = Vault::new();
    let count = import_uris_into_vault(&[migration_uri], &mut vault, Some("Ignored")).expect("ok");
    assert_eq!(count, 2);
    let names: Vec<&str> = vault.entries().iter().map(|e| e.name.as_str()).collect();
    assert!(names.iter().all(|n| *n != "Ignored"));
}
