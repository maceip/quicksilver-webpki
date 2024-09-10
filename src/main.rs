#![doc = include_str!("../README.md")]

use clap::Parser;

pub mod cli;

#[macro_use]
extern crate log;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    debug!("quicksilver-webpki",);

    Ok(())
}
