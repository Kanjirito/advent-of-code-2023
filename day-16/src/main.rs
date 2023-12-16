use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(map: &[Vec<Tile>]) -> usize {
    solve(map, 0, 1, Direction::East)
}

fn part_2(map: &[Vec<Tile>]) -> usize {
    let mut highest = 0;
    for y in 1..(map.len() - 1) {
        highest = highest.max(solve(map, 0, y, Direction::East));
        highest = highest.max(solve(map, map.len() - 1, y, Direction::West));
    }

    for x in 1..(map[0].len() - 1) {
        highest = highest.max(solve(map, x, 0, Direction::South));
        highest = highest.max(solve(map, x, map[0].len() - 1, Direction::North));
    }
    highest
}

fn solve(map: &[Vec<Tile>], x: usize, y: usize, dire: Direction) -> usize {
    let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();
    follow_light(map, x, y, dire, &mut visited);
    let unique: HashSet<(usize, usize)> = visited.into_iter().map(|(x, y, _)| (x, y)).collect();
    unique.len()
}

fn follow_light(
    map: &[Vec<Tile>],
    mut x: usize,
    mut y: usize,
    mut dire: Direction,
    visited: &mut HashSet<(usize, usize, Direction)>,
) {
    loop {
        match dire {
            Direction::North => y -= 1,
            Direction::East => x += 1,
            Direction::South => y += 1,
            Direction::West => x -= 1,
        }

        // Prevents loops
        if visited.contains(&(x, y, dire)) {
            break;
        } else {
            visited.insert((x, y, dire));
        }
        match map[y][x] {
            Tile::Slash => {
                dire = match dire {
                    Direction::North => Direction::East,
                    Direction::East => Direction::North,
                    Direction::South => Direction::West,
                    Direction::West => Direction::South,
                };
            }
            Tile::ReverseSlash => {
                dire = match dire {
                    Direction::North => Direction::West,
                    Direction::East => Direction::South,
                    Direction::South => Direction::East,
                    Direction::West => Direction::North,
                };
            }
            // Vertical and horizontal mirrors split the light so just follow the 2 new lights and
            // stop following the current one
            Tile::Vertical => match dire {
                Direction::North | Direction::South => (),
                Direction::East | Direction::West => {
                    follow_light(map, x, y, Direction::North, visited);
                    follow_light(map, x, y, Direction::South, visited);
                    break;
                }
            },
            Tile::Horizontal => match dire {
                Direction::North | Direction::South => {
                    follow_light(map, x, y, Direction::West, visited);
                    follow_light(map, x, y, Direction::East, visited);
                    break;
                }
                Direction::East | Direction::West => (),
            },
            Tile::Empty => (),
            Tile::Border => {
                // Borders are not in the original input so remove them
                visited.remove(&(x, y, dire));
                break;
            }
        }
    }
}

fn load_input(name: &str) -> Vec<Vec<Tile>> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut map = vec![vec![]];

    for line in reader.lines().map(|l| l.unwrap()) {
        let mut row = vec![Tile::Border];

        for c in line.chars() {
            row.push(match c {
                '/' => Tile::Slash,
                '\\' => Tile::ReverseSlash,
                '|' => Tile::Vertical,
                '-' => Tile::Horizontal,
                '.' => Tile::Empty,
                _ => unreachable!(),
            })
        }
        row.push(Tile::Border);
        map.push(row);
    }
    map[0] = vec![Tile::Border; map[1].len()];
    map.push(vec![Tile::Border; map[1].len()]);
    map
}

#[allow(dead_code)]
fn print_map(map: &[Vec<Tile>]) {
    for row in map {
        for tile in row {
            print!(
                "{}",
                match tile {
                    Tile::Slash => '/',
                    Tile::ReverseSlash => '\\',
                    Tile::Vertical => '|',
                    Tile::Horizontal => '-',
                    Tile::Empty => '.',
                    Tile::Border => '#',
                }
            )
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    /// /
    Slash,
    /// \
    ReverseSlash,
    /// |
    Vertical,
    /// -
    Horizontal,
    /// .
    Empty,
    /// #
    Border,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = load_input("example");
        assert_eq!(crate::part_1(&input), 46);
    }

    #[test]
    fn part_2() {
        let input = load_input("example");
        assert_eq!(crate::part_2(&input), 51);
    }
}
