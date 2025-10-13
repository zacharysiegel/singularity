use crate::button::RectangularButton;
use crate::font::DEFAULT_FONT_SPACING;
use crate::input::ClickResult;
use crate::map::RenderCoord;
use crate::state::STATE;
use crate::title::{BUTTON_FONT_SIZE, BUTTON_MARGIN, BUTTON_TEXT_ARRAY, SCREEN_MARGIN};
use raylib::ffi::GetFontDefault;
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::WeakFont;
use raylib::text::RaylibFont;
use raylib::RaylibHandle;
use shared::environment::RuntimeEnvironment;
use std::sync::{LazyLock, RwLockWriteGuard};

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

pub fn init_title(rl: &mut RaylibHandle) {
    if RuntimeEnvironment::default().is_debug() {
        let mut debug_button = STATE.stage.title.debug_button.write().unwrap();
        *debug_button = Some(create_debug_button(rl));
    }
}

fn create_debug_button(rl: &mut RaylibHandle) -> RectangularButton {
    let mut button: RectangularButton = RectangularButton::new(Rectangle {
        x: rl.get_screen_width() as f32 - SCREEN_MARGIN - BUTTON_DIMENSIONS.x,
        y: SCREEN_MARGIN,
        width: BUTTON_DIMENSIONS.x,
        height: BUTTON_DIMENSIONS.y,
    });

    fn on_click(_rl: &mut RaylibHandle, _mouse_position: RenderCoord) -> ClickResult {
        let mut current_i: RwLockWriteGuard<usize> = STATE.stage.current_index.write().unwrap();
        *current_i = 1;
        ClickResult::Consume
    }
    button.on_click = on_click;

    button
}
