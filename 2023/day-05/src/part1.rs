use std::{env, fs::read_to_string, ops::Range};

use regex::Regex;
struct Ranges {
    destination_start: u64,
    source_start: u64,
    _length: u64,
    _destination: Range<u64>,
    source: Range<u64>,
}

impl Ranges {
    pub fn new(destination_start: u64, source_start: u64, range_length: u64) -> Ranges {
        let destination_range = Range {
            start: destination_start,
            end: destination_start + range_length,
        };

        let source_range = Range {
            start: source_start,
            end: source_start + range_length,
        };

        Ranges {
            destination_start,
            source_start,
            _length: range_length,
            _destination: destination_range,
            source: source_range,
        }
    }
}

struct Map {
    _name: String,
    ranges: Vec<Ranges>,
}

impl Map {
    pub fn push_ranges(&mut self, range: Ranges) {
        self.ranges.push(range);
    }

    pub fn source_to_destination(&self, source: u64) -> u64 {
        for range in &self.ranges {
            if range.source.contains(&source) {
                return source - range.source_start + range.destination_start;
            }
        }

        return source;
    }
}

fn solve(lines: Vec<String>) -> u64 {
    let numbers_re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
    let mut maps: Vec<Map> = Vec::new();

    let mut lines_iter = lines.iter();

    let seeds: Vec<u64> = lines_iter
        .next()
        .unwrap()
        .split(": ")
        .collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
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
                    .push_ranges(Ranges::new(
                        *&caps[1].parse::<u64>().expect("destination number"),
                        *&caps[2].parse::<u64>().expect("destination number"),
                        *&caps[3].parse::<u64>().expect("destination number"),
                    ));
            }
        }
    }

    let destinations: Vec<u64> = seeds
        .into_iter()
        .map(|seed| {
            let mut current = seed;
            for map in maps.iter() {
                current = map.source_to_destination(current);
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

    use crate::Ranges;

    use super::solve;

    #[test]
    fn test_map_range() {
        let m = Ranges::new(50, 98, 2);

        let mut dest = m._destination.into_iter();
        assert_eq!(dest.next(), Some(50));
        assert_eq!(dest.next(), Some(51));
        assert_eq!(dest.next(), None);

        let mut source = m.source.into_iter();
        assert_eq!(source.next(), Some(98));
        assert_eq!(source.next(), Some(99));
        assert_eq!(source.next(), None);
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
