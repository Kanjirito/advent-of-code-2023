use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(input: &[Vec<char>]) -> usize {
    let mut load = 0;
    let max_load = input.len();
    let mut cur_max_load = vec![max_load; input[0].len()];

    for (x, row) in input.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            match c {
                '.' => (),
                '#' => {
                    cur_max_load[y] = max_load - x - 1;
                }
                'O' => {
                    load += cur_max_load[y];
                    cur_max_load[y] -= 1;
                }
                _ => unreachable!(),
            }
        }
    }

    load
}

fn measure_north_load(map: &[Vec<char>]) -> usize {
    let mut load = 0;
    let max_load = map.len();
    for (y, row) in map.iter().enumerate() {
        for c in row {
            if *c == 'O' {
                load += max_load - y;
            }
        }
    }
    load
}

fn part_2(input: &[Vec<char>]) -> usize {
    let mut states: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let mut cur_map = input.to_vec();
    states.insert(cur_map.clone(), 0);

    // Do cycles until you find the loop, then only do before_loop + after_loop cycles
    for x in 1..1000000000 {
        cur_map = do_cycle(cur_map);
        match states.get(&cur_map) {
            Some(loop_index) => {
                let loop_len = x - loop_index;
                let mut cur_map = input.to_vec();
                let needed_loops = ((1000000000 - loop_index) % loop_len) + loop_index;
                for _ in 0..needed_loops {
                    cur_map = do_cycle(cur_map);
                }
                return measure_north_load(&cur_map);
            }
            None => states.insert(cur_map.clone(), x),
        };
    }
    unreachable!()
}

fn do_cycle(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // As with day 13, flipping the map lets you reuse code so you only need to tilt the map horizontally
    // The flipping is definitely not efficient but it's fast enough

    // North
    map = flip_map(&tilt_map(&flip_map(&map), false));
    // West
    map = tilt_map(&map, false);
    // South
    map = flip_map(&tilt_map(&flip_map(&map), true));
    // East
    map = tilt_map(&map, true);
    map
}

// Go from the edge you tilt towards and keep track of the last spot that rocks can roll to in a row
fn tilt_map(map: &[Vec<char>], reverse: bool) -> Vec<Vec<char>> {
    let mut new_map = vec![vec!['.'; map[0].len()]; map.len()];
    let mut x_iter: Vec<usize> = (0..map[0].len()).collect();
    if reverse {
        x_iter.reverse();
    }
    for y in 0..map.len() {
        let mut last_free = 0;
        if reverse {
            last_free = map[0].len() - 1;
        }
        for x in &x_iter {
            match map[y][*x] {
                '.' => (),
                '#' => {
                    // 0 check because of usize subtraction
                    if reverse && x > &0 {
                        last_free = x - 1;
                    } else {
                        last_free = x + 1;
                    }
                    new_map[y][*x] = '#';
                }
                'O' => {
                    new_map[y][last_free] = 'O';
                    // 0 check because of usize subtraction
                    if reverse && last_free > 0 {
                        last_free -= 1;
                    } else {
                        last_free += 1;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    new_map
}

#[allow(clippy::needless_range_loop)]
fn flip_map(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut flipped_map: Vec<Vec<char>> = vec![vec!['.'; map.len()]; map[0].len()];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            flipped_map[x][y] = map[y][x];
        }
    }
    flipped_map
}

fn load_input(name: &str) -> Vec<Vec<char>> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut input = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        input.push(line.chars().collect());
    }
    input
}

#[allow(dead_code)]
fn print_map(map: &[Vec<char>]) {
    for row in map {
        for tile in row {
            print!("{}", tile)
        }
        println!();
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = load_input("example");
        assert_eq!(crate::part_1(&input), 136);
    }

    #[test]
    fn part_2() {
        let input = load_input("example");
        assert_eq!(crate::part_2(&input), 64);
    }
}
