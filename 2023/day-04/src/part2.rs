use std::{env, fs::read_to_string};

#[derive(Debug)]
struct Card {
    winning: Numbers,
    player: Numbers,
}

impl Card {
    pub fn new(winning: Numbers, player: Numbers) -> Card {
        Card { winning, player }
    }
    pub fn num_winners(&self) -> u32 {
        let mut result: u32 = 0;
        for number in self.player.numbers.iter() {
            if self.winning.contains(*number) {
                result = result + 1;
            }
        }
        result
    }
}

#[derive(Debug)]
struct Numbers {
    numbers: Vec<u32>,
}

impl Numbers {
    pub fn contains(&self, number: u32) -> bool {
        self.numbers.contains(&number)
    }
}

impl From<&str> for Numbers {
    fn from(value: &str) -> Self {
        let num_strs: Vec<&str> = value.split_whitespace().collect();
        let nums: Vec<u32> = num_strs
            .iter()
            .map(|num_str| -> u32 { num_str.trim().parse::<u32>().unwrap() })
            .collect();

        Numbers { numbers: nums }
    }
}

fn solve(lines: Vec<String>) -> u32 {
    let mut num_card_copies = vec![1 as u32; lines.len()];

    for (pos, line) in lines.iter().enumerate() {
        let mut splits = line.split(&[':', '|']).collect::<Vec<&str>>();
        splits = splits.iter().map(|s| s.trim()).collect();
        let mut splits_iter = splits.iter();

        let _card = *splits_iter.next().unwrap();
        let card = Card::new(
            Numbers::from(*splits_iter.next().unwrap()),
            Numbers::from(*splits_iter.next().unwrap()),
        );

        // Number of copies of the current card
        let num_copies = num_card_copies[pos];

        let mut num_winners = card.num_winners();

        for (nc_pos, num_card_copy) in num_card_copies.iter_mut().enumerate() {
            if num_winners == 0 {
                break;
            }
            if nc_pos > pos {
                *num_card_copy += num_copies;
                num_winners -= 1;
            }
        }
    }

    num_card_copies.iter().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0..=1 => println!("Pass in filename to solve and part"),
        _ => println!(
            "Solution for part 1 for {} is {}",
            args[1].clone(),
            solve(read_lines(&args[1].clone()))
        ),
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod test {

    use super::solve;

    #[test]
    fn solve_example() {
        let rows = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(solve(rows), 30);
    }
}
