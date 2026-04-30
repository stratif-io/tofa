use crate::cli::CliResult;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct RemoveArgs {
    pub name: String,
}

pub fn run(_args: RemoveArgs, _vault_path: PathBuf) -> CliResult {
    todo!()
}
