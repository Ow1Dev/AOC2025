use std::{fs, path::Path};

use clap::{command, Parser};

mod day1;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    day: u32
}

fn main() {
    let args = Args::parse();

    let file_path = Path::new("./input/day1.txt");
    let content = fs::read_to_string(file_path).unwrap();

    let result = match args.day {
        1 => day1::Solver{}.solve_part_one(&content.trim()),
        _ => panic!("solver not found")
    };

    println!("Anwser: {}" , result);
}
