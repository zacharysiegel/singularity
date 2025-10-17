use crate::facility::control_center::ControlCenter;
use crate::facility::{FacilityCollection, FacilityState};
use crate::map::config::HEX_COUNT_SQRT;
use crate::map::coordinate::HexCoord;
use crate::player::Player;
use crate::state::STATE;
use std::sync::RwLockWriteGuard;

pub fn init_players(player_count: u8) {
    let mut players: RwLockWriteGuard<Vec<Player>> =
        STATE.stage.game.player.players.write().expect("poisoned game state");
    players.reserve_exact(player_count as usize);

    for p in 0..player_count {
        let mut player: Player = Player {
            id: p,
            facilities: FacilityCollection::default(),
        };
        let facility_location: HexCoord = HexCoord {
            i: HEX_COUNT_SQRT / i16::from(player_count) * i16::from(p),
            j: HEX_COUNT_SQRT / i16::from(player_count) * i16::from(p),
        };
        let facility = ControlCenter {
            location: facility_location,
            state: FacilityState::default(),
        };
        player.facilities.control_center_vec.push(facility);
        players.push(player);
    }
}
