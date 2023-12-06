use regex::Regex;
use std::{env, fs::read_to_string, ops::Range};

#[derive(Clone, Debug)]
struct SourceRange {
    source: Range<i64>,
    destination: Range<i64>,
    offset: i64,
}

impl SourceRange {
    pub fn new(destination_start: i64, source_start: i64, range_length: i64) -> SourceRange {
        let source_range = Range {
            start: source_start,
            end: source_start + range_length,
        };

        let destination_range = Range {
            start: destination_start,
            end: destination_start + range_length,
        };

        SourceRange {
            source: source_range,
            destination: destination_range,
            offset: destination_start - source_start as i64,
        }
    }

    pub fn map_to_destination(&self, source_num: i64) -> Option<i64> {
        // match source_num >= self.source.start && source_num < self.source.end {
        match self.source.contains(&source_num) {
            true => Some(source_num + self.offset),
            false => None,
        }
    }

    pub fn map_to_source(&self, dest_num: i64) -> Option<i64> {
        match self.destination.contains(&dest_num) {
            true => Some(dest_num - self.offset),
            false => None,
        }
    }
}

#[derive(Clone, Debug)]
struct Map {
    _name: String,
    ranges: Vec<SourceRange>,
}

impl Map {
    pub fn push_ranges(&mut self, range: SourceRange) {
        self.ranges.push(range);
        self.ranges
            .sort_by(|lhs, rhs| lhs.destination.start.cmp(&rhs.destination.start));
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

    pub fn map_to_source(&self, dest_num: i64) -> i64 {
        for range in &self.ranges {
            match range.map_to_source(dest_num) {
                Some(destination) => return destination,
                None => {}
            }
        }

        return dest_num;
    }
}

fn parse(lines: Vec<String>) -> (Vec<i64>, Vec<Map>) {
    let numbers_re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();

    let mut lines_iter = lines.iter();
    let seeds: Vec<i64> = lines_iter
        .next()
        .unwrap()
        .split(": ")
        .collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    lines_iter.next(); // ignore blank line after seeds

    let mut maps: Vec<Map> = Vec::new();
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

    (seeds, maps)
}

fn solve_brute(seed_ranges: Vec<Range<i64>>, maps: Vec<Map>) -> i64 {
    let nested_destinations: Vec<Vec<i64>> = seed_ranges
        .into_iter()
        .map(|seed_range| {
            seed_range
                .into_iter()
                .map(|seed| {
                    let mut current = seed;
                    for map in maps.iter() {
                        current = map.map_to_destination(current);
                    }
                    current
                })
                .collect()
        })
        .collect();

    let destinations: Vec<i64> = nested_destinations.into_iter().flatten().collect();

    destinations.into_iter().min().unwrap()
}

fn solve_reverse(seed_ranges: Vec<Range<i64>>, mut maps: Vec<Map>) -> i64 {
    let mut result: Option<i64> = None;
    let mut destination: i64 = 0;
    maps.reverse();

    while result.is_none() {
        // println!("destination: {destination}");

        let mut current: i64 = destination;
        for map in maps.iter() {
            current = map.map_to_source(current);
        }

        for seed_range in seed_ranges.iter() {
            if seed_range.contains(&current) {
                // println!("{current} in {seed_range:?}");
                result = Some(destination);
            }
        }

        destination += 1;
    }

    result.unwrap()
}

fn solve(lines: Vec<String>, reverse: bool) -> i64 {
    let (seeds, maps) = parse(lines);

    let seed_ranges: Vec<Range<i64>> = seeds
        .chunks_exact(2)
        .map(|chunk| {
            let seed = chunk[0];
            let length = chunk[1];

            Range {
                start: seed,
                end: seed + length,
            }
        })
        .collect();

    println!("{seed_ranges:?}");

    match reverse {
        true => solve_reverse(seed_ranges, maps),
        false => solve_brute(seed_ranges, maps),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0..=1 => println!("Pass in filename to solve and part"),
        _ => println!(
            "Solution for {} is {}",
            args[1].clone(),
            solve(read_lines(&args[1].clone()), true)
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

        assert_eq!(solve(rows, true), 46,);
    }
}
