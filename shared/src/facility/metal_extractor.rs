use crate::facility::{Facility, FacilityState, FacilityTrait};
use crate::map::HexCoord;
use crate::sync::SyncTrait;

#[derive(Debug, Default, Copy, Clone)]
pub struct MetalExtractor {
    pub location: HexCoord,
    pub state: FacilityState,
}

impl SyncTrait for MetalExtractor {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_size() {
        assert_eq!(
            MetalExtractor::default().fixed_size().unwrap(),
            MetalExtractor::default().as_bytes().len()
        )
    }
}
