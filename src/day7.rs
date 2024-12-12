use std::fs;

pub(crate) fn day7(input_filename: &str) {
    let contents = fs::read_to_string(input_filename).expect("could not read file");
    let equations = parse_equations(&contents);
    let sum = sum_valid_equations(&equations);
    println!("sum of valid equation lhs: {}", sum);
}

fn sum_valid_equations(equations: &Vec<Equation>) -> u64 {
    equations
        .iter()
        .filter(|eqn| eqn.is_valid())
        .map(|eqn| eqn.lhs)
        .sum()
}

fn parse_equations(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let split = line.split(": ").collect::<Vec<_>>();
            let lhs = split[0].parse::<u64>().unwrap();
            let rhs = split[1]
                .split(" ")
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            Equation { lhs, rhs }
        })
        .collect()
}

struct Equation {
    lhs: u64,
    rhs: Vec<u64>,
}

impl Equation {
    fn is_valid(&self) -> bool {
        let n_terms = self.rhs.len() as u32;
        let mut is_valid = false;
        // generates all possible combinations of + and * placement
        // by incrementing a number and then checking its bits.
        // to implement part 2, rewrite this loop to also support the
        // concatenation operator.
        for num in 0..(2_u64.pow(n_terms - 1)) {
            let mut acc = self.rhs[0];
            for idx in 0..(n_terms - 1) {
                let term = self.rhs[idx as usize + 1];
                if num & (1 << idx) != 0 {
                    acc *= term;
                } else {
                    acc += term;
                }
            }
            if self.lhs == acc {
                is_valid = true;
            }
        }

        is_valid
    }
}

mod tests {
    use super::{parse_equations, sum_valid_equations, Equation};

    #[test]
    fn check_parse() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
        let equations = parse_equations(input);
        assert_eq!(equations.len(), 9);
        assert_eq!(equations[0].lhs, 190);
        assert_eq!(equations[0].rhs, vec![10, 19]);
    }

    #[test]
    fn check_is_valid() {
        let eqn = Equation {
            lhs: 3267,
            rhs: vec![81, 40, 27],
        };
        assert_eq!(eqn.is_valid(), true);

        let eqn = Equation {
            lhs: 190,
            rhs: vec![19, 10],
        };
        assert_eq!(eqn.is_valid(), true);

        let eqn = Equation {
            lhs: 83,
            rhs: vec![17, 5],
        };
        assert_eq!(eqn.is_valid(), false);
    }

    #[test]
    fn check_sum() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
        let equations = parse_equations(input);
        let sum = sum_valid_equations(&equations);
        assert_eq!(sum, 3749);
    }
}
