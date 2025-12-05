use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    day: u32
}

fn main() {
    let args = Args::parse();

    println!("{}", args.day);
}
