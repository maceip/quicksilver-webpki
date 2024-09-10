use clap::CommandFactory;
use clap_complete::{generate_to, shells::Shell};
use path_absolutize::*;

#[path = "src/cli.rs"]
mod cli;

fn main() -> std::io::Result<()> {
    let out_dir =
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);
    let mut cmd = cli::Args::command();

    let completion_dir = out_dir.join("../../../completion");
    let absolute_completion_dir = completion_dir.absolutize()?;
    std::fs::create_dir_all(absolute_completion_dir.clone())?;
    for shell in [Shell::Bash, Shell::Fish, Shell::Zsh] {
        generate_to(
            shell,
            &mut cmd,
            "quicksilver-webpki",
            absolute_completion_dir.as_ref(),
        )?;
    }

    println!("cargo:rerun-if-changed=src/cli.rs");

    Ok(())
}
