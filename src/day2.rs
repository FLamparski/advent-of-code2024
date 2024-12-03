use std::{cell::RefCell, collections::HashSet, fs};

pub(crate) fn day2(input_filename: &str) {
    // read and parse the input file

    let contents = fs::read_to_string(input_filename).expect("could not read file");

    let num_safe_reports = count_safe_reports(&contents);
    let num_safe_reports_with_damping = count_safe_reports_with_damping(&contents);

    println!("number of safe reports: {}", num_safe_reports);
}

fn count_safe_reports(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse::<i8>().unwrap())
                .collect::<Vec<i8>>()
        })
        .filter(|report| get_violations(report).is_empty())
        .count()
}

// TODO finish later
fn count_safe_reports_with_damping(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse::<i8>().unwrap())
                .collect::<Vec<i8>>()
        })
        .filter(|report| is_report_safe_with_damping(&report))
        .count()
}

fn is_report_safe_with_damping(report: &Vec<i8>) -> bool {
    let violations = get_violations(report).drain().collect::<Vec<_>>();

    if violations.is_empty() {
        return true;
    }

    let is_safe_after_damping = violations
        .iter()
        .map(|i| {
            let mut report = report.to_owned();
            report.remove(i - 1);
            get_violations(&report)
        })
        .any(|violations| violations.is_empty());

    if !is_safe_after_damping {
        println!("unsafe after damping: {:?}", report);
    }

    is_safe_after_damping
}

fn get_violations(report: &Vec<i8>) -> HashSet<usize> {
    let (violation, _) = (1..report.len())
        .into_iter()
        .map(|i| (i, report[i - 1] - report[i]))
        .fold(
            (RefCell::new(HashSet::new()), None),
            |(violating_idx, last_diff): (RefCell<HashSet<usize>>, Option<i8>),
             (i, current_diff)| {
                if !(1..4i8).contains(&current_diff.abs()) {
                    violating_idx.borrow_mut().insert(i);
                }

                if let Some(last_diff) = last_diff {
                    if last_diff.signum() != current_diff.signum() {
                        violating_idx.borrow_mut().insert(i);
                    }
                }

                (violating_idx, Some(current_diff))
            },
        );

    violation.take()
}

mod tests {
    use crate::day2::{count_safe_reports, count_safe_reports_with_damping};

    #[test]
    fn test_example_data() {
        // 7  6  4  2  1: safe
        //   -1 -2 -2 -1

        // 1  2  7  8  9: unsafe
        //    1  5  3  2

        // 9  7  6  2  1: unsafe
        //   -2 -1 -4 -1

        // 1  3  2  4  5: unsafe
        //    2 -1  2  1

        // 8  6  4  4  1: unsafe
        //   -2 -2  0 -3

        // 1  3  6  7  9: safe
        //    2  3  1  2

        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert!(count_safe_reports(input) == 2);
    }

    #[test]
    fn test_example_data_part2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(count_safe_reports_with_damping(input), 4);
    }

    #[test]
    fn part2_obvious_failures() {
        // 74, 76, 78, 79, 76
        //      2   2   1  -3

        let input = "74 76 78 79 76";
        assert_eq!(count_safe_reports_with_damping(input), 1);
    }
}
