use crate::facility::{Facility, FacilityState, FacilityTrait};
use crate::map::HexCoord;
use crate::sync::SyncTrait;

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

impl SyncTrait for OilExtractor {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_size() {
        assert_eq!(
            OilExtractor::default().fixed_size().unwrap(),
            OilExtractor::default().as_bytes().len()
        )
    }
}
