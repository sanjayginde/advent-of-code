use std::fmt::Display;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Coordinate {
    pub row: usize,
    pub col: usize,
}

impl Coordinate {
    pub fn new(row: usize, col: usize) -> Self {
        Coordinate { row, col }
    }
    /// Get all 8 adjacent coordinates (including diagonals) that are within bounds
    pub fn adjacent(&self, max_row: usize, max_col: usize) -> Vec<Coordinate> {
        let directions = [
            // above
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            // sides
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        directions
            .iter()
            .filter_map(|&(dr, dc)| {
                let new_row = self.row as isize + dr;
                let new_col = self.col as isize + dc;

                if new_row >= 0 && new_col >= 0 {
                    let r = new_row as usize;
                    let c = new_col as usize;
                    if r < max_row && c < max_col {
                        Some(Coordinate::new(r, c))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn check_adjacent<T, F>(grid: &[Vec<T>], coordinate: Coordinate, operation: F) -> usize
where
    F: Fn(&T) -> bool + Copy,
{
    let directions = [
        // above
        (-1, -1),
        (-1, 0),
        (-1, 1),
        // sides
        (0, -1),
        (0, 1),
        // below
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    directions
        .iter()
        .filter_map(|&(dr, dc)| {
            let new_row = coordinate.row as isize + dr;
            let new_col = coordinate.col as isize + dc;

            if new_row >= 0 && new_col >= 0 {
                grid.get(new_row as usize)?.get(new_col as usize)
            } else {
                None
            }
        })
        .filter(|cell| operation(cell))
        .count()
}

pub fn parse_to_chars(lines: &[String]) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = vec![];

    for line in lines.iter() {
        let row = line.chars().collect::<Vec<_>>();
        matrix.push(row);
    }

    matrix
}

pub fn parse<T, F>(lines: &[String], parse: F) -> Vec<Vec<T>>
where
    F: Fn(char) -> T,
    F: Copy,
{
    let mut matrix: Vec<Vec<T>> = vec![];

    for line in lines.iter() {
        let row = line.chars().map(parse).collect::<Vec<_>>();
        matrix.push(row);
    }

    matrix
}

pub fn parse_whitespaced<T, F>(lines: &[String], parse: F) -> Vec<Vec<T>>
where
    F: Fn(&str) -> T,
    F: Copy,
{
    let mut matrix: Vec<Vec<T>> = vec![];

    for line in lines.iter() {
        let items = line.split_ascii_whitespace().collect::<Vec<_>>();
        matrix.push(items.into_iter().map(parse).collect::<Vec<_>>());
    }

    matrix
}

pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
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

pub fn print<T: Display>(grid: &[Vec<T>]) {
    for row in grid.iter() {
        print!("[");
        for col in row.iter() {
            print!("{}", col);
        }
        println!("]");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_creation() {
        let coord = Coordinate::new(5, 10);
        assert_eq!(coord.row, 5);
        assert_eq!(coord.col, 10);
    }

    #[test]
    fn test_adjacent() {
        let coord = Coordinate::new(1, 1);
        let adjacent = coord.adjacent(3, 3);
        assert_eq!(adjacent.len(), 8);
        assert!(adjacent.contains(&Coordinate::new(0, 0)));
        assert!(adjacent.contains(&Coordinate::new(0, 1)));
        assert!(adjacent.contains(&Coordinate::new(0, 2)));
        assert!(adjacent.contains(&Coordinate::new(1, 0)));
        assert!(adjacent.contains(&Coordinate::new(1, 2)));
        assert!(adjacent.contains(&Coordinate::new(2, 0)));
        assert!(adjacent.contains(&Coordinate::new(2, 1)));
        assert!(adjacent.contains(&Coordinate::new(2, 2)));
    }

    #[test]
    fn test_check_adjacent() {
        let grid = vec![
            vec!['X', 'O', 'X'],
            vec!['O', 'X', 'O'],
            vec!['X', 'O', 'X'],
        ];

        let coord = Coordinate::new(1, 1);
        let x_count = check_adjacent(&grid, coord, |&c| c == 'X');
        let o_count = check_adjacent(&grid, coord, |&c| c == 'O');

        assert_eq!(x_count, 4);
        assert_eq!(o_count, 4);
    }

    #[test]
    fn test_parse_to_char_grid() {
        let lines = vec!["ABC".to_string(), "DEF".to_string()];
        let grid = parse_to_chars(&lines);

        assert_eq!(grid.len(), 2);
        assert_eq!(grid[0], vec!['A', 'B', 'C']);
        assert_eq!(grid[1], vec!['D', 'E', 'F']);
    }
}
