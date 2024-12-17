use std::fs::read_to_string;

#[derive(Debug, Clone)]
pub struct Position {
    row: isize,
    col: isize,
}

#[derive(Debug)]
pub struct Map {
    matrix: Vec<Vec<Tile>>,
    start: Position,
}

impl Map {
    pub fn size(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }

    pub fn rows(&self) -> usize {
        self.matrix.len()
    }

    pub fn cols(&self) -> usize {
        self.matrix[0].len()
    }

    pub fn walk(&mut self) {
        let mut direction = Direction::North;
        let mut position = self.start.clone();

        while position.row >= 0
            && position.row < self.rows() as isize
            && position.col >= 0
            && position.col < self.cols() as isize
        {
            let next_position = direction.next_position(&position);

            let next_tile = &mut match self
                .matrix
                .get_mut(next_position.row as usize)
                .and_then(|row| row.get_mut(next_position.col as usize))
            {
                Some(tile) => tile,
                None => {
                    break;
                }
            };

            match next_tile.tile_type {
                TileType::Empty { ref mut visited } => {
                    *visited = true;
                    position = next_position;
                }
                TileType::Start => {
                    position = next_position;
                }
                TileType::Obstruction => {
                    direction = direction.next_direction();
                }
            }
        }
    }

    pub fn num_visited(&self) -> u64 {
        let mut result: u64 = 0;
        for row in &self.matrix {
            for tile in row {
                if tile.visited() {
                    result += 1;
                }
            }
        }
        result
    }

    pub fn print(&self) {
        for row in &self.matrix {
            for tile in row {
                match tile.tile_type {
                    TileType::Start => {
                        print!("^");
                    }
                    TileType::Empty { visited } => {
                        if visited {
                            print!("X");
                        } else {
                            print!(".");
                        }
                    }
                    TileType::Obstruction => {
                        print!("#");
                    }
                }
            }
            println!();
        }
    }
}

impl From<Vec<String>> for Map {
    fn from(lines: Vec<String>) -> Self {
        let mut start = Position { row: 0, col: 0 };
        let mut matrix: Vec<Vec<Tile>> = Vec::with_capacity(lines.len());

        for (row_num, line) in lines.into_iter().enumerate() {
            let mut row: Vec<Tile> = Vec::with_capacity(line.len());
            for (col_num, char) in line.chars().enumerate() {
                let tile = Tile::from(char);
                if matches!(tile.tile_type, TileType::Start) {
                    start = Position {
                        row: row_num as isize,
                        col: col_num as isize,
                    };
                }
                row.push(tile);
            }
            matrix.push(row);
        }

        Map { matrix, start }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn next_position(&self, position: &Position) -> Position {
        match self {
            Direction::North => Position {
                row: position.row - 1,
                col: position.col,
            },
            Direction::South => Position {
                row: position.row + 1,
                col: position.col,
            },
            Direction::East => Position {
                row: position.row,
                col: position.col + 1,
            },
            Direction::West => Position {
                row: position.row,
                col: position.col - 1,
            },
        }
    }

    pub fn next_direction(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug)]
pub struct Tile {
    tile_type: TileType,
}

impl Tile {
    pub fn visited(&self) -> bool {
        match self.tile_type {
            TileType::Empty { visited } => visited,
            TileType::Start => true,
            TileType::Obstruction => false,
        }
    }
}

impl From<char> for Tile {
    fn from(ch: char) -> Self {
        let tile_type = match ch {
            '^' => TileType::Start,
            '#' => TileType::Obstruction,
            _ => TileType::Empty { visited: false },
        };

        Tile { tile_type }
    }
}

#[derive(Debug)]
enum TileType {
    Empty { visited: bool },
    Start,
    Obstruction,
}

fn part1(lines: Vec<String>) -> u64 {
    let mut map = Map::from(lines);

    map.walk();
    map.print();

    map.num_visited()
}

fn main() {
    println!("Solution for part 1 is {}", part1(read_lines("input.txt")));
    // println!("Solution for part 2 is {}", part2(read_lines("input.txt")));
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod test {

    use super::part1;

    const EXAMPLE: [&str; 10] = [
        "....#.....",
        ".........#",
        "..........",
        "..#.......",
        ".......#..",
        "..........",
        ".#..^.....",
        "........#.",
        "#.........",
        "......#...",
    ];

    #[test]
    fn solve_part1() {
        assert_eq!(part1(EXAMPLE.map(String::from).to_vec()), 41);
    }
}
