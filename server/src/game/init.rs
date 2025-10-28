use shared::facility::{ControlCenter, FacilityCollection, FacilityState};
use shared::map::{Hex, HexCoord, ResourceType, HEX_COUNT_SQRT};
use shared::player::Player;
use shared::sync::{SyncGame, SyncMap};

pub fn init_game() -> SyncGame {
    let hexes: Vec<Hex> = {
        let mut hexes = Vec::new();
        init_map(&mut hexes);
        hexes
    };
    let players: Vec<Player> = {
        let mut players = Vec::new();
        init_players(4, &mut players);
        players
    };

    SyncGame {
        map: SyncMap { hexes },
        players,
    }
}

fn init_map(hexes: &mut Vec<Hex>) {
    for i in 0..HEX_COUNT_SQRT {
        for j in 0..HEX_COUNT_SQRT {
            let hex_coord: HexCoord = HexCoord { i, j };
            let resource_type: ResourceType = init_resource_type_from_hex_coord(&hex_coord);
            let hex: Hex = Hex {
                hex_coord,
                resource_type,
            };
            let i: usize = hex.hex_coord.map_index();
            hexes[i] = hex;
        }
    }
}

// todo: implement planned strategy (plan.md)
const fn init_resource_type_from_hex_coord(hex_coord: &HexCoord) -> ResourceType {
    if (hex_coord.i % (HEX_COUNT_SQRT / 4)) == 10 && hex_coord.j % (HEX_COUNT_SQRT / 4) == 4 {
        ResourceType::Metal
    } else if hex_coord.i % (HEX_COUNT_SQRT / 4) == 2 && hex_coord.j % (HEX_COUNT_SQRT / 4) == 12 {
        ResourceType::Oil
    } else {
        ResourceType::None
    }
}

fn init_players(player_count: u8, players: &mut Vec<Player>) {
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
