use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Pair = ((usize, usize), (usize, usize));

fn main() {
    let (galaxies, empty_rows, empty_cols) = load_input("input");
    let solutions = solve(&galaxies, &empty_rows, &empty_cols);
    println!("Solution for part 1: {}", solutions.0);
    println!("Solution for part 2: {}", solutions.1);
}

fn solve(
    galaxies: &[(usize, usize)],
    empty_rows: &HashSet<usize>,
    empty_cols: &HashSet<usize>,
) -> (usize, usize) {
    let mut result_1 = 0;
    let mut result_2 = 0;
    for pair in create_pairs(galaxies) {
        let distance = get_distance(pair, empty_rows, empty_cols);
        result_1 += distance.0;
        result_2 += distance.1;
    }
    (result_1, result_2)
}

fn get_distance(
    pair: Pair,
    empty_rows: &HashSet<usize>,
    empty_cols: &HashSet<usize>,
) -> (usize, usize) {
    // This is just to make the rest more readable
    let first = pair.0;
    let second = pair.1;
    let min_x = first.0.min(second.0);
    let max_x = first.0.max(second.0);
    let min_y = first.1.min(second.1);
    let max_y = first.1.max(second.1);

    let mut distance_normal = 0;
    let mut distance_expand = 0;
    /*
    The shortest distance between 2 points on a grid is the Manhattan distance but because of the expanding
    each "step" can be much bigger. To deal with that you simply walk the path and if you get to a row/column
    that is expanded you add the expand value instead of just 1.
    Walking the path is just going from min(x) + 1 to max(x) and then from min(y) + 1 to max(y). The + 1 is to
    skip the tile you start on.
    */
    for x in (min_x + 1)..=max_x {
        if empty_cols.contains(&x) {
            distance_expand += 1;
        } else {
            distance_normal += 1;
        }
    }
    for y in (min_y + 1)..=max_y {
        if empty_rows.contains(&y) {
            distance_expand += 1;
        } else {
            distance_normal += 1;
        }
    }
    (
        distance_normal + distance_expand * 2,
        distance_normal + distance_expand * 1_000_000,
    )
}

/// Creates unique pairs for all galaxies given
fn create_pairs(galaxies: &[(usize, usize)]) -> HashSet<Pair> {
    let mut pairs: HashSet<Pair> = HashSet::new();
    for first in galaxies {
        for second in galaxies {
            if first == second {
                continue;
            }
            // Sorted slice is used to turn (B, A) into (A, B)
            let mut points = [first, second];
            points.sort();
            pairs.insert((*points[0], *points[1]));
        }
    }
    pairs
}

fn load_input(name: &str) -> (Vec<(usize, usize)>, HashSet<usize>, HashSet<usize>) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut lines = reader.lines().peekable();

    let mut empty_rows = HashSet::new();
    // Peekable iterator is used to create a vec of bools for every column
    let mut empty_column_check = vec![false; lines.peek().unwrap().as_ref().unwrap().len()];
    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for (y, line) in lines.map(|l| l.unwrap()).enumerate() {
        let mut galaxy_in_row_found = false;
        for (x, c) in line.chars().enumerate() {
            if let '#' = c {
                galaxy_in_row_found = true;
                empty_column_check[x] = true;
                galaxies.push((x, y));
            }
        }
        if !galaxy_in_row_found {
            empty_rows.insert(y);
        }
    }
    let empty_columns: HashSet<usize> = empty_column_check
        .iter()
        .enumerate()
        .filter_map(|(x, galaxy)| if !*galaxy { Some(x) } else { None })
        .collect();
    (galaxies, empty_rows, empty_columns)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let (galaxies, empty_rows, empty_cols) = load_input("example");
        assert_eq!(solve(&galaxies, &empty_rows, &empty_cols).0, 374);
    }

    #[test]
    fn part_2() {
        let (galaxies, empty_rows, empty_cols) = load_input("example");
        assert_eq!(solve(&galaxies, &empty_rows, &empty_cols).1, 82000210);
    }
}
