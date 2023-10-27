use piston_window::{types::Color, Context, G2d, Key};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    draw::{draw_big_block, draw_rectangle},
    COLOR_BLACK, COLOR_BLUE, COLOR_EMPTY, COLOR_GREEN, COLOR_RED, COLOR_SECRET, COLOR_WHITE,
    COLOR_YELLOW, FIELD_SIZE, SPACING,
};

const COLOR_CURRENT_POSITION: Color = [0.2, 0.2, 0.2, 0.5];
const SIZE: i32 = 4;
const FLASH_TIMER: i32 = 4;

#[derive(PartialEq, EnumIter, Debug, Copy, Clone)]
pub enum Colors {
    Empty,
    Red,
    Blue,
    Green,
    Yellow,
    Black,
    White,
    Secret,
}

impl Colors {
    pub fn return_color(&self) -> Color {
        match self {
            Colors::Empty => COLOR_EMPTY,
            Colors::Red => COLOR_RED,
            Colors::Blue => COLOR_BLUE,
            Colors::Green => COLOR_GREEN,
            Colors::Yellow => COLOR_YELLOW,
            Colors::Black => COLOR_BLACK,
            Colors::White => COLOR_WHITE,
            Colors::Secret => COLOR_SECRET,
        }
    }

    pub fn create_color_list() -> Vec<Colors> {
        let mut color_options = vec![];
        for color in Colors::iter() {
            if color != Colors::Secret {
                color_options.push(color);
            }
        }
        return color_options;
    }
}

pub struct GuessInputField {
    fields: Vec<Colors>,
    color_options: Vec<Colors>,
    ready: bool,
    send_guess: bool,
    current_position: usize,
    disabled: bool,

    flashing_visible: bool,
    flash_timer: i32,

    gui_position_x: i32,
    gui_position_y: i32,
}

impl GuessInputField {
    pub fn new(gui_position_x: i32, gui_position_y: i32) -> GuessInputField {
        let mut fields = vec![];
        for _ in 0..SIZE {
            fields.push(Colors::Empty);
        }

        let color_options = Colors::create_color_list();

        return GuessInputField {
            fields,
            color_options,
            ready: false,
            send_guess: false,
            current_position: 0,
            disabled: false,
            flashing_visible: true,
            flash_timer: FLASH_TIMER,
            gui_position_x,
            gui_position_y,
        };
    }

    pub fn get_send_guess(&self) -> bool {
        self.send_guess
    }

    pub fn get_guess(&self) -> &Vec<Colors> {
        &self.fields
    }

    pub fn reset_guess(&mut self) {
        self.fields = vec![Colors::Empty; 4];
        self.ready = false;
        self.send_guess = false;
        self.current_position = 0;
    }

    pub fn disable_input(&mut self) {
        self.disabled = true;
    }

    pub fn enable_input(&mut self) {
        self.disabled = false;
    }

    pub fn key_pressed(&mut self, key: Key) {
        if !self.disabled {
            match key {
                Key::Left => self.move_current_position(-1), // move current position left
                Key::Right => self.move_current_position(1), // move current position right
                Key::Up => self.change_color(1),             // choose Color
                Key::Down => self.change_color(-1),          // choose Color
                Key::Return => self.send_guess(),            // if on ready block set ready
                _ => {}
            }
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        let mut i = 0;
        for block in &self.fields {
            draw_big_block(
                block.return_color(),
                self.gui_position_x + i * SPACING + i * FIELD_SIZE,
                self.gui_position_y,
                con,
                g,
            );
            i += 1;
        }
        // draw enterBlock
        let color = match self.ready {
            true => COLOR_GREEN,
            false => COLOR_RED,
        };
        draw_rectangle(
            color,
            self.gui_position_x + i * SPACING + i * FIELD_SIZE,
            self.gui_position_y - 1,
            2,
            3,
            con,
            g,
        );
        // draw current position
        if self.flashing_visible {
            if self.current_position == 4 {
                draw_rectangle(
                    COLOR_CURRENT_POSITION,
                    self.gui_position_x + i * SPACING + i * FIELD_SIZE,
                    self.gui_position_y - 1,
                    2,
                    3,
                    con,
                    g,
                );
            } else {
                draw_big_block(
                    COLOR_CURRENT_POSITION,
                    self.gui_position_x
                        + (self.current_position as i32) * SPACING
                        + (self.current_position as i32) * FIELD_SIZE,
                    self.gui_position_y,
                    con,
                    g,
                );
            }
        }
    }

    pub fn update(&mut self) {
        if !self.fields.contains(&Colors::Empty) {
            self.ready = true;
        } else {
            self.ready = false;
        }
        // allow to flash
        if self.flash_timer <= 0 {
            self.flashing_visible = !self.flashing_visible;
            self.flash_timer = FLASH_TIMER;
        }
        self.flash_timer -= 1;
    }

    fn send_guess(&mut self) {
        if self.current_position == 4 && self.ready {
            self.send_guess = true
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
        let mut index = self
            .color_options
            .iter()
            .position(|color| color == &self.fields[self.current_position])
            .unwrap();
        if dir == -1 {
            index = (index + self.color_options.len() - 1) % self.color_options.len();
        } else {
            index = (index + self.color_options.len() + 1) % self.color_options.len();
        }
        self.fields[self.current_position] = self.color_options[index];
    }
}
