use std::net;

use socket2::{SockAddr, Socket};
use tokio::net::TcpListener;

use shared::error::AppError;
use shared::network::socket;

pub async fn listen() -> Result<TcpListener, AppError> {
    let socket: Socket = socket::create_socket()?;
    let sock_addr: SockAddr = socket::get_sock_addr()?;

    socket.bind(&sock_addr)?;
    socket.listen(128)?;
    log::info!(
        "Listening at {} {}",
        sock_addr.as_socket_ipv4().unwrap().ip(),
        sock_addr.as_socket_ipv4().unwrap().port()
    );

    let listener_std: net::TcpListener = net::TcpListener::from(socket);
    listener_std.set_nonblocking(true)?; // Required for Tokio

    let listener_tokio: TcpListener = TcpListener::from_std(listener_std)?;
    Ok(listener_tokio)
}
