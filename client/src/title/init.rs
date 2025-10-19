use crate::button::RectangularButton;
use crate::font::DEFAULT_FONT_SPACING;
use crate::input::ClickResult;
use crate::map::RenderCoord;
use crate::stage;
use crate::stage::StageType;
use crate::state::STATE;
use crate::title::{
    BUTTON_FONT_SIZE, BUTTON_INTERNAL_MARGIN, BUTTON_TEXT_ARRAY, BUTTON_VERTICAL_MARGIN, SCREEN_MARGIN,
    TITLE_VERTICAL_MARGIN,
};
use raylib::RaylibHandle;
use raylib::ffi::GetFontDefault;
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::WeakFont;
use raylib::text::RaylibFont;
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
        x: max_measure.x + BUTTON_INTERNAL_MARGIN.x * 2.,
        y: max_measure.y + BUTTON_INTERNAL_MARGIN.y * 2.,
    }
});

pub fn init_title(rl: &mut RaylibHandle) {
    if RuntimeEnvironment::default().is_debug() {
        let mut debug_button = STATE.stage.title.debug_button.write().unwrap();
        *debug_button = Some(create_debug_button(rl));
    }

    let mut games_button: RwLockWriteGuard<RectangularButton> = STATE.stage.title.main_buttons[0].write().unwrap();
    *games_button = create_games_button(rl);

    let mut account_button: RwLockWriteGuard<RectangularButton> = STATE.stage.title.main_buttons[1].write().unwrap();
    *account_button = create_account_button(rl);
}

fn create_debug_button(rl: &mut RaylibHandle) -> RectangularButton {
    let mut button: RectangularButton = RectangularButton::new_with_text(
        "Debug",
        Rectangle {
            x: rl.get_screen_width() as f32 - SCREEN_MARGIN - BUTTON_DIMENSIONS.x,
            y: SCREEN_MARGIN,
            width: BUTTON_DIMENSIONS.x,
            height: BUTTON_DIMENSIONS.y,
        },
    );

    fn on_click(_rl: &mut RaylibHandle, _mouse_position: RenderCoord) -> ClickResult {
        stage::register_next(StageType::Game);
        ClickResult::Consume
    }
    button.on_click = on_click;

    button
}

fn create_games_button(rl: &mut RaylibHandle) -> RectangularButton {
    let position: Vector2 = Vector2 {
        x: rl.get_screen_width() as f32 / 2. - BUTTON_DIMENSIONS.x / 2.,
        y: rl.get_screen_height() as f32 / 2. - BUTTON_DIMENSIONS.y / 2. + TITLE_VERTICAL_MARGIN / 2.,
    };

    let button: RectangularButton = RectangularButton::new_with_text(
        BUTTON_TEXT_ARRAY[0],
        Rectangle {
            x: position.x,
            y: position.y,
            width: BUTTON_DIMENSIONS.x,
            height: BUTTON_DIMENSIONS.y,
        },
    );
    button
}

fn create_account_button(rl: &mut RaylibHandle) -> RectangularButton {
    let position: Vector2 = Vector2 {
        x: rl.get_screen_width() as f32 / 2. - BUTTON_DIMENSIONS.x / 2.,
        y: rl.get_screen_height() as f32 / 2. - BUTTON_DIMENSIONS.y / 2.
            + TITLE_VERTICAL_MARGIN / 2.
            + BUTTON_DIMENSIONS.y
            + BUTTON_VERTICAL_MARGIN,
    };

    let button: RectangularButton = RectangularButton::new_with_text(
        BUTTON_TEXT_ARRAY[1],
        Rectangle {
            x: position.x,
            y: position.y,
            width: BUTTON_DIMENSIONS.x,
            height: BUTTON_DIMENSIONS.y,
        },
    );
    button
}
