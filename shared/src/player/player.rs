use crate::error::AppErrorStatic;
use crate::facility::FacilityCollection;
use crate::map::HexCoord;
use crate::sync::SyncTrait;

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub id: u8,
    pub facilities: FacilityCollection,
}

impl SyncTrait for Player {
    fn to_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.push(self.id);
        out.extend(self.facilities.to_bytes());
        out
    }

    fn try_deserialize(value: &[u8]) -> Result<(usize, Self), AppErrorStatic> {
        let mut offset: usize = 0;
        let id: u8 = value[offset];
        offset += size_of::<u8>();

        let (size, facilities): (usize, FacilityCollection) = FacilityCollection::try_deserialize(&value[offset..])?;
        offset += size;

        Ok((offset, Self { id, facilities }))
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
