use clap::Parser;
mod args;
use args::*;
use core::panic;
use std::{fs::{self, read_to_string}, io::ErrorKind};

fn main() {
    let args = Args::parse();
    for i in args.path {
        let file = fs::read_to_string(i);
        let file = match file {
            Ok(file) => file,
            Err(e) => match e.kind() {
                ErrorKind::PermissionDenied => "Permission denied!",
                ErrorKind::NotFound => "File not found!",
                _ => "Unknow error!",
            }.to_owned(),
        };
        println!("{file}");
    }
}
