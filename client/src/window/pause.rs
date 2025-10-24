use crate::button::RectangularButton;
use crate::input::{ClickHandler, ClickResult, HoverHandler, HoverResult, KeyPressHandler, KeyPressResult};
use crate::map::RenderCoord;
use crate::stage::StageType;
use crate::state::STATE;
use crate::window;
use crate::window::state::WindowLayer;
use crate::window::{Window, BORDER_GAP};
use raylib::consts::KeyboardKey;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use raylib::{RaylibHandle, RaylibThread};
use std::sync::RwLockWriteGuard;

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

    fn handle_window_click(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
        self.exit_button.click(rl, mouse_position)
    }

    fn handle_window_hover(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        self.exit_button.hover(rl, mouse_position)
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
        self.exit_button.on_click = on_click;
    }
}

fn on_click(rl: &mut RaylibHandle, _mouse_position: RenderCoord) -> ClickResult {
    let mut next: RwLockWriteGuard<Option<StageType>> = STATE.stage.next.write().unwrap();
    *next = Some(StageType::Title);

    ClickResult::Consume
}

mod draw {
    use crate::color::{DIFF_HOVER_BUTTON, RED, TEXT_COLOR, WINDOW_BACKGROUND_COLOR};
    use crate::font::DEFAULT_FONT_SPACING;
    use crate::shader::{ExitIconShader, SHADER_STORE};
    use crate::state::STATE;
    use crate::texture::ScreenRenderTexture;
    use crate::window::pause::PAUSE_INTERNAL_MARGIN;
    use crate::window::{PauseWindow, BORDER_GAP, BUTTON_INTERNAL_MARGIN};
    use crate::{math, window};
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
            self.draw_exit_button(rl_draw, rl_thread);
        }

        fn draw_exit_button(&self, rl_draw: &mut RaylibDrawHandle, rl_thread: &RaylibThread) {
            window::draw_side_button(rl_draw, &self.exit_button);

            let mut screen_texture: RwLockWriteGuard<ScreenRenderTexture> = STATE.screen_texture.write().unwrap();
            rl_draw.draw_texture_mode(rl_thread, &mut screen_texture, |mut t| {
                t.draw_rectangle_rec(
                    self.exit_button.rectangle,
                    if self.exit_button.is_hovered() {
                        math::color_add(&WINDOW_BACKGROUND_COLOR, &DIFF_HOVER_BUTTON)
                    } else {
                        WINDOW_BACKGROUND_COLOR
                    },
                );
            });

            let source = Rectangle {
                y: screen_texture.height() as f32 - self.exit_button.rectangle.height - self.exit_button.rectangle.y,
                height: -self.exit_button.rectangle.height,
                ..self.exit_button.rectangle
            };
            let dest = Rectangle {
                x: self.exit_button.rectangle.x + BUTTON_INTERNAL_MARGIN.x,
                y: self.exit_button.rectangle.y + BUTTON_INTERNAL_MARGIN.y,
                width: self.exit_button.rectangle.width - BUTTON_INTERNAL_MARGIN.x * 2.,
                height: self.exit_button.rectangle.height - BUTTON_INTERNAL_MARGIN.y * 2.,
            };

            let exit_icon: Rc<ExitIconShader> =
                SHADER_STORE.with_borrow_mut(|store| unsafe { store.assume_init_ref().exit_icon.clone() });
            exit_icon.set_values(rl_draw, self.exit_button.rectangle);

            rl_draw.draw_shader_mode(&mut exit_icon.standard.shader.borrow_mut(), |mut s| {
                s.draw_texture_pro(&*screen_texture, source, dest, Vector2::zero(), 0., RED)
            });
        }
    }
}
