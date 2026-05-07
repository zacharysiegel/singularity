use crate::button::RectangularButton;
use crate::component::scroll_region::VerticalScrollRegion;
use crate::component::text::Text;
use crate::state::{Loading, STATE};
use crate::text_box::TextBox;
use std::sync::{RwLock, RwLockWriteGuard};

#[derive(Debug)]
pub struct TitleState {
    pub debug_button: DebugButton,
    pub debug_text_box: RwLock<Option<TextBox>>,
    pub debug_scroll_region: RwLock<Option<VerticalScrollRegion>>,
    pub main_buttons: [RwLock<RectangularButton>; 2],
}

impl TitleState {
    pub const DEFAULT: TitleState = TitleState {
        debug_button: DebugButton::DEFAULT,
        debug_text_box: RwLock::new(None),
        debug_scroll_region: RwLock::new(None),
        main_buttons: [
            RwLock::new(RectangularButton::DEFAULT),
            RwLock::new(RectangularButton::DEFAULT),
        ],
    };
}

#[derive(Debug)]
pub struct DebugButton {
    pub button: RwLock<Option<RectangularButton>>,
    pub loading: RwLock<Loading>,
}

impl DebugButton {
    pub const DEFAULT: DebugButton = DebugButton {
        button: RwLock::new(None),
        loading: RwLock::new(Loading::Incomplete),
    };
}

pub fn enable_debug() {
    let mut debug_ready: RwLockWriteGuard<Loading> = STATE.stage.title.debug_button.loading.write().unwrap();
    *debug_ready = Loading::Complete;
    drop(debug_ready);

    let mut debug_button: RwLockWriteGuard<Option<RectangularButton>> =
        STATE.stage.title.debug_button.button.write().unwrap();
    if let Some(button) = debug_button.as_mut() {
        button.text = Some(Text::from_str_default("Debug"));
    }
}
