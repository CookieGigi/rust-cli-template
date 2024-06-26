use anyhow::Result;
use clap::Parser;
use {{crate_name}}::{cli::CliArgs, config::Config, errors::errors_handling, run};
use tracing_log::AsTrace;

#[cfg(not(tarpaulin_include))]
fn main() -> Result<()> {
    // Get command line arguments
    let args = CliArgs::parse();

    // Get config
    let _config: Config = confy::load("{{project-name}}", None)?;

    // Initialize trace
    tracing_subscriber::fmt()
        .with_max_level(args.verbose.log_level_filter().as_trace())
        .init();

    match run() {
        Err(error) => errors_handling(error),
        Ok(()) => {
            // Success Message
            tracing::info!("Success !");
            Ok(())
        }
    }
}
