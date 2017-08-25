use piston_window::*;
use side::Side;

pub struct View {
    pub text: String,
    pub side: Option<Side>,
}

impl View {
    pub fn render(&mut self, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
        // we will scale everything according to the window size
        let view_size = c.get_view_size();
        let w = view_size[0];
        let h = view_size[1];

        // calculate proper font size
        let font_size = (w / 512.0 * 32.0) as u32;

        // add some padding for a better view
        let padding = w / 512.0 * 20.0;
        // leave some space for text
        let side_top_padding = (font_size as f64) + padding * 2.0;
        let side_height = (h as f64) - side_top_padding - padding;
        let side_width = (w as f64) * 0.5 - padding * 1.5;

        // which rectangle will be brighter
        let left_color_difference = match self.side {
            None => 0.0,
            Some(Side::Left) => 0.125,
            Some(Side::Right) => -0.125,
        };

        // drawing part

        // clear the screen
        clear([0.5, 0.5, 0.5, 1.0], g);

        // draw text
        text::Text::new(font_size).draw(
            &self.text,
            glyphs,
            &c.draw_state,
            c.transform.trans(padding, (font_size as f64) + padding),
            g,
        );

        // draw left rectangle
        rectangle(
            [0.5 + left_color_difference, 0.0, 0.0, 1.0],
            [padding, side_top_padding, side_width, side_height],
            c.transform,
            g,
        );

        // draw right rectangle
        rectangle(
            [0.5 - left_color_difference, 0.0, 0.0, 1.0],
            [
                side_width + padding * 2.0,
                side_top_padding,
                side_width,
                side_height,
            ],
            c.transform,
            g,
        );
    }
}
