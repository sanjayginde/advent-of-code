pub mod universe;

use std::fs::read_to_string;
use universe::{Galaxy, Universe};

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

fn parse(lines: Vec<String>) -> Universe {
    let mut universe: Universe = Universe::new();

    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut galaxy_id = 1;

    for (r, line) in lines.iter().enumerate() {
        let chars: Vec<_> = line.chars().collect();
        for (c, ch) in chars.iter().enumerate() {
            if *ch == '#' {
                universe.add_galaxy(Galaxy::new(galaxy_id, r, c));
                galaxy_id += 1;
            }
        }
        if line.chars().all(|ch| ch == '.') {
            universe.add_expansion_row(r);
        }

        matrix.push(line.chars().collect());
    }

    for (i, col) in transpose(matrix).iter().enumerate() {
        if col.clone().into_iter().all(|ch| ch == '.') {
            universe.add_expansion_col(i);
        }
    }

    universe
}

fn solve(universe: &Universe, expansion_factor: usize) -> usize {
    let mut total = 0;

    let galaxies = universe.galaxies();

    let mut galaxy = galaxies.get(0);
    let mut skip = 1;
    while galaxy.is_some() {
        let lhs = galaxy.unwrap();
        for rhs in galaxies.iter().skip(skip) {
            total += universe.distance(lhs, rhs, expansion_factor);
        }
        galaxy = galaxies.get(skip);
        skip += 1;
    }

    total
}

fn main() {
    let universe = parse(read_lines("input.txt"));

    println!("Solution for part 1 is {}", solve(&universe, 2));
    println!("Solution for part 2 is {}", solve(&universe, 1000000));
}

#[cfg(test)]
mod test {

    use rstest::rstest;

    use super::{parse, solve};

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

    #[rstest]
    #[case(2, 374)]
    #[case(10, 1030)]
    #[case(100, 8410)]
    fn solve_example(#[case] expand_factor: usize, #[case] expected: usize) {
        let universe = parse(EXAMPLE.map(String::from).to_vec());

        assert_eq!(solve(&universe, expand_factor), expected);
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
