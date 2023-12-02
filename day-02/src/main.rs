use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(input: &[Game]) -> u64 {
    let limit = (12, 13, 14);
    let mut sum = 0;
    for game in input {
        if game.is_possible(&limit) {
            sum += game.id;
        }
    }
    sum
}

fn part_2(input: &[Game]) -> u64 {
    let mut sum = 0;
    for game in input {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for reveal in &game.reveals {
            max_red = max_red.max(reveal.0);
            max_green = max_green.max(reveal.1);
            max_blue = max_blue.max(reveal.2);
        }
        sum += max_red * max_green * max_blue
    }
    sum
}

fn load_input(name: &str) -> Vec<Game> {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);

    // Simple digit regex
    let game_id_regex = Regex::new(r"\d+").unwrap();
    // Looks for "[Number] [color]"
    let color_regex = Regex::new(r"(?:(\d+) (red|green|blue))+").unwrap();

    let mut games: Vec<Game> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        // Splits into the game name and reveals
        let mut split = line.split(": ");
        let game_id: u64 = game_id_regex
            .find(split.next().unwrap())
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let mut reveals: Vec<(u64, u64, u64)> = Vec::new();

        // Splits into reveals
        for reveal in split.next().unwrap().split("; ") {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            // Looks for the individual colours
            let colors = color_regex.captures_iter(reveal);
            for capture in colors {
                let number: u64 = capture.get(1).unwrap().as_str().parse().unwrap();
                match capture.get(2).unwrap().as_str() {
                    "red" => red = number,
                    "green" => green = number,
                    "blue" => blue = number,
                    _ => unreachable!(),
                }
            }
            reveals.push((red, green, blue));
        }
        games.push(Game {
            id: game_id,
            reveals,
        });
    }
    games
}

#[derive(Debug)]
struct Game {
    id: u64,
    /// Red, Green, Blue
    reveals: Vec<(u64, u64, u64)>,
}

impl Game {
    /// Checks if the game fits in the given limit
    fn is_possible(&self, limit: &(u64, u64, u64)) -> bool {
        for reveal in &self.reveals {
            if reveal.0 > limit.0 || reveal.1 > limit.1 || reveal.2 > limit.2 {
                return false;
            }
        }
        true
    }
}

#[test]
fn example() {
    let input = load_input("example");
    assert_eq!(part_1(&input), 8);
    assert_eq!(part_2(&input), 2286);
}
