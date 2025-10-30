//! All multi-byte fields should be interpreted in Big-Endian order.
//! Each frame begins with a 1-byte operation code.
//! A frame can be fixed-length or variable-length.
//! If fixed, the frame's data immediately follows the operation code.
//! If variable, the frame's total length is written as a 2-byte Big-Endian unsigned integer.
//! The operation code and optional length field constitute the frame's "head".
//! The rest of the frame is considered the frame's "body".

use crate::error::{AppError, AppErrorStatic};
use crate::network::connection::WriteBufferT;
use crate::sync::{SyncGame, SyncTrait};
use std::fmt::{self, Display};
use std::mem;
use uuid::Uuid;

macro_rules! fixed_size_impl {
    () => {
        const FIXED_SIZE: ::std::option::Option<usize> = ::std::option::Option::Some(::std::mem::size_of::<Self>());

        fn to_frame(&self) -> $crate::network::protocol::Frame {
            let head = $crate::network::protocol::Head {
                op_type: $crate::network::protocol::OperationType::from_op_code(Self::OP_CODE).unwrap(),
                data_length: ::std::mem::size_of::<Self>(),
            };
            let data = ::std::vec::Vec::from(unsafe {
                mem::transmute_copy::<Self, [u8; ::std::mem::size_of::<Self>()]>(self)
            });

            $crate::network::protocol::Frame { head, data }
        }
    };
}

pub type OpCode = u8;

#[derive(Debug)]
pub struct Frame {
    pub head: Head,
    data: Vec<u8>,
}

impl Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Frame; [{}]", self.head)
    }
}

impl TryFrom<&[u8]> for Frame {
    type Error = AppErrorStatic;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let op_code: &u8 = value.get(0).ok_or_else(|| AppErrorStatic::new("invalid size"))?;
        let op_type: OperationType = OperationType::from_op_code(*op_code)?;
        let data_length: usize = match op_type.fixed_size() {
            Some(size) => size,
            None => {
                let start: usize = size_of::<OpCode>();
                let end: usize = start + size_of::<u16>(); // todo: change to u32
                let bytes: &[u8] = value.get(start..end).ok_or_else(|| AppErrorStatic::new("invalid size"))?;
                let array: [u8; 2] = <[u8; 2]>::try_from(bytes)?;
                usize::from(u16::from_be_bytes(array))
            }
        };

        let head = Head { op_type, data_length };
        let start: usize = head.head_length();
        let end: usize = start + data_length;
        let data: &[u8] = value.get(start..end).ok_or_else(|| AppErrorStatic::new("invalid size"))?;
        Ok(Frame {
            head,
            data: data.to_vec(),
        })
    }
}

impl Frame {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::with_capacity(size_of::<OpCode>() + self.head.data_length);

        out.extend(self.head.op_type.op_code().to_be_bytes());
        if let None = self.head.op_type.fixed_size() {
            out.extend_from_slice((self.head.data_length as u16).to_be_bytes().as_slice()); // todo: change to u32
        }
        out.extend_from_slice(self.data.as_slice());
        out
    }
}

#[derive(Debug)]
pub struct Head {
    pub op_type: OperationType,
    pub data_length: usize,
}

impl Display for Head {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Head; [op_type: {}] [length: {}]", self.op_type, self.data_length)
    }
}

impl Head {
    pub fn head_length(&self) -> usize {
        match self.op_type.fixed_size() {
            Some(_) => size_of::<OpCode>(),
            None => size_of::<OpCode>() + size_of::<u16>(), // todo: change to u32
        }
    }

    pub fn total_length(&self) -> usize {
        size_of::<OpCode>() + self.data_length
    }
}

pub trait Operation: for<'a> TryFrom<&'a Frame> {
    const OP_CODE: OpCode;
    /// None iff not fixed size
    const FIXED_SIZE: Option<usize> = None;

    fn to_frame(&self) -> Frame;
}

#[derive(Debug)]
pub enum OperationType {
    Heartbeat,
    Register,
    Acknowledgement,
    AllGames,
}

impl Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string: &'static str = match self {
            OperationType::Heartbeat => "Heartbeat",
            OperationType::Register => "Register",
            OperationType::Acknowledgement => "Acknowledgement",
            OperationType::AllGames => "AllGames",
        };
        write!(f, "OperationType({})", string)
    }
}

impl OperationType {
    pub fn from_op_code(op_code: OpCode) -> Result<Self, AppError> {
        match op_code {
            Heartbeat::OP_CODE => Ok(OperationType::Heartbeat),
            Register::OP_CODE => Ok(OperationType::Register),
            Acknowledgement::OP_CODE => Ok(OperationType::Acknowledgement),
            AllGames::OP_CODE => Ok(OperationType::AllGames),
            _ => Err(AppError::new(&format!("Invalid op code; [{}]", op_code))),
        }
    }

    pub const fn op_code(&self) -> OpCode {
        match self {
            OperationType::Heartbeat => Heartbeat::OP_CODE,
            OperationType::Register => Register::OP_CODE,
            OperationType::Acknowledgement => Acknowledgement::OP_CODE,
            OperationType::AllGames => AllGames::OP_CODE,
        }
    }

    pub const fn fixed_size(&self) -> Option<usize> {
        match self {
            OperationType::Heartbeat => Heartbeat::FIXED_SIZE,
            OperationType::Register => Register::FIXED_SIZE,
            OperationType::Acknowledgement => Acknowledgement::FIXED_SIZE,
            OperationType::AllGames => AllGames::FIXED_SIZE,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed(1))]
pub struct Heartbeat {}

impl Operation for Heartbeat {
    const OP_CODE: OpCode = 1;

    fixed_size_impl!();
}

impl<'a> From<&'a Frame> for Heartbeat {
    fn from(value: &'a Frame) -> Self {
        Self {}
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed(1))]
pub struct Register {
    pub user_id: Uuid,
    // todo: add debug flag. only send default game setup if debug = true
}

impl Operation for Register {
    const OP_CODE: OpCode = 2;

    fixed_size_impl!();
}

impl<'a> TryFrom<&'a Frame> for Register {
    type Error = AppError;

    fn try_from(value: &'a Frame) -> Result<Self, Self::Error> {
        let uuid: Uuid =
            Uuid::from_slice(value.data.as_slice()).map_err(|e| AppError::from_error_default(Box::new(e)))?;
        Ok(Self { user_id: uuid })
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed(1))]
pub struct Acknowledgement {
    pub op_code_acknowledged: OpCode,
}

impl Operation for Acknowledgement {
    const OP_CODE: OpCode = 3;

    fixed_size_impl!();
}

impl<'a> TryFrom<&'a Frame> for Acknowledgement {
    type Error = AppErrorStatic;

    fn try_from(value: &'a Frame) -> Result<Self, Self::Error> {
        Ok(Self {
            op_code_acknowledged: *value.data.get(0).ok_or_else(|| AppErrorStatic::new("invalid size"))?,
        })
    }
}

// Dynamically-sized frames cannot be directly transmuted from bits, since their size is not statically known
#[derive(Debug)]
pub struct AllGames {
    pub games: Vec<SyncGame>,
}

impl<'a> TryFrom<&'a Frame> for AllGames {
    type Error = AppErrorStatic;

    fn try_from(value: &'a Frame) -> Result<Self, Self::Error> {
        let (_, games): (usize, Vec<SyncGame>) = Vec::<SyncGame>::try_deserialize(value.data.as_slice())?;
        Ok(Self { games })
    }
}

impl Operation for AllGames {
    const OP_CODE: OpCode = 4;

    fn to_frame(&self) -> Frame {
        let data: Vec<u8> = self.games.to_bytes();
        Frame {
            head: Head {
                op_type: OperationType::AllGames,
                data_length: data.len(),
            },
            data,
        }
    }
}

pub async fn enqueue_message<T: Operation>(write_buffer: WriteBufferT, message: T) -> Result<(), AppErrorStatic> {
    let x = message.to_frame().to_bytes();
    write_buffer.write().await.push(x.as_slice())?;
    Ok(())
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
