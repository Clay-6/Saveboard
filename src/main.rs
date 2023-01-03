mod cli;

use std::path::PathBuf;

use clap::Parser;
use cli::Args;
use color_eyre::{Report, Result};

use arboard::Clipboard;

use image::RgbaImage;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let output = if let Some(path) = args.output {
        path
    } else {
        default_dir()?
    };

    let screenshot = Clipboard::new()?.get_image()?;

    if let Some(img) = RgbaImage::from_raw(
        screenshot.width.try_into()?,
        screenshot.height.try_into()?,
        screenshot.bytes.to_vec(),
    ) {
        img.save(output)?;
    }

    Ok(())
}

fn default_dir() -> Result<PathBuf> {
    match home::home_dir() {
        Some(dir) => Ok(dir.join("clipboard-img.png")),
        None => Err(Report::msg("Failed to locate home directory")),
    }
}
