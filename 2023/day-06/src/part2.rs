use std::{env, fs::read_to_string};

#[derive(Debug)]
struct Race {
    elapsed: u64,
    distance: u64,
}

impl Race {
    pub fn num_ways(&self) -> u64 {
        let mut result: u64 = 0;
        let mut push_elapsed: u64 = 1;
        while push_elapsed < self.elapsed {
            let travel_elapsed = self.elapsed - push_elapsed;
            let travel_distance = travel_elapsed * push_elapsed; // push elapsed is equal to the rate, since it's 1mm/ms per ms pushed
            if travel_distance > self.distance {
                result += 1;
            }

            push_elapsed += 1;
        }

        result
    }
}

fn parse_line(line: String) -> u64 {
    let splits: Vec<&str> = line.split(": ").collect();
    let values: Vec<_> = splits.get(1).unwrap().split_whitespace().collect();

    values.concat().parse::<u64>().unwrap()
}

/// Returns a sum of all the numbers passed in
fn solve(lines: Vec<String>) -> u64 {
    let mut input = lines.into_iter();

    let race = Race {
        elapsed: parse_line(input.next().unwrap()),
        distance: parse_line(input.next().unwrap()),
    };

    race.num_ways()
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0..=1 => println!("Pass in filename to solve"),
        _ => println!(
            "Solution for {} is {}",
            args[1].clone(),
            solve(read_lines(&args[1].clone()))
        ),
    }
}

#[cfg(test)]
mod test {

    use super::solve;

    #[test]
    fn solve_example() {
        let rows = ["Time:      7  15   30", "Distance:  9  40  200"]
            .map(String::from)
            .to_vec();

        assert_eq!(solve(rows), 71503);
    }
}
