use crate::state::STATE;
use raylib::consts::{TextureFilter, TextureWrap};
use raylib::prelude::RenderTexture2D;
use raylib::texture::RaylibTexture2D;
use raylib::{RaylibHandle, RaylibThread};
use std::sync::RwLockWriteGuard;

pub fn init(rl: &mut RaylibHandle, rl_thread: &RaylibThread) {
    // todo: update on resize
    let width: u32 = rl.get_screen_width() as u32;
    let height: u32 = rl.get_screen_height() as u32;

    let render_texture: RenderTexture2D = rl.load_render_texture(rl_thread, width, height).unwrap();
    render_texture.set_texture_filter(rl_thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
    render_texture.set_texture_wrap(rl_thread, TextureWrap::TEXTURE_WRAP_CLAMP);

    let mut render_texture_g: RwLockWriteGuard<RenderTexture2D> = STATE.stage.game.render_texture.write().unwrap();
    *render_texture_g = render_texture;
}
