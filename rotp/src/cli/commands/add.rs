use crate::cli::CliResult;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct AddArgs {
    #[arg(long)]
    pub name: Option<String>,
    #[arg(long)]
    pub secret: Option<String>,
    #[arg(long)]
    pub uri: Option<String>,
    #[arg(long)]
    pub qr: Option<PathBuf>,
}

pub fn run(_args: AddArgs, _vault_path: PathBuf) -> CliResult {
    todo!()
}
