#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Coordinate {
    pub row: usize,
    pub col: usize,
}

impl Coordinate {
    pub fn new(row: usize, col: usize) -> Self {
        Coordinate { row, col }
    }
}

pub fn check_adjacent<T, F>(grid: &[Vec<T>], coordinate: Coordinate, operation: F) -> usize
where
    F: Fn(&T) -> bool + Copy,
{
    let mut result = 0;

    if coordinate.row > 0
        && let Some(row_above) = grid.get(coordinate.row - 1)
    {
        let col = coordinate.col;

        if col > 0 && row_above.get(col - 1).is_some_and(operation) {
            result += 1;
        }

        if row_above.get(col).is_some_and(operation) {
            result += 1;
        }

        if row_above.get(col + 1).is_some_and(operation) {
            result += 1;
        }
    }

    if let Some(row) = grid.get(coordinate.row) {
        let col = coordinate.col;
        if col > 0 && row.get(col - 1).is_some_and(operation) {
            result += 1;
        }

        if row.get(col + 1).is_some_and(operation) {
            result += 1;
        }
    }

    if let Some(row_below) = grid.get(coordinate.row + 1) {
        let col = coordinate.col;

        if col > 0 && row_below.get(col - 1).is_some_and(operation) {
            result += 1;
        }

        if row_below.get(col).is_some_and(operation) {
            result += 1;
        }

        if row_below.get(col + 1).is_some_and(operation) {
            result += 1;
        }
    }

    result
}

pub fn parse_to_char_grid(lines: &[String]) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = vec![];

    for line in lines.iter() {
        let row = line.chars().collect::<Vec<_>>();
        matrix.push(row);
    }

    matrix
}

// pub fn parse_to_grid<T, F: Copy>(lines: &Vec<String>, parse: F) -> Vec<Vec<T>>
// where
//     F: Fn(char) -> T,
// {
//     let mut matrix: Vec<Vec<T>> = vec![];

//     for line in lines.iter() {
//         let row = line.chars().map(parse).collect::<Vec<_>>();
//         matrix.push(row);
//     }

//     matrix
// }
