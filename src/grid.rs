use std::{env, fs::read_to_string, path::Path};

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

impl From<&str> for Grid {
    fn from(species_name: &str) -> Self {
        let path = Path::new(&env::current_dir().unwrap())
            .join(Path::new(&format!("species/{species_name}.txt")));

        let schema = read_to_string(path).expect(&format!("{species_name} does not exist"));

        let mut cells: Vec<Vec<bool>> = Vec::new();

        let mut current_row: Vec<bool> = Vec::new();
        let mut last_char: char = ' ';
        let mut max_columns: usize = 0;
        for char in schema.chars() {
            if char == '\n' {
                max_columns = current_row.len().max(max_columns);
                cells.push(current_row);
                current_row = Vec::new();
            } else {
                current_row.push(char == '#');
            }
            last_char = char;
        }
        if last_char != '\n' {
            cells.push(current_row);
        }

        println!("{cells:?}\n{max_columns}");

        let mut cells_by_columns: Vec<Vec<bool>> = Vec::new();
        for x in 0..max_columns {
            let mut column = Vec::new();
            for row in &cells {
                column.push(*row.get(x).or(Some(&false)).unwrap());
            }
            cells_by_columns.push(column);
        }

        cells_by_columns.into()
    }
}
