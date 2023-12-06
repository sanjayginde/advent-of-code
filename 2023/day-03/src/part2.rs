use std::{env, fs::read_to_string};

use once_cell::sync::Lazy;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn starts_with_num(s: &str) -> Option<u32> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)").unwrap());
    match RE.captures(s) {
        Some(capture) => Some(capture.get(0).unwrap().as_str().parse::<u32>().unwrap()),
        None => None,
    }
}

fn ends_with_num(s: &str) -> Option<u32> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)$").unwrap());
    match RE.captures(s) {
        Some(capture) => Some(capture.get(0).unwrap().as_str().parse::<u32>().unwrap()),
        None => None,
    }
}

fn get_adjacent_nums(col: usize, row: Option<&String>) -> Vec<u32> {
    match row {
        None => {}
        Some(row) => {
            let ch = row.chars().nth(col).unwrap();
            match ch.is_numeric() {
                true => {
                    let (lhs, rhs) = row.split_at(col);
                    let mut result: String = "".to_string();
                    match ends_with_num(lhs) {
                        Some(num) => {
                            result.push_str(num.to_string().as_str());
                        }
                        None => {}
                    }

                    match starts_with_num(rhs) {
                        Some(num) => {
                            result.push_str(num.to_string().as_str());
                        }
                        None => {
                            println!("rhs doesn't start with num: {rhs} col:{col}");
                        }
                    }
                    println!("result: {result}");

                    return vec![result.parse::<u32>().unwrap()];
                }
                false => {
                    let (lhs, rhs) = row.split_at(col);

                    return vec![ends_with_num(lhs), starts_with_num(rhs.get(1..).unwrap())]
                        .into_iter()
                        .filter_map(|v| v)
                        .collect();
                }
            }
        }
    }

    vec![]
}

fn solve(rows: Vec<String>) -> u32 {
    let mut total: u32 = 0;

    println!("num lines: {}", rows.len());
    // let rows: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    for (row_num, row) in rows.iter().enumerate() {
        // println!("{row_num}");
        let previous_row = if row_num > 0 {
            rows.get(row_num - 1)
        } else {
            None
        };
        let next_row = rows.get(row_num + 1);

        for (col_num, ch) in row.chars().enumerate() {
            if ch != '*' {
                continue;
            }

            let mut adjacent_nums: Vec<u32> = get_adjacent_nums(col_num, Some(row));
            adjacent_nums.append(&mut get_adjacent_nums(col_num, previous_row));
            adjacent_nums.append(&mut get_adjacent_nums(col_num, next_row));

            // println!("{adjacent_nums:?}");
            // let filtered: Vec<_> = adjacent_nums.into_iter().filter_map(|num| num).collect();

            if adjacent_nums.len() == 2 {
                println!("\tFound 2: {adjacent_nums:?}");
                total += adjacent_nums[0] * adjacent_nums[1];
            }
        }
    }

    total
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

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn solve_example() {
        let rows = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(solve(rows), 467835);
    }

    #[test]
    fn solve_example2() {
        let rows = [
            "....*467..",
            "....**..*.",
            ".467*114..",
            "....*......",
            "....7......",
            "..*.......",
            ".114......",
            ".....840...",
            "79..*......",
            "../.460..#.",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(solve(rows), 53238 + 53238 + 53238 + 386400);
        // assert_eq!(solve(rows), 159714);

        // assert_eq!(solve(rows), 159714 + 218089 + 218089);
    }
}