//! Cli Arguments Parsing

use clap::Parser;

/// Cli arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {}
