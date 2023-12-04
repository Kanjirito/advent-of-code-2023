use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(cards: &[Card]) -> u64 {
    let mut sum = 0;
    for card in cards {
        let mut counter = 0;
        for found in &card.found {
            if card.winning.contains(found) {
                counter += 1;
            }
        }
        match counter {
            0 => {}
            1 => sum += 1,
            _ => sum += 2_u64.pow(counter - 1),
        }
    }
    sum
}

// Instead of doing it normally and following the winning cards every time you can just calculate how many cards any card will give you
// by going from the bottom. The last card will always only give you 1 card, the second to last can only give you 1 + 1 if it wins etc.
// Using that you can just start from the bottom and set the value of the current card to be equal to 1 + the values of every won card:
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11 = 1
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36 = 1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83 = 1 + card 5 == 2
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1 = 1 + card 4 + card 5 == 4
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19 = 1 + card 3 + card 4 == 7
// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53 = 1 + card 2 + card 3 + card 4 + card 5 = 15
// After that you can just sum up all of the values
fn part_2(cards: &[Card]) -> u64 {
    // Vec that keeps track of the values of visited cards
    let mut card_values: Vec<u64> = vec![0; cards.len()];

    // Go in reverse over cards
    for card in cards.iter().rev() {
        // Count how many wins it has
        let mut win_counter = 0;
        for found in &card.found {
            if card.winning.contains(found) {
                win_counter += 1;
            }
        }
        // Value of every card always starts at 1
        let mut cur_card_value = 1;
        // Go over every card current card won
        for (x, other_value) in card_values
            .iter()
            .enumerate()
            .take(card.id + win_counter + 1)
            .skip(card.id + 1)
        {
            // Boundary check
            if x >= cards.len() {
                break;
            }
            cur_card_value += other_value;
        }
        card_values[card.id] = cur_card_value;
    }

    card_values.iter().sum()
}

fn load_input(name: &str) -> Vec<Card> {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
    let mut cards: Vec<Card> = Vec::new();

    for (c, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
        let mut number_split = line.split(": ").nth(1).unwrap().split(" | ");
        let winning_numbers: HashSet<u64> = number_split
            .next()
            .unwrap()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u64>().unwrap())
            .collect();
        let found_numbers: Vec<u64> = number_split
            .next()
            .unwrap()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u64>().unwrap())
            .collect();

        cards.push(Card {
            id: c,
            winning: winning_numbers,
            found: found_numbers,
        })
    }
    cards
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning: HashSet<u64>,
    found: Vec<u64>,
}

#[test]
fn example() {
    let input = load_input("example");
    assert_eq!(part_1(&input), 13);
    assert_eq!(part_2(&input), 30);
}
