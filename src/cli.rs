use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, about, long_about=None)]
pub struct Args {
    /// The file path to output your copied image as
    #[clap(short, long, value_name = "PATH")]
    pub output: Option<PathBuf>,
}
