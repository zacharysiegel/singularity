use crate::map::config::HEX_COUNT_SQRT;
use crate::map::coordinate::HexCoord;
use crate::state::{Facility, FacilityState, FacilityType, Player, STATE};
use std::sync::RwLockWriteGuard;

pub mod state {
    use crate::state::Player;
    use std::sync::RwLock;

    #[derive(Debug)]
    pub struct PlayerState {
        pub players: RwLock<Vec<Player>>,
    }

    impl PlayerState {
        pub const DEFAULT: PlayerState = PlayerState {
            players: RwLock::new(Vec::new()),
        };
    }
}

pub fn init_players(player_count: u8) {
    let mut players: RwLockWriteGuard<Vec<Player>> = STATE.stage.map.player.players.write().expect("poisoned game state");
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
        let facility: Facility = Facility {
            location: facility_location,
            facility_type: FacilityType::default(),
            facility_state: FacilityState::default(),
        };
        player.facilities.push(facility);
        players.push(player);
    }
}
