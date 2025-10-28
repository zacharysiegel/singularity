use crate::state::STATE;
use shared::player::Player;
use std::sync::RwLockWriteGuard;

pub fn init_players(player_count: u8) {
    let mut players: RwLockWriteGuard<Vec<Player>> =
        STATE.stage.game.player.players.write().expect("poisoned game state");
    players.reserve_exact(player_count as usize);

    // todo: fill from server
}
