use crate::facility::{Facility, FacilityState, FacilityTrait};
use crate::map::HexCoord;

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

impl ControlCenter {
    pub const INFLUENCE_RADIUS_STEP: i16 = 4;

    pub fn within_influence(&self, hex_coord: HexCoord) -> bool {
        self.location.step_distance_le(hex_coord, Self::INFLUENCE_RADIUS_STEP)
    }
}
