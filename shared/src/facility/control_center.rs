use crate::facility::{Facility, FacilityState, FacilityTrait};
use crate::map::HexCoord;
use crate::sync::SyncTrait;

#[derive(Debug, Default, Copy, Clone)]
pub struct ControlCenter {
    pub location: HexCoord,
    pub state: FacilityState,
}

impl FacilityTrait for ControlCenter {
    fn location(&self) -> HexCoord {
        self.location
    }

    fn state(&self) -> FacilityState {
        self.state
    }

    fn facility<'a>(&'a self) -> Facility<'a> {
        Facility::ControlCenter(self)
    }
}

impl SyncTrait for ControlCenter {
    fn as_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.fixed_size().unwrap());
        out.extend_from_slice(self.location.as_bytes().as_slice());
        out.extend_from_slice(self.state.as_bytes().as_slice());
        out
    }

    fn fixed_size(&self) -> Option<usize> {
        Some(self.location.fixed_size()? + self.state.fixed_size()?)
    }
}

impl ControlCenter {
    pub const INFLUENCE_RADIUS_STEP: i16 = 4;

    pub fn within_influence(&self, hex_coord: HexCoord) -> bool {
        self.location
            .step_distance_le(hex_coord, Self::INFLUENCE_RADIUS_STEP)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_size() {
        assert_eq!(
            ControlCenter::default().fixed_size().unwrap(),
            ControlCenter::default().as_bytes().len()
        )
    }
}
