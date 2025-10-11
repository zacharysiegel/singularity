use crate::color::{MAP_BACKGROUND_COLOR, TEXT_COLOR};
use crate::config::APPLICATION_NAME;
use crate::map::coordinate::MapCoord;
use crate::map::draw::{draw_map, draw_players, draw_windows};
use crate::map::init::init_map;
use crate::player::init_players;
use crate::state::STATE;
use crate::{connect, input};
use raylib::callbacks::TraceLogLevel;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::ffi::{
    BeginDrawing, ClearBackground, CloseWindow, Color, DrawFPS, DrawText, EndDrawing, GetMouseWheelMoveV,
    GetScreenHeight, InitWindow, IsWindowReady, SetConfigFlags, SetTargetFPS, SetTraceLogLevel, WindowShouldClose,
};
use raylib::{ffi, math, RaylibHandle, RaylibThread};
use shared::error::AppError;
use shared::network::ring_buffer::RingBuffer;
use std::ffi::CString;
use std::ops::{Add, Mul};
use std::sync::{Arc, RwLockReadGuard, RwLockWriteGuard};
use std::{mem, time};
use time::Instant;
use tokio::sync::RwLock;

pub const TARGET_FPS: u8 = 60;
pub const DISPLAY_WIDTH: u16 = 1600;
pub const DISPLAY_HEIGHT: u16 = 900;

fn scrolled_map_origin(map_origin: &MapCoord) -> MapCoord {
    let scroll: ffi::Vector2 = unsafe { GetMouseWheelMoveV() };
    let scroll_inverted: math::Vector2 = math::Vector2::mul(scroll.into(), math::Vector2 { x: -1., y: -1. }).into();
    let unchecked_origin: math::Vector2 = math::Vector2::add(map_origin.0.into(), scroll_inverted);
    MapCoord(unchecked_origin.into()).overflow_adjusted()
}

fn update() {
    input::handle_user_input();

    let mut map_origin: RwLockWriteGuard<MapCoord> = STATE.map_origin.write().expect("global state poisoned");
    let old: MapCoord = map_origin.clone();
    *map_origin = scrolled_map_origin(&old);
}

fn draw(rl_draw: &mut RaylibDrawHandle) {
    unsafe { ClearBackground(MAP_BACKGROUND_COLOR.into()) };

    let map_origin: RwLockReadGuard<MapCoord> = STATE.map_origin.read().expect("global state poisoned");
    draw_map(&map_origin);
    draw_players(&map_origin);
    draw_windows(rl_draw, &map_origin);
    drop(map_origin);

    unsafe {
        // debug
        DrawFPS(10, 10);
    }
}

pub fn init() -> Result<(RaylibHandle, RaylibThread), AppError> {
    let _: Arc<RwLock<RingBuffer<u8, 4096>>> = connect::connect()?;

    let mut rl;
    let rl_thread;
    unsafe {
        SetTraceLogLevel(ffi::TraceLogLevel::LOG_DEBUG as i32);
        SetTargetFPS(TARGET_FPS as i32);

        SetConfigFlags(ffi::ConfigFlags::FLAG_WINDOW_HIGHDPI as u32 | ffi::ConfigFlags::FLAG_WINDOW_RESIZABLE as u32);
        // let name_cstr: CString = CString::new(APPLICATION_NAME).unwrap();

        let (a, b): (RaylibHandle, RaylibThread) = raylib::init()
            .width(i32::from(DISPLAY_WIDTH))
            .height(i32::from(DISPLAY_HEIGHT))
            .title(APPLICATION_NAME)
            .build();
        rl = a;
        rl_thread = b;

        // InitWindow(DISPLAY_WIDTH.into(), DISPLAY_HEIGHT.into(), name_cstr.as_ptr());
        if !IsWindowReady() {
            return Err(AppError::new("Failed to initialize window"));
        }

        // todo: SetWindowIcon

        {
            let mut rl_draw: RaylibDrawHandle = rl.begin_drawing(&rl_thread);
            rl_draw.clear_background(MAP_BACKGROUND_COLOR);
            // ClearBackground(MAP_BACKGROUND_COLOR.into());
            let loading_cstr: CString = CString::new("Loading").unwrap();
            // DrawText(loading_cstr.as_ptr(), 16, GetScreenHeight() - 30, 20, TEXT_COLOR.into());
            rl_draw.draw_text("Loading", 16, rl_draw.get_screen_height() - 30, 20, TEXT_COLOR);
        }
    }

    init_map();
    init_players(4);

    Ok((rl, rl_thread))
}

pub fn destroy(rl: RaylibHandle) -> Result<(), AppError> {
    drop(rl); // Closes the window
    Ok(())
}

pub fn run(rl: &mut RaylibHandle, rl_thread: &RaylibThread) -> Result<(), AppError> {
    while unsafe { !WindowShouldClose() } {
        let frame_start: Instant = Instant::now();

        update();
        let update_end: Instant = Instant::now();

        let draw_end: Instant;
        unsafe {
            let mut rl_draw: RaylibDrawHandle = rl.begin_drawing(rl_thread);
            // BeginDrawing();
            draw(&mut rl_draw);
            draw_end = Instant::now();
            // EndDrawing();
        }

        let mut frame_counter: RwLockWriteGuard<u64> = STATE.frame_counter.write().expect("global state poisoned");
        *frame_counter += 1;
        drop(frame_counter);
        let frame_counter: RwLockReadGuard<u64> = STATE.frame_counter.read().expect("global state poisoned");
        if *frame_counter % 500 == 0 {
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
