use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Map = Vec<Vec<Tile>>;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", solve(&input, true));
    println!("Solution for part 2: {}", solve(&input, false));
}

#[allow(dead_code)]
fn print_map(map: &Map) {
    for row in map {
        for tile in row {
            match tile {
                Tile::Ash => print!("."),
                Tile::Rock => print!("#"),
            }
        }
        println!();
    }
}

fn solve(input: &[Map], strict: bool) -> usize {
    let mut result = 0;
    for map in input.iter() {
        if let Some(n) = check_horizontal(&flip_map(map), strict) {
            result += n;
        } else if let Some(n) = check_horizontal(map, strict) {
            result += n * 100
        } else {
            panic!("No symmetry found");
        }
    }
    result
}

#[allow(clippy::needless_range_loop)]
/// Creates a new Map that's flipped on it's side. Basically changing from map[row][column] to map[column][row].
///
/// This is done so that the same code can be used for the vertical and horizontal check.
fn flip_map(map: &Map) -> Map {
    let mut flipped_map: Map = vec![vec![Tile::Ash; map.len()]; map[0].len()];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            flipped_map[x][y] = map[y][x];
        }
    }
    flipped_map
}

fn check_horizontal(map: &Map, strict: bool) -> Option<usize> {
    'map_loop: for i in 0..(map.len() - 1) {
        // Flag used to determine if a difference was already found
        // Can just be the strict flag since if there is a difference from the start
        // a single difference will cause it to skip to the next Map
        let mut found_difference = strict;
        for (t_1, t_2) in map[i].iter().zip(map[i + 1].iter()) {
            if t_1 != t_2 {
                if found_difference {
                    continue 'map_loop;
                } else {
                    found_difference = true;
                }
            }
        }
        // Found 2 rows/columns that are (almost) equal meaning there could be a mirror between them
        if check_symmetry(map, i, strict) {
            return Some(i + 1);
        }
    }
    None
}

/// Check if 2 Maps are symmetrical if a mirror is between start_i and start_i + 1
fn check_symmetry(map: &Map, start_i: usize, strict: bool) -> bool {
    let mut found_smudge = false;
    let mut x = start_i;
    let mut y = start_i + 1;
    loop {
        if strict && map[x] != map[y] {
            // If no differences allowed just check if the rows/columns are equal
            return false;
        } else {
            for (t_1, t_2) in map[x].iter().zip(map[y].iter()) {
                if t_1 != t_2 {
                    if found_smudge {
                        // Second difference found, mirror point is invalid
                        return false;
                    } else {
                        found_smudge = true;
                    }
                }
            }
        }

        if x == 0 || y == map.len() - 1 {
            break;
        }
        x -= 1;
        y += 1;
    }
    if strict {
        true
    } else {
        found_smudge
    }
}

fn load_input(name: &str) -> Vec<Map> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut input: Vec<Map> = Vec::new();
    let mut rows: Map = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            input.push(rows);
            rows = Vec::new();
            continue;
        };
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(match c {
                '.' => Tile::Ash,
                '#' => Tile::Rock,
                _ => unreachable!(),
            })
        }
        rows.push(row);
    }
    input.push(rows);
    input
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = load_input("example");
        assert_eq!(crate::solve(&input, true), 405);
    }

    #[test]
    fn part_2() {
        let input = load_input("example");
        assert_eq!(crate::solve(&input, false), 400);
    }
}
