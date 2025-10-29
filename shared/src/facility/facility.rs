use crate::error::AppErrorStatic;
use crate::facility::{ControlCenter, MetalExtractor, OilExtractor};
use crate::map::HexCoord;
use crate::sync::{SyncBytes, SyncTrait};
use crate::{sync, try_from_repr};
use strum::FromRepr;

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
#[derive(Debug, Default, Copy, Clone, FromRepr)]
pub enum FacilityState {
    #[default]
    Operating = 0,
    Placing,
    Destroyed,
}

try_from_repr!(FacilityState<u8>);

impl SyncTrait for FacilityState {
    const SYNC_FIXED_SIZE: Option<usize> = Some(u8::SYNC_FIXED_SIZE.unwrap());
}

impl From<FacilityState> for SyncBytes {
    fn from(value: FacilityState) -> Self {
        SyncBytes::from(value as u8)
    }
}

impl TryFrom<SyncBytes> for FacilityState {
    type Error = AppErrorStatic;

    fn try_from(value: SyncBytes) -> Result<Self, Self::Error> {
        check_sync_fixed_size!(value);

        Self::try_from(value[0])
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

impl TryFrom<SyncBytes> for FacilityCollection {
    type Error = AppErrorStatic;

    fn try_from(value: SyncBytes) -> Result<Self, Self::Error> {
        let mut start: usize = 0;
        let (increment, control_center_vec): (usize, Vec<ControlCenter>) = sync::parse_vec(&value[start..])?;
        start += increment;
        let (increment, metal_extractor_vec): (usize, Vec<MetalExtractor>) = sync::parse_vec(&value[start..])?;
        start += increment;
        let (_increment, oil_extractor_vec): (usize, Vec<OilExtractor>) = sync::parse_vec(&value[start..])?;

        Ok(FacilityCollection {
            control_center_vec,
            metal_extractor_vec,
            oil_extractor_vec,
        })
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
                FacilityState::SYNC_FIXED_SIZE.unwrap(),
                SyncBytes::from(FacilityState::default()).len()
            )
        }
    }
}
