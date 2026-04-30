use crate::cli::CliResult;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct RenameArgs {
    pub name: String,
    pub new_name: String,
}

pub fn run(_args: RenameArgs, _vault_path: PathBuf) -> CliResult {
    todo!()
}
