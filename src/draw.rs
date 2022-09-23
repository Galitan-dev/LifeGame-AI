use raylib::prelude::*;

use crate::{
    grid::{Cell, Grid},
    CELL_SIZE, COLUMNS, HEIGHT, ROWS, WIDTH,
};

pub trait Drawable {
    fn draw(&self, d: &mut RaylibDrawHandle);
}

impl Drawable for Grid {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        for cell in self.iter() {
            cell.draw(d);
        }

        for x in 0..COLUMNS {
            d.draw_line(
                x as i32 * CELL_SIZE,
                0,
                x as i32 * CELL_SIZE,
                HEIGHT,
                Color::WHITE,
            );
        }

        for y in 0..ROWS {
            d.draw_line(
                0,
                y as i32 * CELL_SIZE,
                WIDTH,
                y as i32 * CELL_SIZE,
                Color::WHITE,
            );
        }
    }
}

impl Drawable for Cell {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        if self.alive {
            d.draw_rectangle(
                self.x as i32 * CELL_SIZE,
                self.y as i32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                Color::WHITE,
            );
        }
    }
}
