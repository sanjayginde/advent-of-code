pub mod contraption;

use contraption::{Contraption, Direction, Trajectory};
use std::fs::read_to_string;

// Recursive solution to walk
fn walk(contraption: &mut Contraption, trajectory: Trajectory) {
    match contraption.tile_mut(trajectory) {
        None => return,
        Some(tile) => {
            if tile.energize(trajectory.dir()) {
                let next_tiles = tile.get_next(trajectory);
                for new_trajectory in next_tiles.into_iter() {
                    walk(contraption, new_trajectory);
                }
            }
        }
    }
}

fn solve_part1(mut contraption: Contraption) -> usize {
    let start = Trajectory::new(0, 0, Direction::East);

    walk(&mut contraption, start);

    contraption.total_energized()
}

fn solve_part2(mut contraption: Contraption) -> usize {
    let mut starts: Vec<Trajectory> = Vec::new();

    let (num_rows, num_cols) = contraption.size();
    let last_row = num_rows - 1;
    let last_col = num_cols - 1;

    (0..num_rows).for_each(|row| {
        starts.push(Trajectory::new(row, 0, Direction::East));
        starts.push(Trajectory::new(row, last_col, Direction::West));
    });

    (0..num_cols).for_each(|col| {
        starts.push(Trajectory::new(0, col, Direction::South));
        starts.push(Trajectory::new(last_row, col, Direction::North));
    });

    let mut result = 0;
    for start in starts.into_iter() {
        contraption.walk(start);
        let total = contraption.total_energized();
        if total > result {
            result = total;
        }

        contraption.reset()
    }

    result
}

fn main() {
    let contraption: Contraption = Contraption::from(read_lines("input.txt"));

    println!(
        "Solution for part 1 is {}",
        solve_part1(contraption.clone())
    );
    // contraption.reset();
    println!("Solution for part 2 is {}", solve_part2(contraption));
}

#[cfg(test)]
mod test {

    use super::{solve_part1, solve_part2, Contraption};

    const EXAMPLE: [&str; 10] = [
        ".|...\\....",
        "|.-.\\.....",
        ".....|-...",
        "........|.",
        "..........",
        ".........\\",
        "..../.\\\\..",
        ".-.-/..|..",
        ".|....-|.\\",
        "..//.|....",
    ];

    #[test]
    fn solve_example_part1() {
        let contraption: Contraption = Contraption::from(EXAMPLE.map(String::from).to_vec());
        assert_eq!(solve_part1(contraption), 46);
    }

    #[test]
    fn solve_example_part2() {
        let contraption: Contraption = Contraption::from(EXAMPLE.map(String::from).to_vec());
        assert_eq!(solve_part2(contraption), 51);
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
