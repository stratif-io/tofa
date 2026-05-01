use crate::cli::CliResult;
use clap::Args;
use clap_complete::{generate, Shell};
use std::io;

#[derive(Args)]
pub struct CompletionsArgs {
    pub shell: Shell,
}

pub fn run(args: CompletionsArgs) -> CliResult {
    use clap::CommandFactory;
    let mut cmd = crate::cli::Cli::command();
    generate(args.shell, &mut cmd, "tofa", &mut io::stdout());
    Ok(())
}
