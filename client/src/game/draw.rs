use crate::color::WHITE;
use crate::map::MapCoord;
use crate::shader::{ShaderStore, StandardShader, SHADER_STORE};
use crate::state::STATE;
use crate::texture::ScreenRenderTexture;
use crate::window::{PauseWindow, Window};
use crate::{map, window};
use raylib::drawing::{RaylibDraw, RaylibDrawHandle, RaylibShaderModeExt, RaylibTextureModeExt};
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::Shader;
use raylib::texture::RaylibRenderTexture2D;
use raylib::RaylibThread;
use std::cell::RefMut;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub fn draw(rl_draw: &mut RaylibDrawHandle, rl_thread: &RaylibThread) {
    let screen_width: f32 = rl_draw.get_screen_width() as f32;
    let screen_height: f32 = rl_draw.get_screen_height() as f32;

    let blur: Rc<StandardShader> = SHADER_STORE.with_borrow(|store_m| {
        let store: &ShaderStore = unsafe { store_m.assume_init_ref() };
        store.blur.clone()
    });

    blur.shader.borrow_mut().set_shader_value(blur.uniforms.u_resolution, [screen_width, screen_height]);
    let mut blur_shader_r: RefMut<Shader> = blur.shader.borrow_mut();

    let pause_window: RwLockReadGuard<PauseWindow> = STATE.stage.game.window.pause.read().unwrap();
    if pause_window.is_open() {
        let mut screen_texture: RwLockWriteGuard<ScreenRenderTexture> = STATE.screen_texture.write().unwrap();

        rl_draw.draw_texture_mode(rl_thread, &mut screen_texture, |mut t| {
            draw_game(&mut t, rl_thread);
        });

        rl_draw.draw_shader_mode(blur_shader_r.deref_mut(), |mut s| {
            s.draw_texture_pro(
                &screen_texture.texture(),
                Rectangle::new(
                    0.0,
                    0.0,
                    screen_texture.texture.width as f32,
                    -screen_texture.texture.height as f32, // Textures are drawn with the origin at the bottom left of the screen, so we must translate up
                ),
                Rectangle::new(0.0, 0.0, screen_width, screen_height),
                Vector2::default(),
                0.0,
                WHITE,
            );
        });
    } else {
        draw_game(rl_draw, rl_thread);
    }

    window::draw_overlay_windows(rl_draw, rl_thread);
}

fn draw_game(rl_draw: &mut RaylibDrawHandle, rl_thread: &RaylibThread) {
    let map_origin: RwLockReadGuard<MapCoord> = STATE.stage.game.map.map_origin.read().unwrap();
    map::draw(rl_draw, &map_origin);
    window::draw_game_windows(rl_draw, rl_thread);
}
