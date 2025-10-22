use crate::button::RectangularButton;
use crate::input::{KeyPressHandler, KeyPressResult};
use crate::map::RenderCoord;
use crate::window;
use crate::window::state::WindowLayer;
use crate::window::{Window, BORDER_GAP};
use raylib::consts::KeyboardKey;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use raylib::RaylibHandle;

const PAUSE_WIDTH: f32 = 350.;
const PAUSE_HEIGHT: f32 = 400.;
const PAUSE_INTERNAL_MARGIN: f32 = 14.;

#[derive(Debug)]
pub struct PauseWindow {
    pub origin: Option<RenderCoord>,
    pub dimensions: Vector2,
    pub close_button: RectangularButton,
}

impl Window for PauseWindow {
    fn is_open(&self) -> bool {
        self.origin.is_some()
    }

    fn close(&mut self) {
        self.origin = None;
    }

    fn origin(&self) -> Option<RenderCoord> {
        self.origin
    }

    fn dimensions(&self) -> Vector2 {
        self.dimensions
    }

    fn layer(&self) -> WindowLayer {
        WindowLayer::PauseWindowLayer
    }

    fn close_button(&self) -> &RectangularButton {
        &self.close_button
    }

    fn close_button_mut(&mut self) -> &mut RectangularButton {
        &mut self.close_button
    }

    fn draw_content(&self, rl_draw: &mut RaylibDrawHandle) {
        self.draw_title(rl_draw);
        self.draw_buttons(rl_draw);
    }

    fn handle_window_key_press(&mut self, _rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult {
        if key == KeyboardKey::KEY_P {
            self.close();
        }
        KeyPressResult::Consume
    }
}

impl PauseWindow {
    pub const DEFAULT: PauseWindow = PauseWindow {
        origin: None,
        dimensions: Vector2 { x: 0., y: 0. },
        close_button: RectangularButton::DEFAULT,
    };

    pub fn open(&mut self, rl: &mut RaylibHandle) {
        self.dimensions = Vector2 {
            x: PAUSE_WIDTH,
            y: PAUSE_HEIGHT,
        };
        self.origin = Some(RenderCoord(Vector2 {
            x: (rl.get_screen_width() as f32 - PAUSE_WIDTH) / 2.,
            y: (rl.get_screen_height() as f32 - PAUSE_HEIGHT) / 2.,
        }));
        self.close_button = RectangularButton::new(window::side_button_rectangle(self, 0));
    }
}

mod draw {
    use crate::color::TEXT_COLOR;
    use crate::font::DEFAULT_FONT_SPACING;
    use crate::window::pause::PAUSE_INTERNAL_MARGIN;
    use crate::window::{PauseWindow, BORDER_GAP};
    use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
    use raylib::math::Vector2;

    impl PauseWindow {
        pub fn draw_title(&self, rl_draw: &mut RaylibDrawHandle) {
            let position: Vector2 = self.origin.unwrap().0
                + Vector2 {
                    x: BORDER_GAP + PAUSE_INTERNAL_MARGIN,
                    y: BORDER_GAP + PAUSE_INTERNAL_MARGIN,
                };
            rl_draw.draw_text_ex(
                rl_draw.get_font_default(),
                "Paused",
                position,
                24.,
                DEFAULT_FONT_SPACING,
                TEXT_COLOR,
            );
        }

        pub fn draw_buttons(&self, rl_draw: &mut RaylibDrawHandle) {
            
        }
    }
}
