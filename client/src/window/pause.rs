use crate::button::RectangularButton;
use crate::input::{ClickResult, KeyPressHandler, KeyPressResult};
use crate::map::RenderCoord;
use crate::window;
use crate::window::state::WindowLayer;
use crate::window::{Window, BORDER_GAP};
use raylib::consts::KeyboardKey;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use raylib::{RaylibHandle, RaylibThread};

const PAUSE_WIDTH: f32 = 350.;
const PAUSE_HEIGHT: f32 = 400.;
const PAUSE_INTERNAL_MARGIN: f32 = 14.;

#[derive(Debug)]
pub struct PauseWindow {
    pub origin: Option<RenderCoord>,
    pub dimensions: Vector2,
    pub close_button: RectangularButton,
    pub exit_button: RectangularButton,
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

    fn draw_content(&self, rl_draw: &mut RaylibDrawHandle, rl_thread: &RaylibThread) {
        self.draw_title(rl_draw);
        self.draw_buttons(rl_draw, rl_thread);
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
        exit_button: RectangularButton::DEFAULT,
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
        self.exit_button = RectangularButton::new(window::side_button_rectangle(self, 1));
        self.exit_button.on_click = exit;
    }
}

fn exit(rl: &mut RaylibHandle, _mouse_position: RenderCoord) -> ClickResult {
    ClickResult::Consume
}

mod draw {
    use crate::color::{GREEN, TEXT_COLOR, TRANSPARENT};
    use crate::font::DEFAULT_FONT_SPACING;
    use crate::shader::{StandardShader, SHADER_STORE};
    use crate::state::STATE;
    use crate::texture::ScreenRenderTexture;
    use crate::window;
    use crate::window::pause::PAUSE_INTERNAL_MARGIN;
    use crate::window::{PauseWindow, BORDER_GAP, BORDER_THICKNESS};
    use raylib::color::Color;
    use raylib::drawing::{RaylibDraw, RaylibDrawHandle, RaylibShaderModeExt, RaylibTextureModeExt};
    use raylib::math::{Rectangle, Vector2};
    use raylib::texture::RaylibTexture2D;
    use raylib::RaylibThread;
    use std::rc::Rc;
    use std::sync::RwLockWriteGuard;

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

        pub fn draw_buttons(&self, rl_draw: &mut RaylibDrawHandle, rl_thread: &RaylibThread) {
            let exit_icon: Rc<StandardShader> =
                SHADER_STORE.with_borrow_mut(|store| unsafe { store.assume_init_ref().exit_icon.clone() });
            let u_resolution: [f32; 2] = [rl_draw.get_screen_width() as f32, rl_draw.get_screen_height() as f32];
            exit_icon.shader.borrow_mut().set_shader_value(exit_icon.uniforms.u_resolution, u_resolution);

            let mut screen_texture: RwLockWriteGuard<ScreenRenderTexture> = STATE.screen_texture.write().unwrap();

            rl_draw.draw_shader_mode(&mut exit_icon.shader.borrow_mut(), |mut s| {
                s.draw_texture_mode(rl_thread, &mut screen_texture, |mut t| {
                    let m = t.get_mouse_position();
                    t.draw_rectangle_rec(self.exit_button.rectangle, TRANSPARENT);
                    t.draw_circle_v(m, 5., GREEN);
                });
            });

            let source = Rectangle {
                y: screen_texture.height() as f32 - self.exit_button.rectangle.height - self.exit_button.rectangle.y,
                height: -self.exit_button.rectangle.height,
                ..self.exit_button.rectangle
            };
            let dest = Rectangle {
                x: self.exit_button.rectangle.x + BORDER_THICKNESS,
                y: self.exit_button.rectangle.y + BORDER_THICKNESS,
                width: self.exit_button.rectangle.width - BORDER_THICKNESS * 2.,
                height: self.exit_button.rectangle.height - BORDER_THICKNESS * 2.,
            };
            window::draw_side_button(rl_draw, &self.exit_button);
            rl_draw.draw_texture_pro(&*screen_texture, source, dest, Vector2::zero(), 0., Color::WHITE)
        }
    }
}
