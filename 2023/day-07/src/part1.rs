pub mod cards;

use cards::Hand;
use std::{env, fs::read_to_string};

fn solve(lines: Vec<String>) -> u32 {
    let mut hands: Vec<Hand> = lines.iter().map(Hand::from).collect();
    hands.sort();

    let mut total: u32 = 0;
    for (pos, hand) in hands.iter().enumerate() {
        total += (pos as u32 + 1) * hand.bid;
    }

    total
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0..=1 => println!("Pass in filename to solve"),
        _ => println!(
            "Solution for {} is {}",
            args[1].clone(),
            solve(read_lines(&args[1].clone()))
        ),
    }
}

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn test_solve() {
        let rows = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(solve(rows), 5905);
    }
}
