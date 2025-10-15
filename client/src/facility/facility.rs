use crate::map::HexCoord;

#[derive(Debug, Copy, Clone)]
pub enum Facility {
    ControlCenter(ControlCenter),
    MetalExtractor(MetalExtractor),
    OilExtractor(OilExtractor),
}

impl Facility {
    pub fn location(&self) -> HexCoord {
        match self {
            Facility::ControlCenter(facility) => facility.location,
            Facility::MetalExtractor(facility) => facility.location,
            Facility::OilExtractor(facility) => facility.location,
        }
    }

    pub fn state(&self) -> FacilityState {
        match self {
            Facility::ControlCenter(facility) => facility.state,
            Facility::MetalExtractor(facility) => facility.state,
            Facility::OilExtractor(facility) => facility.state,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Default, Copy, Clone)]
pub enum FacilityState {
    #[default]
    Operating = 0,
    Placing,
    Destroyed,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct ControlCenter {
    pub location: HexCoord,
    pub state: FacilityState,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct MetalExtractor {
    pub location: HexCoord,
    pub state: FacilityState,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct OilExtractor {
    pub location: HexCoord,
    pub state: FacilityState,
}
