use std::fs::read_to_string;

fn part1(lines: Vec<String>) -> usize {
    lines
        .into_iter()
        .fold(0, |acc, line| acc + find_joltage(&line, 2))
}

fn part2(lines: Vec<String>) -> usize {
    lines
        .into_iter()
        .fold(0, |acc, line| acc + find_joltage(&line, 12))
}

#[derive(Debug)]
struct Digit {
    position: usize,
    value: String,
}

fn find_joltage(line: &str, num_digits: usize) -> usize {
    let mut joltage = "".to_string();
    let mut curr_digit: Option<Digit> = None;

    for place in (0..num_digits).rev() {
        let offset = curr_digit.map_or(0, |d| d.position + 1);

        let digit = find_largest_digit(line, offset, place);
        joltage.push_str(&digit.value);

        curr_digit = Some(digit);
    }

    joltage.parse::<usize>().unwrap()
}

fn find_largest_digit(line: &str, offset: usize, cutoff: usize) -> Digit {
    let mut position = 0;
    let mut value = 0;

    let search_space = &line[offset..(line.len() - cutoff)];

    for (i, char) in search_space
        .chars()
        .collect::<Vec<_>>()
        .into_iter()
        .enumerate()
    {
        let possible = char.to_digit(10).unwrap() as usize;
        if possible > value {
            position = i;
            value = possible;
        }
    }

    Digit {
        position: position + offset,
        value: value.to_string(),
    }
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

    const EXAMPLE: [&str; 4] = [
        "987654321111111",
        "811111111111119",
        "234234234234278",
        "818181911112111",
    ];

    #[test]
    fn solve_example_part1() {
        assert_eq!(part1(EXAMPLE.map(String::from).to_vec()), 357);
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(part2(EXAMPLE.map(String::from).to_vec()), 3121910778619);
    }
}
