use crate::error::AppErrorStatic;
use crate::map::{HexCoord, ResourceType};
use crate::sync::{SyncBytes, SyncTrait};

#[derive(Debug, Copy, Clone)]
pub struct Hex {
    pub hex_coord: HexCoord,
    pub resource_type: ResourceType,
}

impl Default for Hex {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl SyncTrait for Hex {
    const SYNC_FIXED_SIZE: Option<usize> =
        Some(HexCoord::SYNC_FIXED_SIZE.unwrap() + ResourceType::SYNC_FIXED_SIZE.unwrap());
}

impl From<Hex> for SyncBytes {
    fn from(value: Hex) -> Self {
        let mut out: Vec<u8> = Vec::with_capacity(Hex::SYNC_FIXED_SIZE.unwrap());
        out.extend_from_slice(SyncBytes::from(value.hex_coord).as_slice());
        out.extend_from_slice(SyncBytes::from(value.resource_type).as_slice());
        SyncBytes::new(out)
    }
}

impl TryFrom<SyncBytes> for Hex {
    type Error = AppErrorStatic;

    fn try_from(value: SyncBytes) -> Result<Self, Self::Error> {
        if value.len() != Hex::SYNC_FIXED_SIZE.unwrap() {
            // todo: set number from fixed_size (FIXED_SIZE const)
            return Err(AppErrorStatic::new("invalid size"));
        }

        let hex_coord: HexCoord = HexCoord::try_from(SyncBytes::from(&value[0..4]))?;
        let resource_type: ResourceType = ResourceType::try_from(SyncBytes::from(value[5]))?;

        Ok(Hex {
            hex_coord,
            resource_type,
        })
    }
}

impl Hex {
    pub const DEFAULT: Hex = Hex {
        hex_coord: HexCoord::DEFAULT,
        resource_type: ResourceType::DEFAULT,
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_size() {
        assert_eq!(Hex::SYNC_FIXED_SIZE.unwrap(), SyncBytes::from(Hex::default()).len())
    }
}
