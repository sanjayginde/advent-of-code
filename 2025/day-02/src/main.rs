use std::fs::read_to_string;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let mut split = s.split('-');
        let start = split.next().unwrap().parse::<u64>().unwrap();
        let end = split.next().unwrap().parse::<u64>().unwrap();

        Range { start, end }
    }
}

fn part1(ranges: Vec<Range>) -> u64 {
    ranges.into_iter().fold(0 as u64, |mut acc, range| {
        let mut current = range.start;
        while current <= range.end {
            let s = current.to_string();
            let (lhs, rhs) = s.split_at(s.len() / 2);
            if lhs.eq(rhs) {
                acc += current;
            }

            current += 1;
        }

        acc
    })
}

fn part2(ranges: Vec<Range>) -> u64 {
    ranges.into_iter().fold(0 as u64, |mut acc, range| {
        let mut current = range.start;
        while current <= range.end {
            let s = current.to_string();
            let chunks = s.len() / 2 + 1;
            let chars: Vec<_> = s.chars().collect();

            for i in 1..chunks {
                let windows: Vec<_> = chars.chunks(i).collect();
                let first = windows[0];
                if windows.iter().all(|w| first == *w) {
                    acc += current;
                    break;
                }
            }

            current += 1;
        }

        acc
    })
}

fn main() {
    println!("Solution for part 1 is {}", part1(read_input("input.txt")));
    println!("Solution for part 2 is {}", part2(read_input("input.txt")));
}

// Utilities

fn read_input(filename: &str) -> Vec<Range> {
    let input = read_to_string(filename).unwrap();
    parse_input(&input)
}

fn parse_input(input: &String) -> Vec<Range> {
    input.trim().split(",").map(Range::from).collect()
}

#[cfg(test)]
mod test {
    use super::parse_input;
    use super::part1;
    use super::part2;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn solve_example_part1() {
        let ranges = parse_input(&EXAMPLE.to_owned());
        assert_eq!(part1(ranges), 1227775554);
    }

    #[test]
    fn solve_example_part2() {
        let ranges = parse_input(&EXAMPLE.to_owned());
        assert_eq!(part2(ranges), 4174379265);
    }
}
