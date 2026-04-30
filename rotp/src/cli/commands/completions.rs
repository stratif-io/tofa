use crate::cli::CliResult;
use clap::Args;
use clap_complete::Shell;

#[derive(Args)]
pub struct CompletionsArgs {
    pub shell: Shell,
}

pub fn run(_args: CompletionsArgs) -> CliResult {
    todo!()
}
