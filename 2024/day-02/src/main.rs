use std::fs::read_to_string;

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Ascending,
    Descending,
}

fn get_direction(lhs: i64, rhs: i64) -> Direction {
    match rhs > lhs {
        true => Direction::Ascending,
        false => Direction::Descending,
    }
}

fn is_safe(levels: &[i64]) -> bool {
    let initial_direction: Direction = get_direction(levels[0], levels[1]);

    let mut current = levels[0];
    for (i, &next) in levels.iter().enumerate().skip(1) {
        let diff = (next - current).unsigned_abs();
        let dir = get_direction(current, next);
        match diff {
            1..=3 => match dir == initial_direction {
                false => {
                    println!("direction mismatch at index {}", i);
                    return false;
                }
                true => {}
            },
            d => {
                println!("{} difference at index {}", d, i);
                return false;
            }
        }
        current = next
    }
    true
}

#[derive(Debug)]
struct Report {
    levels: Vec<i64>,
}

impl Report {
    fn safe(&self) -> bool {
        is_safe(&self.levels)
    }

    fn safe_with_dampener(&self) -> bool {
        if is_safe(&self.levels) {
            return true;
        }

        for i in 0..self.levels.len() {
            let mut temp = self.levels.clone();
            temp.remove(i);
            if is_safe(&temp) {
                return true;
            }
        }

        false
    }
}

impl From<&String> for Report {
    fn from(s: &String) -> Self {
        let levels = s
            .split_whitespace() // Split the string by whitespace
            .filter_map(|s| s.parse::<i64>().ok()) // Parse each part into i64, ignoring errors
            .collect();

        Self { levels }
    }
}

fn part1(lines: Vec<String>) -> usize {
    let reports = lines.iter().map(Report::from).collect::<Vec<_>>();

    println!("Num reports: {:?}", reports.len());

    let mut result = 0;
    for report in reports {
        let is_safe = report.safe();
        if is_safe {
            result += 1;
        }

        println!("\t{:?}: {:?}", report.levels, is_safe);
    }

    result
}

fn part2(lines: Vec<String>) -> usize {
    let reports = lines.iter().map(Report::from).collect::<Vec<_>>();

    println!("Num reports: {:?}", reports.len());

    let mut result = 0;
    for report in reports {
        let is_safe = report.safe() || report.safe_with_dampener();
        if is_safe {
            result += 1;
        }

        println!("\t{:?}: {:?}", report.levels, is_safe);
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

    const EXAMPLE: [&str; 6] = [
        "7 6 4 2 1",
        "1 2 7 8 9",
        "9 7 6 2 1",
        "1 3 2 4 5",
        "8 6 4 2 2",
        "1 3 6 7 9",
    ];

    #[test]
    fn solve_example_part1() {
        assert_eq!(part1(EXAMPLE.map(String::from).to_vec()), 2);
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(part2(EXAMPLE.map(String::from).to_vec()), 4);
    }
}
