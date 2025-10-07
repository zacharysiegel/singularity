use server::monitor;
use server::socket;
use shared::environment::{self, RuntimeEnvironment};
use tokio::net::TcpListener;
use tokio::sync;

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

    let (cancellation_sender, cancellation_receiver) = sync::broadcast::channel::<()>(1);

    tokio::spawn(monitor::monitor_listener(cancellation_receiver.resubscribe(), listener));
    tokio::spawn(monitor::monitor_manager(cancellation_receiver.resubscribe()));
    drop(cancellation_receiver);

    match tokio::signal::ctrl_c().await {
        Ok(_) => graceful_shutdown(cancellation_sender),
        Err(err) => {
            log::error!("Failed to await <C-C> signal. Shutting down. [{:#}]", err);
            graceful_shutdown(cancellation_sender);
        }
    };

    Ok(())
}

fn graceful_shutdown(cancellation_sender: sync::broadcast::Sender<()>) {
    match cancellation_sender.send(()) {
        Ok(val) => {
            log::debug!("Shutdown message sent to {} receivers", val);
        }
        Err(err) => {
            log::error!("Failed to send shutdown message [{:#}]", err);
        }
    }
}
