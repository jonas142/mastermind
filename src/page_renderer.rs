use piston_window::{text, types::Color, Context, G2d, Glyphs, Key, Transformed};

use crate::COLOR_BLACK;

const START_PAGE: &str = "Hello World!";

const GAMEOVER_PAGE: &str = "Hello World!";

const GAMEWON_PAGE: &str = "Hello World!";

const HELP_MESSAGE: &str = "                        Help Menu
\r\n    For a general explanation of the
\r\n    game Press \'G\'
\r\n
\r\n                        CONTROL:        
\r\n    Use \'Arrow-Left\' and \'Arrow-Right\' to
\r\n    navigate the input field
\r\n    On an input field use \'Arrow-Up\' and
\r\n    \'Arrow-Down\' to choose a color
\r\n    When you have chosen a color for
\r\n    each input, the button right of the
\r\n    input fields turns green
\r\n    When Submit button is green, move
\r\n    to it and press \'Enter\'
\r\n
\r\n
\r\n
\r\n
\r\n    Press \'Enter\' to return";

const GENERAL_INFO: &str = "                      General Explanation
\r\n    The goal of the game is to find the
\r\n    color combination of the secret.
\r\n    By submitting a color combination,
\r\n    you guess.
\r\n    Your guess is validated by the
\r\n    computer, a black pin in the 
\r\n    validation field next to the guessed 
\r\n    input shows this validation. A black
\r\n    pin means a color is correct and 
\r\n    in the correct position. A white
\r\n    pin means a color is correct but in 
\r\n    the wrong positions. Use these hints
\r\n    to discover the correct color 
\r\n    combination!
\r\n
\r\n    Have Fun!
\r\n
\r\n    Press \'Enter\' to return";

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MenuState {
    Start,
    PausedHelp,
    PausedGeneral,
    GameWon,
    GameOver,
}

pub struct PageRenderer {
    size: u32,
    x: f64,
    y: f64,
    open: bool,
    menu_state: MenuState,
    last_significant_state: MenuState,
}

impl PageRenderer {
    pub fn new(size: u32, x: f64, y: f64, open: bool) -> PageRenderer {
        return PageRenderer {
            size,
            x,
            y,
            open,
            menu_state: MenuState::Start,
            last_significant_state: MenuState::Start,
        };
    }

    pub fn draw_game_state_page(&self, con: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        match self.menu_state {
            MenuState::Start => self.draw_page(START_PAGE, COLOR_BLACK, con, g, glyphs),
            MenuState::PausedHelp => self.draw_page(HELP_MESSAGE, COLOR_BLACK, con, g, glyphs),
            MenuState::PausedGeneral => self.draw_page(GENERAL_INFO, COLOR_BLACK, con, g, glyphs),
            MenuState::GameOver => self.draw_page(GAMEOVER_PAGE, COLOR_BLACK, con, g, glyphs),
            MenuState::GameWon => self.draw_page(GAMEWON_PAGE, COLOR_BLACK, con, g, glyphs),
        }
    }

    pub fn key_pressed(&mut self, key: Key) -> bool {
        /* Returns if it should still show the rendered page */
        match key {
            Key::S
                if {
                    self.menu_state == MenuState::GameOver
                        || self.menu_state == MenuState::GameWon
                        || self.menu_state == MenuState::Start
                } =>
            {
                self.open = false;
                return self.menu_state == MenuState::GameOver
                    || self.menu_state == MenuState::GameWon;
            }
            Key::Return => self.return_or_close(),
            Key::H if { self.menu_state != MenuState::PausedGeneral } => {
                self.menu_state = MenuState::PausedHelp
            }
            Key::G if { self.menu_state == MenuState::PausedHelp } => {
                self.menu_state = MenuState::PausedGeneral
            }
            _ => (),
        }
        return false;
    }

    pub fn is_open(&self) -> bool {
        return self.open;
    }

    pub fn open(&mut self, state: MenuState) {
        self.open = true;
        self.menu_state = state;
        self.last_significant_state = state;
    }

    fn return_or_close(&mut self) {
        match &self.menu_state {
            MenuState::PausedGeneral => self.menu_state = MenuState::PausedHelp,
            MenuState::PausedHelp if { &MenuState::PausedHelp == &self.last_significant_state } => {
                self.open = false
            }
            MenuState::PausedHelp => self.menu_state = self.last_significant_state,
            _ => (),
        }
    }

    fn draw_page(&self, text: &str, color: Color, con: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        let lines = text.lines();
        let mut next_y = self.y;
        for line in lines {
            render_line(self.x, next_y, line, self.size, color, con, g, glyphs);
            next_y += 15.0;
        }
    }
}

fn render_line(
    x: f64,
    y: f64,
    text: &str,
    size: u32,
    color: Color,
    con: &Context,
    g: &mut G2d,
    glyphs: &mut Glyphs,
) {
    text::Text::new_color(color, size)
        .draw(text, glyphs, &con.draw_state, con.transform.trans(x, y), g)
        .unwrap();
}
