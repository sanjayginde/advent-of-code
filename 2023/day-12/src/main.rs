use std::fs::read_to_string;

use springs::Row;

pub mod springs;

fn parse(lines: Vec<String>) -> Vec<Row> {
    lines.into_iter().map(Row::from).collect()
}

fn solve_part1(rows: Vec<Row>) -> usize {
    rows.into_iter()
        .fold(0, |acc, r| acc + r.calc_arrangements())
}

fn solve_part2(rows: Vec<Row>) -> usize {
    let rows: Vec<Row> = rows
        .into_iter()
        .map(|row| row.convert_to_part2_row())
        .collect();

    solve_part1(rows)
}

fn main() {
    let rows = parse(read_lines("input.txt"));

    println!("Solution for part 1 is {}", solve_part1(rows.clone()));
    println!("Solution for part 2 is {}", solve_part2(rows));
}

#[cfg(test)]
mod test {

    use super::{parse, solve_part1, solve_part2};

    const EXAMPLE: [&str; 6] = [
        "???.### 1,1,3",
        ".??..??...?##. 1,1,3",
        "?#?#?#?#?#?#?#? 1,3,1,6",
        "????.#...#... 4,1,1",
        "????.######..#####. 1,6,5",
        "?###???????? 3,2,1",
    ];

    #[test]
    fn solve_example_part1() {
        let rows = parse(EXAMPLE.map(String::from).to_vec());

        assert_eq!(solve_part1(rows), 21);
    }

    #[test]
    fn solve_example_part2() {
        let rows = parse(EXAMPLE.map(String::from).to_vec());

        assert_eq!(solve_part2(rows), 525152);
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
