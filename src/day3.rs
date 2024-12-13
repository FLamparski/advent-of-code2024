use regex::Regex;
use std::fs;

pub(crate) fn day3(input_filename: &str) {
    let contents = fs::read_to_string(input_filename).expect("could not read file");
    let tokens = find_tokens(&contents);

    let mut mul_enabled = true;
    let mut acc: i64 = 0;

    for tok in tokens {
        match tok {
            Token::Do => mul_enabled = true,
            Token::Dont => mul_enabled = false,
            Token::Mul(a, b) => if mul_enabled {
                acc += a * b;
            }
        }
    }
    println!("sum of muls: {}", acc);
}

fn find_tokens(s: &str) -> Vec<Token> {
    let regex = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    regex
        .captures_iter(s)
        .map(|captures| match &captures[0] {
            "do()" => Token::Do,
            "don't()" => Token::Dont,
            _ => Token::Mul(captures[1].parse::<i64>().unwrap(), captures[2].parse::<i64>().unwrap())
        })
        .collect()
}

#[derive(Debug)]
enum Token {
    Do,
    Dont,
    Mul(i64, i64),
}
