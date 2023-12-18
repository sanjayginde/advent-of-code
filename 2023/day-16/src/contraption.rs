use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};
use strum_macros::EnumString;
use Direction::*;

#[derive(Debug, Clone)]
pub struct Contraption {
    matrix: Vec<Vec<Tile>>,
}

impl Contraption {
    pub fn size(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }

    pub fn rows(&self) -> usize {
        self.matrix.len()
    }

    pub fn cols(&self) -> usize {
        self.matrix[0].len()
    }

    pub fn tile_mut(&mut self, trajectory: Trajectory) -> Option<&mut Tile> {
        self.matrix
            .get_mut(trajectory.row)
            .and_then(|row| row.get_mut(trajectory.col).and_then(|tile| Some(tile)))
    }

    pub fn reset(&mut self) {
        for row in self.matrix.iter_mut() {
            for tile in row.iter_mut() {
                tile.reset();
            }
        }
    }

    pub fn total_energized(&self) -> usize {
        let mut total: usize = 0;
        for row in &self.matrix {
            for tile in row {
                if tile.is_energized() {
                    total += 1;
                }
            }
        }

        total
    }

    // Iterative solution to walk
    pub fn walk(&mut self, initial: Trajectory) {
        let mut trajectories = VecDeque::new();
        trajectories.push_back(initial);

        while !trajectories.is_empty() {
            let trajectory = trajectories.pop_front().unwrap();
            match self.tile_mut(trajectory) {
                None => {}
                Some(tile) => {
                    if tile.energize(trajectory.dir) {
                        let next_trajectories = tile.get_next(trajectory);

                        for next_trajectory in next_trajectories.into_iter() {
                            trajectories.push_front(next_trajectory);
                        }
                    }
                }
            }
        }
    }

    pub fn _print_energized(&self) {
        for row in &self.matrix {
            for tile in row {
                match tile.is_energized() {
                    true => print!("#"),
                    false => print!("."),
                }
            }
            println!("");
        }
    }
}

impl From<Vec<String>> for Contraption {
    fn from(lines: Vec<String>) -> Self {
        let mut matrix: Vec<Vec<Tile>> = Vec::with_capacity(lines.len());

        for line in lines {
            let mut row: Vec<Tile> = Vec::with_capacity(line.len());
            for char in line.chars() {
                row.push(Tile::from(char));
            }
            matrix.push(row);
        }

        Contraption { matrix }
    }
}

#[derive(Debug, EnumString, Clone, Copy)]
pub enum TileType {
    #[strum(serialize = ".")]
    Empty,

    #[strum(serialize = "/")]
    MirrorForwardSlash,

    #[strum(serialize = "\\")]
    MirrorBackSlash,

    #[strum(serialize = "|")]
    VerticalSplitter,

    #[strum(serialize = "-")]
    HorizontalSplitter,
}

impl TileType {
    pub fn from_char(value: char) -> Self {
        TileType::from_str(value.to_string().as_str()).unwrap()
    }

    pub fn get_next(&self, t: Trajectory) -> Vec<Trajectory> {
        let next_dirs: Vec<Direction> = match self {
            TileType::Empty => vec![t.dir],
            TileType::MirrorForwardSlash => match t.dir {
                // '/'
                North => vec![East],
                South => vec![West],
                East => vec![North],
                West => vec![South],
            },
            TileType::MirrorBackSlash => match t.dir {
                // '\'
                North => vec![West],
                South => vec![East],
                East => vec![South],
                West => vec![North],
            },
            TileType::VerticalSplitter => match t.dir {
                North | South => vec![t.dir],
                East | West => vec![North, South],
            },
            TileType::HorizontalSplitter => match t.dir {
                North | South => vec![East, West],
                East | West => vec![t.dir],
            },
        };

        let result: Vec<Trajectory> = next_dirs
            .into_iter()
            .map(|next_dir| match next_dir {
                North => match t.row == 0 {
                    true => None,
                    false => Some(Trajectory::new(t.row - 1, t.col, North)),
                },
                South => Some(Trajectory::new(t.row + 1, t.col, South)),
                East => Some(Trajectory::new(t.row, t.col + 1, East)),
                West => match t.col == 0 {
                    true => None,
                    false => Some(Trajectory::new(t.row, t.col - 1, West)),
                },
            })
            .filter_map(|dir: Option<Trajectory>| dir)
            .collect();

        result
    }
}

#[derive(Debug, Clone)]
pub struct Tile {
    tile_type: TileType,
    is_energized: bool,
    energized_from: HashMap<Direction, bool>,
}

impl Tile {
    pub fn tile_type(&self) -> TileType {
        self.tile_type
    }

    pub fn is_energized(&self) -> bool {
        self.is_energized
    }

    pub fn energize(&mut self, from: Direction) -> bool {
        self.is_energized = true;

        match self.energized_from.get(&from) {
            Some(_) => false,
            None => {
                self.energized_from.insert(from, true);
                true
            }
        }
    }

    pub fn reset(&mut self) {
        self.energized_from.clear();
        self.is_energized = false;
    }

    pub fn get_next(&self, t: Trajectory) -> Vec<Trajectory> {
        self.tile_type.get_next(t)
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        Tile {
            tile_type: TileType::from_char(value),
            is_energized: false,
            energized_from: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
pub struct Trajectory {
    row: usize,
    col: usize,
    dir: Direction,
}

impl Trajectory {
    pub fn new(row: usize, col: usize, dir: Direction) -> Self {
        Trajectory { row, col, dir }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn dir(&self) -> Direction {
        self.dir
    }
}
