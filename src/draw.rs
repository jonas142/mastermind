use piston_window::{Context, G2d, rectangle, types::Color};

const BLOCK_SIZE: f64 = 15.0;

pub struct Block {
    x: i32,
    y: i32,
}

fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(color, [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE], con.transform, g);
}

pub fn draw_big_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    let big: f64 = 2.0;
    rectangle(color, [gui_x, gui_y, big * BLOCK_SIZE, big * BLOCK_SIZE], con.transform, g);
}

pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(color, [gui_x, gui_y, (width as f64) * BLOCK_SIZE, (height as f64) * BLOCK_SIZE], con.transform, g);
}