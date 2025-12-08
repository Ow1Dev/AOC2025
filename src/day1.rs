use crate::solve::Solver;

pub struct Day1 {}

impl Day1 {
    fn parse_input(&self, input: &str) -> Vec<String> {
        let input: Vec<_> = input.split('\n').collect();
        input.iter().map(|x| x.to_string()).collect()
    }
}

impl Solver<1> for Day1 {
    type Part1 = i32;
    type Part2 = i32;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let input = self.parse_input(input);

        let mut pass = 0;
        let mut rot: i32 = 50;

        for l in input.into_iter() {
            let dir = l.chars().next().unwrap();
            let num: i32 = l[1..].parse().unwrap();
            rot = match dir {
                'L' => rot + -num,
                'R' => rot + num,
                _ => panic!("unknown direction"),
            };

            rot = (rot + 100) % 100;

            if rot == 0 {
                pass += 1;
            }
        }

        pass
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let input = self.parse_input(input);

        let mut dailpos = 50;
        let mut password = 0;

        for l in input {
            let dir = l.chars().next().expect("expect a direction");
            let num: i32 = l[1..].parse().expect("expect to be a number");

            for _ in 0..num  {
                dailpos = (dailpos + delta).rem_euclid(100);
                dailpos += match dir {
                    'L' => -1,
                    'R' => 1,
                    _ => panic!("unknown direction"),
                };

                if dailpos == 100 {
                    dailpos = 0;
                }

                if dailpos == -1 {
                    dailpos = 99;
                }

                if dailpos == 0 {
                    password += 1;
                }
            }
        }
        password
    }
}

#[cfg(test)]
mod tests {
    use crate::{day1::Day1, solve::Solver};

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

    #[test]
    fn day1_part2() {
        let solve = Day1 {};
        let result = solve.solve_part_two(
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
        assert_eq!(6, result);
    }
}
