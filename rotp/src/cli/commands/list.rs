use crate::cli::CliResult;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct ListArgs {
    #[arg(long)]
    pub codes: bool,
}

pub fn run(_args: ListArgs, _vault_path: PathBuf) -> CliResult {
    todo!()
}
