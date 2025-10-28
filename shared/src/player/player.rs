use crate::facility::FacilityCollection;
use crate::map::HexCoord;
use crate::sync::{SyncBytes, SyncTrait};

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub id: u8,
    pub facilities: FacilityCollection,
}

impl SyncTrait for Player {}

impl From<Player> for SyncBytes {
    fn from(value: Player) -> Self {
        let mut out: SyncBytes = SyncBytes::new(Vec::new());
        out.push(value.id);
        out.extend_from_slice(SyncBytes::from(value.facilities).as_slice());
        out
    }
}

impl Player {
    pub fn new(id: u8) -> Self {
        Player {
            id,
            facilities: FacilityCollection::default(),
        }
    }

    pub fn within_influence(&self, hex_coord: HexCoord) -> bool {
        for facility in &self.facilities.control_center_vec {
            match facility.within_influence(hex_coord) {
                true => return true,
                false => continue,
            }
        }
        false
    }
}
