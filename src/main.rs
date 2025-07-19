// src/main.rs
/*
 * Main executable for DatabaseoptimizationKit
 */

use clap::Parser;
use databaseoptimizationkit::{Result, run};

#[derive(Parser)]
#[command(version, about = DatabaseoptimizationKit - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
