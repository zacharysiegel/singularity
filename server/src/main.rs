use std::error::Error;
use server::listen;
use server::monitor;
use shared::environment::{self};
use tokio::net::TcpListener;
use tokio::sync;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    environment::load_env()?;
    env_logger::builder().filter_level(log::LevelFilter::Debug).format_source_path(true).try_init()?;

    let listener: TcpListener = listen::listen().await?;
    let (cancellation_sender, cancellation_receiver) = sync::broadcast::channel::<()>(1);

    tokio::spawn(monitor::monitor_listener(cancellation_receiver.resubscribe(), listener));
    tokio::spawn(monitor::monitor_manager(cancellation_receiver.resubscribe()));
    drop(cancellation_receiver);

    match tokio::signal::ctrl_c().await {
        Ok(_) => {
            log::info!("Received <C-C> signal");
            graceful_shutdown(cancellation_sender)
        }
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
