use crate::solve::Solver;

pub struct Day1 {}

impl Day1 {
    fn parse_input(&self, input: &str) -> Vec<String> {
        let input:Vec<_> = input.split('\n').collect();
        input.iter().map(|x| x.to_string()).collect()
    }
}


impl Solver<1> for Day1 {
    type Part1 = i32;
    type Part2 = i32;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let input = self.parse_input(input);

        let mut zero_num = 0;
        let mut lock: i32 = 50;
            
        for l in input.into_iter() {
            let dir = l.chars().next().unwrap();
            let num: i32 = l[1..].parse().unwrap();
            lock = match dir {
                'L' => lock + -num + 100,
                'R' => lock + num + 100,
                _ => panic!("unknown direction")
            } % 100;

            if lock == 0 {
                zero_num += 1;
            }
        }

        zero_num
    }

    fn solve_part_two(&self, _input: &str) -> Self::Part2 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{day1::Day1, solve::{Solver}};

    #[test]
    fn day1_part1() {
        let solve = Day1 {};
        let result = solve.solve_part_one(
r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#,
        );
        assert_eq!(3, result);
    }
}
