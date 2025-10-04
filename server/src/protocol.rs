//! Big endian
//! 2 bytes: Frame total length in bytes

use crate::error::AppError;
use crate::ring_buffer::RingBuffer;
use frame::Frame;
use std::io;
use std::io::IoSliceMut;
use std::net::SocketAddr;
use tokio::net::TcpStream;

const BUFFER_SIZE: usize = 4096;

mod frame {
    use uuid::Uuid;

    pub const MAXIMUM_FRAME_SIZE: usize = size_of::<Register>();

    pub enum Frame {
        Heartbeat(Heartbeat),
        Register(Register),
        Acknowledgement(Acknowledgement),
    }

    pub(crate) type OpCode = u8;

    #[repr(C, packed(1))]
    pub struct Heartbeat {
        op_code: OpCode,
    }

    #[repr(C, packed(1))]
    pub struct Register {
        user_id: Uuid,
        op_code: OpCode,
    }

    #[repr(C, packed(1))]
    pub struct Acknowledgement {
        op_code: OpCode,
        op_code_acknowledged: OpCode,
    }

    impl Frame {
        pub const fn is_fixed_size(&self) -> bool {
            match self {
                _ => true,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// We want to be extra careful about accidentally changing the sizes of these structs
        #[test]
        fn size_snapshots() {
            assert_eq!(1, size_of::<OpCode>());
            assert_eq!(1, size_of::<Heartbeat>());
            assert_eq!(17, size_of::<Register>());
            assert_eq!(2, size_of::<Acknowledgement>());
        }
    }
}

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

    pub async fn read_frame(&mut self) -> Result<Option<Frame>, AppError> {
        self.read_chunk().await?;

        // todo: consume all complete frames before reading another chunk
        //  in order to prevent overflowing the buffer

        todo!()
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> Result<(), AppError> {
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
