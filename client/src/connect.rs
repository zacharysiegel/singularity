use std::net;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use crate::route;
use shared::environment::RuntimeEnvironment;
use shared::error::AppError;
use shared::srtp::connection::{Connection, ConnectionReader, ConnectionWriter, WriteBufferT, BUFFER_SIZE};
use shared::srtp::protocol::Register;
use shared::srtp::ring_buffer::RingBuffer;
use shared::srtp::{protocol, socket};
use shared::{srtp, random};
use socket2::{SockAddr, Socket};
use tokio::net::TcpStream;
use tokio::sync::RwLock;
// todo: close connection during engine::destroy

pub fn connect() -> Result<WriteBufferT, AppError> {
    let sock_addr: SockAddr = socket::get_sock_addr()?;
    let socket: Socket = socket::create_socket()?;
    socket.connect_timeout(&sock_addr, Duration::from_secs(3))?;

    let std_tcp_stream: net::TcpStream = net::TcpStream::from(socket);
    std_tcp_stream.set_nonblocking(true)?; // Required for Tokio

    let tcp_stream: TcpStream = TcpStream::from_std(std_tcp_stream)?;
    let peer_addr: SocketAddr = tcp_stream.peer_addr()?;
    let connection: Connection = Connection::new(tcp_stream, peer_addr);
    let write_buffer: Arc<RwLock<RingBuffer<u8, { BUFFER_SIZE }>>> = connection.writer.buffer.clone();

    send_register(write_buffer.clone());
    spawn_reader(connection.reader);
    spawn_writer(connection.writer);

    Ok(write_buffer)
}

fn spawn_reader(reader: ConnectionReader) {
    tokio::spawn(async {
        srtp::monitor::monitor_incoming_frames(reader, |write_buffer, frame| async {
            route::route_frame(write_buffer, frame).await;
        })
        .await;
    });
}

fn spawn_writer(writer: ConnectionWriter) {
    tokio::spawn(async move {
        match srtp::monitor::monitor_outgoing_frames(writer).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("Error writing frame to the network; {:#}", e);
            }
        }
    });
}

fn send_register(write_buffer: WriteBufferT) {
    let message = Register {
        user_id: random::random_uuid(),
        client_debug: RuntimeEnvironment::default().is_debug(),
    };
    tokio::spawn(async move { protocol::enqueue_message(write_buffer, message).await });
}
