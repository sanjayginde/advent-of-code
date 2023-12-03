use std::{env, fs::read_to_string};

#[derive(Debug)]
pub struct Row {
    game_num: Option<u32>,
    max_red: u32,
    max_blue: u32,
    max_green: u32,
}

impl Row {
    pub fn new() -> Row {
        Row {
            game_num: None,
            max_red: 0,
            max_blue: 0,
            max_green: 0,
        }
    }

    pub fn is_possible(&self) -> bool {
        self.max_red <= 12 && self.max_green <= 13 && self.max_blue <= 14
    }

    pub fn power(&self) -> u32 {
        self.max_blue * self.max_green * self.max_red
    }
}

#[derive(Debug)]
struct Game {
    blue: Option<u32>,
    green: Option<u32>,
    red: Option<u32>,
}

// Game 5
fn parse_game_number(game: &str) -> u32 {
    let mut split: Vec<_> = game.split(" ").collect();
    
    return split.pop()
        .expect("format Game <num>")
        .parse::<u32>()
        .expect("expected game number");
}

// 8 green, 6 blue, 20 red
fn parse_game(game: &str) -> Game {
    let mut result = Game {
        blue: None,
        green: None,
        red: None,
    };

    let cube_colors = game.split(",").map(|e| e.trim());
    for cube_color in cube_colors {
        let cubes: Vec<_> = cube_color.split(" ").collect();
        let num = cubes
            .get(0)
            .expect("format <num> color")
            .parse::<u32>()
            .expect("expected cube number");
        let color = *cubes.get(1).expect("format num <color>");
        match color {
            "blue" => result.blue = Some(num),
            "green" => result.green = Some(num),
            "red" => result.red = Some(num),
            &_ => panic!("Invalid color {color}"),
        }
    }

    result
}

fn parse(line: String) -> Row {
    let mut row = Row::new();

    let mut game: Vec<_> = line.split(":").collect();


    let games: Vec<_> = game.pop()
        .expect("list of games")
        .trim()
        .split(";")
        .collect();

    for game in games {
        let parsed_game = parse_game(game);
        // println!("\t{game:?}");

        let blue = parsed_game.blue;
        if blue.is_some() && blue.unwrap() > row.max_blue {
            row.max_blue = blue.unwrap();
        }

        let green = parsed_game.green;
        if green.is_some() && green.unwrap() > row.max_green {
            row.max_green = green.unwrap();
        }

        let red = parsed_game.red;
        if red.is_some() && red.unwrap() > row.max_red {
            row.max_red = red.unwrap();
        }
    }

    row.game_num = Some(parse_game_number(
        game.pop().expect("expected game num split"),
    ));
    // println!("game num: {:?}", row.game_num);

    // println!("\trow: {row:?}");
    row
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn solve_part1(lines: Vec<String>) -> u32 {
    let mut total: u32 = 0;
    for line in lines.iter() {
        let row = parse(line.to_owned());
        if row.is_possible() {
            total += row.game_num.unwrap();
        }
    }
    total
}

fn solve_part2(lines: Vec<String>) -> u32 {
    let mut total: u32 = 0;
    for line in lines.iter() {
        let row = parse(line.to_owned());
        total += row.power();
    }
    total
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0..=1 => println!("Pass in filename to solve and part"),
        3 => println!(
            "Solution for part 2 for {} is {}",
            args[1].clone(),
            solve_part2(read_lines(&args[1].clone()))
        ),
        _ => println!(
            "Solution for part 1 for {} is {}",
            args[1].clone(),
            solve_part1(read_lines(&args[1].clone()))
        ),
    }
}

#[cfg(test)]
mod test {

    use super::solve_part1;
    use super::solve_part2;

    #[test]
    fn solve_example() {
        let rows = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(solve_part1(rows), 8);
    }

    #[test]
    fn solve_part2_example() {
        let rows = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(solve_part2(rows), 2286);
    }
}
