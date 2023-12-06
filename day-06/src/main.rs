use std::fmt::Write;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::write;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
    println!("Solution for part 2 using math: {}", part_2_math(&input));
}

fn part_1(pairs: &[(u64, u64)]) -> usize {
    let mut result = 1;

    for (time, distance) in pairs {
        let mut valid = 0;
        for t in 1..*time {
            if ((time - t) * t) > *distance {
                valid += 1;
            }
        }
        result *= valid;
    }

    result
}

fn part_2(pairs: &[(u64, u64)]) -> u64 {
    let (time, distance) = connect_numbers(pairs);

    let mut result = 0;
    for t in 1..time {
        if ((time - t) * t) > distance {
            result += 1;
        }
    }
    result
}

#[allow(clippy::neg_multiply)]
fn part_2_math(pairs: &[(u64, u64)]) -> u64 {
    // To check if a hold_time will beat the distance with a given time you can use this:
    // (time - hold_time) * hold_time > distance
    // time = total time
    // hold_time = time you hold down the button
    // distance = distance to beat
    //
    // This you can turn into a quadratic equation
    // x = hold_time
    // b = time
    // c = distance
    // (b - x) * x = c
    // (b * x) - (x * x) = c
    // bx - x^2 = c
    // bx - x^2 - c = 0
    // -x^2 + bx - c = 0
    // (-1 * hold_time^2) + (time * hold_time) + -distance = 0
    //
    // The time and distance are known meaning you can calculate the hold_time by finding the root of quadratic equation
    // This will give you the smallest and biggest hold_time that beats the record
    // x = (-b +/- √(b^2 - 4ac)) / 2a
    let (time, distance) = connect_numbers(pairs);
    let time = time as i64;
    let distance = distance as i64;
    // Since c (distance) and a (-1) are negative you can just skip the negatives
    let delta = time.pow(2) - (4 * distance);
    let square_root = (delta as f64).sqrt();

    let x_1 = (-time as f64 + square_root) / -2.0;
    let x_2 = (-time as f64 - square_root) / -2.0;
    // x_1 get ceil because we need the next full number that beats the record
    // x_2 gets floor because we need the last full number that beats the record
    let diff = x_2.floor() - x_1.ceil();
    diff as u64 + 1
}

fn connect_numbers(pairs: &[(u64, u64)]) -> (u64, u64) {
    let time = pairs
        .iter()
        .fold(String::new(), |mut acc, p| {
            let _ = write!(acc, "{}", p.0);
            acc
        })
        .parse::<u64>()
        .unwrap();
    let distance = pairs
        .iter()
        .fold(String::new(), |mut acc, p| {
            let _ = write!(acc, "{}", p.1);
            acc
        })
        .parse::<u64>()
        .unwrap();
    (time, distance)
}

fn load_input(name: &str) -> Vec<(u64, u64)> {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());

    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    times.into_iter().zip(distances).collect()
}

#[test]
fn example() {
    let input = load_input("example");
    assert_eq!(part_1(&input), 288);
    assert_eq!(part_2(&input), 71503);
    assert_eq!(part_2_math(&input), 71503);
}
