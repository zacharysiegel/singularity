use crate::network::connection::Connection;
use crate::network::protocol::{Acknowledgement, Frame, Heartbeat, OperationType, Register, _PlaceholderDynamic};

pub async fn route_frame(connection: &mut Connection, frame: Frame) {
    match frame.head.op_type {
        OperationType::Heartbeat => {
            log::trace!("Heartbeat received; [{}] [{}]", connection, frame);
            heartbeat(frame);
        }
        OperationType::Register => {
            log::trace!("Register received; [{}] [{}]", connection, frame);
            register(frame);
        }
        OperationType::Acknowledgement => {
            log::trace!("Acknowledgement received; [{}] [{}]", connection, frame);
            acknowledgement(frame);
        }
        OperationType::_PlaceholderDynamic => {
            log::trace!("_PlaceholderDynamic received; [{}] [{}]", connection, frame);
            _placeholder_dynamic(frame);
        }
    }
}

fn heartbeat(frame: Frame) {
    let heartbeat: Heartbeat = Heartbeat::from(&frame);
    log::debug!("parsed frame; [{:?}]", heartbeat);
}

fn register(frame: Frame) {
    let register: Register = Register::from(&frame);
    log::debug!("parsed frame; [{:?}]", register);

    todo!();
}

fn acknowledgement(frame: Frame) {
    let acknowledgement: Acknowledgement = Acknowledgement::from(&frame);
    log::debug!("parsed frame; [{:?}]", acknowledgement);

    todo!();
}

fn _placeholder_dynamic(frame: Frame) {
    let _placeholder_dynamic: _PlaceholderDynamic = _PlaceholderDynamic::from(&frame);
    log::debug!("parsed frame; [{:?}]", _placeholder_dynamic);

    todo!();
}
