use clap::Parser;
use rust_cli_template::cli::CliArgs;
use tracing_log::AsTrace;

fn main() {
    // Get command line arguments
    let args = CliArgs::parse();

    // Initialize trace
    tracing_subscriber::fmt()
        .with_max_level(args.verbose.log_level_filter().as_trace())
        .init();
}
