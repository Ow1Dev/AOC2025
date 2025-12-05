use clap::{command, Parser};

use crate::solve::Solver;

mod solve;

mod day1;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    day: u32
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => day1::Day1{}.solve(),
        _ => panic!("solver not found")
    };
}
