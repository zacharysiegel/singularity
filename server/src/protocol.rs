//! All multi-byte fields should be interpreted in Big-Endian order
//! Each frame begins with a 1-byte operation code
//! A frame can be fixed-length or variable-length
//!     If fixed, the frame's data immediately follows the operation code
//!     If variable, the frame's total length is written as a 2-byte Big-Endian unsigned integer

use crate::error::AppError;
use crate::ring_buffer::{RingBuffer, RingBufferView};
use frame::OperationType;
use std::io;
use std::io::IoSliceMut;
use std::net::SocketAddr;
use tokio::net::TcpStream;

const BUFFER_SIZE: usize = 4096;

// todo: move to another file
mod frame {
    use crate::error::AppError;
    use uuid::Uuid;

    pub type OpCode = u8;

    pub enum OperationType {
        Heartbeat,
        Register,
        Acknowledgement,
        _PlaceholderDynamic,
    }

    #[repr(C, packed(1))]
    pub struct Heartbeat {
        pub op_code: OpCode,
    }

    impl Operation for Heartbeat {
        const OP_CODE: OpCode = 1;
        const FIXED_SIZE: Option<usize> = Some(size_of::<Self>());
    }

    #[repr(C, packed(1))]
    pub struct Register {
        pub op_code: OpCode,
        pub user_id: Uuid,
    }

    impl Operation for Register {
        const OP_CODE: OpCode = 2;
        const FIXED_SIZE: Option<usize> = Some(size_of::<Self>());
    }

    #[repr(C, packed(1))]
    pub struct Acknowledgement {
        pub op_code: OpCode,
        pub op_code_acknowledged: OpCode,
    }

    impl Operation for Acknowledgement {
        const OP_CODE: OpCode = 3;
        const FIXED_SIZE: Option<usize> = Some(size_of::<Self>());
    }

    #[repr(C, packed(1))]
    pub struct _PlaceholderDynamic<'a> {
        pub op_code: OpCode,
        pub length: u16,
        pub string: &'a str,
    }

    impl<'a> Operation for _PlaceholderDynamic<'a> {
        const OP_CODE: OpCode = 4;
        const FIXED_SIZE: Option<usize> = None;
    }

    impl OperationType {
        pub fn from_op_code(op_code: &OpCode) -> Result<Self, AppError> {
            match op_code {
                &Heartbeat::OP_CODE => Ok(OperationType::Heartbeat),
                &Register::OP_CODE => Ok(OperationType::Register),
                &Acknowledgement::OP_CODE => Ok(OperationType::Acknowledgement),
                &_PlaceholderDynamic::OP_CODE => Ok(OperationType::_PlaceholderDynamic),
                _ => Err(AppError::new(&format!("Invalid op code; [{}]", op_code))),
            }
        }

        pub const fn fixed_size(&self) -> Option<usize> {
            match self {
                OperationType::Heartbeat => Heartbeat::FIXED_SIZE,
                OperationType::Register => Register::FIXED_SIZE,
                OperationType::Acknowledgement => Acknowledgement::FIXED_SIZE,
                OperationType::_PlaceholderDynamic => _PlaceholderDynamic::FIXED_SIZE,
            }
        }
    }

    pub trait Operation {
        const OP_CODE: OpCode;
        /// None iff not fixed size
        const FIXED_SIZE: Option<usize>;
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
