pub use draw::*;

pub mod draw {
    use crate::color::{TEXT_COLOR, WINDOW_BACKGROUND_COLOR};
    use crate::config::APPLICATION_NAME;
    use crate::font::DEFAULT_FONT_SPACING;
    use crate::util;
    use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
    use raylib::ffi::GetFontDefault;
    use raylib::math::Vector2;
    use raylib::text::{RaylibFont, WeakFont};
    use std::sync::LazyLock;

    const BUTTON_TEXT_ARRAY: [&'static str; 3] = ["Debug", "Account", "Games"];
    const BUTTON_FONT_SIZE: f32 = 18.;
    const BUTTON_MARGIN: f32 = 8.;
    const BUTTON_DIMENSIONS: LazyLock<Vector2> = LazyLock::new(|| {
        let mut max_measure: Vector2 = Vector2 {
            x: f32::NEG_INFINITY,
            y: f32::NEG_INFINITY,
        };

        for text in BUTTON_TEXT_ARRAY {
            let font: WeakFont = unsafe { WeakFont::from_raw(GetFontDefault()) };
            let measure: Vector2 = font.measure_text(text, BUTTON_FONT_SIZE, DEFAULT_FONT_SPACING);
            if measure.x > max_measure.x {
                max_measure = measure;
            }
        }

        Vector2 {
            x: max_measure.x + BUTTON_MARGIN * 2.,
            y: max_measure.y + BUTTON_MARGIN * 2.,
        }
    });

    pub fn draw_title(rl_draw: &mut RaylibDrawHandle) {
        draw_title_text(rl_draw);
        draw_debug_button(rl_draw);
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

    fn draw_debug_button(rl_draw: &mut RaylibDrawHandle) {
        const FONT_SIZE: f32 = 18.;
        const SCREEN_MARGIN: f32 = 20.;

        let text: &str = BUTTON_TEXT_ARRAY[0];
        let position: Vector2 = Vector2 {
            x: rl_draw.get_screen_width() as f32 - SCREEN_MARGIN - BUTTON_DIMENSIONS.x,
            y: SCREEN_MARGIN,
        };

        rl_draw.draw_rectangle_v(position, *BUTTON_DIMENSIONS, WINDOW_BACKGROUND_COLOR);
        rl_draw.draw_text_ex(
            rl_draw.get_font_default(),
            text,
            util::centered_text_origin(
                Vector2 {
                    x: position.x + BUTTON_DIMENSIONS.x / 2.,
                    y: position.y + BUTTON_DIMENSIONS.y / 2.,
                },
                text,
                rl_draw.get_font_default(),
                FONT_SIZE,
                DEFAULT_FONT_SPACING,
            ),
            FONT_SIZE,
            DEFAULT_FONT_SPACING,
            TEXT_COLOR,
        );
    }
}
