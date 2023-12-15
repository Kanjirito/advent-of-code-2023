use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(input: &[String]) -> usize {
    let mut result = 0;
    for string in input {
        result += hash_string(string);
    }
    result
}

fn part_2(input: &[String]) -> usize {
    let mut boxes: Vec<OrderedMap> = vec![OrderedMap::new(); 256];
    for op in input {
        match op.strip_suffix('-') {
            Some(label) => {
                let hash = hash_string(label);
                boxes[hash].remove(label);
            }
            None => {
                let mut split = op.split('=');
                let label = split.next().unwrap();
                let focal_length = split.next().unwrap().parse::<usize>().unwrap();
                let hash = hash_string(label);
                boxes[hash].insert(label, focal_length);
            }
        }
    }

    let mut result = 0;
    for (box_i, b) in boxes.iter().enumerate() {
        for (focus_length, lens_i) in b.get_order() {
            let mut cur = 1;
            cur *= box_i + 1;
            cur *= lens_i + 1;
            cur *= focus_length;
            result += cur;
        }
    }
    result
}

fn hash_string(input: &str) -> usize {
    let mut hash = 0;
    for c in input.chars() {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

fn load_input(name: &str) -> Vec<String> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let mut reader = BufReader::new(file);
    let mut buff = String::new();
    reader.read_line(&mut buff).unwrap();
    buff.trim_end().split(',').map(|s| s.to_string()).collect()
}

#[derive(Debug, Clone)]
/// A wrapper around HashMap that keeps the index of inserted elements
struct OrderedMap {
    map: HashMap<String, (usize, usize)>,
    index: usize,
}

impl OrderedMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            index: 0,
        }
    }

    fn insert(&mut self, key: &str, value: usize) {
        match self.map.get_mut(key) {
            Some(v) => {
                *v = (value, v.1);
            }
            None => {
                self.map.insert(key.to_string(), (value, self.index));
                self.index += 1;
            }
        }
    }

    fn remove(&mut self, key: &str) {
        self.map.remove(key);
    }

    // Returns (value, index)
    fn get_order(&self) -> Vec<(usize, usize)> {
        let mut values: Vec<(usize, usize)> = self.map.values().copied().collect();
        values.sort_unstable_by_key(|v| v.1);
        values.iter().enumerate().map(|(i, v)| (v.0, i)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = load_input("example");
        assert_eq!(crate::part_1(&input), 1320)
    }

    #[test]
    fn part_2() {
        let input = load_input("example");
        assert_eq!(crate::part_2(&input), 145)
    }
}
