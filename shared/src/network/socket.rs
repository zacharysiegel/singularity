use crate::environment::RuntimeEnvironment;
use crate::error::AppError;
use socket2::{Domain, Protocol, SockAddr, Socket, TcpKeepalive, Type};
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

pub fn get_sock_addr() -> Result<SockAddr, AppError> {
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

pub fn create_socket() -> Result<Socket, AppError> {
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
