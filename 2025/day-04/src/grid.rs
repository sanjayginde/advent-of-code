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
