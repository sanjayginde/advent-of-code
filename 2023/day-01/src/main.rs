use std::{fs::read_to_string, env, num::ParseIntError};
use regex::Regex;

fn parse(line: String) -> Result<i32, ParseIntError> {
    let re = Regex::new(r"[\d]").unwrap();
    let nums: Vec<_> = re.find_iter(line.as_str()).collect();

    let first_str = nums.first().unwrap().as_str();
    let last_str = nums.last().unwrap().as_str();
    let result_str = format!("{first_str}{last_str}");

    // println!("{:?}", result_str);
    match result_str.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(e) => Err(e)
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
}