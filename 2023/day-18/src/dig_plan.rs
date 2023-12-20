#[derive(Debug, Clone)]
pub struct DigPlan {
    instructions: Vec<Instruction>,
}

impl DigPlan {
    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }

    pub fn area(&self) -> i64 {
        let (area, perimeter, _pos) = self.instructions().iter().fold(
            (0, 0, Position::default()),
            |(area, perimeter, pos), instr| {
                let next_pos = pos.advance(instr);
                let new_area = area + (pos.x * next_pos.y - next_pos.x * pos.y);
                let new_perimeter =
                    perimeter + (next_pos.x - pos.x).abs() + (next_pos.y - pos.y).abs();

                (new_area, new_perimeter, next_pos)
            },
        );
        (area + perimeter) / 2 + 1
    }
}

impl DigPlan {
    pub fn from_part1(lines: Vec<String>) -> Self {
        let instructions: Vec<_> = lines
            .into_iter()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let direction = match parts.next().unwrap() {
                    "U" => Direction::U,
                    "D" => Direction::D,
                    "L" => Direction::L,
                    "R" => Direction::R,
                    _ => panic!("Invalid direction"),
                };
                let distance = parts.next().unwrap().parse().unwrap();

                Instruction {
                    direction,
                    distance,
                }
            })
            .collect();
        Self {
            instructions: instructions,
        }
    }

    pub fn from_part2(lines: Vec<String>) -> Self {
        let instructions: Vec<_> = lines
            .into_iter()
            .map(|line| {
                let mut parts = line.split_whitespace();
                parts.next();
                parts.next();

                let code = parts
                    .next()
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .strip_prefix("(#")
                    .unwrap();

                let (distance_hex, dir_code) = code.split_at(5);

                let direction = match dir_code {
                    "0" => Direction::R,
                    "1" => Direction::D,
                    "2" => Direction::L,
                    "3" => Direction::U,
                    _ => panic!("Invalid direction"),
                };

                let distance = i64::from_str_radix(distance_hex, 16).unwrap();

                Instruction {
                    direction,
                    distance,
                }
            })
            .collect();
        Self {
            instructions: instructions,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    direction: Direction,
    distance: i64,
}

impl Instruction {
    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn distance(&self) -> i64 {
        self.distance
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    U,
    D,
    L,
    R,
}

pub struct Position {
    x: i64,
    y: i64,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }

    pub fn advance(&self, instr: &Instruction) -> Self {
        let distance = instr.distance();
        match instr.direction() {
            Direction::U => Self {
                x: self.x,
                y: self.y - distance,
            },
            Direction::D => Self {
                x: self.x,
                y: self.y + distance,
            },
            Direction::L => Self {
                x: self.x - distance,
                y: self.y,
            },
            Direction::R => Self {
                x: self.x + distance,
                y: self.y,
            },
        }
    }
}
