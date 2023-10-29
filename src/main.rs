use clap::Arg;
use serde_json::json;
use std::io;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};
mod libs;
use colored::Colorize;
use include_dir::{include_dir, Dir};
use libs::api_client::Agent;

use clap::Parser;
use rustyline::DefaultEditor;
use std::io::prelude::*;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input: String,
}
//
// #[tokio::main]
fn main() {
    let args = Args::parse();
    println!("{}", args.input);
}
