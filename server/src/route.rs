use crate::network::connection::Connection;
use crate::network::protocol::{
    _PlaceholderDynamic, Acknowledgement, Frame, Heartbeat, OperationType, Register,
};

pub async fn route_frame(connection: &Connection, frame: Frame) {
    match frame.head.op_type {
        OperationType::Heartbeat => {
            log::trace!("Heartbeat received; [{}] [{}]", connection, frame);
            heartbeat(connection, frame);
        }
        OperationType::Register => {
            log::trace!("Register received; [{}] [{}]", connection, frame);
            register(connection, frame);
        }
        OperationType::Acknowledgement => {
            log::trace!("Acknowledgement received; [{}] [{}]", connection, frame);
            acknowledgement(connection, frame);
        }
        OperationType::_PlaceholderDynamic => {
            log::trace!("_PlaceholderDynamic received; [{}] [{}]", connection, frame);
            _placeholder_dynamic(connection, frame);
        }
    }
}

#[allow(unused_variables)]
fn heartbeat(connection: &Connection, frame: Frame) {
    let heartbeat: Heartbeat = Heartbeat::from(&frame);
    log::debug!("parsed frame; [{:?}]", heartbeat);
}

#[allow(unused_variables)]
fn register(connection: &Connection, frame: Frame) {
    let register: Register = Register::from(&frame);
    log::debug!("parsed frame; [{:?}]", register);
    todo!();
}

#[allow(unused_variables)]
fn acknowledgement(connection: &Connection, frame: Frame) {
    let acknowledgement: Acknowledgement = Acknowledgement::from(&frame);
    log::debug!("parsed frame; [{:?}]", acknowledgement);
    todo!();
}

#[allow(unused_variables)]
fn _placeholder_dynamic(connection: &Connection, frame: Frame) {
    let _placeholder_dynamic: _PlaceholderDynamic = _PlaceholderDynamic::from(&frame);
    log::debug!("parsed frame; [{:?}]", _placeholder_dynamic);
    todo!();
}
