use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    let results = solve(&input);
    println!("Solution for part 1: {}", results.0);
    println!("Solution for part 2: {}", results.1);
}

fn solve(input: &[Vec<isize>]) -> (isize, isize) {
    let mut result_part_1 = 0;
    let mut result_part_2 = 0;

    for history in input {
        let simulation: Vec<Vec<isize>> = calculate_differences(history);
        let mut prev_last_value = 0;
        let mut prev_first_value = 0;
        for row in simulation.iter().rev() {
            prev_last_value += *row.last().unwrap();
            prev_first_value = row[0] - prev_first_value;
        }
        result_part_1 += prev_last_value;
        result_part_2 += prev_first_value;
    }
    (result_part_1, result_part_2)
}

fn calculate_differences(history: &[isize]) -> Vec<Vec<isize>> {
    let mut simulation: Vec<Vec<isize>> = vec![history.to_vec()];
    while !simulation.last().unwrap().iter().all(|x| *x == 0) {
        let mut new_row: Vec<isize> = Vec::new();
        for window in simulation.last().unwrap().windows(2) {
            new_row.push(window[1] - window[0])
        }
        simulation.push(new_row);
    }
    simulation
}

fn load_input(name: &str) -> Vec<Vec<isize>> {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| {
            l.unwrap()
                .split(' ')
                .map(|n| n.parse::<isize>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = load_input("example");
        assert_eq!(crate::solve(&input).0, 114);
    }

    #[test]
    fn part_2() {
        let input = load_input("example");
        assert_eq!(crate::solve(&input).1, 2);
    }
}
