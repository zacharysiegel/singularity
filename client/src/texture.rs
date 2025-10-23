use crate::state::STATE;
use raylib::consts::{TextureFilter, TextureWrap};
use raylib::prelude::RaylibTexture2D;
use raylib::texture::RenderTexture2D;
use raylib::{ffi, RaylibHandle, RaylibThread};
use std::ops::{Deref, DerefMut};
use std::sync::RwLockWriteGuard;

#[derive(Debug)]
pub struct ScreenRenderTexture(pub RenderTexture2D);

impl Deref for ScreenRenderTexture {
    type Target = RenderTexture2D;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ScreenRenderTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<ffi::Texture2D> for ScreenRenderTexture {
    fn as_ref(&self) -> &ffi::Texture2D {
        &self.0.as_ref()
    }
}

impl ScreenRenderTexture {
    pub fn new(rl: &mut RaylibHandle, rl_thread: &RaylibThread) -> Self {
        let width: u32 = rl.get_screen_width() as u32;
        let height: u32 = rl.get_screen_height() as u32;

        let render_texture: RenderTexture2D = rl.load_render_texture(rl_thread, width, height).unwrap();
        render_texture.set_texture_filter(rl_thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
        render_texture.set_texture_wrap(rl_thread, TextureWrap::TEXTURE_WRAP_CLAMP);

        Self(render_texture)
    }

    pub fn size_differs(&self, width: i32, height: i32) -> bool {
        self.width() != width || self.height() != height
    }
}

pub fn init(rl: &mut RaylibHandle, rl_thread: &RaylibThread) {
    let screen_texture: ScreenRenderTexture = ScreenRenderTexture::new(rl, rl_thread);
    let mut texture_g: RwLockWriteGuard<ScreenRenderTexture> = STATE.screen_texture.write().unwrap();
    *texture_g = screen_texture;
}

pub fn update(rl: &mut RaylibHandle, rl_thread: &RaylibThread) {
    let screen_width: i32 = rl.get_screen_width();
    let screen_height: i32 = rl.get_screen_height();
    if STATE.screen_texture.read().unwrap().size_differs(screen_width, screen_height) {
        let mut screen_texture_g: RwLockWriteGuard<ScreenRenderTexture> = STATE.screen_texture.write().unwrap();
        *screen_texture_g = ScreenRenderTexture::new(rl, rl_thread);
    }
}
