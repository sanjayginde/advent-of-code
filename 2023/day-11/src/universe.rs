#[derive(Debug)]
pub struct Universe {
    expansion_rows: Vec<usize>,
    expansion_cols: Vec<usize>,
    galaxies: Vec<Galaxy>,
}

impl Universe {
    pub fn new() -> Self {
        Universe {
            expansion_rows: Vec::new(),
            expansion_cols: Vec::new(),
            galaxies: Vec::new(),
        }
    }

    pub fn is_expansion_row(&self, r: usize) -> bool {
        self.expansion_rows.contains(&r)
    }

    pub fn is_expansion_col(&self, c: usize) -> bool {
        self.expansion_cols.contains(&c)
    }

    pub fn add_expansion_row(&mut self, r: usize) {
        self.expansion_rows.push(r)
    }

    pub fn add_expansion_col(&mut self, c: usize) {
        self.expansion_cols.push(c)
    }

    pub fn add_galaxy(&mut self, galaxy: Galaxy) {
        self.galaxies.push(galaxy)
    }
    pub fn galaxies(&self) -> &Vec<Galaxy> {
        &self.galaxies
    }

    pub fn distance(&self, lhs: &Galaxy, rhs: &Galaxy, expansion_factor: usize) -> usize {
        let mut rows = vec![lhs.row(), rhs.row()];
        rows.sort();

        let mut cols = vec![lhs.col(), rhs.col()];
        cols.sort();

        let mut row_diff = 0;
        for r in (rows[0]..rows[1]).into_iter() {
            row_diff += match self.is_expansion_row(r) {
                true => expansion_factor,
                false => 1,
            }
        }

        let mut col_diff = 0;
        for c in (cols[0]..cols[1]).into_iter() {
            col_diff += match self.is_expansion_col(c) {
                true => expansion_factor,
                false => 1,
            }
        }

        row_diff + col_diff
    }
}

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
