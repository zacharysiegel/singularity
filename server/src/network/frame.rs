use std::fmt::{self, Display};

use crate::error::AppError;
use crate::network::connection::Connection;
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

pub fn route_frame(connection: &Connection, frame: Frame) {
    match frame.head.op_type {
        OperationType::Heartbeat => {
            log::trace!("Heartbeat received; [{}] [{}]", connection, frame);
        }
        OperationType::Register => {
            log::trace!("Register received; [{}] [{}]", connection, frame);
            todo!();
        }
        OperationType::Acknowledgement => {
            log::trace!("Acknowledgement received; [{}] [{}]", connection, frame);
            todo!();
        }
        OperationType::_PlaceholderDynamic => {
            log::trace!("_PlaceholderDynamic received; [{}] [{}]", connection, frame);
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
