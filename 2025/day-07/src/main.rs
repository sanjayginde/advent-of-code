use std::collections::HashMap;
use std::fmt::Display;

use rust_aoc_utils::grid;
use rust_aoc_utils::read_lines_from_file;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Element {
    Start,
    Splitter,
    Space,
    Beam,
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        match value {
            'S' => Element::Start,
            '^' => Element::Splitter,
            '.' => Element::Space,
            _ => panic!("Invalid character: {}", value),
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Start => write!(f, "S"),
            Element::Splitter => write!(f, "^"),
            Element::Space => write!(f, "."),
            Element::Beam => write!(f, "|"),
        }
    }
}

fn map_beams(grid: &mut [Vec<Element>]) -> usize {
    let mut result = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let next_row = row + 1;
            let next_col = col + 1;

            match grid[row][col] {
                Element::Start => {
                    if next_row < grid.len() {
                        grid[next_row][col] = Element::Beam;
                    }
                }
                Element::Beam => {
                    if next_row == grid.len() {
                        continue;
                    }

                    match grid[next_row][col] {
                        Element::Splitter => {
                            if col > 0 && grid[next_row][col - 1] == Element::Space {
                                grid[next_row][col - 1] = Element::Beam;
                            }
                            if next_col < grid[row].len()
                                && grid[next_row][next_col] == Element::Space
                            {
                                grid[next_row][next_col] = Element::Beam;
                            }
                            result += 1;
                        }
                        Element::Space => {
                            grid[next_row][col] = Element::Beam;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    result
}

fn part1(lines: Vec<String>) -> usize {
    let mut grid = grid::parse(&lines, Element::from);

    let result = map_beams(&mut grid);
    grid::print(&grid);

    result
}

fn part2(lines: Vec<String>) -> usize {
    let grid = grid::parse(&lines, Element::from);

    let start_col = grid
        .first()
        .expect("Grid is empty")
        .iter()
        .position(|el| el == &Element::Start)
        .expect("Could not find S in first row");

    let mut cache = HashMap::new();

    find_timelines(&grid, 1, start_col, &mut cache)
}

fn find_timelines(
    grid: &[Vec<Element>],
    row: usize,
    beam_col: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if row == grid.len() {
        return 1;
    }

    let el = &grid[row][beam_col];
    match el {
        Element::Splitter => {
            let left = match cache.get(&(row + 2, beam_col - 1)) {
                Some(val) => *val,
                None => find_timelines(grid, row + 2, beam_col - 1, cache),
            };

            let right = match cache.get(&(row + 1, beam_col + 1)) {
                Some(val) => *val,
                None => find_timelines(grid, row + 1, beam_col + 1, cache),
            };

            let result = left + right;
            cache.insert((row, beam_col), result);
            result
        }
        _ => find_timelines(grid, row + 1, beam_col, cache),
    }
}

fn main() {
    println!(
        "Solution for part 1 is {}",
        part1(read_lines_from_file("input.txt"))
    );
    println!(
        "Solution for part 2 is {}",
        part2(read_lines_from_file("input.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::part1;
    use super::part2;

    const EXAMPLE: [&str; 16] = [
        ".......S.......",
        "...............",
        ".......^.......",
        "...............",
        "......^.^......",
        "...............",
        ".....^.^.^.....",
        "...............",
        "....^.^...^....",
        "...............",
        "...^.^...^.^...",
        "...............",
        "..^...^.....^..",
        "...............",
        ".^.^.^.^.^...^.",
        "...............",
    ];

    #[test]
    fn solve_example_part1() {
        assert_eq!(part1(EXAMPLE.map(String::from).to_vec()), 21);
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(part2(EXAMPLE.map(String::from).to_vec()), 40);
    }
}
