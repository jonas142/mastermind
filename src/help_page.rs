use piston_window::{text, types::Color, Context, G2d, Glyphs, Key, Transformed};

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
\r\n    Press \'Enter\' to return to the game";

const GENERAL_INFO: &str = "                        General Explanation
\r\n    The goal of the game is to find the color combination of the secret.
\r\n    By submitting a color combination you guess.
\r\n    Your guess is validated by the computer, a black pin in the validation field next to the guessed input shows this validation. A black pin means a color is correct and in the correct position. A white pin means a color is correct but in the wrong positions. Use these hints to discover the correct color combination!
\r\n    Have Fun!";

pub struct PageRenderer {
    size: u32,
    x: f64,
    y: f64,
    current_depth: u8,
}
impl PageRenderer {
    pub fn new(size: u32, x: f64, y: f64) -> PageRenderer {
        return PageRenderer {
            size,
            x,
            y,
            current_depth: 0,
        };
    }

    pub fn draw(&self, color: Color, con: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        match self.current_depth {
            1 => self.draw_page(HELP_MESSAGE, color, con, g, glyphs),
            2 => self.draw_page(GENERAL_INFO, color, con, g, glyphs),
            _ => (),
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        /* Returns if it should still show the rendered page */
        match key {
            Key::Return => self.current_depth -= 1,
            Key::G => self.current_depth = 2,
            _ => (),
        }
    }

    pub fn open_help(&mut self) {
        self.current_depth = 1;
    }

    pub fn is_open(&self) -> bool {
        return self.current_depth > 0;
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
