mod cli;

use std::path::PathBuf;

use clap::Parser;
use cli::Args;
use color_eyre::{eyre::eyre, Result};

use arboard::Clipboard;

use image::ColorType;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let output = if let Some(path) = args.output {
        path
    } else {
        default_dir()?
    };

    let screenshot = Clipboard::new()?.get_image()?;

    image::save_buffer(
        output,
        &screenshot.bytes,
        screenshot.width.try_into()?,
        screenshot.height.try_into()?,
        ColorType::Rgba8,
    )?;

    Ok(())
}

fn default_dir() -> Result<PathBuf> {
    match home::home_dir() {
        Some(dir) => Ok(dir.join("clipboard-img.png")),
        None => Err(eyre!("Failed to locate home directory")),
    }
}
