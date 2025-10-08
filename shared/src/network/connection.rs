use crate::error::AppError;
use crate::network::protocol::Frame;
use crate::network::ring_buffer::RingBuffer;
use std::fmt::Debug;
use std::io;
use std::io::IoSliceMut;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::RwLock;

pub const BUFFER_SIZE: usize = 4096;

pub enum BytesRead {
    Some(usize),
    ReadClosed,
}

#[derive(Debug)]
pub struct Connection {
    pub socket_addr: Arc<SocketAddr>,
    pub reader: ConnectionReader,
    pub writer: ConnectionWriter,
}

impl Connection {
    pub fn new(tcp_stream: TcpStream, socket_addr: SocketAddr) -> Self {
        let socket_addr: Arc<SocketAddr> = Arc::new(socket_addr);
        let (reader, writer): (OwnedReadHalf, OwnedWriteHalf) = tcp_stream.into_split();
        Connection {
            socket_addr: socket_addr.clone(),
            reader: ConnectionReader {
                socket_addr: socket_addr.clone(),
                tcp_stream_read: reader,
                buffer: RingBuffer::new(),
            },
            writer: ConnectionWriter {
                socket_addr: socket_addr.clone(),
                tcp_stream_write: writer,
                buffer: Arc::new(RwLock::new(RingBuffer::new())),
            },
        }
    }
}

#[derive(Debug)]
pub struct ConnectionWriter {
    pub socket_addr: Arc<SocketAddr>,
    pub tcp_stream_write: OwnedWriteHalf,
    pub buffer: Arc<RwLock<RingBuffer<u8, BUFFER_SIZE>>>,
}

impl ConnectionWriter {
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<(), AppError> {
        self.tcp_stream_write.write_all(frame.data.as_slice()).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ConnectionReader {
    pub socket_addr: Arc<SocketAddr>,
    pub tcp_stream_read: OwnedReadHalf,
    pub buffer: RingBuffer<u8, BUFFER_SIZE>,
}

impl ConnectionReader {
    pub async fn read_chunk(&mut self) -> Result<BytesRead, AppError> {
        loop {
            self.tcp_stream_read.readable().await?;
            let mut io_slices: [IoSliceMut; 2] = unsafe { self.buffer.current_empty_slices_as_io_slice_mut() };
            let read_r: io::Result<usize> = self.tcp_stream_read.try_read_vectored(&mut io_slices);

            match read_r {
                Ok(0) => {
                    return Ok(BytesRead::ReadClosed);
                }
                Ok(n) => {
                    self.buffer.advance(n)?;
                    return Ok(BytesRead::Some(n));
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // TcpStream may not be ready for read. TcpStream#readable may return false positives.
                    continue;
                }
                Err(e) => return Err(AppError::from(e)),
            }
        }
    }
}
