pub mod draw;
pub mod grid;

use grid::Grid;

pub struct Game {
    pub grid: Grid,
}

impl Game {
    pub fn new<G>(grid: G) -> Self
    where
        G: Into<Grid>,
    {
        Self { grid: grid.into() }
    }

    pub fn update(&mut self) {
        self.grid = self.grid.next();
    }
}
