//! All multi-byte fields should be interpreted in Big-Endian order
//! Each frame begins with a 1-byte operation code
//! A frame can be fixed-length or variable-length
//!     If fixed, the frame's data immediately follows the operation code
//!     If variable, the frame's total length is written as a 2-byte Big-Endian unsigned integer

use crate::error::AppError;
use crate::network::frame::OperationType;
use crate::network::ring_buffer::{RingBuffer, RingBufferView};
use std::io;
use std::io::IoSliceMut;
use std::net::SocketAddr;
use tokio::net::TcpStream;

const BUFFER_SIZE: usize = 4096;

enum BytesRead {
    Some(usize),
    ReadClosed,
}

pub struct Connection {
    pub tcp_stream: TcpStream,
    pub socket_addr: SocketAddr,
    pub buffer: RingBuffer<u8, BUFFER_SIZE>,
}

impl Connection {
    pub fn new(tcp_stream: TcpStream, socket_addr: SocketAddr) -> Self {
        Connection {
            tcp_stream,
            socket_addr,
            buffer: RingBuffer::new(),
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<(OperationType, Vec<u8>)>, AppError> {
        let bytes_read: BytesRead = self.read_chunk().await?;

        match bytes_read {
            BytesRead::Some(size) => {
                assert!(size > 0); // Precondition of entering this branch

                let op_code_view: RingBufferView<u8> = self.buffer.pop(1)?; // Must be modified if OpCode changes size
                let op_type: OperationType = OperationType::from_op_code(&op_code_view[0])?;

                let mut frame_vec: Vec<u8> = op_code_view.into();
                let frame_size: usize = op_type.fixed_size().unwrap_or_else(|| {
                    let length_view: RingBufferView<u8> = self.buffer.pop(2).unwrap();
                    u16::from_be_bytes([length_view[0], length_view[1]]) as usize
                });
                let rest_view: RingBufferView<u8> = self.buffer.pop(frame_size - 1)?;

                assert_eq!(1, frame_vec.len());
                assert_eq!(frame_size, 1 + rest_view.len());
                frame_vec.reserve_exact(frame_size - 1);
                rest_view.copy_to(&mut frame_vec.as_mut_slice()[1..]);

                assert_eq!(frame_size, frame_vec.len());
                Ok(Some((op_type, frame_vec)))
            }
            BytesRead::ReadClosed => Ok(None),
        }
    }

    pub async fn write_frame(&mut self, frame: &OperationType) -> Result<(), AppError> {
        todo!()
    }

    async fn read_chunk(&mut self) -> Result<BytesRead, AppError> {
        loop {
            self.tcp_stream.readable().await?;
            let mut io_slices: [IoSliceMut; 2] =
                unsafe { self.buffer.current_empty_slices_as_io_slice_mut() };
            let read_r: io::Result<usize> = self.tcp_stream.try_read_vectored(&mut io_slices);

            match read_r {
                Ok(0) => {
                    return Ok(BytesRead::ReadClosed);
                }
                Ok(n) => return Ok(BytesRead::Some(n)),
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // TcpStream may not be ready for read. TcpStream#readable may return false positives.
                    continue;
                }
                Err(e) => return Err(AppError::from(e)),
            }
        }
    }
}
