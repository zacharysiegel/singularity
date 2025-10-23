use crate::state::STATE;
use crate::texture::ScreenRenderTexture;
use raylib::{RaylibHandle, RaylibThread};
use std::sync::RwLockWriteGuard;

pub fn init(rl: &mut RaylibHandle, rl_thread: &RaylibThread) {
    let screen_texture: ScreenRenderTexture = ScreenRenderTexture::new(rl, rl_thread);
    let mut texture_g: RwLockWriteGuard<ScreenRenderTexture> = STATE.screen_texture.write().unwrap();
    *texture_g = screen_texture;
}
