use std::{fmt::Debug, fs, path::Path};

pub trait Solver<const DAY: u32> {
    type Part1: Debug;
    type Part2: Debug;

    fn solve_part_one(&self, input: &str) -> Self::Part1;
    fn solve_part_two(&self, input: &str) -> Self::Part2;

    fn solve(&self) {
        let path = format!("./input/{}.txt", DAY);
        let file_path = Path::new(&path);
        let content = fs::read_to_string(file_path).unwrap();

        println!("part 1: {:?}", self.solve_part_one(&content));
        println!("part 2: {:?}", self.solve_part_two(&content));
    }
}
