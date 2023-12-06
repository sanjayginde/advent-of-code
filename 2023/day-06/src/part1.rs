use std::{env, fs::read_to_string};

#[derive(Debug)]
struct Race {
    elapsed: u32,
    distance: u32,
}

impl Race {
    pub fn num_ways(&self) -> u32 {
        let mut result: u32 = 0;
        let mut push_elapsed: u32 = 1;
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

fn parse_line(line: String) -> Vec<u32> {
    let splits: Vec<&str> = line.split(": ").collect();
    let values: Vec<_> = splits.get(1).unwrap().split_whitespace().collect();
    values.iter().map(|v| v.parse::<u32>().unwrap()).collect()
}

/// Returns a sum of all the numbers passed in
fn solve(lines: Vec<String>) -> u32 {
    let mut input = lines.into_iter();
    let times = parse_line(input.next().unwrap());
    let distances = parse_line(input.next().unwrap());

    let mut races: Vec<Race> = Vec::new();
    for (pos, time) in times.into_iter().enumerate() {
        races.push(Race {
            elapsed: time,
            distance: distances[pos],
        })
    }

    let num_ways: Vec<u32> = races
        .into_iter()
        .map(|race| {
            let num_ways = race.num_ways();
            println!("race: {race:?} [{num_ways}]");
            num_ways
        })
        .collect();

    num_ways.into_iter().product()
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

        assert_eq!(solve(rows), 288);
    }
}
