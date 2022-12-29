use std::path::PathBuf;

use clap::Parser;
use cli::Args;
use color_eyre::{Report, Result};

mod cli;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let output = if let Some(path) = args.output {
        path
    } else {
        default_dir()?
    };

    println!("{}", output.display());

    Ok(())
}

fn default_dir() -> Result<PathBuf> {
    match home::home_dir() {
        Some(dir) => Ok(dir.join("clipboard-img.png")),
        None => Err(Report::msg("Failed to locate home directory")),
    }
}
