use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/*
This whole solution only works if each of the "paths":
- encounters only 1 end node
- enters a loop
- the distance from the start node to the end node is the same as from the end node to the start of the loop
*/

type Mapping = HashMap<String, (String, String)>;

fn main() {
    let (moves, maps) = load_input("input");
    println!("Solution for part 1: {}", part_1(&moves, &maps));
    println!("Solution for part 2: {}", part_2(&moves, &maps));
}

fn part_1(moves: &[char], maps: &Mapping) -> usize {
    let mut counter = 0;
    let mut cur_node = "AAA";
    for m in moves.iter().cycle() {
        if cur_node == "ZZZ" {
            break;
        }
        counter += 1;
        cur_node = get_next_node(m, maps, cur_node);
    }
    counter
}

fn part_2(moves: &[char], maps: &Mapping) -> usize {
    let loops: Vec<usize> = maps
        .keys()
        .filter_map(|k| {
            if k.ends_with('A') {
                Some(k.as_str())
            } else {
                None
            }
        })
        .map(|x| get_loop_len(moves, maps, x))
        .collect();
    let mut cur_lcm = 1;
    for l in loops {
        cur_lcm = lcm(cur_lcm, l);
    }
    cur_lcm
}

// Least common multiple
fn lcm(x: usize, y: usize) -> usize {
    let big = x.max(y);
    let small = x.min(y);
    (x * y) / gcd(big, small)
}

// Greatest common divisor
fn gcd(mut big: usize, mut small: usize) -> usize {
    while small != 0 {
        let tmp = small;
        small = big % small;
        big = tmp;
    }
    big
}

// Simply follows the path until it finds a end node. After it does it keeps going down the path but starts counting the moves.
// Once it gets to the end node again it stops and returns the count.
fn get_loop_len(moves: &[char], maps: &Mapping, start: &str) -> usize {
    let mut cur_node = start;
    let mut loop_len: Option<usize> = None;

    for m in moves.iter().cycle() {
        if cur_node.ends_with('Z') {
            match loop_len {
                Some(_) => break,
                None => {
                    loop_len = Some(0);
                }
            }
        }
        if let Some(x) = loop_len {
            loop_len = Some(x + 1);
        }
        cur_node = get_next_node(m, maps, cur_node);
    }
    loop_len.unwrap()
}

fn get_next_node<'a>(m: &char, maps: &'a Mapping, cur_node: &str) -> &'a str {
    let targets = maps.get(cur_node).unwrap();
    match *m {
        'L' => &targets.0,
        'R' => &targets.1,
        _ => unreachable!(),
    }
}

fn load_input(name: &str) -> (Vec<char>, Mapping) {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
    let mut mapping: Mapping = HashMap::new();
    let mut lines = reader.lines().map(|l| l.unwrap());
    let moves: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();

    let reg = Regex::new(r"\w\w\w").unwrap();
    for l in lines {
        let matches: Vec<&str> = reg.find_iter(&l).map(|x| x.as_str()).collect();
        mapping.insert(
            matches[0].to_string(),
            (matches[1].to_string(), matches[2].to_string()),
        );
    }
    (moves, mapping)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let (moves, maps) = load_input("example");
        assert_eq!(crate::part_1(&moves, &maps), 2);
    }

    #[test]
    fn part_2() {
        let (moves, maps) = load_input("example2");
        assert_eq!(crate::part_2(&moves, &maps), 6);
    }
}
