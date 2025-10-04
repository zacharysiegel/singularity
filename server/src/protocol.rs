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
    use crate::error::AppError;
    use std::mem::MaybeUninit;
    use uuid::Uuid;

    pub const MAXIMUM_FRAME_SIZE: usize = size_of::<Register>();

    pub type OpCode = u8;

    pub enum Frame {
        Heartbeat(MaybeUninit<Heartbeat>),
        Register(MaybeUninit<Register>),
        Acknowledgement(MaybeUninit<Acknowledgement>),
    }

    #[repr(C, packed(1))]
    pub struct Heartbeat {
        op_code: OpCode,
    }

    impl Operation for Heartbeat {
        const OP_CODE: OpCode = 1;
        const IS_FIXED_SIZE: bool = true;
    }

    #[repr(C, packed(1))]
    pub struct Register {
        op_code: OpCode,
        user_id: Uuid,
    }

    impl Operation for Register {
        const OP_CODE: OpCode = 2;
        const IS_FIXED_SIZE: bool = true;
    }

    #[repr(C, packed(1))]
    pub struct Acknowledgement {
        op_code: OpCode,
        op_code_acknowledged: OpCode,
    }

    impl Operation for Acknowledgement {
        const OP_CODE: OpCode = 3;
        const IS_FIXED_SIZE: bool = true;
    }

    impl Frame {
        pub fn from_op_code(op_code: OpCode) -> Result<Self, AppError> {
            match op_code {
                Heartbeat::OP_CODE => Ok(Frame::Heartbeat(MaybeUninit::uninit())),
                Register::OP_CODE => Ok(Frame::Register(MaybeUninit::uninit())),
                Acknowledgement::OP_CODE => Ok(Frame::Acknowledgement(MaybeUninit::uninit())),
                _ => Err(AppError::new(&format!("Invalid op code; [{}]", op_code))),
            }
        }
    }

    pub trait Operation {
        const OP_CODE: OpCode;
        const IS_FIXED_SIZE: bool;
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
