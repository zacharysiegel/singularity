use crate::map::config::HEX_COUNT_SQRT;
use crate::state::{Facility, FacilityState, FacilityType, Player, State, STATE};
use std::sync::RwLockWriteGuard;
use crate::map::coordinate::HexCoord;

pub fn init_players(player_count: u8) {
    let mut state: RwLockWriteGuard<State> = STATE.write().expect("poisoned game state");
    state.players.reserve_exact(player_count as usize);

    for p in 0..player_count {
        let mut player: Player = Player {
            id: p,
            facilities: Vec::new(),
        };
        let facility_location: HexCoord = HexCoord {
            i: HEX_COUNT_SQRT / u16::from(player_count) * u16::from(p),
            j: HEX_COUNT_SQRT / u16::from(player_count) * u16::from(p),
        };
        let facility: Facility = Facility {
            location: facility_location,
            facility_type: FacilityType::default(),
            facility_state: FacilityState::default(),
        };
        player.facilities.push(facility);
        state.players.push(player);
    }
}
