use once_cell::sync::Lazy;
use regex::Regex;
use std::{env, fs::read_to_string, num::ParseIntError};

fn find_first_digit(line: String) -> Option<i32> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap());

    let mut result: Option<i32> = None;
    let mut current_word = "".to_string();

    for c in line.chars() {
        match c {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                result = Some(c.to_string().parse::<i32>().unwrap());
            }
            _ => {
                current_word.push(c);
                result = match RE.find(&current_word) {
                    Some(m) => match m.as_str() {
                        "one" => Some(1),
                        "two" => Some(2),
                        "three" => Some(3),
                        "four" => Some(4),
                        "five" => Some(5),
                        "six" => Some(6),
                        "seven" => Some(7),
                        "eight" => Some(8),
                        "nine" => Some(9),
                        &_ => None,
                    },
                    None => None,
                }
            }
        }

        if result.is_some() {
            break;
        }
    }

    return result;
}

fn find_last_digit(line: String) -> Option<i32> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap());

    let mut result: Option<i32> = None;
    let mut current_word = "".to_string();

    for c in line.chars().rev() {
        match c {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                result = Some(c.to_string().parse::<i32>().unwrap());
                current_word = "".to_string()
            }
            _ => {
                current_word.push(c);
                result = match RE.find(&current_word) {
                    Some(m) => match m.as_str() {
                        "eno" => Some(1),
                        "owt" => Some(2),
                        "eerht" => Some(3),
                        "ruof" => Some(4),
                        "evif" => Some(5),
                        "xis" => Some(6),
                        "neves" => Some(7),
                        "thgie" => Some(8),
                        "enin" => Some(9),
                        &_ => None,
                    },
                    None => None,
                }
            }
        }

        if result.is_some() {
            break;
        }
    }

    return result;
}

fn parse(line: String) -> Result<i32, ParseIntError> {
    let first = find_first_digit(line.clone()).unwrap();
    let last = find_last_digit(line).unwrap();
    
    let result_str = format!("{first}{last}");

    // println!("{:?}", result_str);
    match result_str.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(e) => Err(e),
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn solve(filename: String) -> i32 {
    let lines = read_lines(&filename);
    let mut total = 0;
    for (pos, line) in lines.iter().enumerate() {
        match parse(line.to_owned()) {
            Ok(num) => total += num,
            Err(_) => println!("Error parsing line {}: {}", pos + 1, line),
        }
    }
    total
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0..=1 => println!("Pass in filename to solve"),
        _ => println!(
            "Solution for {} is {}",
            args[1].clone(),
            solve(args[1].clone())
        ),
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::find_first_digit;
    use super::find_last_digit;
    use super::parse;

    #[rstest]
    #[case("dqc57tf1xmkdnll", 51)]
    #[case("4beight8", 48)]
    #[case("37rrsn", 37)]
    #[case("fjdihy40", 40)]
    #[case("fjdihy402947", 47)]
    #[case("7874", 74)]
    #[case("87", 87)]
    #[case("jp3", 33)]
    #[case("7", 77)]
    #[case("4dkj", 44)]
    #[case("djubh8dnsk", 88)]
    fn test_parse(#[case] input: &str, #[case] expected:i32) {
        assert_eq!(parse(input.to_string()).unwrap(), expected);
    }

    #[rstest]
    #[case("xtwone3four", 2)]
    #[case("x3twone3four", 3)]
    fn test_find_first_digit(#[case] input: &str, #[case] expected:i32) {
        assert_eq!(find_first_digit(input.to_string()).unwrap(), expected);
    }

    // #[test]
    // fn test_err() {
    //     assert!(parse("jslaskljdsf".to_string()).is_err());
    // }

    #[rstest]
    #[case("8twone", 1)]
    #[case("8sevenine", 9)]
    #[case("8seven5ine", 5)]
    #[case("8sevenineihn", 9)]
    fn test_find_last_digit(#[case] input: &str, #[case] expected:i32) {
        assert_eq!(find_last_digit(input.to_string()).unwrap(), expected);
    }
}
