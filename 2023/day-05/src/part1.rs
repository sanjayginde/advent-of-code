use std::{env, fs::read_to_string};

use regex::Regex;
struct SourceRange {
    source: std::ops::Range<i64>,
    offset: i64,
}

impl SourceRange {
    pub fn new(destination_start: i64, source_start: i64, range_length: i64) -> SourceRange {
        let source_range = std::ops::Range {
            start: source_start,
            end: source_start + range_length,
        };

        SourceRange {
            source: source_range,
            offset: destination_start - source_start as i64,
        }
    }

    pub fn map_to_destination(&self, source_num: i64) -> Option<i64> {
        match self.source.contains(&source_num) {
            true => Some(source_num + self.offset),
            false => None,
        }
    }
}

struct Map {
    _name: String,
    ranges: Vec<SourceRange>,
}

impl Map {
    pub fn push_ranges(&mut self, range: SourceRange) {
        self.ranges.push(range);
    }

    pub fn map_to_destination(&self, source_num: i64) -> i64 {
        for range in &self.ranges {
            match range.map_to_destination(source_num) {
                Some(destination) => return destination,
                None => {}
            }
        }

        return source_num;
    }
}

fn solve(lines: Vec<String>) -> i64 {
    let numbers_re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
    let mut maps: Vec<Map> = Vec::new();

    let mut lines_iter = lines.iter();

    let seeds: Vec<i64> = lines_iter
        .next()
        .unwrap()
        .split(": ")
        .collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    lines_iter.next();

    for line in lines_iter {
        let captures: Option<regex::Captures<'_>> = numbers_re.captures(&line);
        match captures {
            None => {
                if line.trim() != "" {
                    maps.push(Map {
                        _name: line.to_string(),
                        ranges: Vec::new(),
                    });
                }
            }
            Some(caps) => {
                maps.last_mut()
                    .expect("no last map available")
                    .push_ranges(SourceRange::new(
                        *&caps[1].parse::<i64>().expect("destination number"),
                        *&caps[2].parse::<i64>().expect("destination number"),
                        *&caps[3].parse::<i64>().expect("destination number"),
                    ));
            }
        }
    }

    let destinations: Vec<i64> = seeds
        .into_iter()
        .map(|seed| {
            let mut current = seed;
            for map in maps.iter() {
                current = map.map_to_destination(current);
            }
            current
        })
        .collect();

    destinations.into_iter().min().unwrap()
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

    use crate::SourceRange;

    use super::solve;

    #[test]
    fn test_source_range() {
        let source = SourceRange::new(50, 98, 2);

        assert_eq!(source.map_to_destination(97), None);
        assert_eq!(source.map_to_destination(98), Some(50));
        assert_eq!(source.map_to_destination(99), Some(51));
        assert_eq!(source.map_to_destination(100), None);
    }

    #[test]
    fn solve_example() {
        let rows = [
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(solve(rows), 35);
    }
}
