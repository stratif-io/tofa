mod render;

use anyhow::{bail, Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use std::fs;
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
        Cmd::GenDocs { check } => gen_docs(check),
    }
}

fn gen_docs(check: bool) -> Result<()> {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let ref_dir = workspace_root.join("docs/book/src/reference");
    fs::create_dir_all(&ref_dir).with_context(|| format!("creating {}", ref_dir.display()))?;

    let root = tofa::cli::Cli::command();
    let mut diffs: Vec<String> = Vec::new();

    for sub in root.get_subcommands() {
        let name = sub.get_name();
        let block = render::render_help_block(sub);
        let path = ref_dir.join(format!("{name}.md"));
        let new_content = if path.exists() {
            let current = fs::read_to_string(&path)?;
            render::replace_block(&current, &block)
        } else {
            render::scaffold_page(name, &block)
        };

        let on_disk = fs::read_to_string(&path).unwrap_or_default();
        if on_disk != new_content {
            if check {
                diffs.push(path.display().to_string());
            } else {
                fs::write(&path, &new_content)
                    .with_context(|| format!("writing {}", path.display()))?;
                println!("updated {}", path.display());
            }
        }
    }

    if check && !diffs.is_empty() {
        eprintln!("docs out of sync; run `cargo run -p xtask -- gen-docs`:");
        for f in diffs {
            eprintln!("  {f}");
        }
        bail!("gen-docs --check failed");
    }
    Ok(())
}
