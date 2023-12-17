use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(map: &[Vec<isize>]) -> isize {
    dijkstra(map, 0, 3)
}

fn part_2(map: &[Vec<isize>]) -> isize {
    dijkstra(map, 4, 10)
}

fn dijkstra(map: &[Vec<isize>], min_steps: usize, max_steps: usize) -> isize {
    let target_x = map[0].len() - 2;
    let target_y = map.len() - 2;
    let mut shortest: HashMap<(usize, usize, Direction), isize> = HashMap::new();

    let mut heap: BinaryHeap<MinHeapEle> = BinaryHeap::new();
    heap.push(MinHeapEle::new(1, 1, 0, Direction::Right));
    heap.push(MinHeapEle::new(1, 1, 0, Direction::Down));

    while let Some(cur) = heap.pop() {
        if cur.x == target_x && cur.y == target_y {
            return cur.dist;
        }
        match shortest.get(&(cur.x, cur.y, cur.dire)) {
            Some(other) => {
                if other <= &cur.dist {
                    continue;
                }
            }
            None => {
                shortest.insert((cur.x, cur.y, cur.dire), cur.dist);
            }
        };
        let mut new_heat_loss = 0;

        // Go in the current direction for max_steps
        // Keep track of the heat loss
        // If 0 is found stop because it's the boundary
        // If min_steps was made make that a valid target for both 90 degree turns
        match cur.dire {
            Direction::Up => {
                for new_y in ((cur.y.saturating_sub(max_steps))..=(cur.y.saturating_sub(1))).rev() {
                    match map[new_y][cur.x] {
                        0 => break,
                        value => {
                            new_heat_loss += value;
                        }
                    }
                    if cur.y - new_y >= min_steps {
                        heap.push(MinHeapEle::new(
                            cur.x,
                            new_y,
                            cur.dist + new_heat_loss,
                            Direction::Left,
                        ));
                        heap.push(MinHeapEle::new(
                            cur.x,
                            new_y,
                            cur.dist + new_heat_loss,
                            Direction::Right,
                        ));
                    }
                }
            }
            #[allow(clippy::needless_range_loop)]
            Direction::Down => {
                for new_y in (cur.y + 1)..=(cur.y + max_steps) {
                    match map[new_y][cur.x] {
                        0 => break,
                        value => {
                            new_heat_loss += value;
                        }
                    }
                    if new_y - cur.y >= min_steps {
                        heap.push(MinHeapEle::new(
                            cur.x,
                            new_y,
                            cur.dist + new_heat_loss,
                            Direction::Left,
                        ));
                        heap.push(MinHeapEle::new(
                            cur.x,
                            new_y,
                            cur.dist + new_heat_loss,
                            Direction::Right,
                        ));
                    }
                }
            }
            Direction::Left => {
                for new_x in ((cur.x.saturating_sub(max_steps))..=(cur.x.saturating_sub(1))).rev() {
                    match map[cur.y][new_x] {
                        0 => break,
                        value => {
                            new_heat_loss += value;
                        }
                    }
                    if cur.x - new_x >= min_steps {
                        heap.push(MinHeapEle::new(
                            new_x,
                            cur.y,
                            cur.dist + new_heat_loss,
                            Direction::Up,
                        ));
                        heap.push(MinHeapEle::new(
                            new_x,
                            cur.y,
                            cur.dist + new_heat_loss,
                            Direction::Down,
                        ));
                    }
                }
            }
            Direction::Right => {
                for new_x in (cur.x + 1)..=(cur.x + max_steps) {
                    match map[cur.y][new_x] {
                        0 => break,
                        value => {
                            new_heat_loss += value;
                        }
                    }
                    if new_x - cur.x >= min_steps {
                        heap.push(MinHeapEle::new(
                            new_x,
                            cur.y,
                            cur.dist + new_heat_loss,
                            Direction::Up,
                        ));
                        heap.push(MinHeapEle::new(
                            new_x,
                            cur.y,
                            cur.dist + new_heat_loss,
                            Direction::Down,
                        ));
                    }
                }
            }
        }
    }
    unreachable!();
}

fn load_input(name: &str) -> Vec<Vec<isize>> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);

    let mut input = vec![vec![]];

    for line in reader.lines().map(|l| l.unwrap()) {
        let mut row = vec![0];
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as isize);
        }
        row.push(0);
        input.push(row);
    }
    input[0] = vec![0; input[1].len()];
    input.push(vec![0; input[1].len()]);
    input
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MinHeapEle {
    dist: isize,
    x: usize,
    y: usize,
    dire: Direction,
}

impl MinHeapEle {
    fn new(x: usize, y: usize, dist: isize, dire: Direction) -> Self {
        Self { dist, x, y, dire }
    }
}

impl Ord for MinHeapEle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.dist.cmp(&other.dist) {
            std::cmp::Ordering::Less => Ordering::Greater,
            std::cmp::Ordering::Equal => Ordering::Equal,
            std::cmp::Ordering::Greater => Ordering::Less,
        }
    }
}

impl PartialOrd for MinHeapEle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = load_input("example");
        assert_eq!(crate::part_1(&input), 102);
    }

    #[test]
    fn part_2() {
        let input = load_input("example");
        assert_eq!(crate::part_2(&input), 94);
    }
}
