pub mod history;

use std::{env, fs::read_to_string};

use history::History;

fn parse(lines: Vec<String>) -> Vec<History> {
    lines.iter().map(History::from).collect()
}

fn solve(lines: Vec<String>) -> i64 {
    let histories = parse(lines);

    histories
        .into_iter()
        .fold(0, |acc, history| acc + history.next_value())
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
    fn solve_example() {
        let rows = ["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"]
            .map(String::from)
            .to_vec();

        assert_eq!(solve(rows), 114);
    }
}
