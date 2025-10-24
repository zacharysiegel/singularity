use std::net;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use shared::error::AppError;
use shared::network::connection::{Connection, ConnectionReader, ConnectionWriter, WriteBufferT};
use shared::network::protocol::{Operation, Register};
use shared::network::ring_buffer::RingBuffer;
use shared::network::socket;
use shared::{network, random};
use socket2::{SockAddr, Socket};
use tokio::net::TcpStream;
use tokio::sync::RwLock;

pub fn connect() -> Result<WriteBufferT, AppError> {
    let sock_addr: SockAddr = socket::get_sock_addr()?;
    let socket: Socket = socket::create_socket()?;
    socket.connect_timeout(&sock_addr, Duration::from_secs(3))?;

    let std_tcp_stream: net::TcpStream = net::TcpStream::from(socket);
    std_tcp_stream.set_nonblocking(true)?; // Required for Tokio

    let tcp_stream: TcpStream = TcpStream::from_std(std_tcp_stream)?;
    let peer_addr: SocketAddr = tcp_stream.peer_addr()?;
    let connection: Connection = Connection::new(tcp_stream, peer_addr);
    let write_buffer: Arc<RwLock<RingBuffer<u8, 4096>>> = connection.writer.buffer.clone();

    send_register(write_buffer.clone());
    spawn_reader(connection.reader);
    spawn_writer(connection.writer);

    Ok(write_buffer)
}

fn spawn_reader(reader: ConnectionReader) {
    tokio::spawn(async move {
        network::monitor::monitor_incoming_frames(reader, |w, frame| async move {
            log::debug!("write buffer: {:?}", w);
            log::debug!("frame: {:?}", frame);
            // todo: route frames
        })
        .await;
    });
}

fn spawn_writer(writer: ConnectionWriter) {
    tokio::spawn(async move {
        match network::monitor::monitor_outgoing_frames(writer).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("Error writing frame to the network; {:#}", e);
            }
        }
    });
}

fn send_register(write_buffer: WriteBufferT) {
    let message = Register {
        op_code: Register::OP_CODE,
        user_id: random::random_uuid(),
    };
    tokio::spawn(async move {
        write_buffer.write().await.push(message.as_bytes().as_slice()).expect("Register message failed");
    });
}
