use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
}

fn load_input(name: &str) {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
}

#[test]
fn example() {
    let input = load_input("example");
}
