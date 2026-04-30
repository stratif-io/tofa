use clap::Parser;

fn main() {
    let cli = rotp::cli::Cli::parse();
    match cli.command {
        None => {
            if let Err(e) = rotp::tui::run() {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        Some(cmd) => {
            if let Err(e) = rotp::cli::dispatch(cmd, cli.vault) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    }
}
