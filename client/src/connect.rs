use std::net;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use shared::error::AppError;
use shared::network;
use shared::network::connection::Connection;
use shared::network::socket;
use socket2::{SockAddr, Socket};
use tokio::net::TcpStream;

pub fn connect() -> Result<Arc<Connection>, AppError> {
    let sock_addr: SockAddr = socket::get_sock_addr()?;
    let socket: Socket = socket::create_socket()?;
    socket.connect_timeout(&sock_addr, Duration::from_secs(3))?;

    let std_tcp_stream: net::TcpStream = net::TcpStream::from(socket);
    std_tcp_stream.set_nonblocking(true)?; // Required for Tokio

    let tcp_stream: TcpStream = TcpStream::from_std(std_tcp_stream)?;
    let peer_addr: SocketAddr = tcp_stream.peer_addr()?;
    let connection: Arc<Connection> = Arc::new(Connection::new(tcp_stream, peer_addr));

    spawn_reader(connection.clone());
    spawn_writer(connection.clone());

    Ok(connection)
}

fn spawn_reader(connection: Arc<Connection>) {
    tokio::spawn(async move {
        network::monitor::monitor_incoming_frames(connection.clone(), |_, frame| async move {
            log::debug!("frame: {:?}", frame);
            // todo: route frames
        })
        .await;
    });
}

fn spawn_writer(connection: Arc<Connection>) {
    // tokio::spawn(async {
    //     todo!("writer")
    // });
}
