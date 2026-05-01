use clap::Parser;

fn main() {
    let cli = tofa::cli::Cli::parse();
    match cli.command {
        None => {
            if let Err(e) = tofa::tui::run() {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        Some(cmd) => {
            if let Err(e) = tofa::cli::dispatch(cmd, cli.vault) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    }
}
