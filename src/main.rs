mod cli;

use std::path::PathBuf;

use clap::Parser;
use cli::Args;
use color_eyre::{Report, Result};

use arboard::Clipboard;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let output = if let Some(path) = args.output {
        path
    } else {
        default_dir()?
    };

    let mut clipboard = Clipboard::new()?;
    let img = clipboard.get_image()?.to_owned_img();

    println!("{img:?}");

    Ok(())
}

fn default_dir() -> Result<PathBuf> {
    match home::home_dir() {
        Some(dir) => Ok(dir.join("clipboard-img.png")),
        None => Err(Report::msg("Failed to locate home directory")),
    }
}
