use rust_aoc_utils::{parse_to_whitespaced_grid, transpose};
use std::fs::read_to_string;

#[derive(Debug, Clone)]
enum Element {
    Plus,
    Times,
    Number(usize),
}

impl From<&str> for Element {
    fn from(s: &str) -> Self {
        match s {
            "+" => Element::Plus,
            "*" => Element::Times,
            _ => Element::Number(s.parse::<usize>().expect("Expected number")),
        }
    }
}

fn part1(lines: Vec<String>) -> usize {
    let grid = transpose(parse_to_whitespaced_grid(&lines, |s| Element::from(s)));

    let mut result = 0;
    for row in grid.iter() {
        let mut iter = row.iter().rev();
        let operator = iter.next().unwrap();
        result += match operator {
            Element::Plus => iter.fold(0, |acc, el| match el {
                Element::Number(num) => acc + num,
                _ => {
                    panic!("Expected number, but found {:?}", el)
                }
            }),
            Element::Times => iter.fold(1, |acc, el| match el {
                Element::Number(num) => acc * num,
                _ => {
                    panic!("Expected number, but found {:?}", el)
                }
            }),
            Element::Number(num) => {
                panic!("Expected operator, but found number {}", num);
            }
        };
    }

    result
}

// fn part2(lines: Vec<String>) -> usize {
//     let (ranges, _) = parse(lines);

//     let mut result = 0;
//     for range in squash_ranges(&ranges) {
//         result += range.end - range.start;
//     }

//     result
// }

fn main() {
    println!("Solution for part 1 is {}", part1(read_lines("input.txt")));
    // println!("Solution for part 2 is {}", part2(read_lines("input.txt")));
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
    // use super::part2;

    const EXAMPLE: [&str; 4] = [
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
        "*   +   *   +  ",
    ];

    #[test]
    fn solve_example_part1() {
        assert_eq!(part1(EXAMPLE.map(String::from).to_vec()), 4277556);
    }

    // #[test]
    // fn _solve_example_part2() {
    //     assert_eq!(part2(EXAMPLE.map(String::from).to_vec()), 14);
    // }
}
