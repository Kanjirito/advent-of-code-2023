use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Brick = ((usize, usize, usize), (usize, usize, usize));

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input.0, input.1, input.2));
}

fn part_1(bricks: &[Brick], max_x: usize, max_y: usize) -> usize {
    let (supported_by, supporting_map) = simulate_falling(bricks, max_x, max_y);

    let mut counter = 0;

    'main: for id in 1..=bricks.len() {
        if id == 0 {
            continue;
        }
        if let Some(supported_bricks) = supporting_map.get(&id) {
            for t in supported_bricks {
                match supported_by.get(t) {
                    Some(x) => {
                        if x.len() <= 1 {
                            continue 'main;
                        }
                    }
                    None => continue 'main,
                }
            }
        }
        counter += 1;
    }
    counter
}

#[allow(clippy::needless_range_loop)]
fn simulate_falling(
    bricks: &[Brick],
    max_x: usize,
    max_y: usize,
) -> (
    HashMap<usize, HashSet<usize>>,
    HashMap<usize, HashSet<usize>>,
) {
    let mut cur_top: Vec<Vec<(usize, usize)>> = vec![vec![(0, 0); max_y]; max_x];
    let mut supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut supporting: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (id, (start, end)) in bricks.iter().enumerate() {
        let mut highest_point = 0;
        for new_x in start.0..=end.0 {
            for new_y in start.1..=end.1 {
                highest_point = highest_point.max(cur_top[new_x][new_y].0)
            }
        }

        let new_height = highest_point + 1 + (end.2 - start.2);
        for new_x in start.0..=end.0 {
            for new_y in start.1..=end.1 {
                if cur_top[new_x][new_y].0 == highest_point {
                    supported_by
                        .entry(id + 1)
                        .or_default()
                        .insert(cur_top[new_x][new_y].1);
                    supporting
                        .entry(cur_top[new_x][new_y].1)
                        .or_default()
                        .insert(id + 1);
                }
                cur_top[new_x][new_y] = (new_height, id + 1);
            }
        }
    }
    (supported_by, supporting)
}

fn load_input(name: &str) -> (Vec<Brick>, usize, usize) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut bricks = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in reader.lines().map(|l| l.unwrap()) {
        let mut split = line.split('~');
        let first_cords: Vec<usize> = split
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let second_cords: Vec<usize> = split
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        max_x = max_x.max(second_cords[0]);
        max_y = max_y.max(second_cords[1]);
        bricks.push((
            (first_cords[0], first_cords[1], first_cords[2]),
            (second_cords[0], second_cords[1], second_cords[2]),
        ))
    }
    bricks.sort_unstable_by_key(|x| x.0 .2);
    (bricks, max_x + 1, max_y + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = load_input("example");
        assert_eq!(crate::part_1(&input.0, input.1, input.2), 5);
    }

    #[test]
    fn part_2() {
        let input = load_input("example");
    }
}
