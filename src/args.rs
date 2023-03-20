use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author = "Cat", version, about, long_about = None)]
pub struct Args {
    /// Path to file
   #[arg()]
   pub path: Vec<String>,
}