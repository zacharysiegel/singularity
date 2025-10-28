use crate::facility::{Facility, FacilityState, FacilityTrait};
use crate::map::HexCoord;

#[derive(Debug, Default, Copy, Clone)]
pub struct MetalExtractor {
    pub location: HexCoord,
    pub state: FacilityState,
}

impl FacilityTrait for MetalExtractor {
    fn location(&self) -> HexCoord {
        self.location
    }

    fn state(&self) -> FacilityState {
        self.state
    }

    fn facility<'a>(&'a self) -> Facility<'a> {
        Facility::MetalExtractor(self)
    }
}
