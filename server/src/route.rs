use shared::network::connection::Connection;
use shared::network::protocol::{
    Acknowledgement, Frame, Head, Heartbeat, Operation, OperationType, Register, _PlaceholderDynamic,
};
use std::sync::Arc;

pub async fn route_frame(connection: Arc<Connection>, frame: Frame) {
    match frame.head.op_type {
        OperationType::Heartbeat => {
            log::trace!("Heartbeat received; [{:?}] [{}]", connection, frame);
            heartbeat(frame);
            let mut writer = connection.writer.write().await;
            writer
                .write_frame(&Frame {
                    head: Head {
                        op_type: OperationType::Heartbeat,
                        length: 1,
                    },
                    data: vec![Heartbeat::OP_CODE],
                })
                .await
                .unwrap();
        }
        OperationType::Register => {
            log::trace!("Register received; [{:?}] [{}]", connection, frame);
            register(frame);
        }
        OperationType::Acknowledgement => {
            log::trace!("Acknowledgement received; [{:?}] [{}]", connection, frame);
            acknowledgement(frame);
        }
        OperationType::_PlaceholderDynamic => {
            log::trace!("_PlaceholderDynamic received; [{:?}] [{}]", connection, frame);
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
