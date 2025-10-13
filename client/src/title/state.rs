use std::sync::RwLock;
use crate::button::RectangularButton;

#[derive(Debug)]
pub struct TitleState {
    pub debug_button: RwLock<RectangularButton>,
}

impl TitleState {
    pub const DEFAULT: TitleState = TitleState {
        debug_button: RwLock::new(RectangularButton::DEFAULT),
    };
}
