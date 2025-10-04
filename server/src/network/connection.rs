//! All multi-byte fields should be interpreted in Big-Endian order.
//! Each frame begins with a 1-byte operation code.
//! A frame can be fixed-length or variable-length.
//! If fixed, the frame's data immediately follows the operation code.
//! If variable, the frame's total length is written as a 2-byte Big-Endian unsigned integer.
//! The operation code and optional length field constitute the frame's "head".
//! The rest of the frame is considered the frame's "body".

use crate::error::AppError;
use crate::network::frame;
use crate::network::frame::{Frame, OperationType};
use crate::network::ring_buffer::{RingBuffer, RingBufferView};
use frame::Head;
use std::fmt::{Display, Formatter};
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

    pub async fn read_frames(&mut self) -> Result<Option<Vec<Frame>>, AppError> {
        let mut frames: Vec<Frame> = Vec::new();
        let bytes_read: BytesRead = self.read_chunk().await?;

        match bytes_read {
            BytesRead::Some(size) => {
                assert!(size > 0); // Precondition of entering this branch

                loop {
                    let bytes_remaining: usize = self.buffer.used_space();
                    let Some(head) = self.peek_frame_head()? else {
                        break;
                    };
                    if head.length > bytes_remaining {
                        break;
                    }

                    let frame_data: Vec<u8> = self.pop_frame(&head)?;
                    let frame: Frame = Frame {
                        head,
                        data: frame_data,
                    };
                    frames.push(frame);
                }
            }
            BytesRead::ReadClosed => {
                return Ok(None);
            }
        }

        Ok(Some(frames))
    }

    #[allow(unused)]
    pub async fn write_frame(&mut self, frame: &OperationType) -> Result<(), AppError> {
        todo!()
    }

    fn peek_frame_head(&self) -> Result<Option<Head>, AppError> {
        if self.buffer.used_space() < 1 {
            return Ok(None);
        }
        let op_code_view: RingBufferView<u8> = self.buffer.peek(1)?; // Must be modified if OpCode changes size
        let op_type: OperationType = OperationType::from_op_code(&op_code_view[0])?;

        let frame_size: usize = match op_type.fixed_size() {
            None => {
                if self.buffer.used_space() < 3 {
                    return Ok(None);
                }
                let length_view: RingBufferView<u8> = self.buffer.peek(3)?;
                u16::from_be_bytes([length_view[1], length_view[2]]) as usize
            }
            Some(size) => size,
        };

        Ok(Some(Head {
            op_type,
            length: frame_size,
        }))
    }

    fn pop_frame(&mut self, head: &Head) -> Result<Vec<u8>, AppError> {
        let view: RingBufferView<u8> = self.buffer.pop(head.length)?;
        let frame_vec: Vec<u8> = view.into();
        Ok(frame_vec)
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
                Ok(n) => {
                    self.buffer.advance(n)?;
                    return Ok(BytesRead::Some(n))
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // TcpStream may not be ready for read. TcpStream#readable may return false positives.
                    continue;
                }
                Err(e) => return Err(AppError::from(e)),
            }
        }
    }
}

impl Display for Connection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Connection; [{}] [{}]", self.socket_addr, self.buffer)
    }
}
