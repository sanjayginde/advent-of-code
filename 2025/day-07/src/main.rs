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

fn part1(lines: Vec<String>) -> usize {
    let mut result = 0;

    let mut grid = grid::parse_to_grid(&lines, Element::from);

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let row_below = i + 1;
            let next_column = j + 1;

            match grid[i][j] {
                Element::Start => {
                    if row_below < grid.len() {
                        grid[row_below][j] = Element::Beam;
                    }
                }
                Element::Beam => {
                    if row_below < grid.len() {
                        match grid[row_below][j] {
                            Element::Splitter => {
                                if j > 0 && grid[row_below][j - 1] == Element::Space {
                                    grid[row_below][j - 1] = Element::Beam;
                                }
                                if next_column < grid[i].len()
                                    && grid[row_below][next_column] == Element::Space
                                {
                                    grid[row_below][next_column] = Element::Beam;
                                }
                                result += 1;
                            }
                            Element::Space => {
                                grid[row_below][j] = Element::Beam;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    result
}

// NOTE: input.txt needs to be 'right padded' to create a uniform grid!
fn main() {
    println!(
        "Solution for part 1 is {}",
        part1(read_lines_from_file("input.txt"))
    );
    // println!(
    //     "Solution for part 2 is {}",
    //     part2(read_lines_from_file("input.txt"))
    // );
}

#[cfg(test)]
mod test {
    use super::part1;
    // use super::part2;

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

    // #[test]
    // fn solve_example_part2() {
    //     assert_eq!(part2(EXAMPLE.map(String::from).to_vec()), 3263827);
    // }
}
