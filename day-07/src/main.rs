use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {:?}", solve(&input, false));
    println!("Solution for part 1: {:?}", solve(&input, true));
}

fn solve(input: &[(String, u64)], part_2: bool) -> u64 {
    let mut hands = turn_into_hands(input, part_2);
    hands.sort_unstable();
    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bid * (i as u64 + 1);
    }
    sum
}

fn load_input(name: &str) -> Vec<(String, u64)> {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        let mut split = line.split(' ');
        let cards = split.next().unwrap().to_string();
        let bid = split.next().unwrap().parse::<u64>().unwrap();
        data.push((cards, bid))
    }
    data
}

fn turn_into_hands(input: &[(String, u64)], joker: bool) -> Vec<Hand> {
    let mut hands = Vec::new();
    for (s, bid) in input {
        let cards: Vec<char> = s.chars().collect();
        let card_values: Vec<u64> = cards.iter().map(|n| get_card_value(n, joker)).collect();
        let hand_type = HandType::new_from_cards(&cards, joker);
        hands.push(Hand {
            cards,
            card_values,
            bid: *bid,
            hand_type,
        });
    }

    hands
}

fn get_card_value(card: &char, joker: bool) -> u64 {
    if joker {
        match card {
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            '9' => 8,
            'T' => 9,
            'J' => 0,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => unreachable!(),
        }
    } else {
        match card {
            '2' => 0,
            '3' => 1,
            '4' => 2,
            '5' => 3,
            '6' => 4,
            '7' => 5,
            '8' => 6,
            '9' => 7,
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, Clone)]
struct Hand {
    cards: Vec<char>,
    card_values: Vec<u64>,
    bid: u64,
    hand_type: HandType,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            for (x, y) in self.card_values.iter().zip(other.card_values.iter()) {
                match x.cmp(y) {
                    Ordering::Less => {
                        return Ordering::Less;
                    }
                    Ordering::Equal => (),
                    Ordering::Greater => {
                        return Ordering::Greater;
                    }
                }
            }
            Ordering::Equal
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum HandType {
    Single,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

impl HandType {
    fn new_from_cards(cards: &[char], joker: bool) -> Self {
        let mut counter: Vec<u64> = vec![0; 13];
        let mut joker_count = 0;
        if joker {
            for c in cards {
                if *c == 'J' {
                    joker_count += 1;
                } else {
                    counter[get_card_value(c, false) as usize] += 1;
                }
            }
        } else {
            for c in cards {
                counter[get_card_value(c, false) as usize] += 1;
            }
        }
        counter.sort_unstable();
        counter.reverse();
        counter[0] += joker_count;
        match counter[0] {
            5 => Self::Five,
            4 => Self::Four,
            3 => {
                if counter[1] == 2 {
                    Self::FullHouse
                } else {
                    Self::Three
                }
            }
            2 => {
                if counter[1] == 2 {
                    Self::TwoPair
                } else {
                    Self::Pair
                }
            }
            1 => Self::Single,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = load_input("example");
        assert_eq!(crate::solve(&input, false), 6440);
    }

    #[test]
    fn part_2() {
        let input = load_input("example");
        assert_eq!(crate::solve(&input, true), 5905);
    }

    // Input data taken from
    // https://old.reddit.com/r/adventofcode/comments/18cr4xr/2023_day_7_better_example_input_not_a_spoiler/
    #[test]
    fn part_1_2() {
        let input = load_input("example2");
        assert_eq!(crate::solve(&input, false), 6592);
    }

    #[test]
    fn part_2_2() {
        let input = load_input("example2");
        assert_eq!(crate::solve(&input, true), 6839);
    }
}
