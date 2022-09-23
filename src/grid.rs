use crate::{COLUMNS, ROWS};

#[derive(Clone, Copy)]
pub struct Grid {
    cells: [[bool; ROWS]; COLUMNS],
}

impl From<[bool; ROWS * COLUMNS]> for Grid {
    fn from(val: [bool; ROWS * COLUMNS]) -> Self {
        let mut grid = Grid::new();

        for (index, &alive) in val.iter().enumerate() {
            if alive {
                grid.set(index / COLUMNS, index % COLUMNS, true);
            }
        }

        grid
    }
}

impl From<Vec<Vec<u8>>> for Grid {
    fn from(val: Vec<Vec<u8>>) -> Self {
        let cols = val.len();
        let rows = val[0].len();
        let start_x = (COLUMNS - cols) / 2;
        let start_y = (ROWS - rows) / 2;

        let mut grid = Grid::new();

        for (x, column) in val.iter().enumerate() {
            for (y, &alive) in column.iter().enumerate() {
                if alive == 1 {
                    grid.set(start_x + x, start_y + y, true);
                }
            }
        }

        grid
    }
}

impl From<Vec<Vec<bool>>> for Grid {
    fn from(val: Vec<Vec<bool>>) -> Self {
        let cols = val.len();
        let rows = val[0].len();
        let start_x = (COLUMNS - cols) / 2;
        let start_y = (ROWS - rows) / 2;

        let mut grid = Grid::new();

        for (x, column) in val.iter().enumerate() {
            for (y, &alive) in column.iter().enumerate() {
                if alive {
                    grid.set(start_x + x, start_y + y, true);
                }
            }
        }

        grid
    }
}

pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub alive: bool,
}

pub struct GridIterator {
    current_x: usize,
    current_y: usize,
    grid: Grid,
}

impl Iterator for GridIterator {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_x >= COLUMNS {
            return None;
        }

        let cell = Cell {
            x: self.current_x as i32,
            y: self.current_y as i32,
            alive: self.grid.get(self.current_x, self.current_y),
        };

        self.current_y += 1;
        if self.current_y >= ROWS {
            self.current_y = 0;
            self.current_x += 1;
        }

        Some(cell)
    }
}

impl IntoIterator for Grid {
    type IntoIter = GridIterator;
    type Item = Cell;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            current_x: 0,
            current_y: 0,
            grid: self,
        }
    }
}

impl Grid {
    pub fn new() -> Grid {
        Self {
            cells: [[false; ROWS]; COLUMNS],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, alive: bool) {
        self.cells[x][y] = alive;
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.cells[x][y]
    }

    pub fn iter(&self) -> GridIterator {
        self.into_iter()
    }
}
