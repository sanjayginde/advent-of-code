use once_cell::sync::Lazy;
use regex::Regex;
use std::fs::read_to_string;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(mul\((?<lhs>[\d]+),(?<rhs>[\d]+)\))").unwrap());

fn part1(lines: Vec<String>) -> u64 {
    println!("lines {:?}", lines.len());
    let haystack = lines.join("\n");

    let mults = RE
        .captures_iter(&haystack)
        .map(|capture| {
            // println!("lhs: {:?}", capture.name("lhs").unwrap().as_str());
            // println!("rhs: {:?}", capture.name("rhs").unwrap().as_str());
            let lhs = capture.get(2).unwrap().as_str().parse::<u64>().unwrap();
            let rhs = capture.get(3).unwrap().as_str().parse::<u64>().unwrap();

            lhs * rhs
        })
        .collect::<Vec<u64>>();

    println!("found {:?}", mults.len());
    println!("mults {:?}", mults);

    mults.iter().sum()
}

fn main() {
    println!("Solution for part 1 is {}", part1(read_lines("input.txt")));
}

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

    const EXAMPLE: [&str; 1] =
        ["mul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"];

    #[test]
    fn solve_example_part1() {
        assert_eq!(part1(EXAMPLE.map(String::from).to_vec()), 161);
    }
}
