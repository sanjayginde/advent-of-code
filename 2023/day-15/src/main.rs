use std::fs::read_to_string;

fn parse(lines: Vec<String>) -> Vec<String> {
    match lines.get(0) {
        Some(line) => {
            let steps: Vec<&str> = line.split(",").collect();
            steps.into_iter().map(String::from).collect()
        }
        None => {
            panic!("No input data")
        }
    }
}

fn hash(step: String) -> usize {
    let mut result: usize = 0;
    for ch in step.chars() {
        let code = ch as usize;

        // Increase the current value by the ASCII code you just determined.
        result += code;

        // Set the current value to itself multiplied by 17.
        result *= 17;

        // Set the current value to the remainder of dividing itself by 256.
        result %= 256;
    }

    result
}

fn solve_part1(steps: Vec<String>) -> usize {
    let mut total: usize = 0;

    for step in steps {
        total += hash(step);
    }

    total
}

fn main() {
    let steps: Vec<String> = parse(read_lines("input.txt"));

    println!("Solution for part 1 is {}", solve_part1(steps));
}

#[cfg(test)]
mod test {

    use super::{parse, solve_part1};

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn solve_example() {
        let steps: Vec<String> = parse(vec![EXAMPLE.to_string()]);
        assert_eq!(solve_part1(steps), 1320);
    }
}

// Utilities

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
