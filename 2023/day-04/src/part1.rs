use std::{env, fs::read_to_string};

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
    let mut total: u32 = 0;

    "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    for line in lines {
        let mut splits = line.split(&[':', '|']).collect::<Vec<&str>>();
        splits = splits.iter().map(|s| s.trim()).collect();
        let mut splits_iter = splits.iter();

        let _card = *splits_iter.next().unwrap();
        // println!("{card}");

        let winning_numbers = Numbers::from(*splits_iter.next().unwrap());
        // println!("{winning_numbers:?}");
        let players_numbers = Numbers::from(*splits_iter.next().unwrap());
        // println!("{players_numbers:?}");

        let mut points: Option<u32> = None;
        for number in players_numbers.numbers {
            if winning_numbers.contains(number) {
                points = match points {
                    None => Some(1),
                    Some(val) => Some(val * 2),
                }
            }
        }

        total += match points {
            None => 0,
            Some(val) => val,
        }
    }

    total
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

        assert_eq!(solve(rows), 13);
    }
}
