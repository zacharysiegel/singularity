use crate::map::MapState;
use crate::player::PlayerState;
use crate::window::WindowState;

#[derive(Debug)]
pub struct GameState {
    pub map: MapState,
    pub player: PlayerState,
    pub window: WindowState,
}

impl GameState {
    pub const DEFAULT: GameState = GameState {
        map: MapState::DEFAULT,
        player: PlayerState::DEFAULT,
        window: WindowState::DEFAULT,
    };
}
