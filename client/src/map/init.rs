use crate::map;
use crate::map::config::HEX_COUNT_SQRT;
use crate::map::coordinate::HexCoord;
use crate::state::{Hex, ResourceType, STATE};
use std::sync::RwLockWriteGuard;

pub fn init_map() {
    let mut hexes: RwLockWriteGuard<[Hex; map::config::HEX_COUNT as usize]> =
        STATE.hexes.write().expect("global state poisoned");
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
