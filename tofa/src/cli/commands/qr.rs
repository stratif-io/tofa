use crate::cli::{find_entry, open_vault, read_passphrase, CliResult};
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct QrArgs {
    /// Entry id or name (partial match)
    pub name: Option<String>,
    /// Export all accounts as a migration QR
    #[arg(long, conflicts_with = "name")]
    pub all: bool,
    /// Emit one otpauth:// QR per entry instead of a single migration QR.
    /// Requires `--all` and `--output-dir`. Preserves period/algorithm/digits
    /// for every entry — use this when the migration format would refuse
    /// because the selection mixes 30s and non-30s entries.
    #[arg(long, requires = "all", requires = "output_dir")]
    pub multi: bool,
    /// Save QR as PNG instead of displaying in terminal (single-QR modes).
    #[arg(long, value_name = "PATH", conflicts_with = "output_dir")]
    pub output: Option<PathBuf>,
    /// Directory to write per-entry PNGs into when using `--multi`.
    #[arg(long, value_name = "DIR")]
    pub output_dir: Option<PathBuf>,
}

pub fn run(args: QrArgs, vault_path: PathBuf) -> CliResult {
    let pass = read_passphrase("Passphrase: ")?;
    let vault = open_vault(&vault_path, &pass)?;

    if args.multi {
        // --multi: write one otpauth:// PNG per entry into --output-dir.
        let dir = args
            .output_dir
            .as_deref()
            .ok_or("--multi requires --output-dir")?;
        std::fs::create_dir_all(dir).map_err(|e| format!("create output dir: {e}"))?;
        let entries = vault.entries();
        if entries.is_empty() {
            return Err("vault has no entries".into());
        }
        for (i, entry) in entries.iter().enumerate() {
            let uri = tofa_core::qr::build_otpauth_uri(entry);
            let filename = format!("{:02}-{}.png", i + 1, sanitize_filename(&entry.name));
            let path = dir.join(&filename);
            tofa_core::uri_to_qr_png(&uri, &path)
                .map_err(|e| format!("PNG generation failed for {filename}: {e}"))?;
        }
        eprintln!("Wrote {} QR PNG(s) to {}", entries.len(), dir.display());
        return Ok(());
    }

    let uri = if args.all {
        // build_selection_uri picks the right format: a single otpauth:// when
        // there's exactly one entry, otpauth-migration:// when multiple all-30s
        // entries, and refuses with NonStandardPeriod when a multi-selection
        // includes a non-30s entry that the migration format can't carry.
        let entries: Vec<_> = vault.entries().to_vec();
        tofa_core::build_selection_uri(&entries).map_err(|e| e.to_string())?
    } else {
        let name = args.name.as_deref().ok_or("provide a name or --all")?;
        let (_, entry) = find_entry(&vault, name)?;
        tofa_core::qr::build_otpauth_uri(entry)
    };

    if let Some(out_path) = args.output {
        tofa_core::uri_to_qr_png(&uri, &out_path)
            .map_err(|e| format!("PNG generation failed: {e}"))?;
        eprintln!("QR saved to {}", out_path.display());
    } else {
        for line in tofa_core::uri_to_qr_lines(&uri) {
            println!("{line}");
        }
    }
    Ok(())
}

/// Replace path-unsafe characters in an entry name so it can be a filename.
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' => c,
            _ => '_',
        })
        .collect()
}
