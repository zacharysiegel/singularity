use std::ffi::CString;
use crate::config::{APPLICATION_NAME, BACKGROUND_COLOR};
use shared::error::AppError;
use crate::map::{MapCoord, draw_map, draw_players, init_map};
use crate::player::init_players;
use crate::state::{STATE, State};
use raylib::ffi::{
    BeginDrawing, ClearBackground, CloseWindow, Color, DrawFPS, DrawText, EndDrawing,
    GetMouseWheelMoveV, GetScreenHeight, InitWindow, IsKeyPressed, IsWindowReady, SetConfigFlags,
    SetTargetFPS, SetTraceLogLevel, WindowShouldClose,
};
use raylib::{ffi, math};
use std::ops::{Add, Mul};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};
use std::time;
use time::Instant;

pub const TARGET_FPS: u8 = 60;
pub const DISPLAY_WIDTH: u16 = 1600;
pub const DISPLAY_HEIGHT: u16 = 900;

fn scrolled_map_origin(map_origin: &MapCoord) -> MapCoord {
    let scroll: ffi::Vector2 = unsafe { GetMouseWheelMoveV() };
    let scroll_inverted: math::Vector2 =
        math::Vector2::mul(scroll.into(), math::Vector2 { x: -1., y: -1. }).into();
    let unchecked_origin: math::Vector2 = math::Vector2::add(map_origin.0.into(), scroll_inverted);
    MapCoord(unchecked_origin.into()).overflow_adjusted()
}

fn update() {
    if unsafe { IsKeyPressed(ffi::KeyboardKey::KEY_A as i32) } {
        log::debug!("a pressed");
    }

    let mut state: RwLockWriteGuard<State> = STATE.write().expect("global state poisoned");
    let old: MapCoord = state.map_origin.clone();
    state.map_origin = scrolled_map_origin(&old);
    if old.x != state.map_origin.x || old.y != state.map_origin.y {
        log::debug!("({}, {})", state.map_origin.x, state.map_origin.y);
    }
}

fn draw() {
    unsafe { ClearBackground(BACKGROUND_COLOR.into()) };

    let state: RwLockReadGuard<State> = STATE.read().expect("global state poisoned");
    draw_map(&state.map_origin);
    draw_players(&state.map_origin);

    unsafe {
        // debug
        DrawFPS(10, 10);
    }
}

pub fn init() -> Result<(), AppError> {
    unsafe {
        SetTraceLogLevel(ffi::TraceLogLevel::LOG_DEBUG as i32);
        SetTargetFPS(TARGET_FPS as i32);

        SetConfigFlags(
            ffi::ConfigFlags::FLAG_WINDOW_HIGHDPI as u32
                | ffi::ConfigFlags::FLAG_WINDOW_RESIZABLE as u32,
        );
        let name_cstr: CString = CString::new(APPLICATION_NAME).unwrap();
        InitWindow(
            DISPLAY_WIDTH.into(),
            DISPLAY_HEIGHT.into(),
            name_cstr.as_ptr(),
        );
        if !IsWindowReady() {
            return Err(AppError::new("Failed to initialize window"));
        }

        BeginDrawing();
        ClearBackground(BACKGROUND_COLOR.into());
        let loading_cstr: CString = CString::new("Loading").unwrap();
        DrawText(
            loading_cstr.as_ptr(),
            16,
            GetScreenHeight() - 30,
            20,
            Color {
                r: 0xf0,
                g: 0xf0,
                b: 0xf0,
                a: 0xff,
            },
        );
        EndDrawing();
    }

    init_map();
    init_players(4);

    Ok(())
}

pub fn destroy() -> Result<(), AppError> {
    unsafe { CloseWindow() };
    Ok(())
}

pub fn run() -> Result<(), AppError> {
    while unsafe { !WindowShouldClose() } {
        let frame_start: Instant = Instant::now();

        update();
        let update_end: Instant = Instant::now();

        let draw_end: Instant;
        unsafe {
            BeginDrawing();
            draw();
            draw_end = Instant::now();
            EndDrawing();
        }

        let mut state: RwLockWriteGuard<State> = STATE.write().expect("global state poisoned");
        state.frame_counter += 1;
        drop(state);
        let state: RwLockReadGuard<State> = STATE.read().expect("global state poisoned");
        if state.frame_counter % 1000 == 0 {
            log::debug!(
                "Frame: {}; Update: {:?}; Draw: {:?}; Total: {:?};",
                state.frame_counter,
                update_end - frame_start,
                draw_end - update_end,
                draw_end - frame_start
            );
        }
    }

    Ok(())
}
