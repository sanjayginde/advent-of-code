use std::fmt::Display;

use itertools::Itertools;
use rust_aoc_utils::read_lines_from_file;

#[derive(Debug)]
struct Tile {
    x: usize,
    y: usize,
}

impl From<&String> for Tile {
    fn from(s: &String) -> Self {
        let parts = s
            .split(",")
            .map(|part| part.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Tile {
            x: parts[0],
            y: parts[1],
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn tile_area(a: &Tile, b: &Tile) -> usize {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

fn part1(lines: Vec<String>) -> usize {
    let mut result = 0;
    let tiles = lines.iter().map(Tile::from).collect::<Vec<_>>();
    let combos = tiles.iter().combinations(2);

    for combo in combos {
        let a = combo[0];
        let b = combo[1];

        let area = tile_area(a, b);
        println!("[{:?}, {:?}]: {:?}", a, b, area);
        if area > result {
            result = area;
        }
    }

    result
}

fn part2(lines: Vec<String>) -> usize {
    let mut result = 0;

    result
}

fn main() {
    println!("Part 1: {}", part1(read_lines_from_file("input.txt")));
    println!("Part 2: {}", part2(read_lines_from_file("input.txt")));
}

// Utilities

#[cfg(test)]
mod test {
    use super::part1;
    use super::part2;

    const EXAMPLE: [&str; 8] = ["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"];

    #[test]
    fn solve_example_part1() {
        assert_eq!(part1(EXAMPLE.map(String::from).to_vec()), 50);
    }

    #[test]
    fn _solve_example_part2() {
        assert_eq!(part2(EXAMPLE.map(String::from).to_vec()), 14);
    }
}
