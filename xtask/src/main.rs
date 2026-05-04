use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "xtask", about = "Project automation tasks")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Regenerate auto-generated CLI reference blocks
    GenDocs {
        /// Fail with non-zero exit if any file would change
        #[arg(long)]
        check: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::GenDocs { check } => {
            let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .to_path_buf();
            let diffs = xtask::gen_docs_at(&workspace_root, check)?;
            if check && !diffs.is_empty() {
                eprintln!("docs out of sync; run `cargo run -p xtask -- gen-docs`:");
                for f in &diffs {
                    eprintln!("  {f}");
                }
                bail!("gen-docs --check failed");
            }
            Ok(())
        }
    }
}
