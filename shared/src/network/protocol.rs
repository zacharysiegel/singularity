//! All multi-byte fields should be interpreted in Big-Endian order.
//! Each frame begins with a 1-byte operation code.
//! A frame can be fixed-length or variable-length.
//! If fixed, the frame's data immediately follows the operation code.
//! If variable, the frame's total length is written as a 2-byte Big-Endian unsigned integer.
//! The operation code and optional length field constitute the frame's "head".
//! The rest of the frame is considered the frame's "body".

use crate::error::{AppError, AppErrorStatic};
use crate::network::connection::WriteBufferT;
use crate::sync::{SyncBytes, SyncGame};
use std::fmt::{self, Display};
use std::mem;
use uuid::Uuid;

macro_rules! fixed_size_impl {
    () => {
        const FIXED_SIZE: ::std::option::Option<usize> = ::std::option::Option::Some(::std::mem::size_of::<Self>());

        fn as_bytes(&self) -> ::std::vec::Vec<u8> {
            ::std::vec::Vec::from(unsafe { mem::transmute_copy::<Self, [u8; Self::FIXED_SIZE.unwrap()]>(self) })
        }
    };
}

macro_rules! from_frame_fixed {
    ($id:ident) => {
        impl<'a> ::std::convert::From<&'a Frame> for $id {
            fn from(frame: &'a Frame) -> Self {
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

// todo: + Into<Vec<u8>>
pub trait Operation: for<'a> From<&'a Frame> {
    const OP_CODE: OpCode;
    /// None iff not fixed size
    const FIXED_SIZE: Option<usize>;

    fn as_bytes(&self) -> Vec<u8>;
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

// Dynamically-sized frames cannot be directly interpreted from bits, since their size is not statically known
pub struct AllGames {
    pub games: Vec<SyncGame>,
}

impl<'a> From<&'a Frame> for AllGames {
    fn from(value: &'a Frame) -> Self {
        todo!()
    }
}

impl Operation for AllGames {
    const OP_CODE: OpCode = 5;
    const FIXED_SIZE: Option<usize> = None;

    fn as_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.push(Self::OP_CODE);
        out.extend_from_slice(SyncBytes::from(self.games.len() as u16).as_slice());
        out.extend(self.games.iter().map(|game| SyncBytes::from(game.clone())).flatten());
        out
    }
}

pub async fn enqueue_message<T: Operation>(write_buffer: WriteBufferT, message: T) -> Result<(), AppErrorStatic> {
    write_buffer.write().await.push(message.as_bytes().as_slice())?;
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
