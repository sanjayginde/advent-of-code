use regex::Regex;
use std::fs::read_to_string;
use std::ops::Range;

fn part1(lines: Vec<String>) -> usize {
    let mut result = 0;

    let (ranges, ids) = parse(lines);
    for id in ids {
        if ranges.iter().any(|range| range.contains(&id)) {
            result += 1;
        }
    }

    result
}

fn part2(lines: Vec<String>) -> usize {
    let mut _result = 0;
    let (_ranges, _ids) = parse(lines);

    _result
}

fn main() {
    println!("Solution for part 1 is {}", part1(read_lines("input.txt")));
    println!("Solution for part 2 is {}", part2(read_lines("input.txt")));
}

// Utilities

fn parse(lines: Vec<String>) -> (Vec<Range<usize>>, Vec<usize>) {
    let range_regex = Regex::new(r"(\d+)-(\d+)").unwrap();

    let mut ranges = vec![];
    let mut ids = vec![];

    for line in lines {
        match range_regex.captures(&line) {
            None => {
                if !line.is_empty() {
                    ids.push(line.parse::<usize>().expect("id"));
                }
            }
            Some(capture) => ranges.push(Range {
                start: capture[1].parse::<usize>().expect("range start"),
                end: capture[2].parse::<usize>().expect("range end") + 1,
            }),
        }
    }

    (ranges, ids)
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
    use super::part1;
    use super::part2;

    const EXAMPLE: [&str; 11] = [
        "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
    ];

    #[test]
    fn solve_example_part1() {
        assert_eq!(part1(EXAMPLE.map(String::from).to_vec()), 3);
    }

    // #[test]
    fn _solve_example_part2() {
        assert_eq!(part2(EXAMPLE.map(String::from).to_vec()), 14);
    }
}
