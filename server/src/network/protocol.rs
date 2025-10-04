//! All multi-byte fields should be interpreted in Big-Endian order.
//! Each frame begins with a 1-byte operation code.
//! A frame can be fixed-length or variable-length.
//! If fixed, the frame's data immediately follows the operation code.
//! If variable, the frame's total length is written as a 2-byte Big-Endian unsigned integer.
//! The operation code and optional length field constitute the frame's "head".
//! The rest of the frame is considered the frame's "body".

use crate::error::AppError;
use crate::network::connection::Connection;
use std::fmt::{self, Display};
use std::hint::black_box;
use std::mem;
use uuid::Uuid;

pub struct Frame {
    pub head: Head,
    pub data: Vec<u8>,
}

impl Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Frame; [{}]", self.head)
    }
}

pub struct Head {
    pub op_type: OperationType,
    pub length: usize,
}

impl Display for Head {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Head; [op_type: {}] [length: {}]",
            self.op_type, self.length
        )
    }
}

pub type OpCode = u8;

pub enum OperationType {
    Heartbeat,
    Register,
    Acknowledgement,
    _PlaceholderDynamic,
}

impl Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string: &'static str = match self {
            OperationType::Heartbeat => "Heartbeat",
            OperationType::Register => "Register",
            OperationType::Acknowledgement => "Acknowledgement",
            OperationType::_PlaceholderDynamic => "_PlaceholderDynamic",
        };
        write!(f, "OperationType({})", string)
    }
}

#[derive(Debug)]
#[repr(C, packed(1))]
pub struct Heartbeat {
    pub op_code: OpCode,
}

impl Operation for Heartbeat {
    const OP_CODE: OpCode = 1;
    const FIXED_SIZE: Option<usize> = Some(size_of::<Self>());
}

#[derive(Debug)]
#[repr(C, packed(1))]
pub struct Register {
    pub op_code: OpCode,
    pub user_id: Uuid,
}

impl Operation for Register {
    const OP_CODE: OpCode = 2;
    const FIXED_SIZE: Option<usize> = Some(size_of::<Self>());
}

#[derive(Debug)]
#[repr(C, packed(1))]
pub struct Acknowledgement {
    pub op_code: OpCode,
    pub op_code_acknowledged: OpCode,
}

impl Operation for Acknowledgement {
    const OP_CODE: OpCode = 3;
    const FIXED_SIZE: Option<usize> = Some(size_of::<Self>());
}

#[derive(Debug)]
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

pub fn route_frame(connection: &Connection, frame: Frame) {
    match frame.head.op_type {
        OperationType::Heartbeat => {
            log::trace!("Heartbeat received; [{}] [{}]", connection, frame);

            let heartbeat: Heartbeat = unsafe {
                mem::transmute::<[u8; Heartbeat::FIXED_SIZE.unwrap()], Heartbeat>(
                    frame.data.try_into().unwrap(),
                )
            };
            log::debug!("parsed frame; [{:?}]", heartbeat);
        }
        OperationType::Register => {
            log::trace!("Register received; [{}] [{}]", connection, frame);

            let register: Register = unsafe {
                mem::transmute::<[u8; Register::FIXED_SIZE.unwrap()], Register>(
                    frame.data.try_into().unwrap(),
                )
            };
            log::debug!("parsed frame; [{:?}]", register);
            todo!();
        }
        OperationType::Acknowledgement => {
            log::trace!("Acknowledgement received; [{}] [{}]", connection, frame);
            let acknowledgement: Acknowledgement = unsafe {
                mem::transmute::<[u8; Acknowledgement::FIXED_SIZE.unwrap()], Acknowledgement>(
                    frame.data.try_into().unwrap(),
                )
            };
            log::debug!("parsed frame; [{:?}]", acknowledgement);
            todo!();
        }
        OperationType::_PlaceholderDynamic => {
            log::trace!("_PlaceholderDynamic received; [{}] [{}]", connection, frame);
            let _placeholder_dynamic: _PlaceholderDynamic = {
                let length: u16 = u16::from_be_bytes(frame.data[1..3].try_into().unwrap());
                _PlaceholderDynamic {
                    op_code: frame.data[0],
                    length,
                    string: str::from_utf8(&frame.data[3..(length as usize)]).unwrap(),
                }
            };
            log::debug!("parsed frame; [{:?}]", _placeholder_dynamic);
            todo!();
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
