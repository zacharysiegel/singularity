use crate::facility::{Facility, FacilityState};
use crate::map::config::HEX_COUNT_SQRT;
use crate::map::coordinate::HexCoord;
use crate::player::Player;
use crate::state::STATE;
use std::sync::RwLockWriteGuard;
use crate::facility::control_center::ControlCenter;

pub fn init_players(player_count: u8) {
    let mut players: RwLockWriteGuard<Vec<Player>> =
        STATE.stage.map.player.players.write().expect("poisoned game state");
    players.reserve_exact(player_count as usize);

    for p in 0..player_count {
        let mut player: Player = Player {
            id: p,
            facilities: Vec::new(),
        };
        let facility_location: HexCoord = HexCoord {
            i: HEX_COUNT_SQRT / i16::from(player_count) * i16::from(p),
            j: HEX_COUNT_SQRT / i16::from(player_count) * i16::from(p),
        };
        let facility = ControlCenter {
            location: facility_location,
            state: FacilityState::default(),
        };
        player.facilities.push(Facility::ControlCenter(facility));
        players.push(player);
    }
}
