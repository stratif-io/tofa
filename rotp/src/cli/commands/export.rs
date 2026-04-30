use crate::cli::CliResult;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct ExportArgs {
    #[arg(long)]
    pub output: Option<PathBuf>,
}

pub fn run(_args: ExportArgs, _vault_path: PathBuf) -> CliResult {
    todo!()
}
