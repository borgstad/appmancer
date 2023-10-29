mod libs;
use clap::Parser;

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
