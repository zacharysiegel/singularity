use crate::game;
use shared::network::connection::WriteBufferT;
use shared::network::protocol;
use shared::network::protocol::{Acknowledgement, AllGames, Frame, Heartbeat, OperationType, Register};
use shared::sync::SyncGame;

pub async fn route_frame(write_buffer: WriteBufferT, frame: Frame) {
    match frame.head.op_type {
        OperationType::Heartbeat => {
            log::trace!("Heartbeat received; [{}]", frame);
            heartbeat(frame);
        }
        OperationType::Register => {
            log::trace!("Register received; [{}]", frame);
            register(write_buffer, frame);
        }
        OperationType::Acknowledgement => {
            log::trace!("Acknowledgement received; [{}]", frame);
            acknowledgement(frame);
        }
        _ => {}
    }
}

fn heartbeat(frame: Frame) {
    let heartbeat: Heartbeat = Heartbeat::from(&frame);
    log::debug!("parsed frame; [{:?}]", heartbeat);
}

fn register(write_buffer: WriteBufferT, frame: Frame) {
    let register: Register = Register::try_from(&frame).unwrap();
    log::debug!("parsed frame; [{:?}]", register);

    // todo: fetch game collection from database

    tokio::spawn(async {
        let game: SyncGame = game::init_game();
        protocol::enqueue_message(write_buffer, AllGames { games: vec![game] }).await.unwrap();
    });
}

fn acknowledgement(frame: Frame) {
    let acknowledgement: Acknowledgement = Acknowledgement::try_from(&frame).unwrap();
    log::debug!("parsed frame; [{:?}]", acknowledgement);
}
