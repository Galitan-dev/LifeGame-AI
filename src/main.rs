mod draw;
mod grid;

use grid::Grid;
use raylib::prelude::*;

pub const COLUMNS: usize = 60;
pub const ROWS: usize = 40;
pub const WIDTH: i32 = 1200;
pub const CELL_SIZE: i32 = WIDTH / COLUMNS as i32;
pub const HEIGHT: i32 = ROWS as i32 * CELL_SIZE;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Life-Game AI")
        .build();

    let grid = Grid::from(vec![vec![1]]);

    while !rl.window_should_close() {
        use draw::Drawable;
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        grid.draw(&mut d);
    }
}
