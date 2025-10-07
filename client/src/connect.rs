use std::net;
use std::time::Duration;

use shared::error::AppError;
use socket2::{SockAddr, Socket};
use tokio::net::TcpStream;
use shared::network::socket;

pub fn connect() -> Result<TcpStream, AppError> {
    let sock_addr: SockAddr = socket::get_sock_addr()?;
    let socket: Socket = socket::create_socket()?;
    socket.connect_timeout(&sock_addr, Duration::from_secs(3))?;

    let std_tcp_stream: net::TcpStream = net::TcpStream::from(socket);
    std_tcp_stream.set_nonblocking(true)?; // Required for Tokio

    Ok(TcpStream::from_std(std_tcp_stream)?)
}
