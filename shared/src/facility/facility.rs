use crate::facility::{ControlCenter, MetalExtractor, OilExtractor};
use crate::map::HexCoord;
use crate::sync::{SyncBytes, SyncTrait};

#[derive(Debug, Copy, Clone)]
pub enum Facility<'a> {
    ControlCenter(&'a ControlCenter),
    MetalExtractor(&'a MetalExtractor),
    OilExtractor(&'a OilExtractor),
}

impl<'a> Facility<'a> {
    pub fn location(&self) -> HexCoord {
        match self {
            Facility::ControlCenter(facility) => facility.location(),
            Facility::MetalExtractor(facility) => facility.location(),
            Facility::OilExtractor(facility) => facility.location(),
        }
    }

    pub fn state(&self) -> FacilityState {
        match self {
            Facility::ControlCenter(facility) => facility.state(),
            Facility::MetalExtractor(facility) => facility.state(),
            Facility::OilExtractor(facility) => facility.state(),
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Facility::ControlCenter(_) => "Control Center",
            Facility::MetalExtractor(_) => "Metal Extractor",
            Facility::OilExtractor(_) => "Oil Extractor",
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

impl SyncTrait for FacilityState {
    fn fixed_size(&self) -> Option<usize> {
        (*self as u8).fixed_size()
    }
}

impl From<FacilityState> for SyncBytes {
    fn from(value: FacilityState) -> Self {
        SyncBytes::from(value as u8)
    }
}

pub trait FacilityTrait {
    fn location(&self) -> HexCoord;
    fn state(&self) -> FacilityState;
    fn facility<'a>(&'a self) -> Facility<'a>;
}

#[derive(Debug, Default, Clone)]
pub struct FacilityCollection {
    pub control_center_vec: Vec<ControlCenter>,
    pub metal_extractor_vec: Vec<MetalExtractor>,
    pub oil_extractor_vec: Vec<OilExtractor>,
}

impl SyncTrait for FacilityCollection {}

impl From<FacilityCollection> for SyncBytes {
    fn from(value: FacilityCollection) -> Self {
        let mut out: SyncBytes = SyncBytes::new(Vec::new());
        out.extend(SyncBytes::from(value.control_center_vec.len() as u16));
        out.extend(value.control_center_vec.iter().map(|facility| SyncBytes::from(facility.clone())).flatten());
        out.extend(SyncBytes::from(value.metal_extractor_vec.len() as u16));
        out.extend(value.metal_extractor_vec.iter().map(|facility| SyncBytes::from(facility.clone())).flatten());
        out.extend(SyncBytes::from(value.metal_extractor_vec.len() as u16));
        out.extend(value.oil_extractor_vec.iter().map(|facility| SyncBytes::from(facility.clone())).flatten());
        out
    }
}

impl FacilityCollection {
    pub fn all_facilities<'a>(&'a self) -> Vec<Facility<'a>> {
        let mut output: Vec<Facility> = Vec::with_capacity(
            self.control_center_vec.len() + self.metal_extractor_vec.len() + self.oil_extractor_vec.len(),
        );
        output.extend(self.control_center_vec.iter().map(|f| f.facility()).collect::<Vec<Facility>>());
        output.extend(self.metal_extractor_vec.iter().map(|f| f.facility()).collect::<Vec<Facility>>());
        output.extend(self.oil_extractor_vec.iter().map(|f| f.facility()).collect::<Vec<Facility>>());
        output
    }

    pub fn at<'a>(&'a self, hex_coord: HexCoord) -> Option<Facility<'a>> {
        for facility in self.all_facilities() {
            if hex_coord == facility.location() {
                return Some(facility);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod correct_size {
        use super::*;

        #[test]
        fn facility_state() {
            assert_eq!(
                FacilityState::default().fixed_size().unwrap(),
                SyncBytes::from(FacilityState::default()).len()
            )
        }
    }
}
