use crate::component::button::RectangularButton;
use crate::component::vertical_scroll_region::{VerticalScrollRegionUpdate, VerticalScrollRegion};
use crate::component::text::Text;
use crate::component::text::DEFAULT_FONT_SPACING;
use crate::input::{ClickButton, ClickResult};
use crate::stage::StageType;
use crate::state::{Loading, STATE};
use crate::component::text_input::TextInput;
use crate::title::{
    ACCOUNT_BUTTON_INDEX, BUTTON_INTERNAL_MARGIN, BUTTON_TEXT_ARRAY, BUTTON_VERTICAL_MARGIN,
    DEBUG_SCROLL_TEXT, GAMES_BUTTON_INDEX, SCREEN_MARGIN, TITLE_VERTICAL_MARGIN,
};
use raylib::ffi::GetFontDefault;
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::WeakFont;
use raylib::text::RaylibFont;
use raylib::RaylibHandle;
use shared::environment::RuntimeEnvironment;
use shared::map::RenderCoord;
use std::sync::{LazyLock, RwLockWriteGuard};
use crate::conversation;

const BUTTON_DIMENSIONS: LazyLock<Vector2> = LazyLock::new(|| {
    let mut max_measure: Vector2 = Vector2 {
        x: f32::NEG_INFINITY,
        y: f32::NEG_INFINITY,
    };

    for text in BUTTON_TEXT_ARRAY {
        let font: WeakFont = unsafe { WeakFont::from_raw(GetFontDefault()) };
        let measure: Vector2 = font.measure_text(text, crate::component::text::DEFAULT_FONT_SIZE, DEFAULT_FONT_SPACING);
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
        let mut debug_button: RwLockWriteGuard<Option<RectangularButton>> =
            STATE.stage.title.debug_button.button.write().unwrap();
        *debug_button = Some(create_debug_button(rl));

        let mut debug_text_box: RwLockWriteGuard<Option<TextInput>> = STATE.stage.title.debug_text_box.write().unwrap();
        *debug_text_box = Some(create_debug_text_box(rl));

        let mut debug_scroll_region: RwLockWriteGuard<Option<VerticalScrollRegion>> =
            STATE.stage.title.debug_scroll_region.write().unwrap();
        *debug_scroll_region = Some(create_debug_scroll_region(rl));

        conversation::debug::seed_debug_conversations(); // todo: remove
    }

    let mut games_button: RwLockWriteGuard<RectangularButton> = STATE.stage.title.main_buttons[GAMES_BUTTON_INDEX].write().unwrap();
    *games_button = create_games_button(rl);

    let mut account_button: RwLockWriteGuard<RectangularButton> = STATE.stage.title.main_buttons[ACCOUNT_BUTTON_INDEX].write().unwrap();
    *account_button = create_account_button(rl);
}

fn create_debug_button(rl: &mut RaylibHandle) -> RectangularButton {
    let debug_ready: std::sync::RwLockReadGuard<Loading> = STATE.stage.title.debug_button.loading.read().unwrap();
    let label: &str = if *debug_ready == Loading::Complete { "Debug" } else { "Loading..." };
    drop(debug_ready);

    let mut button: RectangularButton = RectangularButton::new_with_text(
        Rectangle {
            x: rl.get_screen_width() as f32 - SCREEN_MARGIN - BUTTON_DIMENSIONS.x,
            y: SCREEN_MARGIN,
            width: BUTTON_DIMENSIONS.x,
            height: BUTTON_DIMENSIONS.y,
        },
        Text::from_str_default(label),
    );

    fn on_click(_rl: &mut RaylibHandle, _button: ClickButton, _press_position: RenderCoord, _release_position: RenderCoord) -> ClickResult {
        STATE.stage.register_next(StageType::Game);
        ClickResult::Consume
    }
    button.on_click = on_click;

    button
}

fn create_debug_text_box(rl: &mut RaylibHandle) -> TextInput {
    TextInput::new_empty(Rectangle {
        x: rl.get_screen_width() as f32 - SCREEN_MARGIN - 300.,
        y: rl.get_screen_height() as f32 - SCREEN_MARGIN - 30.,
        width: 300.,
        height: 30.,
    })
}

fn create_debug_scroll_region(rl: &mut RaylibHandle) -> VerticalScrollRegion {
    let text_box_y: f32 = rl.get_screen_height() as f32 - SCREEN_MARGIN - 30.;
    let scroll_height: f32 = 200.;
    let gap: f32 = 10.;
    let padding: f32 = 8.;
    let line_height: f32 = 18.;
    let viewport_width: f32 = 300.;

    let font: WeakFont = rl.get_font_default();
    let wrap_width: f32 = viewport_width - padding * 2.;
    let wrapped_line_count: usize = crate::component::text_wrap::wrap_text(
        DEBUG_SCROLL_TEXT, &font, 14., 1., wrap_width,
    ).len();
    let content_height: f32 = wrapped_line_count as f32 * line_height;

    let mut scroll_region: VerticalScrollRegion = VerticalScrollRegion::new(
        Rectangle {
            x: rl.get_screen_width() as f32 - SCREEN_MARGIN - viewport_width,
            y: text_box_y - gap - scroll_height,
            width: viewport_width,
            height: scroll_height,
        },
        padding,
    );
    scroll_region.update(VerticalScrollRegionUpdate {
        viewport: None,
        content_height: Some(content_height),
        padding: None,
    });
    scroll_region
}

fn create_games_button(rl: &mut RaylibHandle) -> RectangularButton {
    let position: Vector2 = Vector2 {
        x: rl.get_screen_width() as f32 / 2. - BUTTON_DIMENSIONS.x / 2.,
        y: rl.get_screen_height() as f32 / 2. - BUTTON_DIMENSIONS.y / 2. + TITLE_VERTICAL_MARGIN / 2.,
    };

    let mut button: RectangularButton = RectangularButton::new_with_text(
        Rectangle {
            x: position.x,
            y: position.y,
            width: BUTTON_DIMENSIONS.x,
            height: BUTTON_DIMENSIONS.y,
        },
        Text::from_str_default(BUTTON_TEXT_ARRAY[GAMES_BUTTON_INDEX]),
    );
    button.on_click = on_click;
    fn on_click(_rl: &mut RaylibHandle, _button: ClickButton, _press_position: RenderCoord, _release_position: RenderCoord) -> ClickResult {
        STATE.stage.register_next(StageType::Browser);
        ClickResult::Consume
    }
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
        Rectangle {
            x: position.x,
            y: position.y,
            width: BUTTON_DIMENSIONS.x,
            height: BUTTON_DIMENSIONS.y,
        },
        Text::from_str_default(BUTTON_TEXT_ARRAY[ACCOUNT_BUTTON_INDEX]),
    );
    button
}
