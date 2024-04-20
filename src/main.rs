use clap::Parser;
use rust_cli_template::cli::CliArgs;
use tracing_log::AsTrace;

fn main() {
    let args = CliArgs::parse();

    tracing_subscriber::fmt()
        .with_max_level(args.verbose.log_level_filter().as_trace())
        .init();
}
