use crate::error::AppErrorStatic;
use crate::facility::{ControlCenter, MetalExtractor, OilExtractor};
use crate::map::HexCoord;
use crate::sync::SyncTrait;
use crate::try_from_repr;
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

    fn to_bytes(&self) -> Vec<u8> {
        [self.clone() as u8].to_vec()
    }

    fn try_deserialize(value: &[u8]) -> Result<(usize, Self), AppErrorStatic> {
        check_sync_fixed_size!(value);

        let (size, byte): (usize, u8) = u8::try_deserialize(value)?;
        assert_eq!(size_of::<u8>(), size);

        let state: FacilityState = Self::try_from(byte)?;
        Ok((size, state))
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

impl SyncTrait for FacilityCollection {
    fn to_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(self.control_center_vec.to_bytes());
        out.extend(self.metal_extractor_vec.to_bytes());
        out.extend(self.oil_extractor_vec.to_bytes());
        out
    }

    fn try_deserialize(value: &[u8]) -> Result<(usize, Self), AppErrorStatic> {
        let mut offset: usize = 0;
        let (increment, control_center_vec): (usize, Vec<ControlCenter>) =
            Vec::<ControlCenter>::try_deserialize(&value[offset..])?;
        offset += increment;
        let (increment, metal_extractor_vec): (usize, Vec<MetalExtractor>) =
            Vec::<MetalExtractor>::try_deserialize(&value[offset..])?;
        offset += increment;
        let (increment, oil_extractor_vec): (usize, Vec<OilExtractor>) =
            Vec::<OilExtractor>::try_deserialize(&value[offset..])?;
        offset += increment;

        Ok((
            offset,
            FacilityCollection {
                control_center_vec,
                metal_extractor_vec,
                oil_extractor_vec,
            },
        ))
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
                FacilityState::default().to_bytes().len()
            )
        }

        #[test]
        fn facility_collection() {
            assert_eq!(6, FacilityCollection::default().to_bytes().len());
        }
    }
}
