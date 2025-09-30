use std::net::SocketAddr;

use server::environment::{self, RuntimeEnvironment};
use server::socket::{self};
use tokio::net::{TcpListener, TcpStream};

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

    loop {
        let accept_r = listener.accept().await;

        match accept_r {
            Ok(val) => {
                register_client(val.0, val.1).await;
            }
            Err(err) => {
                log::error!("Failed to accept TCP connection; {:#}", err);
                break;
            }
        }
    }

    Ok(())
}

#[allow(unused)]
async fn register_client(tcp_stream: TcpStream, socket_addr: SocketAddr) {}
