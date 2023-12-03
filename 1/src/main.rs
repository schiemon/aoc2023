use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_digit(s: &str, digit_literals: Vec<String>) -> Option<u8> {
    let mut s = s;
    while s.len() > 0 {
        let digit = match s.chars().next().map(|c| c.to_digit(10)) {
            Some(Some(digit)) => Some(digit as u8),
            _ => None,
        };

        if digit.is_some() {
            return digit;
        } else {
            let digit = digit_literals
                .iter()
                .enumerate()
                .find(|(_, digit_literal)| s.starts_with(*digit_literal))
                .map(|(digit, _)| u8::try_from(digit + 1).unwrap());

            if digit.is_some() {
                return digit;
            }
        }

        s = &s[1..];
    }

    return None;
}

static DIGIT_LITERALS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(input_filename) = args.get(1) {
        let input_file_reader = BufReader::new(File::open(input_filename)?);
        let mut answer: u32 = 0;

        for (line_index, line) in input_file_reader
            .lines()
            .map(|line| line.unwrap())
            .enumerate()
        {
            let digit_literals: Vec<String> =
                DIGIT_LITERALS.iter().map(|s| s.to_string()).collect();

            let digit_literals_rev: Vec<String> = DIGIT_LITERALS
                .iter()
                .map(|digit_literal| digit_literal.chars().rev().collect::<String>())
                .collect();

            let first_digit = find_digit(&line, digit_literals);
            let last_digit =
                find_digit(&line.chars().rev().collect::<String>(), digit_literals_rev);

            if let [Some(first_digit), Some(last_digit)] = [first_digit, last_digit] {
                answer += (10 * first_digit + last_digit) as u32;
            } else {
                eprintln!("Could not find two digits in line {}.", line_index + 1);
            }
        }

        println!("{}", answer);
    } else {
        eprintln!("Usage: cargo run <input_filename>");
    }

    return Ok(());
}
