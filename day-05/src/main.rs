use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec;

type Map = Vec<(u64, u64, u64)>;

fn main() {
    let (seeds, maps) = load_input("input");
    println!("Solution for part 1: {}", part_1(&seeds, &maps));
    println!("Solution for part 2: {}", part_2(&seeds, &maps));
}

fn part_1(seeds: &[u64], maps: &[Map]) -> u64 {
    let mut lowest = u64::MAX;

    for seed in seeds {
        let mut cur_id = *seed;
        for map in maps {
            for (source, target, range) in map {
                if cur_id >= *source && cur_id < (source + range) {
                    cur_id = target + (cur_id - source);
                    break;
                }
            }
        }
        lowest = lowest.min(cur_id);
    }
    lowest
}

fn part_2(seeds: &[u64], maps: &[Map]) -> u64 {
    let mut lowest = u64::MAX;

    for s in seeds.chunks_exact(2) {
        for seed in (s[0])..(s[0] + s[1]) {
            let mut cur_id = seed;
            for map in maps {
                for (source, target, range) in map {
                    if cur_id >= *source && cur_id < (source + range) {
                        cur_id = target + (cur_id - source);
                        break;
                    }
                }
            }
            lowest = lowest.min(cur_id);
        }
    }
    lowest
}

fn load_input(name: &str) -> (Vec<u64>, Vec<Map>) {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);

    let mut maps: Vec<Vec<(u64, u64, u64)>> = vec![Vec::new(); 7];
    let mut lines = reader.lines().map(|l| l.unwrap());

    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    // Skip empty line to get to maps
    lines.next();

    for map in maps.iter_mut() {
        // Skip over the header line
        lines.next();
        for cur_line in lines.by_ref() {
            // Each map block has an empty row after it
            if cur_line.is_empty() {
                break;
            }
            let mut split = cur_line.split(' ').map(|n| n.parse::<u64>().unwrap());
            let target = split.next().unwrap();
            let source = split.next().unwrap();
            let range = split.next().unwrap();
            map.push((source, target, range));
        }
        map.sort_by_key(|k| k.0)
    }

    (seeds, maps)
}

#[test]
fn example() {
    let (seeds, maps) = load_input("example");
    assert_eq!(part_1(&seeds, &maps), 35);
    assert_eq!(part_2(&seeds, &maps), 46);
}
