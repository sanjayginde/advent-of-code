use itertools::Itertools;
use regex::Regex;
use rust_aoc_utils::read_lines_from_file;

#[derive(Debug)]
enum Light {
    On,
    Off,
}

#[derive(Debug)]
struct Button {
    toggles: Vec<usize>,
}

#[derive(Debug)]
struct Machine {
    lights: Vec<Light>,
    buttons: Vec<Button>,
}

impl From<String> for Machine {
    fn from(s: String) -> Self {
        let lights_regex = Regex::new(r"\[[.#]+\]").unwrap();
        let buttons_regex = Regex::new(r"\([\d,]+\)").unwrap();
        let _joltage_regex = Regex::new(r"\{[\d,]+\}").unwrap();

        let lights = match lights_regex.captures(&s) {
            None => {
                panic!("Could not find lights in '{}'", s)
            }
            Some(capture) => {
                let m = &capture[0];
                m[1..m.len() - 1]
                    .chars()
                    .map(|ch| match ch {
                        '.' => Light::Off,
                        '#' => Light::On,
                        _ => panic!("Invalid light code '{}'", ch),
                    })
                    .collect::<Vec<_>>()
            }
        };

        let buttons: Vec<Button> = buttons_regex
            .captures_iter(&s)
            .map(|captures| {
                let capture = &captures[0];
                let toggles: Vec<usize> = capture[1..capture.len() - 1]
                    .split(",")
                    .filter_map(|n| n.parse().ok())
                    .collect();
                Button { toggles }
            })
            .collect();

        Machine { lights, buttons }
    }
}

pub fn min_button_presses(machine: &Machine) -> usize {
    let result = 0;

    result
}

fn part1(lines: Vec<String>) -> usize {
    let machines: Vec<Machine> = lines.into_iter().map(Machine::from).collect();
    // println!("{:?}", machines);

    machines.iter().map(min_button_presses).sum()
}

fn part2(_lines: Vec<String>) -> usize {
    0
}

fn main() {
    println!("Part 1: {}", part1(read_lines_from_file("input.txt")));
    println!("Part 2: {}", part2(read_lines_from_file("input.txt")));
}

// Utilities

#[cfg(test)]
mod test {
    use super::part1;
    use super::part2;

    const EXAMPLE: [&str; 3] = [
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    ];

    #[test]
    fn solve_example_part1() {
        assert_eq!(part1(EXAMPLE.map(String::from).to_vec()), 50);
    }

    #[test]
    fn _solve_example_part2() {
        assert_eq!(part2(EXAMPLE.map(String::from).to_vec()), 14);
    }
}
