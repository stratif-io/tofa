use crate::cli::CliResult;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct ImportArgs {
    pub file: PathBuf,
}

pub fn run(_args: ImportArgs, _vault_path: PathBuf) -> CliResult {
    todo!()
}
