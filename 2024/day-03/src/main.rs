use once_cell::sync::Lazy;
use regex::Regex;
use std::fs::read_to_string;

static MULTS_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(mul\((?<lhs>[\d]+),(?<rhs>[\d]+)\))").unwrap());

fn find_mults(input: &str, do_mults: bool) -> Vec<u64> {
    if !do_mults {
        return vec![];
    }

    MULTS_RE
        .captures_iter(input)
        .map(|capture| {
            let lhs = capture.get(2).unwrap().as_str().parse::<u64>().unwrap();
            let rhs = capture.get(3).unwrap().as_str().parse::<u64>().unwrap();

            lhs * rhs
        })
        .collect::<Vec<u64>>()
}

fn part1(input: String) -> u64 {
    find_mults(&input, true).iter().sum()
}

fn part2(input: String) -> u64 {
    let mut result = 0;
    let mut do_mults = true;

    let buffer: &mut String = &mut String::new();
    for ch in input.chars() {
        buffer.push(ch);
        if buffer.ends_with("do()") {
            result += find_mults(buffer, do_mults).iter().sum::<u64>();
            do_mults = true;
            buffer.clear();
        } else if buffer.ends_with("don't()") {
            result += find_mults(buffer, do_mults).iter().sum::<u64>();
            do_mults = false;
            buffer.clear();
        }
    }

    result += find_mults(buffer, do_mults).iter().sum::<u64>();

    result
}

fn main() {
    println!("Solution for part 1 is {}", part1(read_input("input.txt")));
    println!("Solution for part 2 is {}", part2(read_input("input.txt")));
}

fn read_input(filename: &str) -> String {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod test {
    use super::part1;
    use super::part2;

    const EXAMPLE_PART_1: &str =
        "mul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn solve_example_part1() {
        assert_eq!(part1(EXAMPLE_PART_1.to_string()), 161);
    }

    const EXAMPLE_PART_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn solve_example_part2() {
        assert_eq!(part2(EXAMPLE_PART_2.to_string()), 48);
    }
}
