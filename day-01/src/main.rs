use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Regular regex patter for finding digits
static REG: &str = r"(?:\d|zero|one|two|three|four|five|six|seven|eight|nine)";
/// Regex pattern for finding digits but the names are reversed
static REV_REG: &str = r"(?:\d|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|orez)";

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(input: &[String]) -> u64 {
    let mut codes: Vec<u64> = Vec::new();
    for line in input.iter() {
        let mut numbers: Vec<char> = Vec::new();
        for c in line.chars() {
            if c.is_ascii_digit() {
                numbers.push(c);
            }
        }
        let code: u64 = format!(
            "{}{}",
            numbers.first().unwrap(),
            numbers.iter().last().unwrap()
        )
        .parse()
        .unwrap();
        codes.push(code)
    }
    codes.iter().sum()
}

fn part_2(input: &[String]) -> u64 {
    let mut codes: Vec<u64> = Vec::new();
    let normal_reg = Regex::new(REG).unwrap();
    let reverse_reg = Regex::new(REV_REG).unwrap();
    for line in input {
        // Find the first digit
        let first_digit = match_number(normal_reg.find(line).unwrap().as_str());
        // Find the last digit by reversing the string and using the reverse regex
        let second_digit = match_number(
            &reverse_reg
                .find(&line.chars().rev().collect::<String>())
                .unwrap()
                .as_str()
                .chars()
                .rev()
                .collect::<String>(),
        );
        let code: u64 = format!("{}{}", first_digit, second_digit).parse().unwrap();
        codes.push(code)
    }
    codes.iter().sum()
}

fn match_number(number: &str) -> u64 {
    match number {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => number.parse().unwrap(),
    }
}

fn load_input(name: &str) -> Vec<String> {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

#[test]
fn test_part_1() {
    let input = load_input("example");
    assert_eq!(part_1(&input), 142);
}

#[test]
fn test_part_2() {
    let input = load_input("example2");
    assert_eq!(part_2(&input), 281);
}
