use crate::state::STATE;
use shared::map::{Hex, HEX_COUNT};
use shared::network::connection::WriteBufferT;
use shared::network::protocol::{Acknowledgement, AllGames, Frame, Heartbeat, Operation, OperationType, Register};
use shared::player::Player;
use std::sync::RwLockWriteGuard;
use shared::network::protocol;

pub async fn route_frame(write_buffer: WriteBufferT, frame: Frame) {
    match frame.head.op_type {
        OperationType::AllGames => all_games(write_buffer, frame),
        _ => {}
    }
}

fn all_games(write_buffer: WriteBufferT, frame: Frame) {
    let all_games: AllGames = AllGames::try_from(&frame).unwrap();
    log::debug!("parsed frame; [{:?}]", all_games);

    let mut hexes = STATE.stage.game.map.hexes.write().unwrap();
    *hexes = <[Hex; HEX_COUNT as usize]>::try_from(all_games.games[0].map.hexes.clone()).unwrap();
    drop(hexes);

    let mut players: RwLockWriteGuard<Vec<Player>> = STATE.stage.game.player.players.write().unwrap();
    *players = all_games.games[0].players.clone();
    drop(players);

    tokio::spawn(async {
        protocol::enqueue_message(write_buffer, Acknowledgement {
            op_code_acknowledged: AllGames::OP_CODE,
        }).await.unwrap();
    });
}
