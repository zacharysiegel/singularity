use crate::state::STATE;
use crate::title;
use shared::environment::RuntimeEnvironment;
use shared::map::{Hex, HEX_COUNT};
use shared::network::connection::WriteBufferT;
use shared::network::protocol;
use shared::network::protocol::{Acknowledgement, AllGames, DebugGame, Frame, Operation, OperationType};
use shared::player::Player;
use std::sync::RwLockWriteGuard;

pub async fn route_frame(write_buffer: WriteBufferT, frame: Frame) {
    match frame.head.op_type {
        OperationType::DebugGame => debug_game(write_buffer, frame),
        _ => {}
    }
}

fn debug_game(write_buffer: WriteBufferT, frame: Frame) {
    if !RuntimeEnvironment::default().is_debug() {
        return;
    }

    let debug_game: DebugGame = DebugGame::try_from(&frame).unwrap();
    log::debug!("parsed frame; [AllGames]");

    let mut hexes = STATE.stage.game.map.hexes.write().unwrap();
    let vec: Vec<Hex> = debug_game.game.map.hexes.clone();
    *hexes = <[Hex; HEX_COUNT as usize]>::try_from(vec).unwrap();
    drop(hexes);

    let mut players: RwLockWriteGuard<Vec<Player>> = STATE.stage.game.player.players.write().unwrap();
    *players = debug_game.game.players.clone();
    drop(players);

    tokio::spawn(async {
        protocol::enqueue_message(
            write_buffer,
            Acknowledgement {
                op_code_acknowledged: DebugGame::OP_CODE,
            },
        )
        .await
        .unwrap();
    });

    title::enable_debug();
}
