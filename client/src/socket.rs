use std::net;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use shared::environment::RuntimeEnvironment;
use shared::error::AppError;
use socket2::{Domain, Protocol, SockAddr, Socket, TcpKeepalive, Type};
use tokio::net::TcpStream;

pub fn connect() -> Result<TcpStream, AppError> {
    let sock_addr: SockAddr = get_sock_addr()?;
    let socket: Socket = create_socket()?;
    socket.connect_timeout(&sock_addr, Duration::from_secs(3))?;

    let std_tcp_stream: net::TcpStream = net::TcpStream::from(socket);
    std_tcp_stream.set_nonblocking(true)?; // Required for Tokio

    Ok(TcpStream::from_std(std_tcp_stream)?)
}

fn get_sock_addr() -> Result<SockAddr, AppError> {
    let runtime_env: RuntimeEnvironment = RuntimeEnvironment::from_env()?;
    let address: &str = runtime_env.get_address();
    let socket_addr: SocketAddr = SocketAddr::from_str(address).map_err(|err| {
        AppError::from_error(
            &format!("Error translating address string; [{}]", address),
            Box::new(err),
        )
    })?;
    Ok(SockAddr::from(socket_addr))
}

fn create_socket() -> Result<Socket, AppError> {
    let tcp_keep_alive: TcpKeepalive = TcpKeepalive::new()
        .with_time(Duration::from_secs(60))
        .with_interval(Duration::from_secs(10))
        .with_retries(3);

    let socket: Socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
    socket.set_tcp_nodelay(true)?;
    socket.set_tcp_keepalive(&tcp_keep_alive)?;
    socket.set_linger(Some(Duration::from_secs(4)))?;
    socket.set_write_timeout(Some(Duration::from_secs(10)))?;
    Ok(socket)
}
