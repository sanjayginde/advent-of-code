use std::{env, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn is_symbol(ch: Option<&char>) -> bool {
    match ch {
        Some(c) => !c.is_alphanumeric() && *c != '.',
        None => false,
    }
}

fn has_adjacent_symbol(col: usize, line: Option<&Vec<char>>) -> bool {
    match line {
        Some(chars) => {
            let prev: bool = if col > 0 {
                is_symbol(chars.get(col - 1))
            } else {
                false
            };

            prev || is_symbol(chars.get(col)) || is_symbol(chars.get(col + 1))
        }
        None => false,
    }
}

fn get_num_if_adjacent(num_str: Option<String>, is_adjacent: bool) -> u32 {
    match num_str {
        Some(num) => match is_adjacent {
            true => num.parse::<u32>().unwrap(),
            false => 0,
        },
        None => 0,
    }
}

fn solve_part1(lines: Vec<String>) -> u32 {
    let mut total: u32 = 0;

    let rows: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    for (row, line) in rows.iter().enumerate() {
        // println!("{row}");

        let mut current_num: Option<String> = None;
        let mut is_adjacent = false;
        for (col, ch) in line.iter().enumerate() {
            // println!("\t{col} {ch}");
            if ch.is_numeric() {
                match current_num.as_mut() {
                    Some(num) => num.push(ch.clone()),
                    None => current_num = Some(ch.to_string()),
                }

                let previous_row = if row > 0 { rows.get(row - 1) } else { None };
                let next_row = rows.get(row + 1);

                is_adjacent = is_adjacent
                    || has_adjacent_symbol(col, previous_row)
                    || has_adjacent_symbol(col, next_row);
            } else {
                is_adjacent = is_adjacent || is_symbol(Some(ch));
                total += get_num_if_adjacent(current_num, is_adjacent);

                // Reset
                current_num = None;
                is_adjacent = is_symbol(Some(ch));
            }

            // println!("{row} {col} {ch}: {is_adjacent}");
        }

        // Handle case at the end of the row
        total += get_num_if_adjacent(current_num, is_adjacent);
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
            solve_part1(read_lines(&args[1].clone()))
        ),
    }
}

#[cfg(test)]
mod test {

    use super::solve_part1;

    #[test]
    fn solve_example() {
        let rows = [
            "467..114..",
            "...*......",
            "..35...633",
            ".......#..",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(solve_part1(rows), 4361);
    }
}
