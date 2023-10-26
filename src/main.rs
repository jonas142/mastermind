extern crate piston_window;
extern crate rand;

use piston_window::types::Color;

use crate::guess::GuessInputField;

mod draw;
mod guess;
mod game;

pub const COLOR_GREEN: Color = [0.0, 0.8, 0.0, 1.0];
pub const COLOR_RED: Color = [0.8, 0.0, 0.0, 1.0];
pub const COLOR_BLUE: Color = [0.0, 0.0, 0.8, 1.0];
pub const COLOR_EMPTY: Color = [0.8, 0.8, 0.8, 1.0];
pub const SPACING: i32 = 5;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let index = v.iter().position(|&x| x == 3).unwrap();
    println!("element at {}: {}", index, v[index]);

    let mut guess = GuessInputField::new(40, 40);
    guess.change_color(-1);
}
