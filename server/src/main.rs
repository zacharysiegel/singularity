use server::environment::{self, RuntimeEnvironment};
use server::{monitor, socket};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    environment::load_env()?;
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_source_path(true)
        .try_init()?;

    let runtime_environment: RuntimeEnvironment = RuntimeEnvironment::from_env()?;
    let address: &str = runtime_environment.get_address();
    let listener: TcpListener = socket::create_listener(address).await?;
    log::info!("Listening at {}", address);

    tokio::spawn(monitor::monitor_listener(listener));
    tokio::spawn(monitor::monitor_manager());

    match tokio::signal::ctrl_c().await {
        Ok(_) => graceful_shutdown(),
        Err(err) => {
            log::error!("Failed to await <C-C> signal. Shutting down. [{:#}]", err);
            graceful_shutdown();
        }
    };

    Ok(())
}

fn graceful_shutdown() {
    
}
