use std::fs::read_to_string;

fn part1(lines: Vec<String>) -> usize {
    lines.into_iter().fold(0, |acc, line| {
        let first_digit_search: Vec<_> =
            line.split_at(line.len() - 1).0.chars().collect::<Vec<_>>();

        let mut offset = 0;
        let mut first_digit = 0;
        for (i, char) in first_digit_search.into_iter().enumerate() {
            let possible = char.to_digit(10).unwrap();
            if possible > first_digit {
                offset = i;
                first_digit = possible;
            }
        }

        let second_digit_search: Vec<_> = line.split_at(offset + 1).1.chars().collect::<Vec<_>>();

        let mut second_digit = 0;
        for char in second_digit_search.into_iter() {
            let possible = char.to_digit(10).unwrap();
            if possible > second_digit {
                second_digit = possible;
            }
        }

        let joltage = format!("{}{}", first_digit, second_digit)
            .parse::<usize>()
            .unwrap();

        acc + joltage
    })
}

fn _part2(_lines: Vec<String>) -> u32 {
    todo!()
}

fn main() {
    println!("Solution for part 1 is {}", part1(read_lines("input.txt")));
    // println!("Solution for part 2 is {}", part2(read_lines("input.txt")));
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
    // use super::part2;

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

    // #[test]
    // fn solve_example_part2() {
    //     assert_eq!(part2(EXAMPLE.map(String::from).to_vec()), 6);
    // }
}
