use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Row {
    first: i64,
    second: i64,
}

impl From<&String> for Row {
    fn from(s: &String) -> Self {
        let mut parts = s.split_whitespace();
        let first = parts.next().unwrap().parse::<i64>().unwrap();
        let second = parts.next().unwrap().parse::<i64>().unwrap();
        Self { first, second }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Lists {
    first: Vec<i64>,
    second: Vec<i64>,
}

impl From<Vec<Row>> for Lists {
    fn from(rows: Vec<Row>) -> Self {
        let mut first = vec![];
        let mut second = vec![];
        for row in rows {
            first.push(row.first);
            second.push(row.second);
        }

        first.sort();
        second.sort();

        Self { first, second }
    }
}

impl Iterator for Lists {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.first.is_empty() || self.second.is_empty() {
            return None;
        }

        let first = self.first.remove(0);
        let second = self.second.remove(0);

        Some((first, second))
    }
}

fn part1(lines: Vec<String>) -> usize {
    let lists: Lists = Lists::from(lines.iter().map(Row::from).collect::<Vec<_>>());

    lists.into_iter().fold(0usize, |acc, (first, second)| {
        let diff: i64 = second - first;
        acc + diff.abs() as usize
    })
}

fn part2(_lines: Vec<String>) -> i32 {
    todo!();
}

fn main() {
    println!("Solution for part 1 is {}", part1(read_lines("input.txt")));
    println!("Solution for part 2 is {}", part2(read_lines("input.txt")));
}

// Utilities

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

    const EXAMPLE: [&str; 6] = ["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"];

    #[test]
    fn solve_example_part1() {
        assert_eq!(part1(EXAMPLE.map(String::from).to_vec()), 11);
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(part2(EXAMPLE.map(String::from).to_vec()), 31);
    }
}
