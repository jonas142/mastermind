use crate::guess;
use guess::{Colors, GuessInputField};

pub struct Game {
    guess_input_field: GuessInputField,

    width: i32,
    height: i32,

    game_over: bool,
    game_won: bool,
}

