fn main() {
    rotp::cli::parse();

    if let Err(e) = rotp::tui::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
