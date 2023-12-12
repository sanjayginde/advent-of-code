use std::{env, fs::read_to_string};

#[derive(Debug, Eq, PartialEq)]
pub struct Galaxy {
    id: usize,
    row: usize,
    col: usize,
}

impl Galaxy {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn distance(&self, galaxy: &Galaxy) -> usize {
        let row_dff = galaxy.row().abs_diff(self.row());
        let col_diff = galaxy.col().abs_diff(self.col());

        row_dff + col_diff
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn expand(lines: Vec<String>) -> Vec<Vec<char>> {
    // Expand empty rows
    let mut expand_rows: Vec<Vec<char>> = Vec::new();
    for line in lines.iter() {
        if line.chars().all(|ch| ch == '.') {
            expand_rows.push(line.chars().collect());
        }

        expand_rows.push(line.chars().collect());
    }

    // Expand empty columns (via transpose)
    let mut expand_columns: Vec<Vec<char>> = Vec::new();
    for col in transpose(expand_rows).into_iter() {
        if col.clone().into_iter().all(|ch| ch == '.') {
            expand_columns.push(col.clone());
        }
        expand_columns.push(col);
    }

    transpose(expand_columns)
}

fn parse(lines: Vec<String>) -> Vec<Galaxy> {
    let mut result = Vec::new();

    let mut galaxy_id = 1;
    for (row, line) in expand(lines).into_iter().enumerate() {
        for (col, ch) in line.into_iter().enumerate() {
            // println!("char: {ch}");
            if ch == '#' {
                result.push(Galaxy {
                    id: galaxy_id,
                    row,
                    col,
                });
                galaxy_id += 1;
            }
        }
    }

    result
}

fn solve(lines: Vec<String>) -> usize {
    let mut total = 0;

    let galaxies = parse(lines);

    let mut galaxy = galaxies.get(0);
    let mut skip = 1;
    while galaxy.is_some() {
        let lhs = galaxy.unwrap();
        for rhs in galaxies.iter().skip(skip) {
            total += lhs.distance(rhs);
        }
        galaxy = galaxies.get(skip);
        skip += 1;
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
            solve(read_lines(&args[1].clone()))
        ),
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const EXAMPLE: [&str; 10] = [
        "...#......",
        ".......#..",
        "#.........",
        "..........",
        "......#...",
        ".#........",
        ".........#",
        "..........",
        ".......#..",
        "#...#.....",
    ];

    #[test]
    fn solve_example() {
        let rows = EXAMPLE.map(String::from).to_vec();

        assert_eq!(solve(rows), 374);
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
