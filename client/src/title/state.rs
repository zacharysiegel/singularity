use crate::button::RectangularButton;
use std::sync::RwLock;

#[derive(Debug)]
pub struct TitleState {
    pub debug_button: RwLock<Option<RectangularButton>>,
    pub main_buttons: [RwLock<RectangularButton>; 2],
}

impl TitleState {
    pub const DEFAULT: TitleState = TitleState {
        debug_button: RwLock::new(None),
        main_buttons: [
            RwLock::new(RectangularButton::DEFAULT),
            RwLock::new(RectangularButton::DEFAULT),
        ],
    };
}
