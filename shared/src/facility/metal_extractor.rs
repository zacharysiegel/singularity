use crate::error::AppErrorStatic;
use crate::facility::{ControlCenter, Facility, FacilityState, FacilityTrait};
use crate::map::HexCoord;
use crate::sync::SyncTrait;

#[derive(Debug, Default, Copy, Clone)]
pub struct MetalExtractor {
    pub location: HexCoord,
    pub state: FacilityState,
}

impl SyncTrait for MetalExtractor {
    const SYNC_FIXED_SIZE: Option<usize> =
        Some(HexCoord::SYNC_FIXED_SIZE.unwrap() + FacilityState::SYNC_FIXED_SIZE.unwrap());

    fn to_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::with_capacity(ControlCenter::SYNC_FIXED_SIZE.unwrap());
        out.extend(self.location.to_bytes());
        out.extend(self.state.to_bytes());
        out
    }

    fn try_deserialize(bytes: &[u8]) -> Result<(usize, Self), AppErrorStatic> {
        check_sync_fixed_size!(bytes);

        let pivot: usize = HexCoord::SYNC_FIXED_SIZE.unwrap();
        let (_, location): (usize, HexCoord) = HexCoord::try_deserialize(&bytes[0..pivot])?;
        let state: FacilityState = FacilityState::try_from(bytes[pivot])?;

        Ok((Self::SYNC_FIXED_SIZE.unwrap(), Self { location, state }))
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
            MetalExtractor::default().to_bytes().len()
        )
    }
}
