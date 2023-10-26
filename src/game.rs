use std::process;

use crate::{guess, SPACING, draw::{draw_big_block, draw_block}, FIELD_SIZE};
use guess::{Colors, GuessInputField};
use piston_window::{Context, G2d, Key};
use rand::{thread_rng, Rng};

const MOVING_PERIOD: f64 = 0.1;

struct GuessField {
    x: i32,
    y: i32,
    color: Colors,
}

impl GuessField {
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_big_block(self.color.return_color(), self.x, self.y, con, g);
    }
}

#[derive(Clone)]
struct ValidationField {
    x: i32,
    y: i32,
    pins: Vec<Colors>
}

impl ValidationField {
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_block(self.pins[0].return_color(), self.x, self.y, con, g);
        draw_block(self.pins[1].return_color(), self.x + 1, self.y, con, g);
        draw_block(self.pins[2].return_color(), self.x, self.y + 1, con, g);
        draw_block(self.pins[3].return_color(), self.x + 1, self.y + 1, con, g);
    }
}


pub struct Game {
    guess_input_field: GuessInputField,

    width: i32,
    height: i32,
    
    number_of_guesses: usize,
    next_guess: i32,
    secret: Vec<Colors>,
    guessed: Vec<Vec<GuessField>>,
    guess_validation: Vec<ValidationField>,
    guess_pointer: usize,

    game_over: bool,
    game_won: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32, number_of_guesses: usize) -> Game {

        let (gui_position_x, gui_position_y) = Game::calculate_guess_input_position(width, height, number_of_guesses as i32);
        println!("{}", gui_position_y);
        let secret = Game::create_new_secret();
        let guessed = Game::create_empty_guessed(number_of_guesses as i32);
        let guess_validation = Game::create_empty_guess_validation(number_of_guesses as i32);

        Game { guess_input_field: GuessInputField::new(gui_position_x, gui_position_y), width, height, number_of_guesses, next_guess: 0, secret, guessed, guess_validation, guess_pointer: 0, game_over: false, game_won: false, waiting_time: 0.0 }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;


        if self.game_over {
            return;
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_guess_input_field();
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        self.guess_input_field.key_pressed(key);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        
        for fields in &self.guessed {
            for field in fields {
                field.draw(con, g);
            }
        }
        
        for field in &self.guess_validation {
            field.draw(con, g);
        }
        self.guess_input_field.draw(con, g);
    }

    fn update_guess_input_field(&mut self) {
        self.guess_input_field.update();
        if self.check_send_guess() {
            // get guess
            let current_guess = self.guess_input_field.get_guess().clone();
            // delete guess from input field
            self.guess_input_field.reset_guess();
            // run logic on guess
            let (black_pins, white_pins) = self.check_guess_against_secret(&current_guess);
            // set validation pins
            todo!();
            // add guess to guessed
            for i in 0..4 {
                self.guessed[self.guess_pointer][i].color = current_guess[i];
            }
            self.guess_pointer += 1;
        }

        self.waiting_time = 0.0;
    }

    fn check_guess_against_secret(&mut self, current_guess: &Vec<Colors>) -> (i32, i32) {
        let color_list = Colors::create_color_list();
        let mut color_occurences_secret = vec![0; color_list.len()];
        let mut color_occurences_guess = vec![0; color_list.len()];
        let mut color_occurences_both = vec![0; color_list.len()];
        let mut c;
        let mut index;

        for i in 0..4 {
            c = &self.secret[i];
            index = color_list.iter().position(|x| x == c).unwrap();
            color_occurences_secret[index] += 1;
            
            c = &current_guess[i];
            index = color_list.iter().position(|x| x == c).unwrap();
            color_occurences_guess[index] += 1;
        }

        for i in 0..color_occurences_both.len() {
            if color_occurences_guess[i] <=color_occurences_secret[i] {
                color_occurences_both[i] = color_occurences_guess[i];
            } else {
                color_occurences_both[i] = color_occurences_secret[i];
            }
        }

        let mut black_pins = 0;

        for i in 0..4 {
            if current_guess[i] == self.secret[i] {
                black_pins += 1;
                index = color_list.iter().position(|x| x == &current_guess[i]).unwrap();
                color_occurences_both[index] -= 1;
            }
        }
        let white_pins: i32 = color_occurences_both.iter().sum();

        (black_pins, white_pins)
    }

    fn check_send_guess(&self) -> bool {
        self.guess_input_field.get_send_guess()
    }

    fn create_empty_guessed(number_of_guesses: i32) -> Vec<Vec<GuessField>> {
        let mut guessed = vec![];
        let mut row = 1 + FIELD_SIZE + SPACING;
        for _ in 0..number_of_guesses {
            guessed.push(vec![
                GuessField { x: 1, y: row, color: Colors::Empty },
                GuessField { x: 1 + FIELD_SIZE + SPACING, y: row, color: Colors::Empty },
                GuessField { x: 1 + 2 * FIELD_SIZE + 2 * SPACING, y: row, color: Colors::Empty },
                GuessField { x: 1 + 3 * FIELD_SIZE + 3 * SPACING, y: row, color: Colors::Empty },
            ]);
            row = row + SPACING + FIELD_SIZE;
        }
        return guessed;
    }

    fn create_empty_guess_validation(number_of_guesses: i32) -> Vec<ValidationField> {
        let mut guess_validation = vec![];
        let mut row = 1 + FIELD_SIZE + SPACING;
        let column = 1 + 4 * FIELD_SIZE + 4 * SPACING;
        for _ in 0..number_of_guesses {
            guess_validation.push(ValidationField { x: column, y: row, pins: vec![Colors::Empty; 4] });
            row = row + SPACING + FIELD_SIZE;
        }
        return guess_validation;
    }

    fn create_new_secret() -> Vec<Colors> {
        let color_options = Colors::create_color_list();

        let mut s = 0;
        let mut rng = thread_rng();
        let mut secret = vec![];
        for _ in 0..4 {
            s = rng.gen_range(1..(color_options.len()-1));
            secret.push(color_options[s]);
        }
        return secret;
    }

    fn calculate_guess_input_position(width: i32, height: i32, number_of_guesses: i32) -> (i32, i32) {
        // 2 blocks for sides, for each guess field 2 blocks, for guess validation field 2 blocks = 12 + 4 * Spacing
        let min_width = 12 + 4 * SPACING;
        // 2 sides, secret, guesses, inputfield
        let min_height = 2 + 2 + 2 * number_of_guesses + number_of_guesses * SPACING + SPACING + 2;
        if min_width > width || min_height > height {
            println!("Game configuration too small, min_width: {min_width}, min_height: {min_height}");
            process::exit(0);
        }
        let x = 1;
        let y = 1 + 2 + 2 * number_of_guesses + number_of_guesses * SPACING + SPACING;
        return (x, y);
    }
}

