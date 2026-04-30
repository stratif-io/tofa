use clap::Parser;

#[derive(Parser)]
#[command(
    name = "rotp",
    about = "Eye-candy terminal OTP manager",
    version,
    long_about = None
)]
pub struct Cli {}

pub fn parse() -> Cli {
    Cli::parse()
}
