use crate::error::AppErrorStatic;
use crate::facility::{Facility, FacilityState, FacilityTrait};
use crate::map::HexCoord;
use crate::sync::{SyncBytes, SyncTrait};

#[derive(Debug, Default, Copy, Clone)]
pub struct ControlCenter {
    pub location: HexCoord,
    pub state: FacilityState,
}

impl FacilityTrait for ControlCenter {
    fn location(&self) -> HexCoord {
        self.location
    }

    fn state(&self) -> FacilityState {
        self.state
    }

    fn facility<'a>(&'a self) -> Facility<'a> {
        Facility::ControlCenter(self)
    }
}

impl SyncTrait for ControlCenter {
    const SYNC_FIXED_SIZE: Option<usize> =
        Some(HexCoord::SYNC_FIXED_SIZE.unwrap() + FacilityState::SYNC_FIXED_SIZE.unwrap());
}

impl From<ControlCenter> for SyncBytes {
    fn from(value: ControlCenter) -> Self {
        let mut out: Vec<u8> = Vec::with_capacity(ControlCenter::SYNC_FIXED_SIZE.unwrap());
        out.extend_from_slice(SyncBytes::from(value.location).as_slice());
        out.extend_from_slice(SyncBytes::from(value.state).as_slice());
        SyncBytes::new(out)
    }
}

impl TryFrom<SyncBytes> for ControlCenter {
    type Error = AppErrorStatic;

    fn try_from(value: SyncBytes) -> Result<Self, Self::Error> {
        check_sync_fixed_size!(value);

        let pivot: usize = HexCoord::SYNC_FIXED_SIZE.unwrap();
        let location: HexCoord = HexCoord::try_from(SyncBytes::from(&value[0..pivot]))?;
        let state: FacilityState = FacilityState::try_from(value[pivot])?;

        Ok(Self { location, state })
    }
}

impl ControlCenter {
    pub const INFLUENCE_RADIUS_STEP: i16 = 4;

    pub fn within_influence(&self, hex_coord: HexCoord) -> bool {
        self.location.step_distance_le(hex_coord, Self::INFLUENCE_RADIUS_STEP)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_size() {
        assert_eq!(
            ControlCenter::SYNC_FIXED_SIZE.unwrap(),
            SyncBytes::from(ControlCenter::default()).len()
        )
    }
}
