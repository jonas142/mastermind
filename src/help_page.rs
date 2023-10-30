use piston_window::{text, types::Color, Context, G2d, Glyphs, Transformed};

const HELP_MESSAGE: &str = "";

pub fn draw_help_page(
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
