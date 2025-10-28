use crate::state::STATE;
use shared::map::HEX_COUNT;
use shared::map::Hex;
use std::sync::RwLockWriteGuard;

pub fn init_map() {
    let mut hexes: RwLockWriteGuard<[Hex; HEX_COUNT as usize]> =
        STATE.stage.game.map.hexes.write().expect("global state poisoned");
    
    // todo: fill from server
}
