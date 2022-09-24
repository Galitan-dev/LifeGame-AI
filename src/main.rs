mod game;

use game::Game;
use raylib::prelude::*;

pub const COLUMNS: usize = 60;
pub const ROWS: usize = 40;
pub const WIDTH: i32 = 1200;
pub const CELL_SIZE: i32 = WIDTH / COLUMNS as i32;
pub const HEIGHT: i32 = ROWS as i32 * CELL_SIZE;
pub const SLOWNESS: i32 = 300;

pub const SPECIES_NAME: &str = "pentadecathlon";

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Life-Game AI")
        .build();

    let mut game = Game::new(SPECIES_NAME);

    let mut frame_count = 1;
    while !rl.window_should_close() {
        if frame_count % SLOWNESS == 0 && frame_count > 0 {
            game.update();
        }

        use game::draw::Drawable;
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        game.draw(&mut d);
        frame_count += 1;
    }
}
