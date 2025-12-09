use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
struct Rotation {
    ticks: i32,
    full_rotations: u32,
}

impl From<&String> for Rotation {
    fn from(s: &String) -> Self {
        let direction: Direction = s.get(0..1).unwrap().parse::<Direction>().unwrap();
        let mut ticks = s.get(1..).unwrap().parse::<i32>().unwrap();

        if direction == Direction::Left {
            ticks *= -1;
        }

        Self {
            ticks: ticks % 100,
            full_rotations: (ticks.abs() as u32) / 100,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(format!("Invalid direction: {}", s)),
        }
    }
}

fn part1(lines: Vec<String>) -> u32 {
    let rotations = lines.iter().map(Rotation::from).collect::<Vec<_>>();

    let mut count = 0;
    rotations.iter().fold(50, |mut pos, r| {
        pos += r.ticks;
        if pos < 0 {
            pos = 100 + pos;
        } else if pos > 99 {
            pos = pos - 100;
        }

        if pos == 0 {
            count += 1;
        }

        pos
    });

    count
}

fn part2(lines: Vec<String>) -> u32 {
    let rotations = lines.iter().map(Rotation::from).collect::<Vec<_>>();

    let mut count = 0;
    rotations.iter().fold(50, |mut pos, r| {
        count += r.full_rotations;

        let mut new_pos = pos + r.ticks;
        if new_pos < 0 {
            new_pos = 100 + new_pos;
            if pos != 0 && new_pos != 0 && new_pos > pos {
                count += 1;
            }
            pos = new_pos;
        } else if new_pos > 99 {
            new_pos = new_pos - 100;
            if pos != 0 && new_pos != 0 && new_pos < pos {
                count += 1;
            }
            pos = new_pos;
        } else {
            pos = new_pos;
        }

        if pos == 0 {
            count += 1;
        }

        pos
    });

    count
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

    const EXAMPLE: [&str; 10] = [
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];

    #[test]
    fn solve_example_part1() {
        assert_eq!(part1(EXAMPLE.map(String::from).to_vec()), 3);
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(part2(EXAMPLE.map(String::from).to_vec()), 6);
    }
}
