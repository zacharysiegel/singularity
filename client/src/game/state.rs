use crate::map::MapState;
use crate::player::PlayerState;
use crate::window::WindowState;
use raylib::texture::RenderTexture2D;
use std::mem;
use std::sync::RwLock;

#[derive(Debug)]
pub struct GameState {
    pub map: MapState,
    pub player: PlayerState,
    pub window: WindowState,
    pub render_texture: RwLock<RenderTexture2D>,
}

impl GameState {
    pub const DEFAULT: GameState = GameState {
        map: MapState::DEFAULT,
        player: PlayerState::DEFAULT,
        window: WindowState::DEFAULT,
        render_texture: RwLock::new(unsafe { mem::zeroed() }),
    };
}
