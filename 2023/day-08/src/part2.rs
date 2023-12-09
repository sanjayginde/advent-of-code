pub mod maps;
use maps::{Node, Part2Node, Route};
use std::{collections::HashMap, env, fs::read_to_string};

fn parse(
    lines: Vec<String>,
) -> (
    String,
    Vec<Route<Part2Node>>,
    HashMap<Part2Node, Route<Part2Node>>,
) {
    let instructions = lines.get(0).unwrap().trim();
    let mut starts: Vec<Route<Part2Node>> = Vec::new();
    let mut map: HashMap<Part2Node, Route<Part2Node>> = HashMap::new();

    for line in lines.iter().skip(2) {
        let route: Route<Part2Node> = Route::from(line);

        if route.node.is_start() {
            starts.push(route.clone());
        }

        match map.insert(route.node, route) {
            Some(original_route) => {
                println!("WARNING: replaced node {:?}", original_route.node);
            }
            None => {}
        }
    }

    (instructions.to_owned(), starts, map)
}

fn calc_steps(
    start: Route<Part2Node>,
    instrs: &[u8],
    map: &HashMap<Part2Node, Route<Part2Node>>,
) -> u64 {
    let mut steps: u64 = 0;
    let mut pos: usize = 0;

    let mut route = &start;
    while !route.node.is_end() {
        if pos >= instrs.len() {
            pos = 0;
        }

        let instruction = instrs[pos];

        route = match instruction {
            b'R' => map.get(&route.right).unwrap(),
            b'L' => map.get(&route.left).unwrap(),
            i => panic!("Invalid instruction found: {i}"),
        };

        steps += 1;
        pos += 1;
    }

    steps
}

// Reference: https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: Vec<u64>) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(nums.iter().cloned().skip(1).collect());
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    match b == 0 {
        true => a,
        false => gcd(b, a % b),
    }
}

fn solve(lines: Vec<String>) -> u64 {
    let (instructions, starts, map) = parse(lines);
    let instrs: &[u8] = instructions.as_bytes();

    let steps: Vec<_> = starts
        .iter()
        .map(|start| calc_steps(start.to_owned(), instrs, &map))
        .collect();

    lcm(steps)
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
        let rows = [
            "LR",
            "",
            "11A = (11B, XXX)",
            "11B = (XXX, 11Z)",
            "11Z = (11B, XXX)",
            "22A = (22B, XXX)",
            "22B = (22C, 22C)",
            "22C = (22Z, 22Z)",
            "22Z = (22B, 22B)",
            "XXX = (XXX, XXX)",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(solve(rows), 6);
    }
}
