mod render;

use anyhow::Result;
use clap::{Parser, Subcommand};

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
        Cmd::GenDocs { check: _ } => {
            println!("gen-docs: not implemented yet");
            Ok(())
        }
    }
}
