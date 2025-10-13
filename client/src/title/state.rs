use std::sync::RwLock;
use crate::button::RectangularButton;

#[derive(Debug)]
pub struct TitleState {
    pub debug_button: RwLock<Option<RectangularButton>>,
}

impl TitleState {
    pub const DEFAULT: TitleState = TitleState {
        debug_button: RwLock::new(None),
    };
}
