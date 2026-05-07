use raylib::math::Vector2;

pub const BUTTON_TEXT_ARRAY: [&'static str; 2] = ["Games", "Account"];
pub const GAMES_BUTTON_INDEX: usize = 0;
pub const ACCOUNT_BUTTON_INDEX: usize = 1;
pub const BUTTON_INTERNAL_MARGIN: Vector2 = Vector2 { x: 14., y: 10. };
pub const SCREEN_MARGIN: f32 = 20.;
pub const TITLE_VERTICAL_MARGIN: f32 = 80.;
pub const BUTTON_VERTICAL_MARGIN: f32 = 20.;

pub const DEBUG_SCROLL_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
    Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, \
    quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. \
    Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. \
    Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
