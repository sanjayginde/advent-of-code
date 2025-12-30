use itertools::Itertools;
use regex::Regex;
use rust_aoc_utils::read_lines_from_file;

#[derive(Debug)]
struct LightBits(u64);

#[derive(Debug)]
struct ButtonMask(u64);

#[derive(Debug)]
struct Machine {
    light_bits: LightBits,
    button_masks: Vec<ButtonMask>,
}

impl From<String> for Machine {
    fn from(s: String) -> Self {
        let lights_regex = Regex::new(r"\[[.#]+\]").unwrap();
        let buttons_regex = Regex::new(r"\([\d,]+\)").unwrap();
        let _joltage_regex = Regex::new(r"\{[\d,]+\}").unwrap();

        let light_bits = match lights_regex.captures(&s) {
            None => {
                panic!("Could not find lights in '{}'", s)
            }
            Some(capture) => {
                let m = &capture[0];
                let light_chars = &m[1..m.len() - 1];
                let mut lights_bits = 0u64;
                for (i, ch) in light_chars.chars().enumerate() {
                    match ch {
                        '.' => {}
                        '#' => lights_bits |= 1 << i,
                        _ => panic!("Invalid light code '{}'", ch),
                    }
                }
                LightBits(lights_bits)
            }
        };

        let button_masks: Vec<ButtonMask> = buttons_regex
            .captures_iter(&s)
            .map(|captures| {
                let capture = &captures[0];
                let toggles: Vec<usize> = capture[1..capture.len() - 1]
                    .split(",")
                    .filter_map(|n| n.parse().ok())
                    .collect();

                let mut button_mask = 0u64;
                for pos in toggles {
                    button_mask |= 1 << pos;
                }

                ButtonMask(button_mask)
            })
            .collect();

        Machine {
            light_bits,
            button_masks,
        }
    }
}

const MAX_LENGTH: usize = 10;
fn min_button_presses(machine: &Machine) -> Option<usize> {
    println!("Processing machine {:b}", machine.light_bits.0);

    let mut i = 1;

    while i < MAX_LENGTH {
        let mask_variations = variations_of_length(i, &machine.button_masks);

        for masks in mask_variations {
            let mut current = 0;
            for mask in masks {
                current ^= mask.0;
            }
            if current == machine.light_bits.0 {
                return Some(i);
            }
        }

        i += 1;
    }

    None
}

fn variations_of_length<T>(length: usize, items: &[T]) -> impl Iterator<Item = Vec<&T>> {
    std::iter::repeat_n(items.iter(), length).multi_cartesian_product()
}

fn part1(lines: Vec<String>) -> usize {
    let machines: Vec<Machine> = lines.into_iter().map(Machine::from).collect();

    machines
        .iter()
        .map(min_button_presses)
        .fold(0, |acc, presses| match presses {
            Some(num) => acc + num,
            None => {
                println!("Couldn't find solution for a machine");
                0
            }
        })
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
        assert_eq!(part1(EXAMPLE.map(String::from).to_vec()), 7);
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(part2(EXAMPLE.map(String::from).to_vec()), 0);
    }
}
