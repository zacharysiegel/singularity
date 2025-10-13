pub use draw::*;

pub mod draw {
    use crate::color::TEXT_COLOR;
    use crate::config::APPLICATION_NAME;
    use crate::util;
    use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
    use raylib::math::Vector2;

    pub fn draw_title(rl_draw: &mut RaylibDrawHandle) {
        draw_title_text(rl_draw);
    }

    fn draw_title_text(rl_draw: &mut RaylibDrawHandle) {
        const FONT_SIZE: f32 = 40.;
        const FONT_SPACING: f32 = 4.;
        let text: String = {
            let mut text: String = String::from(APPLICATION_NAME);
            text.replace_range(0..1, &text[0..1].to_uppercase());
            text
        };
        let position: Vector2 = util::centered_text_origin(
            Vector2 {
                x: (rl_draw.get_screen_width() / 2) as f32,
                y: (rl_draw.get_screen_height() / 2) as f32,
            },
            &text,
            rl_draw.get_font_default(),
            FONT_SIZE,
            FONT_SPACING,
        );

        rl_draw.draw_text_ex(
            rl_draw.get_font_default(),
            &text,
            position,
            FONT_SIZE,
            FONT_SPACING,
            TEXT_COLOR,
        );
    }
}
