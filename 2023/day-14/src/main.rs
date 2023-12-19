use std::{collections::HashMap, fs::read_to_string};

pub type Dish = Vec<Vec<char>>;

fn parse(lines: Vec<String>) -> Dish {
    let mut dish: Dish = Vec::new();

    for line in lines {
        dish.push(line.chars().collect());
    }

    dish
}

fn dish_to_string(dish: &Dish) -> String {
    dish.iter().flat_map(|row| row).collect::<String>()
}

fn shift_left(chars: Vec<char>) -> Vec<char> {
    let len = chars.len();
    let mut result: Vec<char> = chars.into_iter().filter(|ch| ch == &'O').collect();

    result.resize(len, '.');

    result
}

fn shift_all_left(dish: Dish) -> Dish {
    let mut shifted: Dish = Vec::new();
    for row in dish {
        let mut shifted_row: Vec<char> = Vec::with_capacity(row.len());
        for section in row.split(|ch| ch == &'#') {
            shifted_row.append(&mut shift_left(section.to_vec()));
            if shifted_row.len() != row.len() {
                shifted_row.push('#');
            }
        }

        shifted.push(shifted_row)
    }

    shifted
}

fn tilt_east(dish: Dish) -> Dish {
    let mut reversed: Dish = Vec::with_capacity(dish.len());
    for row in dish {
        let mut rev_row = row.clone();
        rev_row.reverse();
        reversed.push(rev_row)
    }

    let shifted = shift_all_left(reversed);

    let mut unreversed = Vec::with_capacity(shifted.len());
    for row in shifted {
        let mut rev_row = row.clone();
        rev_row.reverse();
        unreversed.push(rev_row)
    }

    unreversed
}

fn tilt_west(dish: Dish) -> Dish {
    shift_all_left(dish)
}

fn tilt_south(dish: Dish) -> Dish {
    let mut reversed: Dish = Vec::with_capacity(dish.len());
    for row in transpose(dish) {
        let mut rev_row = row.clone();
        rev_row.reverse();
        reversed.push(rev_row)
    }

    let shifted = shift_all_left(reversed);
    let mut unreversed = Vec::with_capacity(shifted.len());
    for row in shifted {
        let mut rev_row = row.clone();
        rev_row.reverse();
        unreversed.push(rev_row)
    }

    transpose(unreversed)
}

fn tilt_north(dish: Dish) -> Dish {
    let transposed: Dish = transpose(dish);
    let shifted: Dish = shift_all_left(transposed);

    transpose(shifted)
}

fn calculate_load(dish: &Dish) -> usize {
    let mut total: usize = 0;
    let mut multiplier = dish.len();
    for row in dish {
        let rocks = row.into_iter().filter(|ch| ch == &&'O').count();
        total += rocks * multiplier;
        multiplier -= 1;
    }

    total
}
fn solve_part1(dish: Dish) -> usize {
    let tilted_dish = tilt_north(dish);

    calculate_load(&tilted_dish)
}

fn spin(dish: &Dish) -> Dish {
    tilt_east(tilt_south(tilt_west(tilt_north(dish.clone()))))
}

fn find_repeat(dish: Dish) -> Option<(usize, Dish)> {
    let mut cache: HashMap<String, Dish> = HashMap::new();
    let mut result = dish;

    for i in 0..1_000_000_000 {
        let key = dish_to_string(&result);

        match cache.get(&key) {
            Some(value) => {
                println!("HIT");
                result = value.clone();
                return Some((i, result));
            }
            None => {
                println!("MISS");
                result = spin(&result);
                cache.insert(key, result.clone());
            }
        }

        if i % 100000 == 0 {
            println!("Finished {i} spin cycles");
        }
    }

    None
}

fn solve_part2(dish: Dish) -> usize {
    match find_repeat(dish) {
        Some((iteration, mut result)) => {
            let cycles_left = 1_000_000_000 % iteration;

            for _i in 0..cycles_left {
                result = spin(&result);
            }
            return calculate_load(&result);
        }
        None => {
            panic!("Found no repeating cycle!")
        }
    }

    // calculate_load(&result)
}

fn main() {
    let dish: Dish = parse(read_lines("input.txt"));

    println!("Solution for part 1 is {}", solve_part1(dish.clone()));
    println!("Solution for part 2 is {}", solve_part2(dish));
}

#[cfg(test)]
mod test {

    use super::{parse, solve_part1, solve_part2, spin, Dish};

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
    fn solve_example_part1() {
        let dish: Dish = parse(EXAMPLE.map(String::from).to_vec());
        assert_eq!(solve_part1(dish), 136);
    }

    // #[test]
    // fn solve_example_part2() {
    //     let dish: Dish = parse(EXAMPLE.map(String::from).to_vec());
    //     assert_eq!(solve_part2(dish), 64);
    // }

    #[test]
    fn solve_example_part2_print() {
        let mut dish: Dish = parse(EXAMPLE.map(String::from).to_vec());

        for i in 0..4 {
            print_dish(i, dish.clone());
            dish = spin(&dish);
        }
    }

    fn print_dish(i: usize, dish: Dish) {
        println!("\n\nCycle {}", i);
        for row in dish {
            for ch in row {
                print!("{}", ch);
            }
            println!();
        }
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
