use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let (map, animal) = load_input("input");
    let (p_1, p_2) = solve(map, animal);
    println!("Solution for part 1: {}", p_1);
    println!("Solution for part 2: {}", p_2);
}

fn solve(mut map: Vec<Vec<Tile>>, animal: (usize, usize)) -> (usize, usize) {
    replace_animal(&mut map, animal);
    let loop_map = find_loop(&map, animal);
    (loop_map.len() / 2, count_inside_loop(&map, loop_map))
}

fn count_inside_loop(map: &[Vec<Tile>], loop_tiles: HashSet<(usize, usize)>) -> usize {
    let mut counter = 0;
    for (y, row) in map.iter().enumerate() {
        let mut inside = false;

        // Not a corner so will always work
        let mut last_corner = Tile::Vertical;
        for (x, tile) in row.iter().enumerate() {
            if loop_tiles.contains(&(x, y)) {
                /*
                Going over a vertical piece means you either went into or out of the loop.
                Horizontal piece don't matter because we are moving horizontally.
                Corner pipes only matter if we cross two opposite pipes because they are basically just a broken vertical pipe:
                │          │         │       │
                └──┐  -->  └─┐  -->  └┐  --> │
                   │         │        │      │
                This is the same for the ┌┘ bend. No other corners matter so you only need to keep track of the east facing corners
                and flip the inside bool when going over the opposite corner. 
                */
                match tile {
                    Tile::Vertical => inside = !inside,
                    Tile::Horizontal => (),
                    Tile::NorthEast => last_corner = Tile::NorthEast,
                    Tile::NorthWest => {
                        if last_corner == Tile::SouthEast {
                            inside = !inside;
                        }
                    }
                    Tile::SouthWest => {
                        if last_corner == Tile::NorthEast {
                            inside = !inside;
                        }
                    }
                    Tile::SouthEast => last_corner = Tile::SouthEast,
                    Tile::Ground | Tile::Animal => unreachable!(),
                }
            } else if inside {
                counter += 1;
            }
        }
    }
    counter
}

// Simply follows the pipes and avoids going back. Once it arrives at the start the loop is found
fn find_loop(map: &[Vec<Tile>], start: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut loop_points: Vec<(usize, usize)> = Vec::new();

    let mut cur_point = start;
    // (0, 0) can't be visited so it's fine as a placeholder for the start
    let mut last_visited = (0, 0);
    loop {
        let cur_x = cur_point.0;
        let cur_y = cur_point.1;
        let (first, second) = map[cur_y][cur_x].get_connected(cur_x, cur_y);

        // Pick the one that isn't going backwards
        if first != last_visited {
            loop_points.push(first);
        } else {
            loop_points.push(second)
        }
        last_visited = (cur_x, cur_y);
        cur_point = *loop_points.last().unwrap();
        if cur_point == start {
            break;
        }
    }
    HashSet::from_iter(loop_points)
}

/// Replaces the animal with the pipe piece
fn replace_animal(map: &mut [Vec<Tile>], animal: (usize, usize)) {
    let animal_x = animal.0;
    let animal_y = animal.1;

    let mut n = false;
    let mut s = false;
    let mut w = false;
    let mut e = false;
    // Check the 4 directions and see if they have a pipe that points to the animal
    if matches!(
        map[animal_y - 1][animal_x],
        Tile::Vertical | Tile::SouthWest | Tile::SouthEast
    ) {
        n = true;
    }
    if matches!(
        map[animal_y + 1][animal_x],
        Tile::Vertical | Tile::NorthEast | Tile::NorthWest
    ) {
        s = true;
    }
    if matches!(
        map[animal_y][animal_x - 1],
        Tile::Horizontal | Tile::NorthEast | Tile::SouthEast
    ) {
        w = true;
    }
    if matches!(
        map[animal_y][animal_x + 1],
        Tile::Horizontal | Tile::SouthWest | Tile::NorthWest
    ) {
        e = true;
    }

    // Replace the animal with the pipe that points in the 2 directions that have connections
    map[animal_y][animal_x] = match (n, s, w, e) {
        (true, true, false, false) => Tile::Vertical,
        (true, false, true, false) => Tile::NorthWest,
        (true, false, false, true) => Tile::NorthEast,
        (false, true, true, false) => Tile::SouthWest,
        (false, true, false, true) => Tile::SouthEast,
        (false, false, true, true) => Tile::Horizontal,
        _ => unreachable!(),
    };
}

fn load_input(name: &str) -> (Vec<Vec<Tile>>, (usize, usize)) {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<Tile>> = Vec::new();
    map.push(Vec::new());
    let mut animal_cords = (0, 0);

    for (y, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
        let mut cur_row = vec![Tile::Ground];
        for (x, c) in line.chars().enumerate() {
            let tile: Tile = c.into();
            if tile == Tile::Animal {
                animal_cords = (x + 1, y + 1);
            }
            cur_row.push(tile);
        }
        cur_row.push(Tile::Ground);
        map.push(cur_row);
    }
    // Add a ground border to avoid bound problems
    map[0] = vec![Tile::Ground; map[1].len()];
    map.push(vec![Tile::Ground; map[1].len()]);
    (map, animal_cords)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Animal,
}

impl Tile {
    /// Get coordinates to the 2 pipes that the current tile points to. Panics if not a pipe.
    fn get_connected(self, x: usize, y: usize) -> ((usize, usize), (usize, usize)) {
        match self {
            Tile::Vertical => ((x, y - 1), (x, y + 1)),
            Tile::Horizontal => ((x - 1, y), (x + 1, y)),
            Tile::NorthEast => ((x, y - 1), (x + 1, y)),
            Tile::NorthWest => ((x, y - 1), (x - 1, y)),
            Tile::SouthWest => ((x, y + 1), (x - 1, y)),
            Tile::SouthEast => ((x, y + 1), (x + 1, y)),
            Tile::Ground | Tile::Animal => panic!("No valid connections for type: {:?}", self),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Vertical => "|",
                Tile::Horizontal => "-",
                Tile::NorthEast => "L",
                Tile::NorthWest => "J",
                Tile::SouthWest => "7",
                Tile::SouthEast => "F",
                Tile::Ground => ".",
                Tile::Animal => "S",
            }
        )
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Animal,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let (map, animal) = load_input("example");
        assert_eq!(crate::solve(map, animal).0, 8);
    }

    #[test]
    fn part_2() {
        let (map, animal) = load_input("example2");
        assert_eq!(crate::solve(map, animal).1, 8);
    }

    #[test]
    fn part_2_2() {
        let (map, animal) = load_input("example3");
        assert_eq!(crate::solve(map, animal).1, 10);
    }
}
