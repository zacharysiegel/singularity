//! Big endian
//! 2 bytes: Frame total length in bytes

use crate::error::AppError;
use crate::ring_buffer::RingBuffer;
use frame::FrameContent;
use std::io;
use std::io::IoSliceMut;
use std::net::SocketAddr;
use tokio::net::TcpStream;

const BUFFER_SIZE: usize = 4096;

mod frame {
    use crate::error::AppError;
    use std::mem::MaybeUninit;
    use uuid::Uuid;

    pub const MAXIMUM_FRAME_SIZE: usize = size_of::<Register>();

    pub struct Frame {
        op_code: OpCode,
        content: FrameContent,
    }

    pub type OpCode = u8;

    pub enum FrameContent {
        Heartbeat(MaybeUninit<Heartbeat>),
        Register(MaybeUninit<Register>),
        Acknowledgement(MaybeUninit<Acknowledgement>),
    }

    #[repr(C, packed(1))]
    pub struct Heartbeat {}

    #[repr(C, packed(1))]
    pub struct Register {
        user_id: Uuid,
    }

    #[repr(C, packed(1))]
    pub struct Acknowledgement {
        op_code_acknowledged: OpCode,
    }

    impl FrameContent {
        pub const fn is_fixed_size(&self) -> bool {
            match self {
                _ => true,
            }
        }

        pub fn from_op_code(op_code: OpCode) -> Result<Self, AppError> {
            match op_code {
                1 => Ok(FrameContent::Heartbeat(MaybeUninit::uninit())),
                2 => Ok(FrameContent::Register(MaybeUninit::uninit())),
                3 => Ok(FrameContent::Acknowledgement(MaybeUninit::uninit())),
                _ => Err(AppError::new(&format!("Invalid op code; [{}]", op_code))),
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
            assert_eq!(0, size_of::<Heartbeat>());
            assert_eq!(16, size_of::<Register>());
            assert_eq!(1, size_of::<Acknowledgement>());
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

    pub async fn read_frame(&mut self) -> Result<Option<FrameContent>, AppError> {
        self.read_chunk().await?;

        // todo: consume all complete frames before reading another chunk
        //  in order to prevent overflowing the buffer

        todo!()
    }

    pub async fn write_frame(&mut self, frame: &FrameContent) -> Result<(), AppError> {
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
