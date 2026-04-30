mod cli;
mod tui;

fn main() {
    cli::parse();

    if let Err(e) = tui::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
