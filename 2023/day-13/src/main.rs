use itertools::diff_with;
use std::fs::read_to_string;
use Orientation::*;

pub type Pattern = Vec<Vec<char>>;

enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    pub fn multiply(&self, value: usize) -> usize {
        match self {
            Orientation::Horizontal => value * 100,
            Orientation::Vertical => value,
        }
    }
}

fn parse_pattern(lines: Vec<String>) -> Pattern {
    let mut matrix: Pattern = Vec::with_capacity(lines.len());

    for (_r, line) in lines.iter().enumerate() {
        matrix.push(line.chars().collect());
    }

    matrix
}

fn parse(lines: Vec<String>) -> Vec<Pattern> {
    let mut patterns: Vec<Pattern> = Vec::new();

    let mut pattern_to_parse: Vec<String> = Vec::new();

    for line in lines {
        match line.trim().len() == 0 {
            true => {
                patterns.push(parse_pattern(pattern_to_parse));
                pattern_to_parse = Vec::new();
            }
            false => {
                pattern_to_parse.push(line);
            }
        }
    }

    patterns.push(parse_pattern(pattern_to_parse));

    patterns
}

fn diff(pattern: &Pattern, pos: usize) -> Option<usize> {
    let mut row1_pos = pos;
    let mut row2_pos = pos + 1;
    let mut row1 = pattern.get(row1_pos);
    let mut row2 = pattern.get(row2_pos);

    let pivot = pos + 1;

    while row1.is_some() && row2.is_some() {
        match diff_with(row1.unwrap(), row2.unwrap(), |lhs, rhs| lhs == rhs) {
            Some(_) => {
                return None;
            }
            None => {
                if row1_pos == 0 {
                    break;
                }
                row1_pos -= 1;
                row2_pos += 1;
                row1 = pattern.get(row1_pos);
                row2 = pattern.get(row2_pos);
            }
        }
    }
    return Some(pivot);
}

fn calc(pattern: Pattern, orientation: Orientation) -> Option<usize> {
    for (pos, _rows) in pattern.windows(2).into_iter().enumerate() {
        match diff(&pattern, pos) {
            Some(pivot) => return Some(orientation.multiply(pivot)),
            None => {}
        }
    }

    return None;
}

fn solve_part1(patterns: Vec<Pattern>) -> usize {
    let mut total: usize = 0;
    println!("num patterns: {}", patterns.len());
    for (pos, pattern) in patterns.into_iter().enumerate() {
        match calc(pattern.clone(), Horizontal) {
            Some(value) => {
                println!("Horizonal mirror found for {pos}: {value}");
                total += value;
            }

            None => {
                let transposed_pattern = transpose(pattern.clone());
                match calc(transposed_pattern, Vertical) {
                    Some(value) => {
                        println!("Vertical mirror found for {pos}: {value}");
                        total += value;
                    }
                    None => {
                        println!("No mirror found for {pos}");
                        println!("{pattern:?}");
                        println!("");
                    }
                }
            }
        }
    }

    total
}

fn main() {
    let patterns: Vec<Pattern> = parse(read_lines("input.txt"));

    println!("Solution for part 1 is {}", solve_part1(patterns));
}

#[cfg(test)]
mod test {

    use super::{parse_pattern, solve_part1, Pattern};

    const EXAMPLE_VERTICAL: [&str; 7] = [
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
    ];

    const EXAMPLE_HORIZONTAL: [&str; 7] = [
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ];

    const HORIZONTAL_2: [&str; 9] = [
        "#.###.#..#.#.",
        "#.###.#..###.",
        ".#####.##.#.#",
        "...#..#......",
        "...##.#..##.#",
        "###.#.##.#.##",
        "###.#.##.#.##",
        "...##.#..##.#",
        "...#..#......",
    ];

    #[test]
    fn solve_example_vertical() {
        let pattern: Pattern = parse_pattern(EXAMPLE_VERTICAL.map(String::from).to_vec());

        assert_eq!(solve_part1(vec![pattern]), 5);
    }

    #[test]
    fn solve_example_horizontal() {
        let pattern: Pattern = parse_pattern(EXAMPLE_HORIZONTAL.map(String::from).to_vec());

        assert_eq!(solve_part1(vec![pattern]), 400);
    }

    #[test]
    fn solve_example_horizontal2() {
        let pattern: Pattern = parse_pattern(HORIZONTAL_2.map(String::from).to_vec());

        assert_eq!(solve_part1(vec![pattern]), 600);
    }
}

// Utilities

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
