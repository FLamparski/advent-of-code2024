use regex::Regex;
use std::fs;

pub(crate) fn day3(input_filename: &str) {
    let contents = fs::read_to_string(input_filename).expect("could not read file");
    let muls = find_muls(&contents);
    let sum: i64 = muls.iter().map(|&(a, b)| a * b).sum();
    println!("sum of muls: {}", sum);

    // part 2 is to write a tokenizer to decide if the next set of multiplication instructions
    // is enabled or not. i'm not doing that today.
}

fn find_muls(s: &str) -> Vec<(i64, i64)> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    regex
        .captures_iter(s)
        .map(|captures| {
            (
                captures[1].parse::<i64>().unwrap(),
                captures[2].parse::<i64>().unwrap(),
            )
        })
        .collect()
}

mod tests {
    use super::find_muls;

    #[test]
    fn test_find_muls() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let output = find_muls(input);
        println!("output: {:?}", output);

        assert_eq!(output.len(), 4);
    }

    #[test]
    fn test_find_muls_2() {
        let input = "[  (:mul(100,354who(544,766)+,]>!['/;mul(949,115)";
        let output = find_muls(input);
        println!("output: {:?}", output);

        assert_eq!(output.len(), 1);
    }
}
