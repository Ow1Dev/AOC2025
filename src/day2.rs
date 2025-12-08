use crate::solve::Solver;

pub struct Day2 {}

impl Day2 {
    fn parse_input(&self, input: &str) -> Vec<(u64, u64)> {
        let result: Vec<_> = input.split(',').collect();
        result
            .into_iter()
            .map(|x| {
                let nums: Vec<_> = x.split('-').map(|n| n.parse::<_>().expect("Could not parse input")).collect();
                (nums[0], nums[1])
            })
            .collect()
    }
}

impl Solver<2> for Day2 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let mut result = 0;
        for (start, end) in self.parse_input(input) {
            let half = end.to_string().len() / 2;
            for i in start..=end {
                let val = i.to_string();
                let (first, second) = val.split_at(half);

                if first == second {
                    result += i;
                }
            }
        }

        result
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let _ = self.parse_input(input);
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{day2::Day2, solve::Solver};

    #[test]
    fn day2_part1() {
        let solve = Day2 {};
        let result = solve.solve_part_one("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124");
        assert_eq!(1227775554, result);
    }
}
