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

    let img = Clipboard::new()?.get_image()?;

    image::save_buffer(
        &output,
        &img.bytes,
        img.width.try_into()?,
        img.height.try_into()?,
        ColorType::Rgba8,
    )?;

    println!(
        "Image saved as {}",
        output.canonicalize()?.to_string_lossy()
    );

    Ok(())
}

fn default_dir() -> Result<PathBuf> {
    match home::home_dir() {
        Some(dir) => Ok(dir.join("clipboard-img.png")),
        None => Err(eyre!("Failed to locate home directory")),
    }
}
