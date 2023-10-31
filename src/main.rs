extern crate piston_window;
extern crate rand;

use std::env;

use piston_window::{
    clear, types::Color, Button, EventLoop, PistonWindow, PressEvent, UpdateEvent, WindowSettings,
};

use crate::game::Game;

use draw::to_coord_u32;

mod draw;
mod game;
mod guess;
mod help_page;

pub const COLOR_RED: Color = [0.8, 0.0, 0.0, 1.0];
pub const COLOR_GREEN: Color = [0.0, 0.8, 0.0, 1.0];
pub const COLOR_BLUE: Color = [0.0, 0.0, 0.8, 1.0];
pub const COLOR_YELLOW: Color = [0.8, 0.8, 0.0, 1.0];
pub const COLOR_BLACK: Color = [0.0, 0.0, 0.0, 1.0];
pub const COLOR_WHITE: Color = [1.0, 1.0, 1.0, 1.0];
pub const COLOR_EMPTY: Color = [0.5, 0.5, 0.5, 1.0];
pub const COLOR_SECRET: Color = [0.2, 0.2, 0.2, 1.0];
pub const COLOR_SUCCESS: Color = [0.0, 1.0, 0.0, 0.5];
pub const COLOR_GAMEOVER: Color = [1.0, 0.0, 0.0, 0.5];
pub const SPACING: i32 = 3;
pub const FIELD_SIZE: i32 = 2;

const BACK_COLOR: Color = [0.4, 0.4, 0.4, 1.0];

fn main() {
    // parse arguments (for debugging)
    let mut debug = false;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "--debug" {
            debug = true;
        }
    }

    let (width, height) = (24, 39);
    let mut window: PistonWindow =
        WindowSettings::new("Mastermind", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();
    // window.set_lazy(true);

    let mut game = Game::new(width, height, 6, debug);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, device| {
            clear(BACK_COLOR, g);
            game.draw(&c, g, &mut glyphs);
            // draw_help_page(
            //     10.0,
            //     100.0,
            //     "Hello world!",
            //     32,
            //     COLOR_BLACK,
            //     &c,
            //     g,
            //     &mut glyphs,
            // );
            glyphs.factory.encoder.flush(device);
        });

        event.update(|args| {
            game.update(args.dt);
        });
    }
}
