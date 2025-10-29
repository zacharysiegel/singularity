use crate::error::AppErrorStatic;
use crate::facility::{Facility, FacilityState, FacilityTrait};
use crate::map::HexCoord;
use crate::sync::{SyncBytes, SyncTrait};

#[derive(Debug, Default, Copy, Clone)]
pub struct MetalExtractor {
    pub location: HexCoord,
    pub state: FacilityState,
}

impl SyncTrait for MetalExtractor {
    const SYNC_FIXED_SIZE: Option<usize> =
        Some(HexCoord::SYNC_FIXED_SIZE.unwrap() + FacilityState::SYNC_FIXED_SIZE.unwrap());
}

impl From<MetalExtractor> for SyncBytes {
    fn from(value: MetalExtractor) -> Self {
        let mut out: Vec<u8> = Vec::with_capacity(MetalExtractor::SYNC_FIXED_SIZE.unwrap());
        out.extend_from_slice(SyncBytes::from(value.location).as_slice());
        out.extend_from_slice(SyncBytes::from(value.state).as_slice());
        SyncBytes::new(out)
    }
}

impl TryFrom<SyncBytes> for MetalExtractor {
    type Error = AppErrorStatic;

    fn try_from(value: SyncBytes) -> Result<Self, Self::Error> {
        check_sync_fixed_size!(value);

        let pivot: usize = HexCoord::SYNC_FIXED_SIZE.unwrap();
        let location: HexCoord = HexCoord::try_from(SyncBytes::from(&value[0..pivot]))?;
        let state: FacilityState = FacilityState::try_from(value[pivot])?;

        Ok(Self { location, state })
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
            MetalExtractor::SYNC_FIXED_SIZE.unwrap(),
            SyncBytes::from(MetalExtractor::default()).len()
        )
    }
}
