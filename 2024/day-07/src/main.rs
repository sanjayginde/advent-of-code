use std::fs::read_to_string;

#[derive(Debug)]
struct Equation {
    value: u64,
    numbers: Vec<u64>,
}

fn possible_solutions(numbers: &[u64]) -> Vec<u64> {
    match numbers.len() {
        0 => vec![],
        1 => vec![numbers[0]],
        2 => vec![numbers[0] + numbers[1], numbers[0] * numbers[1]],
        _ => {
            let first = numbers[0];
            let second = numbers[1];
            let rest = numbers[2..].to_vec();

            let mut added = vec![first + second];
            added.extend(&rest);

            let mut multiplied = vec![first * second];
            multiplied.extend(&rest);

            [possible_solutions(&added), possible_solutions(&multiplied)].concat()
        }
    }
}

impl Equation {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn solvable(&self) -> bool {
        let possible_solutions = possible_solutions(&self.numbers);
        for solution in possible_solutions {
            if solution == self.value {
                return true;
            }
        }
        false
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
    let equations = lines.iter().map(Equation::from).collect::<Vec<_>>();

    let mut result: u64 = 0;
    for equation in equations {
        if equation.solvable() {
            result += equation.value();
        }
    }

    result
}

fn part2(_lines: Vec<String>) -> usize {
    todo!()
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

    // #[test]
    fn _solve_example_part2() {
        assert_eq!(part2(EXAMPLE.map(String::from).to_vec()), 4);
    }
}
