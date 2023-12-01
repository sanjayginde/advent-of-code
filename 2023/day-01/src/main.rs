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
    use super::find_first_digit;
    use super::find_last_digit;
    use super::parse;

    #[test]
    fn test_parse() {
        assert_eq!(parse("dqc57tf1xmkdnll".to_string()), Ok(51));
        assert_eq!(parse("4beight8".to_string()), Ok(48));
        assert_eq!(parse("37rrsn".to_string()), Ok(37));
        assert_eq!(parse("fjdihy40".to_string()), Ok(40));
        assert_eq!(parse("fjdihy402947".to_string()), Ok(47));
    }

    #[test]
    fn test_all_numbers() {
        assert_eq!(parse("7874".to_string()), Ok(74));
        assert_eq!(parse("87".to_string()), Ok(87));
    }

    #[test]
    fn test_single_number() {
        assert_eq!(parse("jp3".to_string()), Ok(33));
        assert_eq!(parse("7".to_string()), Ok(77));
        assert_eq!(parse("4dkj".to_string()), Ok(44));
        assert_eq!(parse("djubh8dnsk".to_string()), Ok(88));
    }

    // #[test]
    // fn test_err() {
    //     assert!(parse("jslaskljdsf".to_string()).is_err());
    // }


    #[test]
    fn test_find_first_digit() {
        assert_eq!(find_first_digit("xtwone3four".to_string()), Some(2));
        assert_eq!(find_first_digit("x3twone3four".to_string()), Some(3));
    }

    #[test]
    fn test_find_last_digit() {
        assert_eq!(find_last_digit("8twone".to_string()), Some(1));
        assert_eq!(find_last_digit("8sevenine".to_string()), Some(9));
        assert_eq!(find_last_digit("8seven5ine".to_string()), Some(5));
        assert_eq!(find_last_digit("8sevenineihn".to_string()), Some(9));
    }
}
