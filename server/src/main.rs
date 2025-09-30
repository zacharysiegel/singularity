use server::environment::{self};
use server::socket::{self};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    environment::load_env()?;
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_source_path(true)
        .try_init()?;

    let listener: TcpListener = socket::create_listener().await?;
    log::debug!("{:?}", listener.ttl());

    Ok(())
}

