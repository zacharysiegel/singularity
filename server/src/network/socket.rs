use std::net;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use socket2::{Domain, Protocol, SockAddr, Socket, TcpKeepalive, Type};
use tokio::net::TcpListener;

use crate::error::AppError;

pub async fn create_listener(address: &str) -> Result<TcpListener, AppError> {
    let socket: Socket = create_socket(address)?;
    let listener_std: net::TcpListener = net::TcpListener::from(socket);
    listener_std.set_nonblocking(true)?;

    let listener_tokio: TcpListener = TcpListener::from_std(listener_std)?;
    Ok(listener_tokio)
}

fn create_socket(address: &str) -> Result<Socket, AppError> {
    let tcp_keep_alive: TcpKeepalive = TcpKeepalive::new()
        .with_time(Duration::from_secs(60))
        .with_interval(Duration::from_secs(10))
        .with_retries(3);
    let socket_addr =
        SocketAddr::from_str(address).map_err(|err| AppError::from_error_default(Box::new(err)))?;
    let sock_addr = SockAddr::from(socket_addr);

    let socket: Socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
    socket.set_tcp_nodelay(true)?;
    socket.set_tcp_keepalive(&tcp_keep_alive)?;
    socket.set_linger(Some(Duration::from_secs(4)))?;
    socket.set_write_timeout(Some(Duration::from_secs(10)))?;
    socket.bind(&sock_addr)?;
    Ok(socket)
}
