use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", solve(&input.0));
    println!("Solution for part 1: {}", solve(&input.1));
}

fn solve(input: &[(Dire, isize)]) -> isize {
    let mut points = vec![(0, 0)];
    let mut trench_len = 0;
    let mut cur_x = 0;
    let mut cur_y = 0;
    for dig in input {
        trench_len += dig.1;
        match dig.0 {
            Dire::U => cur_y += dig.1,
            Dire::D => cur_y -= dig.1,
            Dire::L => cur_x -= dig.1,
            Dire::R => cur_x += dig.1,
        }
        points.push((cur_x, cur_y));
    }
    calculate_area(&points, trench_len)
}

// Shoelace theory
fn calculate_area(points: &[(isize, isize)], trench_len: isize) -> isize {
    let mut sum = 0;
    let mut reverse_points = points.to_vec();
    reverse_points.reverse();
    for double in reverse_points.windows(2) {
        sum += (double[0].0 * double[1].1) - (double[1].0 * double[0].1)
    }
    // trench_len / 2 because the points are in the middle of
    // the trenches so half is already in the area
    (sum / 2) + (trench_len / 2) + 1
}

#[allow(clippy::type_complexity)]
fn load_input(name: &str) -> (Vec<(Dire, isize)>, Vec<(Dire, isize)>) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut input = Vec::new();
    let mut input_2 = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let mut split = line.split(' ');
        let dire = match split.next().unwrap() {
            "U" => Dire::U,
            "D" => Dire::D,
            "L" => Dire::L,
            "R" => Dire::R,
            _ => unreachable!(),
        };
        let distance = split.next().unwrap().parse::<isize>().unwrap();
        input.push((dire, distance));

        let color = split
            .next()
            .unwrap()
            .strip_prefix("(#")
            .unwrap()
            .strip_suffix(')')
            .unwrap();
        let new_dire = match &color[5..] {
            "0" => Dire::R,
            "1" => Dire::D,
            "2" => Dire::L,
            "3" => Dire::U,
            _ => unreachable!(),
        };
        let new_distance = isize::from_str_radix(&color[0..5], 16).unwrap();
        input_2.push((new_dire, new_distance));
    }

    (input, input_2)
}

#[derive(Debug, Clone, Copy)]
enum Dire {
    U,
    D,
    L,
    R,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = load_input("example");
        assert_eq!(crate::solve(&input.0), 62);
    }

    #[test]
    fn part_2() {
        let input = load_input("example");
        assert_eq!(crate::solve(&input.1), 952408144115);
    }
}
