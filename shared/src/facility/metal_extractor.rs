use crate::facility::{Facility, FacilityState, FacilityTrait};
use crate::map::HexCoord;
use crate::sync::{SyncBytes, SyncTrait};

#[derive(Debug, Default, Copy, Clone)]
pub struct MetalExtractor {
    pub location: HexCoord,
    pub state: FacilityState,
}

impl SyncTrait for MetalExtractor {
    fn fixed_size(&self) -> Option<usize> {
        Some(self.location.fixed_size()? + self.state.fixed_size()?)
    }
}

impl From<MetalExtractor> for SyncBytes {
    fn from(value: MetalExtractor) -> Self {
        let mut out: Vec<u8> = Vec::with_capacity(value.fixed_size().unwrap());
        out.extend_from_slice(SyncBytes::from(value.location).as_slice());
        out.extend_from_slice(SyncBytes::from(value.state).as_slice());
        SyncBytes::new(out)
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
            SyncBytes::from(MetalExtractor::default()).len()
        )
    }
}
