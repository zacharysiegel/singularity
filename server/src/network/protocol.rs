//! All multi-byte fields should be interpreted in Big-Endian order.
//! Each frame begins with a 1-byte operation code.
//! A frame can be fixed-length or variable-length.
//! If fixed, the frame's data immediately follows the operation code.
//! If variable, the frame's total length is written as a 2-byte Big-Endian unsigned integer.
//! The operation code and optional length field constitute the frame's "head".
//! The rest of the frame is considered the frame's "body".

use shared::error::AppError;
use std::fmt::{self, Display};
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
        write!(f, "Head; [op_type: {}] [length: {}]", self.op_type, self.length)
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

#[derive(Debug, Copy, Clone)]
#[repr(C, packed(1))]
pub struct Heartbeat {
    pub op_code: OpCode,
}

impl<'a> From<&'a Frame> for Heartbeat {
    fn from(frame: &'a Frame) -> Self {
        unsafe { *(frame.data.as_ptr() as *const Heartbeat) }
    }
}

impl<'a> Operation for Heartbeat {
    const OP_CODE: OpCode = 1;
    const FIXED_SIZE: Option<usize> = Some(size_of::<Self>());
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed(1))]
pub struct Register {
    pub op_code: OpCode,
    pub user_id: Uuid,
}

impl<'a> From<&'a Frame> for Register {
    fn from(frame: &'a Frame) -> Self {
        unsafe { *(frame.data.as_ptr() as *const Register) }
    }
}

impl<'a> Operation for Register {
    const OP_CODE: OpCode = 2;
    const FIXED_SIZE: Option<usize> = Some(size_of::<Self>());
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed(1))]
pub struct Acknowledgement {
    pub op_code: OpCode,
    pub op_code_acknowledged: OpCode,
}

impl<'a> From<&'a Frame> for Acknowledgement {
    fn from(frame: &'a Frame) -> Self {
        unsafe { *(frame.data.as_ptr() as *const Acknowledgement) }
    }
}

impl<'a> Operation for Acknowledgement {
    const OP_CODE: OpCode = 3;
    const FIXED_SIZE: Option<usize> = Some(size_of::<Self>());
}

// Dynamically-sized frames cannot be directly interpreted from bits, since their size is not statically known
#[derive(Debug)]
#[repr(C, packed(1))]
pub struct _PlaceholderDynamic<'a> {
    pub op_code: OpCode,
    pub length: u16,
    pub string: &'a str,
}

impl<'a> From<&'a Frame> for _PlaceholderDynamic<'a> {
    fn from(frame: &'a Frame) -> Self {
        _PlaceholderDynamic {
            op_code: frame.data[0],
            length: u16::from_be_bytes(frame.data[1..3].try_into().unwrap()),
            string: str::from_utf8(&frame.data[3..]).unwrap(),
        }
    }
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
