extern crate piston_window;
extern crate rand;

use piston_window::{types::Color, PistonWindow, WindowSettings, Button, PressEvent, clear, UpdateEvent};

use crate::game::Game;

use draw::to_coord_u32;

mod draw;
mod guess;
mod game;

pub const COLOR_GREEN: Color = [0.0, 0.8, 0.0, 1.0];
pub const COLOR_RED: Color = [0.8, 0.0, 0.0, 1.0];
pub const COLOR_BLUE: Color = [0.0, 0.0, 0.8, 1.0];
pub const COLOR_EMPTY: Color = [0.8, 0.8, 0.8, 1.0];
pub const SPACING: i32 = 3;
pub const FIELD_SIZE: i32 = 2;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (24, 39);
    let mut window: PistonWindow = WindowSettings::new("Mastermind", [to_coord_u32(width), to_coord_u32(height)]).exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game::new(width, height, 6);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|args| {
            game.update(args.dt);
        });
    }
}
