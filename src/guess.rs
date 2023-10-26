use piston_window::{Key, types::Color, Context, G2d};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{COLOR_BLUE, COLOR_GREEN, COLOR_RED, COLOR_EMPTY, SPACING, draw::{draw_block, draw_rectangle}};

const COLOR_CURRENT_POSITION: Color = [0.2, 0.2, 0.2, 0.5];
const SIZE: i32 = 4;

#[derive(PartialEq, EnumIter, Debug, Copy, Clone)]
pub enum Colors {
    Empty,
    Red,
    Blue,
    Green,
}

pub struct GuessInputField {
    fields: Vec<Colors>,
    color_options: Vec<Colors>,
    empty: bool,
    ready: bool,
    current_position: usize,

    gui_position_x: i32,
    gui_position_y: i32,
}

impl GuessInputField {
    pub fn new(gui_position_x: i32, gui_position_y: i32) -> GuessInputField {
        let mut fields = vec![];
        for _ in 0..SIZE {
            fields.push(Colors::Empty);
        }

        let mut color_options = vec![];
        for color in Colors::iter() {
            color_options.push(color);
        }

        return GuessInputField { fields, color_options, empty: true, ready: false , current_position: 0, gui_position_x, gui_position_y};
    }

    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::Left => self.move_current_position(-1), // move current position left
            Key::Right => self.move_current_position(1), // move current position right
            Key::Up => self.change_color(1), // choose Color 
            Key::Down => self.change_color(-1), // choose Color 
            Key::H => todo!(), //print help message
            Key::Return => todo!(), // if on ready block set ready
            _ => {},
        }
        // update the view here? or in game?
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        let mut i = 0;
        for block in &self.fields {
            match block {
                Colors::Empty => draw_block(COLOR_EMPTY, self.gui_position_x + i * SPACING, self.gui_position_y, con, g),
                Colors::Red => draw_block(COLOR_RED, self.gui_position_x + i * SPACING, self.gui_position_y, con, g),
                Colors::Blue => draw_block(COLOR_BLUE, self.gui_position_x + i * SPACING, self.gui_position_y, con, g),
                Colors::Green => draw_block(COLOR_GREEN, self.gui_position_x + i * SPACING, self.gui_position_y, con, g),
            }
            i += 1;
        }
        // draw enterBlock
        let color = match self.ready {
            true => COLOR_GREEN,
            false => COLOR_RED,
        };
        draw_rectangle(color, self.gui_position_x + i * SPACING, self.gui_position_y, 2, 1, con, g);
        // draw current position
        if self.current_position == 4 {
            draw_rectangle(COLOR_CURRENT_POSITION, self.gui_position_x + i * SPACING, self.gui_position_y, 2, 1, con, g);
        } else {
            draw_block(COLOR_CURRENT_POSITION, self.gui_position_x + (self.current_position as i32) * SPACING, self.gui_position_y, con, g);
        }
    }

    fn move_current_position(&mut self, dir: i32) {
        if dir == -1 && self.current_position > 0 {
            self.current_position -= 1
        } else if dir == 1 && self.current_position < 4 {
            self.current_position += 1
        } else {
            return;
        }
    }

    fn change_color(&mut self, dir: i32) {
        // check we are not at ready position (= 4)
        if self.current_position == 4 {
            return;
        }
        let mut index = self.color_options.iter().position(|color| color == &self.fields[self.current_position]).unwrap();
        println!("{index} : {:?}", self.color_options[index]);
        if dir == -1 {
            index = (index + self.color_options.len() - 1) % self.color_options.len();
        } else {
            index = (index + self.color_options.len() + 1) % self.color_options.len();
        }
        self.fields[self.current_position] = self.color_options[index];
        println!("{:?}", self.fields[self.current_position]);
    }
}