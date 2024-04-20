use anyhow::Result;
use clap::Parser;
use rust_cli_template::{cli::CliArgs, errors::errors_handling, run};
use tracing_log::AsTrace;

fn main() -> Result<()> {
    // Get command line arguments
    let args = CliArgs::parse();

    // Initialize trace
    tracing_subscriber::fmt()
        .with_max_level(args.verbose.log_level_filter().as_trace())
        .init();

    match run() {
        Err(error) => errors_handling(error),
        Ok(()) => Ok(()),
    }
}
