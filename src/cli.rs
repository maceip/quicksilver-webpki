//! Command line arguments
use clap_verbosity_flag::Verbosity;
use std::path::PathBuf;

use clap::Parser;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "root")]
    pub root: String,

    #[clap(flatten)]
    pub verbose: Verbosity,

    pub input: Option<PathBuf>,
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_args() {
        use clap::CommandFactory;
        super::Args::command().debug_assert()
    }
}
