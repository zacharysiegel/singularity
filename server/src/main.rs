use server::environment;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    environment::load_env()?;
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_source_path(true)
        .try_init()?;

    Ok(())
}

