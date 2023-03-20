use clap::Parser;
mod args;
use args::*;
use std::fs::{self, read_to_string};

fn main() {
    let args = Args::parse();
    for i in &args.path {
        let file = fs::read_to_string(i);
        match file {
            Err(e) => println!("{}", e),
            _ => println!("{}", fs::read_to_string(i).unwrap())
        }
    }
}
