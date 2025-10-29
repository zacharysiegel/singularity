use crate::error::AppErrorStatic;
use crate::facility::FacilityCollection;
use crate::map::HexCoord;
use crate::sync::{SyncBytes, SyncTrait};

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub id: u8,
    pub facilities: FacilityCollection,
}

impl SyncTrait for Player {
    fn try_deserialize(value: &[u8]) -> Result<(usize, Self), AppErrorStatic> {
        let mut offset: usize = 0;
        let id: u8 = value[offset];
        offset += size_of::<u8>();

        let (size, facilities): (usize, FacilityCollection) = FacilityCollection::try_deserialize(&value[offset..])?;
        offset += size;

        Ok((offset, Self { id, facilities }))
    }
}

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
