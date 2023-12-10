pub mod maps;
use maps::{Node, Part1Node, Route, START_NODE_PART_1};
use std::{collections::HashMap, env, fs::read_to_string};

fn parse(lines: Vec<String>) -> (String, HashMap<Part1Node, Route<Part1Node>>) {
    let instructions = lines.get(0).unwrap().trim();
    let mut map: HashMap<Part1Node, Route<Part1Node>> = HashMap::new();

    for line in lines.iter().skip(2) {
        let route: Route<Part1Node> = Route::from(line);
        match map.insert(route.node.clone(), route) {
            Some(original_route) => {
                println!("WARNING: replaced node {:?}", original_route.node);
            }
            None => {}
        }
    }
    (instructions.to_owned(), map)
}

fn solve(lines: Vec<String>) -> u32 {
    let mut steps: u32 = 0;

    let (instructions, map) = parse(lines);
    let chars = instructions.as_bytes();

    let mut pos: usize = 0;

    let mut route = map.get(&START_NODE_PART_1).unwrap();
    while !route.node.is_end() {
        if pos >= instructions.len() {
            pos = 0;
        }

        route = match chars[pos] {
            b'R' => map.get(&route.right).unwrap(),
            b'L' => map.get(&route.left).unwrap(),
            i => panic!("Invalid instruction found: {i}"),
        };

        steps += 1;
        pos += 1;
    }

    steps
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
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(solve(rows), 2);
    }

    #[test]
    fn solve_example2() {
        let rows = [
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(solve(rows), 6);
    }
}
