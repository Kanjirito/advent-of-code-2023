use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Input = Vec<(Vec<char>, Vec<usize>)>;
/// (index, group_index, broken_len, overwrite)
type Cache = HashMap<(usize, usize, usize, Option<char>), usize>;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> usize {
    let mut result = 0;
    for (s, g) in input {
        let mut cache = HashMap::new();
        let mut new_s = s.to_vec();
        // A trailing dot doesn't change anything but makes it easy to check the last group
        new_s.push('.');
        result += solve(&new_s, g, 0, 0, 0, None, &mut cache);
    }
    result
}

fn part_2(input: &Input) -> usize {
    let mut result = 0;
    for (s, g) in input {
        let mut cache = HashMap::new();

        // Create the new string
        let mut new_s = Vec::new();
        for _ in 0..5 {
            new_s.extend_from_slice(s);
            new_s.push('?');
        }
        // Remove the trailing ?
        new_s.pop();
        new_s.push('.');
        let new_g = g.repeat(5);
        result += solve(&new_s, &new_g, 0, 0, 0, None, &mut cache);
    }
    result
}

/// Wrapper around the actual function to cache results based on index, group index, current broken chain and cur character
// `overwrite` needs to be used in the key because ? turn into 2 different situations
fn solve(
    symbols: &[char],
    groups: &[usize],
    i: usize,
    gi: usize,
    broken_len: usize,
    overwrite: Option<char>,
    cache: &mut Cache,
) -> usize {
    if let Some(result) = cache.get(&(i, gi, broken_len, overwrite)) {
        *result
    } else {
        let r = solve_wrapped(symbols, groups, i, gi, broken_len, overwrite, cache);
        cache.insert((i, gi, broken_len, overwrite), r);
        r
    }
}

fn solve_wrapped(
    symbols: &[char],
    groups: &[usize],
    i: usize,
    mut gi: usize,
    mut broken_len: usize,
    overwrite: Option<char>,
    cache: &mut Cache,
) -> usize {
    if i == symbols.len() && gi == groups.len() {
        // If you are past the string and the groups it means you found a valid combination
        return 1;
    } else if i == symbols.len() {
        // If string has ended but groups not you can't progress
        return 0;
    } else if broken_len != 0 {
        // There are still "ungrouped" broken springs...
        #[allow(clippy::if_same_then_else)]
        if gi == groups.len() {
            // ...but groups ended so it's invalid
            return 0;
        } else if broken_len > groups[gi] {
            // ...and groups didn't end but the current chain of springs is too long
            return 0;
        }
    }

    // This is used in the case of ?. They don't advance anything but force the char to be # or .
    // Normally just pick the current char
    let cur_symbol = match overwrite {
        Some(s) => s,
        None => symbols[i],
    };

    if cur_symbol != '?' {
        // Either # or .
        if cur_symbol == '#' {
            // Broken springs just increase the number of seen broken strings in a row
            broken_len += 1;
        } else if cur_symbol == '.' {
            // When broken_len == 0 the last seen char was a . so nothing needs to happen
            if broken_len != 0 {
                // The group of broken springs ended so check if they are the right length
                if broken_len == groups[gi] {
                    // If they are advance the group and reset the counter
                    gi += 1;
                    broken_len = 0;
                } else {
                    return 0;
                }
            }
        }
        solve(symbols, groups, i + 1, gi, broken_len, None, cache)
    } else {
        // Found ?. Consider both options by overwriting the symbol
        let mut count = 0;
        count += solve(symbols, groups, i, gi, broken_len, Some('.'), cache);
        count += solve(symbols, groups, i, gi, broken_len, Some('#'), cache);
        count
    }
}

fn load_input(name: &str) -> Input {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut input: Vec<(Vec<char>, Vec<usize>)> = Vec::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        let mut split = line.split(' ');
        let characters: Vec<char> = split.next().unwrap().chars().collect();
        let groups = split
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        input.push((characters, groups));
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = load_input("example");
        assert_eq!(crate::part_1(&input), 21);
    }

    #[test]
    fn part_2() {
        let input = load_input("example");
        assert_eq!(crate::part_2(&input), 525152);
    }
}
