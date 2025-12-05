pub struct Solver {}

impl Solver {
    fn parse_input(&self, input: &str) -> Vec<String> {
        let input:Vec<_> = input.split('\n').collect();
        input.iter().map(|x| x.to_string()).collect()
    }
}


type Part1 = i32;

impl Solver {
    pub fn solve_part_one(&self, input: &str) -> Part1 {
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
}

#[cfg(test)]
mod tests {
    use crate::day1::{self};

    #[test]
    fn day1_part1() {
        let solver = day1::Solver {};
        let result = solver.solve_part_one(
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
