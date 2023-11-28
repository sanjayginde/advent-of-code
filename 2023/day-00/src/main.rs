use std::{fs::read_to_string, env};

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

/// Returns a sum of all the numbers passed in
/// 
fn solve(filename: String) -> i32 {
    let lines = read_lines(&filename);
    let mut total = 0;
    for (pos, line) in lines.iter().enumerate() {
        match line.parse::<i32>()  {
            Ok(num) => total += num,
            Err(_) => println!("Error parsing line {}: {}", pos + 1, line),
        }
    }
    total
}

fn main() {    
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0..=1 => println!("Pass in filename to solve"),
        _ => println!(
            "Solution for {} is {}",
            args[1].clone(),
            solve(args[1].clone())
        ),
    }
}
