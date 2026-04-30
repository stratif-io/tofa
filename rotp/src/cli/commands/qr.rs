use crate::cli::CliResult;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct QrArgs {
    pub name: Option<String>,
    #[arg(long, conflicts_with = "name")]
    pub all: bool,
    #[arg(long)]
    pub output: Option<PathBuf>,
}

pub fn run(_args: QrArgs, _vault_path: PathBuf) -> CliResult {
    todo!()
}
