use std::{fmt::Debug, fs};

pub trait Solver<const DAY: u32> {
    type Part1: Debug;
    type Part2: Debug;

    fn solve_part_one(&self, input: &str) -> Self::Part1;
    fn solve_part_two(&self, input: &str) -> Self::Part2;

    fn solve(&self) {
        let path = format!("./input/day{}.txt", DAY);
        let content = fs::read_to_string(&path)
            .expect("failed to read input file");

        let content = content.trim();

        println!("part 1: {:?}", self.solve_part_one(&content));
        println!("part 2: {:?}", self.solve_part_two(&content));
    }
}
