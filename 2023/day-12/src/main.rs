#![allow(dead_code)]

use std::fs::read_to_string;

use springs::Row;

pub mod springs;

fn parse(lines: Vec<String>) -> Vec<Row> {
    lines.into_iter().map(Row::from).collect()
}

fn solve(rows: Vec<Row>) -> usize {
    rows.into_iter()
        .fold(0, |acc, r| acc + r.calc_arrangements())
}

fn main() {
    let rows = parse(read_lines("input.txt"));

    println!("Solution for part 1 is {}", solve(rows));
    // println!("Solution for part 2 is {}", solve(&universe, 1000000));
}

#[cfg(test)]
mod test {

    // use super::{parse, solve};

    const EXAMPLE: [&str; 6] = [
        "???.### 1,1,3",
        ".??..??...?##. 1,1,3",
        "?#?#?#?#?#?#?#? 1,3,1,6",
        "????.#...#... 4,1,1",
        "????.######..#####. 1,6,5",
        "?###???????? 3,2,1",
    ];

    // #[test]
    // fn solve_example() {
    //     let rows = parse(EXAMPLE.map(String::from).to_vec());

    //     assert_eq!(solve(rows), 21);
    // }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
