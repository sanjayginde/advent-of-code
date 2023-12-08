use std::{env, fs::read_to_string};

/// Returns a sum of all the numbers passed in
fn solve(lines: Vec<String>) -> i32 {
    let mut total = 0;
    for (pos, line) in lines.iter().enumerate() {
        match line.parse::<i32>() {
            Ok(num) => total += num,
            Err(_) => println!("Error parsing line {}: {}", pos + 1, line),
        }
    }
    total
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0..=1 => println!("Pass in filename to solve"),
        _ => println!(
            "Solution for {} is {}",
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
        let rows = ["7", "9"].map(String::from).to_vec();

        assert_eq!(solve(rows), 16);
    }
}
