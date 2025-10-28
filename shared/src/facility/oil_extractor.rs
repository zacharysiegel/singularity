use crate::facility::{Facility, FacilityState, FacilityTrait};
use crate::map::HexCoord;

#[derive(Debug, Default, Copy, Clone)]
pub struct OilExtractor {
    pub location: HexCoord,
    pub state: FacilityState,
}

impl FacilityTrait for OilExtractor {
    fn location(&self) -> HexCoord {
        self.location
    }

    fn state(&self) -> FacilityState {
        self.state
    }

    fn facility<'a>(&'a self) -> Facility<'a> {
        Facility::OilExtractor(self)
    }
}
