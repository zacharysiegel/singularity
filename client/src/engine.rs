use crate::color::{MAP_BACKGROUND_COLOR, TEXT_COLOR};
use crate::config::APPLICATION_NAME;
use crate::stage::StageType;
use crate::state::STATE;
use crate::{connect, input, map, player, shader, stage, texture, title};
use raylib::callbacks::TraceLogLevel;
use raylib::consts::KeyboardKey;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::ffi::rlGetVersion;
use raylib::{RaylibHandle, RaylibThread};
use shared::environment::RuntimeEnvironment;
use shared::error::AppError;
use shared::network::ring_buffer::RingBuffer;
use std::sync::{Arc, RwLockReadGuard, RwLockWriteGuard};
use std::time;
use time::Instant;
use tokio::sync::RwLock;

pub const TARGET_FPS: u8 = 60;
pub const DISPLAY_WIDTH: u16 = 1600;
pub const DISPLAY_HEIGHT: u16 = 900;

fn update(rl: &mut RaylibHandle, rl_thread: &RaylibThread) {
    texture::update(rl, rl_thread);
    stage::update();
    input::handle_user_input(rl);
}

fn draw(rl_draw: &mut RaylibDrawHandle, rl_thread: &RaylibThread) {
    let current_stage: RwLockReadGuard<StageType> = STATE.stage.current.read().unwrap();
    current_stage.draw(rl_draw, rl_thread);
    drop(current_stage);

    if RuntimeEnvironment::default().is_debug() {
        rl_draw.draw_fps(10, 10);
    }
}

pub fn init() -> Result<(RaylibHandle, RaylibThread), AppError> {
    let _: Arc<RwLock<RingBuffer<u8, 4096>>> = connect::connect()?;

    unsafe {
        log::info!("OpenGL version: {}", rlGetVersion());
    }

    let (mut rl, rl_thread): (RaylibHandle, RaylibThread) = raylib::init()
        .width(i32::from(DISPLAY_WIDTH))
        .height(i32::from(DISPLAY_HEIGHT))
        .title(APPLICATION_NAME)
        .log_level(TraceLogLevel::LOG_DEBUG)
        .resizable()
        .msaa_4x()
        .build();

    if !rl.is_window_ready() {
        return Err(AppError::new("Failed to initialize window"));
    }

    // todo: SetWindowIcon
    rl.set_target_fps(u32::from(TARGET_FPS));

    if RuntimeEnvironment::default().is_debug() {
        rl.set_exit_key(Some(KeyboardKey::KEY_F1));
    } else {
        rl.set_exit_key(None);
    }

    draw_loading_init(&mut rl, &rl_thread);

    texture::init(&mut rl, &rl_thread);
    shader::init(&mut rl, &rl_thread);
    title::init_title(&mut rl);
    map::init_map();
    player::init_players(4);

    Ok((rl, rl_thread))
}

pub fn destroy(rl: RaylibHandle) -> Result<(), AppError> {
    drop(rl); // Closes the window
    Ok(())
}

pub fn run(rl: &mut RaylibHandle, rl_thread: &RaylibThread) -> Result<(), AppError> {
    while !rl.window_should_close() {
        let frame_start: Instant = Instant::now();

        update(rl, rl_thread);
        let update_end: Instant = Instant::now();

        let draw_end: Instant;
        let mut rl_draw: RaylibDrawHandle = rl.begin_drawing(rl_thread);
        draw(&mut rl_draw, rl_thread);
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

pub fn draw_loading_init(rl: &mut RaylibHandle, rl_thread: &RaylibThread) {
    let mut rl_draw: RaylibDrawHandle = rl.begin_drawing(&rl_thread);
    rl_draw.clear_background(MAP_BACKGROUND_COLOR);
    rl_draw.draw_text("Loading", 16, rl_draw.get_screen_height() - 30, 20, TEXT_COLOR);
    drop(rl_draw);
}
