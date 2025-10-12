use crate::color::MAP_BACKGROUND_COLOR;
use crate::config::APPLICATION_NAME;
use crate::map::coordinate::MapCoord;
use crate::map::draw;
use crate::map::draw::{draw_map, draw_players, draw_windows};
use crate::map::init::init_map;
use crate::player::init_players;
use crate::state::STATE;
use crate::{connect, input};
use raylib::callbacks::TraceLogLevel;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::ffi::SetConfigFlags;
use raylib::{ffi, math, RaylibHandle, RaylibThread};
use shared::error::AppError;
use shared::network::ring_buffer::RingBuffer;
use std::ops::{Add, Mul};
use std::sync::{Arc, RwLockReadGuard, RwLockWriteGuard};
use std::time;
use time::Instant;
use tokio::sync::RwLock;

pub const TARGET_FPS: u8 = 60;
pub const DISPLAY_WIDTH: u16 = 1600;
pub const DISPLAY_HEIGHT: u16 = 900;

fn scrolled_map_origin(rl: &mut RaylibHandle, map_origin: &MapCoord) -> MapCoord {
    let scroll: ffi::Vector2 = rl.get_mouse_wheel_move_v();
    let scroll_inverted: math::Vector2 = math::Vector2::mul(scroll.into(), math::Vector2 { x: -1., y: -1. }).into();
    let unchecked_origin: math::Vector2 = math::Vector2::add(map_origin.0.into(), scroll_inverted);
    MapCoord(unchecked_origin.into()).overflow_adjusted()
}

fn update(rl: &mut RaylibHandle) {
    input::handle_user_input(rl);

    let mut map_origin: RwLockWriteGuard<MapCoord> = STATE.map_origin.write().expect("global state poisoned");
    let old: MapCoord = map_origin.clone();
    *map_origin = scrolled_map_origin(rl, &old);
}

fn draw(rl_draw: &mut RaylibDrawHandle) {
    rl_draw.clear_background(MAP_BACKGROUND_COLOR);

    let map_origin: RwLockReadGuard<MapCoord> = STATE.map_origin.read().expect("global state poisoned");
    draw_map(rl_draw, &map_origin);
    draw_players(rl_draw, &map_origin);
    draw_windows(rl_draw);
    drop(map_origin);

    rl_draw.draw_fps(10, 10); // debug
}

pub fn init() -> Result<(RaylibHandle, RaylibThread), AppError> {
    let _: Arc<RwLock<RingBuffer<u8, 4096>>> = connect::connect()?;

    unsafe {
        SetConfigFlags(ffi::ConfigFlags::FLAG_WINDOW_HIGHDPI as u32 | ffi::ConfigFlags::FLAG_WINDOW_RESIZABLE as u32);
    }
    let (mut rl, rl_thread): (RaylibHandle, RaylibThread) = raylib::init()
        .width(i32::from(DISPLAY_WIDTH))
        .height(i32::from(DISPLAY_HEIGHT))
        .title(APPLICATION_NAME)
        .log_level(TraceLogLevel::LOG_DEBUG)
        .build();

    if !rl.is_window_ready() {
        return Err(AppError::new("Failed to initialize window"));
    }

    // todo: SetWindowIcon
    rl.set_target_fps(u32::from(TARGET_FPS));
    draw::draw_loading_init(&mut rl, &rl_thread);

    init_map();
    init_players(4);

    Ok((rl, rl_thread))
}

pub fn destroy(rl: RaylibHandle) -> Result<(), AppError> {
    drop(rl); // Closes the window
    Ok(())
}

pub fn run(rl: &mut RaylibHandle, rl_thread: &RaylibThread) -> Result<(), AppError> {
    while !rl.window_should_close() {
        let frame_start: Instant = Instant::now();

        update(rl);
        let update_end: Instant = Instant::now();

        let draw_end: Instant;
        let mut rl_draw: RaylibDrawHandle = rl.begin_drawing(rl_thread);
        draw(&mut rl_draw);
        draw_end = Instant::now();

        let mut frame_counter: RwLockWriteGuard<u64> = STATE.frame_counter.write().expect("global state poisoned");
        *frame_counter += 1;
        drop(frame_counter);

        let frame_counter: RwLockReadGuard<u64> = STATE.frame_counter.read().expect("global state poisoned");
        if *frame_counter % 1000 == 0 {
            log::debug!(
                "Frame: {}; Update: {:?}; Draw: {:?}; Total: {:?};",
                frame_counter,
                update_end - frame_start,
                draw_end - update_end,
                draw_end - frame_start
            );
        }
    }

    Ok(())
}
