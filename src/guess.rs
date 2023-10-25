use piston_window::Key;



const SIZE: i32 = 4;

#[derive(PartialEq)]
pub enum Colors {
    Red,
    Blue,
    Green,
    Empty
}

pub struct GuessInputField {
    fields: Vec<Colors>,
    empty: bool,
    ready: bool,
    current_position: i32,
}

impl GuessInputField {
    pub fn new() -> GuessInputField {
        let mut fields = vec![];
        for _ in 0..SIZE {
            fields.push(Colors::Empty);
        }

        return GuessInputField { fields , empty: true, ready: false , current_position: 0};
    }

    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::Left => todo!(), // move current position left
            Key::Right => todo!(), // move current position right
            Key::Up => todo!(), // choose Color 
            Key::Down => todo!(), // choose Color 
            Key::H => todo!(), //print help message
            _ => {},
        }
    }
}