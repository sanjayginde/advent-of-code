use std::fs::read_to_string;

#[derive(Debug, Eq, PartialEq)]
pub struct Galaxy {
    id: usize,
    row: usize,
    col: usize,
}

impl Galaxy {
    pub fn new(id: usize, row: usize, col: usize) -> Self {
        Galaxy { id, row, col }
    }
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }
}

#[derive(Debug)]
pub struct Universe {
    expanded_rows: Vec<usize>,
    expanded_cols: Vec<usize>,
    expand_factor: usize,
    galaxies: Vec<Galaxy>,
}

impl Universe {
    pub fn new(expand_factor: usize) -> Self {
        Universe {
            expanded_rows: Vec::new(),
            expanded_cols: Vec::new(),
            expand_factor,
            galaxies: Vec::new(),
        }
    }

    pub fn is_expanded_row(&self, r: usize) -> bool {
        self.expanded_rows.contains(&r)
    }

    pub fn is_expanded_col(&self, c: usize) -> bool {
        self.expanded_cols.contains(&c)
    }

    pub fn add_expanded_row(&mut self, r: usize) {
        self.expanded_rows.push(r)
    }

    pub fn add_expanded_col(&mut self, c: usize) {
        self.expanded_cols.push(c)
    }

    pub fn add_galaxy(&mut self, galaxy: Galaxy) {
        self.galaxies.push(galaxy)
    }
    pub fn galaxies(&self) -> &Vec<Galaxy> {
        &self.galaxies
    }

    pub fn distance(&self, lhs: &Galaxy, rhs: &Galaxy) -> usize {
        let mut rows = vec![lhs.row(), rhs.row()];
        rows.sort();

        let mut cols = vec![lhs.col(), rhs.col()];
        cols.sort();

        let mut row_diff = 0;
        for r in (rows[0]..rows[1]).into_iter() {
            row_diff += match self.is_expanded_row(r) {
                true => self.expand_factor,
                false => 1,
            }
        }

        let mut col_diff = 0;
        for c in (cols[0]..cols[1]).into_iter() {
            col_diff += match self.is_expanded_col(c) {
                true => self.expand_factor,
                false => 1,
            }
        }

        row_diff + col_diff
    }
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

fn parse(lines: Vec<String>, expand_factor: usize) -> Universe {
    let mut universe: Universe = Universe::new(expand_factor);

    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut galaxy_id = 1;

    for (r, line) in lines.iter().enumerate() {
        let chars: Vec<_> = line.chars().collect();
        for (c, ch) in chars.iter().enumerate() {
            if *ch == '#' {
                universe.add_galaxy(Galaxy::new(galaxy_id, r, c));
                galaxy_id += 1;
            }
        }
        if line.chars().all(|ch| ch == '.') {
            universe.add_expanded_row(r);
        }

        matrix.push(line.chars().collect());
    }

    for (i, col) in transpose(matrix).iter().enumerate() {
        if col.clone().into_iter().all(|ch| ch == '.') {
            universe.add_expanded_col(i);
        }
    }

    universe
}

fn solve(lines: Vec<String>, expand_factor: usize) -> usize {
    let mut total = 0;

    let universe = parse(lines, expand_factor);

    // println!("universe: {:?}", universe);
    // println!("galaxies: {:?}", galaxies);

    let galaxies = universe.galaxies();

    let mut galaxy = galaxies.get(0);
    let mut skip = 1;
    while galaxy.is_some() {
        let lhs = galaxy.unwrap();
        for rhs in galaxies.iter().skip(skip) {
            total += universe.distance(lhs, rhs);
        }
        galaxy = galaxies.get(skip);
        skip += 1;
    }

    total
}

fn main() {
    println!(
        "Solution for part 1 is {}",
        solve(read_lines("input.txt"), 2)
    );
    println!(
        "Solution for part 2 is {}",
        solve(read_lines("input.txt"), 1000000)
    );
}

#[cfg(test)]
mod test {

    use rstest::rstest;

    use super::solve;

    const EXAMPLE: [&str; 10] = [
        "...#......",
        ".......#..",
        "#.........",
        "..........",
        "......#...",
        ".#........",
        ".........#",
        "..........",
        ".......#..",
        "#...#.....",
    ];

    #[rstest]
    #[case(2, 374)]
    #[case(10, 1030)]
    #[case(100, 8410)]
    fn solve_example(#[case] expand_factor: usize, #[case] expected: usize) {
        let rows = EXAMPLE.map(String::from).to_vec();

        assert_eq!(solve(rows, expand_factor), expected);
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
