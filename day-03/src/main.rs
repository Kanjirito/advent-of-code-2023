use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let (parts, symbols) = load_input("input");
    println!("Solution for part 1: {}", part_1(&parts, &symbols));
    println!("Solution for part 2: {}", part_2(&parts, &symbols));
}

// Simply goes over every Part and checks it's every neighbour. If one of the neighbours
// is a symbol add the part number and stop looking.
fn part_1(parts: &[Part], symbols: &HashMap<(usize, usize), bool>) -> u64 {
    let mut sum: u64 = 0;
    for part in parts {
        'searh_loop: for new_x in part.start.saturating_sub(1)..=(part.end + 1) {
            for new_y in part.row.saturating_sub(1)..=(part.row + 1) {
                if symbols.contains_key(&(new_y, new_x)) {
                    sum += part.number;
                    break 'searh_loop;
                }
            }
        }
    }
    sum
}

// Similar to part 1 but instead of stopping after finding a neighbour symbol it checks if it's a gear.
// If it is a gear it adds the part number to a HashMap where the gear coordinates are the key.
// Then just look through all of those gears and check which are valid.
fn part_2(parts: &[Part], symbols: &HashMap<(usize, usize), bool>) -> u64 {
    let mut gears: HashMap<(usize, usize), Vec<u64>> = HashMap::new();
    let mut sum = 0;
    for part in parts {
        for new_x in part.start.saturating_sub(1)..=(part.end + 1) {
            for new_y in part.row.saturating_sub(1)..=(part.row + 1) {
                match symbols.get(&(new_y, new_x)) {
                    None => {}
                    Some(value) => {
                        if *value {
                            gears.entry((new_y, new_x)).or_default().push(part.number);
                        }
                    }
                }
            }
        }
    }

    for group in gears.into_values() {
        if group.len() == 2 {
            sum += group.into_iter().reduce(|a, b| a * b).unwrap();
        }
    }
    sum
}

fn load_input(name: &str) -> (Vec<Part>, HashMap<(usize, usize), bool>) {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);

    let mut parts: Vec<Part> = Vec::new();
    // (y, x, if_gear)
    let mut symbols: HashMap<(usize, usize), bool> = HashMap::new();

    for (y, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
        let mut cur_num: Vec<char> = Vec::new();

        // Iterate over characters
        for (x, c) in line.chars().enumerate() {
            // When digit is found save it
            if c.is_ascii_digit() {
                cur_num.push(c);
            } else {
                // When it's not a digit and there are saved digits it means the number ended
                if !cur_num.is_empty() {
                    // Add the number to the Vec
                    parts.push(Part {
                        number: cur_num.iter().collect::<String>().parse::<u64>().unwrap(),
                        row: y,
                        start: x - cur_num.len(),
                        // The number ended on the _previous_ char
                        end: x - 1,
                    });
                    // Reset the saved digits
                    cur_num.clear();
                }
                match c {
                    '.' => (),
                    '*' => {
                        symbols.insert((y, x), true);
                    }
                    _ => {
                        symbols.insert((y, x), true);
                    }
                };
            }
        }
        // This has to be done at the end of the line because a number might be at the edge
        if !cur_num.is_empty() {
            parts.push(Part {
                number: cur_num.iter().collect::<String>().parse::<u64>().unwrap(),
                row: y,
                start: line.len() - cur_num.len(),
                end: line.len() - 1,
            });
        }
    }
    (parts, symbols)
}

#[derive(Debug)]
struct Part {
    number: u64,
    row: usize,
    start: usize,
    end: usize,
}

#[test]
fn example() {
    let (parts, symbols) = load_input("example");
    assert_eq!(part_1(&parts, &symbols), 4361);
    assert_eq!(part_2(&parts, &symbols), 467835);
}

#[test]
fn example_2() {
    // Example taken from a reddit post
    // https://www.reddit.com/r/adventofcode/comments/189q9wv/2023_day_3_another_sample_grid_to_use/
    let (parts, symbols) = load_input("example2");
    assert_eq!(part_1(&parts, &symbols), 925);
    assert_eq!(part_2(&parts, &symbols), 6756);
}
