use std::fs;

pub(crate) fn day1(input_filename: &str) {

    // read and parse the input file

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

    // part 1: compute absolute difference between nth lowest number from left and nth lowest
    //         number from right, then sum those differences.

    left.sort();
    right.sort();

    let sum_of_differences: &u32 = &left
        .iter()
        .zip(&right)
        .map(|(&lnum, &rnum)| lnum.abs_diff(rnum))
        .sum();

    println!("sum of differences: {}", sum_of_differences);

    // part 2: compute a "similarity score", defined as
    //         number from left * how many times it appears in right

    let similarity_score: &u32 = &left
        .iter()
        .map(|&lnum| {
            let count_in_right = &right.iter().filter(|&&rnum| rnum == lnum).count();
            lnum * (count_in_right.clone() as u32)
        })
        .sum();

    println!("similarity score: {}", similarity_score);
}

struct Lists<T>(pub Vec<T>, pub Vec<T>);
impl<T> Lists<T> {
    fn new() -> Self {
        Self(vec![], vec![])
    }
}
