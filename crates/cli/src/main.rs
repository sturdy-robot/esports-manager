use std::process::ExitCode;

use clap::Parser;

fn main() -> ExitCode {
    let cli = cli::Cli::parse();
    ExitCode::from(cli::run(
        &cli,
        &mut std::io::stdout(),
        &mut std::io::stderr(),
    ))
}
