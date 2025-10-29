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
                length: Self::FIXED_SIZE.unwrap(),
            };
            let all =
                ::std::vec::Vec::from(unsafe { mem::transmute_copy::<Self, [u8; Self::FIXED_SIZE.unwrap()]>(self) });

            $crate::network::protocol::Frame {
                head,
                data: all[size_of::<$crate::network::protocol::Head>()..].to_vec(),
            }
        }
    };
}

macro_rules! from_frame_fixed {
    ($id:ident) => {
        impl<'a> ::std::convert::From<&'a $crate::network::protocol::Frame> for $id {
            fn from(frame: &'a $crate::network::protocol::Frame) -> Self {
                unsafe { *(frame.data.as_ptr() as *const $id) }
            }
        }
    };
}

pub type OpCode = u8;

#[derive(Debug)]
pub struct Frame {
    pub head: Head,
    pub data: Vec<u8>,
}

impl Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Frame; [{}]", self.head)
    }
}

impl Frame {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::with_capacity(size_of::<Head>() + self.data.len());
        let head_bytes = unsafe { mem::transmute_copy::<Head, [u8; size_of::<Head>()]>(&self.head) };

        out.extend_from_slice(&head_bytes);
        out.extend_from_slice(self.data.as_slice());
        out
    }
}

#[derive(Debug)]
pub struct Head {
    pub op_type: OperationType,
    pub length: usize,
}

impl Display for Head {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Head; [op_type: {}] [length: {}]", self.op_type, self.length)
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
pub struct Heartbeat {
    pub op_code: OpCode,
}

from_frame_fixed!(Heartbeat);

impl<'a> Operation for Heartbeat {
    const OP_CODE: OpCode = 1;

    fixed_size_impl!();
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed(1))]
pub struct Register {
    pub op_code: OpCode,
    pub user_id: Uuid,
}

from_frame_fixed!(Register);

impl<'a> Operation for Register {
    const OP_CODE: OpCode = 2;

    fixed_size_impl!();
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed(1))]
pub struct Acknowledgement {
    pub op_code: OpCode,
    pub op_code_acknowledged: OpCode,
}

from_frame_fixed!(Acknowledgement);

impl<'a> Operation for Acknowledgement {
    const OP_CODE: OpCode = 3;

    fixed_size_impl!();
}

// Dynamically-sized frames cannot be directly transmuted from bits, since their size is not statically known
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
                length: data.len(),
            },
            data,
        }
    }
}

pub async fn enqueue_message<T: Operation>(write_buffer: WriteBufferT, message: T) -> Result<(), AppErrorStatic> {
    write_buffer.write().await.push(message.to_frame().to_bytes().as_slice())?;
    Ok(())
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
