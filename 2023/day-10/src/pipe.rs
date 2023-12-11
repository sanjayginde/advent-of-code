use std::str::FromStr;

use strum_macros::EnumString;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    pub fn new(row: usize, col: usize) -> Self {
        Coordinate { row, col }
    }
    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }
}

#[derive(Debug, Clone, Hash)]
pub struct Step {
    coordinate: Coordinate,
    from: Direction,
}

#[derive(Debug, Clone, Hash)]
pub struct Map {
    matrix: Vec<Vec<Pipe>>,
    start: Coordinate,
    rows: usize,
    cols: usize,
}

impl Map {
    pub fn steps_to_farthest_pos(&self) -> usize {
        let mut path: Vec<Coordinate> = Vec::new();
        self.walk(self.start, &mut path);

        path.len() / 2
    }

    pub fn get_at(&self, c: Coordinate) -> Option<Pipe> {
        match self.matrix.get(c.row) {
            Some(r) => match r.get(c.col) {
                Some(p) => Some(*p),
                None => None,
            },
            None => None,
        }
    }

    pub fn get_coordinate(&self, from: Coordinate, dir: Direction) -> Option<Coordinate> {
        let to = match dir {
            Direction::North => match from.row > 0 {
                true => Some(Coordinate::new(from.row - 1, from.col)),
                false => None,
            },
            Direction::South => match from.row < self.rows {
                true => Some(Coordinate::new(from.row + 1, from.col)),
                false => None,
            },
            Direction::East => match from.col < self.cols {
                true => Some(Coordinate::new(from.row, from.col + 1)),
                false => None,
            },
            Direction::West => match from.col > 0 {
                true => Some(Coordinate::new(from.row, from.col - 1)),
                false => None,
            },
        };

        match to {
            Some(c) => {
                let pipe = self.get_at(c).unwrap();
                match pipe.accept(dir.opposite()) {
                    true => Some(c),
                    false => None,
                }
            }
            None => None,
        }
    }

    fn walk(&self, pos: Coordinate, path: &mut Vec<Coordinate>) -> Vec<Coordinate> {
        let last = path.last();

        let next_positions: Vec<(Direction, Coordinate)> = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .iter()
        .filter_map(|dir| {
            let next: Option<Coordinate> = self.get_coordinate(pos, *dir);
            match next {
                None => return None,
                Some(next) => match last.is_some() && next == *last.unwrap() {
                    true => None,
                    false => Some((*dir, next)),
                },
            }
        })
        .collect();

        let (mut dir, mut pos) = next_positions.first().unwrap().to_owned();

        let mut pipe = self.get_at(pos).unwrap();
        path.push(pos);
        while pipe != Pipe::Start {
            dir = pipe.out_direction(dir.opposite());
            pos = self.get_coordinate(pos, dir).unwrap();
            pipe = self.get_at(pos).unwrap();
            path.push(pos);
        }

        return path.clone();
    }
}

impl From<Vec<String>> for Map {
    fn from(lines: Vec<String>) -> Self {
        let mut start: Option<Coordinate> = None;
        let mut matrix: Vec<Vec<Pipe>> = Vec::with_capacity(lines.len());

        let rows: usize = lines.len();
        let cols: usize = lines.first().unwrap().len();

        for (r, line) in lines.into_iter().enumerate() {
            let mut row: Vec<Pipe> = Vec::with_capacity(line.len());
            for (c, char) in line.chars().into_iter().enumerate() {
                let pipe = Pipe::from_char(char);
                row.push(pipe);

                if pipe == Pipe::Start {
                    println!("found start at [{r}, {c}]");
                    start = Some(Coordinate { row: r, col: c });
                }
            }

            matrix.push(row);
        }

        match start {
            Some(start) => Map {
                matrix,
                start,
                rows: rows,
                cols: cols,
            },
            None => panic!("start pipe not found"),
        }
    }
}

#[derive(Debug, EnumString, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Direction {
    #[strum(serialize = "N")]
    North,

    #[strum(serialize = "S")]
    South,

    #[strum(serialize = "E")]
    East,

    #[strum(serialize = "W")]
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, EnumString, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Pipe {
    #[strum(serialize = "|")]
    NorthSouth, // | is a vertical pipe connecting north and south.

    #[strum(serialize = "-")]
    EastWest, // - is a horizontal pipe connecting east and west.

    #[strum(serialize = "L")]
    NorthEast, // L is a 90-degree bend connecting north and east.

    #[strum(serialize = "J")]
    NorthWest, // J is a 90-degree bend connecting north and west.

    #[strum(serialize = "7")]
    SouthWest, // 7 is a 90-degree bend connecting south and west.

    #[strum(serialize = "F")]
    SouthEast, // F is a 90-degree bend connecting south and east.

    #[strum(serialize = ".")]
    Ground, // . is ground; there is no pipe in this tile.

    #[strum(serialize = "S")]
    Start, // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
}

impl Pipe {
    pub fn accept(&self, dir: Direction) -> bool {
        match self {
            Pipe::NorthSouth => dir == Direction::North || dir == Direction::South,
            Pipe::EastWest => dir == Direction::East || dir == Direction::West,
            Pipe::NorthEast => dir == Direction::North || dir == Direction::East,
            Pipe::NorthWest => dir == Direction::North || dir == Direction::West,
            Pipe::SouthWest => dir == Direction::South || dir == Direction::West,
            Pipe::SouthEast => dir == Direction::South || dir == Direction::East,
            Pipe::Ground => false,
            Pipe::Start => true,
        }
    }

    pub fn out_direction(&self, from: Direction) -> Direction {
        match self {
            Pipe::NorthSouth => {
                return from.opposite();
            }
            Pipe::EastWest => {
                return from.opposite();
            }
            Pipe::NorthEast => {
                if from == Direction::North {
                    return Direction::East;
                } else {
                    return Direction::North;
                }
            }
            Pipe::NorthWest => {
                if from == Direction::North {
                    return Direction::West;
                } else {
                    return Direction::North;
                }
            }
            Pipe::SouthWest => {
                if from == Direction::South {
                    return Direction::West;
                } else {
                    return Direction::South;
                }
            }
            Pipe::SouthEast => {
                if from == Direction::South {
                    return Direction::East;
                } else {
                    return Direction::South;
                }
            }
            p => panic!("can't travel through {p:?}"),
        }
    }
}

impl Pipe {
    fn from_char(value: char) -> Self {
        Pipe::from_str(value.to_string().as_str()).unwrap()
    }
}

#[cfg(test)]
mod test {

    use super::{Coordinate, Map};

    const EXAMPLE_SIMPLE: [&str; 5] = [".....", ".S-7.", ".|.|.", ".L-J.", "....."];
    const EXAMPLE_SIMPLE_EXTRA_PIPES: [&str; 5] = ["-L|F7", "7S-7|", "L|7||", "-L-J|", "L|-JF"];
    const EXAMPLE_COMPLEX: [&str; 5] = ["..F7.", ".FJ|.", "SJ.L7", "|F--J", "LJ..."];

    fn build_map(strs: [&str; 5]) -> Map {
        Map::from(strs.map(String::from).to_vec())
    }

    #[test]
    fn start() {
        let simple = build_map(EXAMPLE_SIMPLE);
        assert_eq!(simple.start, Coordinate { row: 1, col: 1 });

        let simple_extra_pipes = build_map(EXAMPLE_SIMPLE_EXTRA_PIPES);
        assert_eq!(simple_extra_pipes.start, Coordinate { row: 1, col: 1 });

        let complex = build_map(EXAMPLE_COMPLEX);
        assert_eq!(complex.start, Coordinate { row: 2, col: 0 });
    }

    #[test]
    fn farthest_position() {
        let simple = build_map(EXAMPLE_SIMPLE);
        assert_eq!(simple.steps_to_farthest_pos(), 4);

        let simple_extra_pipes = Map::from(EXAMPLE_SIMPLE_EXTRA_PIPES.map(String::from).to_vec());
        assert_eq!(simple_extra_pipes.steps_to_farthest_pos(), 4);

        let simple_extra_pipes = Map::from(EXAMPLE_COMPLEX.map(String::from).to_vec());
        assert_eq!(simple_extra_pipes.steps_to_farthest_pos(), 8);
    }
}
