use client::engine;
use std::error::Error;
use raylib::{RaylibHandle, RaylibThread};
use shared::environment;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    environment::load_env()?;
    env_logger::builder().filter_level(log::LevelFilter::Debug).format_source_path(true).try_init()?;

    let (mut rl, rl_thread): (RaylibHandle, RaylibThread) = engine::init()?;
    engine::run(&mut rl, &rl_thread)?;
    engine::destroy(rl)?;
    Ok(())
}
