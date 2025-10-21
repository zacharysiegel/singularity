use crate::color::{MAP_BACKGROUND_COLOR, WHITE};
use crate::map::MapCoord;
use crate::shader::{ShaderStore, StandardShader, SHADER_STORE};
use crate::state::STATE;
use crate::{map, title};
use raylib::drawing::{RaylibDraw, RaylibDrawHandle, RaylibShaderModeExt, RaylibTextureModeExt};
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::RenderTexture2D;
use raylib::texture::RaylibRenderTexture2D;
use raylib::RaylibThread;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub fn draw_stage_title(rl_draw: &mut RaylibDrawHandle) {
    title::draw_title(rl_draw);
}

pub fn draw_stage_map(rl_draw: &mut RaylibDrawHandle, rl_thread: &RaylibThread) {
    let blur: Rc<StandardShader> = SHADER_STORE.with_borrow_mut(|store_m| {
        let store: &mut ShaderStore = unsafe { store_m.assume_init_mut() };
        store.blur.clone()
    });

    let dimensions: [f32; 2] = [rl_draw.get_screen_width() as f32, rl_draw.get_screen_height() as f32];
    blur.shader.borrow_mut().set_shader_value_v(blur.uniforms.u_dimensions, &dimensions);

    let mut game_texture: RwLockWriteGuard<RenderTexture2D> = STATE.stage.game.render_texture.write().unwrap();
    rl_draw.draw_texture_mode(rl_thread, &mut game_texture, |mut t| {
        draw_map_texture(t.deref_mut());
    });

    rl_draw.draw_shader_mode(&mut blur.shader.borrow_mut(), |mut s| {
        s.draw_texture_pro(
            &game_texture.texture(),
            Rectangle::new(
                0.0,
                0.0,
                game_texture.texture.width as f32,
                -game_texture.texture.height as f32, // Textures are drawn with the origin at the bottom left of the screen, so we must translate up
            ),
            Rectangle::new(0.0, 0.0, s.get_screen_width() as f32, s.get_screen_height() as f32),
            Vector2::zero(),
            0.0,
            WHITE,
        );
    });
    map::draw_windows(rl_draw);
}

fn draw_map_texture(rl_draw: &mut RaylibDrawHandle) {
    rl_draw.clear_background(MAP_BACKGROUND_COLOR);

    let map_origin: RwLockReadGuard<MapCoord> = STATE.stage.game.map.map_origin.read().expect("global state poisoned");
    map::draw_map(rl_draw, &map_origin);
    map::draw_players(rl_draw, &map_origin);
}
