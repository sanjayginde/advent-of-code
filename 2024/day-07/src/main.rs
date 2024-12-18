use std::fs::read_to_string;

#[derive(Debug)]
struct Equation {
    value: u64,
    numbers: Vec<u64>,
}

fn possible_solutions(numbers: &[u64], support_concatenations: bool) -> Vec<u64> {
    match numbers.len() {
        0 => vec![],
        1 => vec![numbers[0]],
        2 => {
            let first = numbers[0];
            let second = numbers[1];

            let mut poss_solutions = vec![first + second, first * second];

            if support_concatenations {
                poss_solutions.push(
                    format!("{}{}", numbers[0], numbers[1])
                        .parse::<u64>()
                        .unwrap(),
                );
            }

            poss_solutions
        }
        _ => {
            let first = numbers[0];
            let second = numbers[1];
            let rest = numbers[2..].to_vec();

            let mut added = vec![first + second];
            added.extend(&rest);

            let mut multiplied = vec![first * second];
            multiplied.extend(&rest);

            let mut poss_solutions = [
                possible_solutions(&added, support_concatenations),
                possible_solutions(&multiplied, support_concatenations),
            ]
            .to_vec();

            if support_concatenations {
                let mut concatentated =
                    vec![format!("{}{}", first, second).parse::<u64>().unwrap()];
                concatentated.extend(&rest);
                poss_solutions.push(possible_solutions(&concatentated, support_concatenations));
            }

            poss_solutions.concat()
        }
    }
}

impl Equation {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn solvable(&self, support_concatenations: bool) -> bool {
        possible_solutions(&self.numbers, support_concatenations)
            .iter()
            .any(|solution| solution == &self.value)
    }
}

impl From<&String> for Equation {
    fn from(s: &String) -> Self {
        let split = s.split(": ").collect::<Vec<_>>();
        let value = split[0].parse::<u64>().unwrap();
        let numbers = split[1]
            .split(" ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        Self { value, numbers }
    }
}

fn part1(lines: Vec<String>) -> u64 {
    let mut result: u64 = 0;

    let equations = lines.iter().map(Equation::from).collect::<Vec<_>>();
    for equation in equations {
        if equation.solvable(false) {
            result += equation.value();
        }
    }

    result
}

fn part2(lines: Vec<String>) -> u64 {
    let mut result: u64 = 0;

    let equations = lines.iter().map(Equation::from).collect::<Vec<_>>();
    for equation in equations {
        if equation.solvable(true) {
            result += equation.value();
        }
    }

    result
}

fn main() {
    println!("Solution for part 1 is {}", part1(read_lines("input.txt")));
    println!("Solution for part 2 is {}", part2(read_lines("input.txt")));
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
    use super::part2;

    const EXAMPLE: [&str; 9] = [
        "190: 10 19",
        "3267: 81 40 27",
        "83: 17 5",
        "156: 15 6",
        "7290: 6 8 6 15",
        "161011: 16 10 13",
        "192: 17 8 14",
        "21037: 9 7 18 13",
        "292: 11 6 16 20",
    ];

    #[test]
    fn solve_example_part1() {
        assert_eq!(part1(EXAMPLE.map(String::from).to_vec()), 3749);
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(part2(EXAMPLE.map(String::from).to_vec()), 11387);
    }
}
