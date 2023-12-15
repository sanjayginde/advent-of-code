use std::fs::read_to_string;

pub type Dish = Vec<Vec<char>>;

fn parse(lines: Vec<String>) -> Dish {
    let mut dish: Dish = Vec::new();

    for line in lines {
        dish.push(line.chars().collect());
    }

    dish
}

fn shift_left(chars: Vec<char>) -> Vec<char> {
    let len = chars.len();
    let mut result: Vec<char> = chars.into_iter().filter(|ch| ch == &'O').collect();

    let diff = len - result.len();
    for _ in 0..diff {
        result.push('.');
    }

    result
}

fn tilt_north(dish: Dish) -> Dish {
    let transposed: Dish = transpose(dish);

    let mut tilted: Dish = Vec::new();
    for row in transposed {
        let mut titled_row: Vec<char> = Vec::with_capacity(row.len());
        for section in row.split(|ch| ch == &'#') {
            titled_row.append(&mut shift_left(section.to_vec()));
            if titled_row.len() != row.len() {
                titled_row.push('#');
            }
        }

        tilted.push(titled_row)
    }

    transpose(tilted)
}

fn solve_part1(dish: Dish) -> usize {
    let mut total: usize = 0;

    let tilted_dish = tilt_north(dish);

    let mut multiplier = tilted_dish.len();
    for row in tilted_dish {
        let rocks = row.into_iter().filter(|ch| ch == &'O').count();
        total += rocks * multiplier;
        multiplier -= 1;
    }

    total
}

fn main() {
    let dish: Dish = parse(read_lines("input.txt"));

    println!("Solution for part 1 is {}", solve_part1(dish));
}

#[cfg(test)]
mod test {

    use super::{parse, solve_part1, Dish};

    const EXAMPLE: [&str; 10] = [
        "O....#....",
        "O.OO#....#",
        ".....##...",
        "OO.#O....O",
        ".O.....O#.",
        "O.#..O.#.#",
        "..O..#O..O",
        ".......O..",
        "#....###..",
        "#OO..#....",
    ];

    #[test]
    fn solve_example() {
        let dish: Dish = parse(EXAMPLE.map(String::from).to_vec());
        assert_eq!(solve_part1(dish), 136);
    }
}

// Utilities

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
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
