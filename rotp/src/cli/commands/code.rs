use crate::cli::CliResult;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct CodeArgs {
    pub name: String,
    #[arg(long)]
    pub raw: bool,
    #[arg(long)]
    pub copy: bool,
    #[arg(long)]
    pub watch: bool,
}

pub fn run(_args: CodeArgs, _vault_path: PathBuf) -> CliResult {
    todo!()
}
