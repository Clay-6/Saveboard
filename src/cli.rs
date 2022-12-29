use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(short, long)]
    pub output: Option<PathBuf>,
}
