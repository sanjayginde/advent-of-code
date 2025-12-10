use rust_aoc_utils::{Coordinate, check_adjacent, parse_to_char_grid};
use std::fs::read_to_string;

const PAPER_ROLL: char = '@';

fn check(c: &char) -> bool {
    c == &PAPER_ROLL
}

fn part1(lines: Vec<String>) -> usize {
    let mut result = 0;

    let grid = parse_to_char_grid(&lines);
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, char) in row.iter().enumerate() {
            let coordinate = Coordinate::new(row_index, col_index);
            if check(char) && check_adjacent(&grid, coordinate, check) < 4 {
                result += 1;
            }
        }
    }

    result
}

fn part2(lines: Vec<String>) -> usize {
    let mut result = 0;
    let mut grid = parse_to_char_grid(&lines);

    loop {
        let rolls = forklift_rolls(&grid);
        if rolls.is_empty() {
            break;
        }

        result += rolls.len();
        for coordinate in rolls.into_iter() {
            grid[coordinate.row][coordinate.col] = '.';
        }
    }

    result
}

fn forklift_rolls(grid: &[Vec<char>]) -> Vec<Coordinate> {
    let mut result = vec![];

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, char) in row.iter().enumerate() {
            let coordinate = Coordinate::new(row_index, col_index);
            if check(char) && check_adjacent(grid, coordinate, check) < 4 {
                result.push(coordinate);
            }
        }
    }

    result
}

fn main() {
    println!("Solution for part 1 is {}", part1(read_lines("input.txt")));
    println!("Solution for part 2 is {}", part2(read_lines("input.txt")));
}

// Utilities

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

    const EXAMPLE: [&str; 10] = [
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    ];

    #[test]
    fn solve_example_part1() {
        assert_eq!(part1(EXAMPLE.map(String::from).to_vec()), 13);
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(part2(EXAMPLE.map(String::from).to_vec()), 43);
    }
}
