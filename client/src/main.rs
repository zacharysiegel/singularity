use client::engine;
use std::error::Error;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_source_path(true)
        .try_init()?;

    engine::init()?;
    engine::run()?;
    engine::destroy()?;
    Ok(())
}
