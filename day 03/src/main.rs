use std::str::FromStr;

use regex::Regex;
use lazy_static::lazy_static;

#[derive(PartialEq)]
enum Token {
    Do,
    Dont,
    Mul(i32, i32),
}

lazy_static! {
    static ref MUL_REGEX: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
}

impl FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "do()" {
            return Ok(Self::Do);
        } else if s == "don't()" {
            return Ok(Self::Dont);
        } else if let Some(captures) = MUL_REGEX.captures(s) {
            return match (captures[1].parse::<i32>(), captures[2].parse::<i32>()) {
                (Ok(x), Ok(y)) => {
                    return Ok(Self::Mul(x, y));
                },
                _ => Err("Failed to parse".to_string()),
            };
        }

        return Err(format!("Couldn't parse the input {}", s))
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");

    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let res = mul_regex.captures_iter(&input).fold(0, |acc, caps| acc + parse_caps(&caps[1], &caps[2]));

    println!("Silver star: {}", res);

    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");

    // Create regex pattern that matches all three cases
    let pattern = Regex::new(r"do\(\)|don't\(\)|mul\(\d+,\d+\)").unwrap();

    // Find all matches and join them together
    let res2 = pattern.find_iter(&input)
        .map(|m| m.as_str())
        .map(|s| Token::from_str(s))
        .filter_map(Result::ok)
        .fold((0, true), |(res, active), tok| {
            match tok {
                Token::Do => (res, true),
                Token::Dont => (res, false),
                Token::Mul(x, y) => match active {
                    true => (res + (x * y), true),
                    false => (res, false),
                }
            }
        });

    println!("Gold star: {}", res2.0);
}

fn parse_caps(a: &str, b: &str) -> i32 {
    let x = a.parse::<i32>().unwrap();
    let y = b.parse::<i32>().unwrap();
    x * y
}
