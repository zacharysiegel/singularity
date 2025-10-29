use crate::error::AppErrorStatic;
use crate::facility::{Facility, FacilityState, FacilityTrait};
use crate::map::HexCoord;
use crate::sync::{SyncBytes, SyncTrait};

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
    const SYNC_FIXED_SIZE: Option<usize> =
        Some(HexCoord::SYNC_FIXED_SIZE.unwrap() + FacilityState::SYNC_FIXED_SIZE.unwrap());

    fn try_deserialize(bytes: &[u8]) -> Result<(usize, Self), AppErrorStatic> {
        check_sync_fixed_size!(bytes);

        let pivot: usize = HexCoord::SYNC_FIXED_SIZE.unwrap();
        let (_, location): (usize, HexCoord) = HexCoord::try_deserialize(&bytes[0..pivot])?;
        let state: FacilityState = FacilityState::try_from(bytes[pivot])?;

        Ok((Self::SYNC_FIXED_SIZE.unwrap(), Self { location, state }))
    }
}

impl From<OilExtractor> for SyncBytes {
    fn from(value: OilExtractor) -> Self {
        let mut out = Vec::with_capacity(OilExtractor::SYNC_FIXED_SIZE.unwrap());
        out.extend_from_slice(SyncBytes::from(value.location).as_slice());
        out.extend_from_slice(SyncBytes::from(value.state).as_slice());
        SyncBytes::new(out)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_size() {
        assert_eq!(
            OilExtractor::SYNC_FIXED_SIZE.unwrap(),
            SyncBytes::from(OilExtractor::default()).len()
        )
    }
}
