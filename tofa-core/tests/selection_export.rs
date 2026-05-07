use tofa_core::qr::{build_selection_uri, parse_input, parse_migration, SelectionExportError};
use tofa_core::store::VaultEntry;

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
fn empty_selection_is_rejected() {
    let result = build_selection_uri(&[]);
    assert!(matches!(result, Err(SelectionExportError::Empty)));
}

#[test]
fn single_entry_uses_otpauth_and_preserves_period() {
    // A single non-default entry must round-trip every field — the migration
    // format would clobber the period, so single-entry selections must take
    // the otpauth:// path.
    let e = entry(
        "GitHub:alice@example.com",
        "SELEXSINGLEAAAAA",
        "SHA256",
        8,
        60,
    );
    let uri = build_selection_uri(std::slice::from_ref(&e)).expect("ok");
    assert!(uri.starts_with("otpauth://totp/"), "uri: {uri}");

    let parsed = parse_input(&uri).expect("parse");
    assert_eq!(parsed.secret, "SELEXSINGLEAAAAA");
    assert_eq!(parsed.meta.algorithm.as_deref(), Some("SHA256"));
    assert_eq!(parsed.meta.digits, Some(8));
    assert_eq!(parsed.meta.period, Some(60));
}

#[test]
fn multiple_30s_entries_use_migration_format() {
    let entries = vec![
        entry("GitHub:alice", "SELEXPAIRAAAAAAA", "SHA1", 6, 30),
        entry("GitLab:bob", "SELEXPAIRBBBBBBB", "SHA256", 6, 30),
    ];
    let uri = build_selection_uri(&entries).expect("ok");
    assert!(uri.starts_with("otpauth-migration://"), "uri: {uri}");

    let parsed = parse_migration(&uri).expect("parse migration");
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].secret, "SELEXPAIRAAAAAAA");
    assert_eq!(parsed[1].secret, "SELEXPAIRBBBBBBB");
}

#[test]
fn multi_selection_with_mixed_periods_is_rejected() {
    let entries = vec![
        entry("Standard:alice", "SELEXMIXPAAAAAAA", "SHA1", 6, 30),
        entry("Custom:bob", "SELEXMIXPBBBBBBB", "SHA1", 6, 60),
    ];
    let result = build_selection_uri(&entries);
    match result {
        Err(SelectionExportError::NonStandardPeriod {
            offending_count,
            total,
        }) => {
            assert_eq!(offending_count, 1);
            assert_eq!(total, 2);
        }
        other => panic!("expected NonStandardPeriod, got {other:?}"),
    }
}

#[test]
fn multi_selection_all_non_30s_is_rejected() {
    let entries = vec![
        entry("Custom:alice", "SELEXALLSIXTAAAA", "SHA1", 6, 60),
        entry("Custom:bob", "SELEXALLSIXTBBBB", "SHA1", 6, 60),
    ];
    let result = build_selection_uri(&entries);
    match result {
        Err(SelectionExportError::NonStandardPeriod {
            offending_count,
            total,
        }) => {
            assert_eq!(offending_count, 2);
            assert_eq!(total, 2);
        }
        other => panic!("expected NonStandardPeriod, got {other:?}"),
    }
}

#[test]
fn single_non_30s_entry_is_allowed_via_otpauth() {
    // Single-entry path doesn't go through the migration format, so non-30s
    // is fine — the otpauth:// URI carries the period.
    let e = entry("Custom:alice", "SELEXSOLOSIXTAAA", "SHA1", 6, 60);
    let uri = build_selection_uri(std::slice::from_ref(&e)).expect("ok");
    assert!(uri.starts_with("otpauth://totp/"));
    let parsed = parse_input(&uri).expect("parse");
    assert_eq!(parsed.meta.period, Some(60));
}
