use std::fs;

pub(crate) fn day1(input_filename: &str) {
    let contents = fs::read_to_string(input_filename).expect("could not read file");

    let Lists(mut left, mut right) = contents
        .lines()
        .map(|line| line.split("   ").collect::<Vec<&str>>())
        .fold(Lists::new(), |lists, split_line| {
            let Lists(mut left, mut right) = lists;
            let numbers = split_line
                .iter()
                .map(|num_txt| num_txt.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            left.push(*numbers.first().unwrap());
            right.push(*numbers.last().unwrap());
            Lists(left, right)
        });

    left.sort();
    right.sort();

    let sum_of_differences: u32 = left
        .iter()
        .zip(right)
        .map(|(lnum, rnum)| lnum.abs_diff(rnum))
        .sum();

    println!("sum of differences: {}", sum_of_differences);
}

struct Lists<T>(pub Vec<T>, pub Vec<T>);
impl<T> Lists<T> {
    fn new() -> Self {
        Self(vec![], vec![])
    }
}
