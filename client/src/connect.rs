use std::net;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use crate::route;
use shared::environment::RuntimeEnvironment;
use shared::error::AppError;
use shared::srtp;
use shared::srtp::connection::{BUFFER_SIZE, Connection, ConnectionReader, ConnectionWriter, WriteBufferT};
use shared::srtp::protocol::Register;
use shared::srtp::ring_buffer::RingBuffer;
use shared::srtp::{protocol, socket};
use socket2::{SockAddr, Socket};
use tokio::net::TcpStream;
use tokio::sync::{Notify, RwLock};
use uuid::Uuid;
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
    let shutdown: Arc<Notify> = Arc::new(Notify::new());

    send_register(write_buffer.clone());
    spawn_reader(connection.reader, shutdown.clone());
    spawn_writer(connection.writer, shutdown);

    Ok(write_buffer)
}

fn spawn_reader(reader: ConnectionReader, shutdown: Arc<Notify>) {
    tokio::spawn(async move {
        srtp::monitor::monitor_incoming_frames(reader, |write_buffer, frame| async {
            route::route_frame(write_buffer, frame).await;
        })
        .await;
        shutdown.notify_one();
    });
}

fn spawn_writer(writer: ConnectionWriter, shutdown: Arc<Notify>) {
    tokio::spawn(async move {
        match srtp::monitor::monitor_outgoing_frames(writer, shutdown).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("Error writing frame to the network; {:#}", e);
            }
        }
    });
}

fn send_register(write_buffer: WriteBufferT) {
    let message = Register {
        user_id: Uuid::now_v7(),
        client_debug: RuntimeEnvironment::default().is_debug(),
    };
    tokio::spawn(async move { protocol::enqueue_message(write_buffer, message).await });
}
