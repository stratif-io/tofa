use crate::cli::CliResult;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct InitArgs {}

pub fn run(_args: InitArgs, _vault_path: PathBuf) -> CliResult {
    todo!()
}
