use clap::{command, Parser};

use crate::solve::Solver;

mod solve;

mod day1;
mod day2;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    day: u32
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => day1::Day1{}.solve(),
        2 => day2::Day2{}.solve(),
        _ => panic!("solver not found")
    };
}
