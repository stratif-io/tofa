use std::fs;
use tempfile::tempdir;

#[test]
fn gen_docs_creates_one_file_per_subcommand() {
    let tmp = tempdir().unwrap();
    let diffs = xtask::gen_docs_at(tmp.path(), false).unwrap();
    assert!(diffs.is_empty(), "non-check mode should not return diffs");

    let ref_dir = tmp.path().join("docs/book/src/reference");
    let names = [
        "init",
        "list",
        "code",
        "add",
        "remove",
        "rename",
        "qr",
        "rekey",
        "export",
        "import",
        "scan",
        "cam",
        "completions",
        "destroy",
    ];
    for n in names {
        let path = ref_dir.join(format!("{n}.md"));
        assert!(path.exists(), "missing {}", path.display());
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("<!-- BEGIN auto:help -->"));
        assert!(content.contains("<!-- END auto:help -->"));
        assert!(content.contains(&format!("# tofa {n}")));
    }
}

#[test]
fn gen_docs_check_returns_diffs_on_stale_file() {
    let tmp = tempdir().unwrap();
    let _ = xtask::gen_docs_at(tmp.path(), false).unwrap();

    // Corrupt one file
    let p = tmp.path().join("docs/book/src/reference/add.md");
    fs::write(&p, "# tofa add\n\nstale\n").unwrap();

    let diffs = xtask::gen_docs_at(tmp.path(), true).unwrap();
    assert_eq!(diffs.len(), 1);
    assert!(diffs[0].ends_with("add.md"));
}
