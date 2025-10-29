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

    fn try_deserialize(bytes: &[u8]) -> Result<(usize, Self), AppErrorStatic> {
        check_sync_fixed_size!(bytes);

        let pivot: usize = size_of::<HexCoord>();
        let (_, hex_coord): (usize, HexCoord) = HexCoord::try_deserialize(&bytes[0..pivot])?;
        let (_, resource_type): (usize, ResourceType) = ResourceType::try_deserialize(&bytes[pivot..])?;

        Ok((
            Self::SYNC_FIXED_SIZE.unwrap(),
            Hex {
                hex_coord,
                resource_type,
            },
        ))
    }
}

impl From<Hex> for SyncBytes {
    fn from(value: Hex) -> Self {
        let mut out: Vec<u8> = Vec::with_capacity(Hex::SYNC_FIXED_SIZE.unwrap());
        out.extend_from_slice(SyncBytes::from(value.hex_coord).as_slice());
        out.extend_from_slice(SyncBytes::from(value.resource_type).as_slice());
        SyncBytes::new(out)
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
