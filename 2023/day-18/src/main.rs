pub mod dig_plan;

use std::fs::read_to_string;

use dig_plan::DigPlan;

fn main() {
    println!(
        "Solution for part 1 is {}",
        DigPlan::from_part1(read_lines("input.txt")).area()
    );

    println!(
        "Solution for part 2 is {}",
        DigPlan::from_part2(read_lines("input.txt")).area()
    );
}

#[cfg(test)]
mod test {

    use super::DigPlan;

    const EXAMPLE: [&str; 14] = [
        "R 6 (#70c710)",
        "D 5 (#0dc571)",
        "L 2 (#5713f0)",
        "D 2 (#d2c081)",
        "R 2 (#59c680)",
        "D 2 (#411b91)",
        "L 5 (#8ceee2)",
        "U 2 (#caa173)",
        "L 1 (#1b58a2)",
        "U 2 (#caa171)",
        "R 2 (#7807d2)",
        "U 3 (#a77fa3)",
        "L 2 (#015232)",
        "U 2 (#7a21e3)",
    ];

    #[test]
    fn solve_example_part1() {
        let plan: DigPlan = DigPlan::from_part1(EXAMPLE.map(String::from).to_vec());
        assert_eq!(plan.area(), 62);
    }

    #[test]
    fn solve_example_part2() {
        let plan: DigPlan = DigPlan::from_part2(EXAMPLE.map(String::from).to_vec());
        assert_eq!(plan.area(), 952408144115);
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

fn _transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
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
