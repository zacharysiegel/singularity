use crate::facility::FacilityCollection;
use crate::map::HexCoord;
use crate::sync::SyncTrait;

#[derive(Debug, Default)]
pub struct Player {
    pub id: u8,
    pub facilities: FacilityCollection,
}

impl SyncTrait for Player {
    fn as_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.push(self.id);
        out.extend_from_slice(self.facilities.as_bytes().as_slice());
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
